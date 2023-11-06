"use client";

import {
  Player,
  PlayerSidebar,
} from "@/components/views/players_sidebar/PlayersSidebar";
import { SelectStrategyCardView } from "@/components/views/strategy_card_select/SelectStrategyCard";
import { StrategyCard } from "@/resources/types/strategyCards";
import { GameState, PlayerId } from "@/api/Game";
import { ActionPhaseView } from "@/components/views/action_phase_View/ActionPhaseView";
import { useEffect, useRef, useState } from "react";
import { GameOptions } from "@/api/GameOptions";
import useWebSocket from "react-use-websocket";
import { SetupPhase } from "@/components/views/setup/SetupPhase";
import styles from "./styles.module.scss";
import { StrategyCardView } from "@/components/views/strategy_card_view/StrategyCardView";
import { StatusPhaseView } from "@/components/views/status_phase_view/StatusPhaseView";
import { TacticalView } from "@/components/views/tactical_view/TacticalView";

const NEW_GAME_ID = "new";

export default function Game({ params }: { params: { gameId: string } }) {
  const [gameOptions, setGameOptions] = useState<GameOptions | null>(null);
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [gameId, setGameId] = useState<string | null>(null);

  const { sendMessage, lastMessage, readyState } = useWebSocket(
    "ws://localhost:5555"
  );

  const initialized = useRef(false);

  useEffect(() => {
    if (lastMessage !== null) {
      console.log("MESSAGE", lastMessage);
      const data = JSON.parse(lastMessage.data);

      const joinedGameId = data["JoinedGame"];
      if (joinedGameId && !gameId) {
        setGameId(joinedGameId);
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
  }, [lastMessage, gameOptions]);

  const sendMsg = (data: any) => sendMessage(JSON.stringify(data));
  const sendEvent = (data: any) => sendMsg({ Event: data });

  useEffect(() => {
    if (!initialized.current) {
      initialized.current = true;
      if (params.gameId === NEW_GAME_ID) {
        sendMessage('"NewGame"');
      } else {
        sendMessage(
          JSON.stringify({
            JoinGame: params.gameId,
          })
        );
      }
    }
  }, [params]);

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
}

const PhaseView = ({
  gameState,
  gameOptions,
  sendMessage,
}: {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}) => {
  switch (gameState.phase) {
    case "Setup":
      return (
        <SetupPhase
          gameOptions={gameOptions}
          gameState={gameState}
          sendMessage={sendMessage}
        />
      );
    case "Strategy":
      return (
        <SelectStrategyCardView
          selectedCards={Object.entries(gameState.strategyCardHolders).map(
            ([strategyCard, playerId]) => {
              return {
                card: strategyCard as StrategyCard,
                faction: gameState.players[playerId].faction,
              };
            }
          )}
          expectedStrategyCards={getExpectedStrategyCards(
            Object.keys(gameState.players).length
          )}
          selectCard={(card) => {
            sendMessage({
              TakeStrategyCard: {
                player: gameState.currentPlayer,
                card: card,
              },
            });
          }}
          startActionPhase={() => sendMessage("CompleteStrategyPhase")}
        />
      );
    case "Action":
      return (
        <ActionPhaseView
          gameState={gameState}
          systems={gameOptions.systems}
          sendMessage={sendMessage}
        />
      );
    case "StrategicAction":
      return (
        <StrategyCardView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      );
    case "TacticalAction":
      return (
        <TacticalView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      );
    case "Status":
      return (
        <StatusPhaseView gameOptions={gameOptions} sendMessage={sendMessage} />
      );
    default:
      return <div>PHASE NOT YET IMPLEMENTED</div>;
  }
};

function getPlayersFromGame(
  gameState: GameState,
  gameOptions: GameOptions
): Player[] {
  return getPlayerOrder(gameState).map((id) => {
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

// TODO: Look over these
function getPlayerOrder(gameState: GameState): PlayerId[] {
  const phase = gameState.phase;
  switch (phase) {
    case "Setup":
      return Object.keys(gameState.players);
    case "Strategy":
      return gameState.tableOrder;
    default:
      return gameState.turnOrder;
  }
}

function getExpectedStrategyCards(noPlayers: number): number {
  if (noPlayers > 4) {
    return 1 * noPlayers;
  }

  return 2 * noPlayers;
}
