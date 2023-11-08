"use client";

import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { useEffect, useRef, useState } from "react";
import useWebSocket from "react-use-websocket";
import {
  PlayerSidebar,
  SidebarPlayer,
} from "../players_sidebar/PlayersSidebar";
import { PhaseView } from "@/components/PhaseView/PhaseView";
import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./GameView.module.scss";
import { useRouter } from "next/navigation";

const NEW_GAME_ID = "new";

interface GameViewProps {
  wsUri: string;
  gameId: string;
}

export const GameView = ({ gameId, wsUri }: GameViewProps) => {
  const [gameOptions, setGameOptions] = useState<GameOptions | null>(null);
  const [gameState, setGameState] = useState<GameState | null>(null);

  const router = useRouter();

  console.log("Connecting to server on: ", wsUri);
  const { sendMessage, lastMessage, readyState } = useWebSocket(wsUri);

  const initialized = useRef(false);

  /* General message handling */
  useEffect(() => {
    if (lastMessage !== null) {
      console.log("MESSAGE", lastMessage);
      const data = JSON.parse(lastMessage.data);

      const joinedGameId = data["JoinedGame"];
      if (joinedGameId && gameId !== joinedGameId) {
        router.push(`/game/${joinedGameId}`);
      }

      const msgOpts = data["GameOptions"];
      if (msgOpts && !gameOptions) {
        setGameOptions(msgOpts as GameOptions);
      }

      const gs = data["GameState"];
      if (gs) {
        setGameState(gs as GameState);
      }
    }
  }, [lastMessage, gameOptions, gameId, router]);

  const sendMsg = (data: any) => sendMessage(JSON.stringify(data));
  const sendEvent = (data: any) => sendMsg({ Event: data });

  /* Send join game message */
  useEffect(() => {
    if (!initialized.current) {
      initialized.current = true;
      if (gameId === NEW_GAME_ID) {
        sendMessage('"NewGame"');
      } else {
        sendMessage(
          JSON.stringify({
            JoinGame: gameId,
          })
        );
      }
    }
  }, [gameId, sendMessage]);

  return (
    <>
      <div className="card">
        <h4>Game: {gameId}</h4>
      </div>
      {gameOptions && gameState && (
        <div className={styles.gamePageContainer}>
          <PlayerSidebar players={getPlayersFromGame(gameState, gameOptions)} />
          <PhaseView
            gameState={gameState}
            gameOptions={gameOptions}
            sendMessage={sendEvent}
          />
        </div>
      )}
    </>
  );
};

function getPlayersFromGame(
  gameState: GameState,
  gameOptions: GameOptions
): SidebarPlayer[] {
  return gameState.turnOrder.map((id) => {
    const p = gameState.players[id];
    return {
      name: p.name,
      faction: gameOptions.factions.filter((f) => f.faction === p.faction)[0],
      color: p.color,
      isActive: gameState.currentPlayer === p.name,
      hasPassed: gameState.passedPlayers.includes(id),
      cards: Object.entries(gameState.strategyCardHolders)
        .filter(([_, playerId]) => id === playerId)
        .map(([card]) => {
          const stratCard = card as StrategyCard;
          const played = gameState.spentStrategyCards.includes(stratCard);
          const isActive = gameState.actionProgress?.Strategic?.card === card;
          return {
            name: stratCard,
            played: played,
            isActive: isActive,
          };
        }),
      planets: p.planets.map((p) => {
        return {
          planet: p,
          info: gameOptions.planetInfos[p],
        };
      }),
      technologies: p.technologies.map((t) => {
        return {
          tech: t,
          info: gameOptions.technologies[t],
        };
      }),
      isSpeaker: gameState.speaker === p.name,
    };
  });
}
