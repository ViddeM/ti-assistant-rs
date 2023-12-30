"use client";

import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { useEffect, useRef, useState } from "react";
import useWebSocket from "react-use-websocket";
import {
  PlayerSidebar,
  SidebarPlayer,
} from "../players_sidebar/PlayersSidebar";
import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./GameView.module.scss";
import { useRouter } from "next/navigation";
import { PhaseView } from "../phase_view/PhaseView";
import { Button } from "@/components/elements/button/Button";
import { ScoreViewMode } from "../score_view_mode/ScoreViewMode";
import { TechViewMode } from "../tech_view_mode/TechViewMode";
import { PlanetViewMode } from "../planet_view_mode/PlanetViewMode";
import { LawsViewMode } from "../laws_view_mode/LawsViewMode";

const NEW_GAME_ID = "new";

interface GameViewProps {
  wsUri: string;
  gameId: string;
}

type View = "Game" | "Score" | "Planets" | "Techs" | "Laws";

export const GameView = ({ gameId, wsUri }: GameViewProps) => {
  const [gameOptions, setGameOptions] = useState<GameOptions | null>(null);
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [currentViewMode, setCurrentViewMode] = useState<View>("Game");

  const router = useRouter();

  const { sendMessage, lastMessage, readyState } = useWebSocket(wsUri);

  const initialized = useRef(false);

  /* General message handling */
  useEffect(() => {
    if (lastMessage !== null) {
      console.log("MESSAGE", lastMessage);
      const data = JSON.parse(lastMessage.data);

      const joinedGameId = data["JoinedGame"];
      if (joinedGameId && gameId !== joinedGameId) {
        router.replace(`/game/${joinedGameId}`);
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
      <div className={`card ${styles.gameInfoCard}`}>
        <h4>Game: {gameId}</h4>
        <div className={styles.viewModeButtonGroup}>
          <Button
            onClick={() => setCurrentViewMode("Game")}
            disabled={currentViewMode === "Game"}
          >
            Game
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Score")}
            disabled={currentViewMode === "Score"}
          >
            Score
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Techs")}
            disabled={currentViewMode === "Techs"}
          >
            Techs
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Planets")}
            disabled={currentViewMode === "Planets"}
          >
            Planets
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Laws")}
            disabled={currentViewMode === "Laws"}
          >
            Laws
          </Button>
        </div>
        <p>Round {gameState?.round}</p>
      </div>
      {gameOptions && gameState && (
        <DisplayViewMode
          viewMode={currentViewMode}
          gameOptions={gameOptions}
          gameState={gameState}
          sendEvent={sendEvent}
        />
      )}
    </>
  );
};

interface DisplayViewModeProps {
  viewMode: View;
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

const DisplayViewMode = ({
  viewMode,
  gameOptions,
  gameState,
  sendEvent,
}: DisplayViewModeProps) => {
  switch (viewMode) {
    case "Game":
      return (
        <div className={styles.gamePageContainer}>
          <PlayerSidebar
            players={getPlayersFromGame(gameState, gameOptions)}
            score={gameState.score}
            gameOptions={gameOptions}
            currentTurnStartTime={gameState.currentTurnStartTime}
            isPaused={gameState.timeTrackingPaused}
          />
          <div className={styles.phaseContainer}>
            <PhaseView
              gameState={gameState}
              gameOptions={gameOptions}
              sendMessage={sendEvent}
            />
          </div>
        </div>
      );
    case "Score":
      return (
        <ScoreViewMode
          gameOptions={gameOptions}
          gameState={gameState}
          sendEvent={sendEvent}
        />
      );
    case "Techs":
      return (
        <TechViewMode
          gameOptions={gameOptions}
          gameState={gameState}
          sendEvent={sendEvent}
        />
      );
    case "Planets":
      return (
        <PlanetViewMode
          gameOptions={gameOptions}
          gameState={gameState}
          sendEvent={sendEvent}
        />
      );
    case "Laws":
      return (
        <LawsViewMode
          gameOptions={gameOptions}
          gameState={gameState}
          sendEvent={sendEvent}
        />
      );
    default:
      return <p>Unknown view mode ({viewMode})?</p>;
  }
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
      playTime: {
        secs: gameState.playersPlayTime[id]?.secs ?? 0,
        nanos: gameState.playersPlayTime[id]?.nanos ?? 0,
      },
    };
  });
}
