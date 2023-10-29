"use client";

import {
  Player,
  PlayerSidebar,
} from "@/components/views/players_sidebar/PlayersSidebar";
import { SelectStrategyCardView } from "@/components/views/strategy_card_select/SelectStrategyCard";
import { StrategyCard } from "@/resources/types/strategyCards";
import { GameState, PlayerId } from "@/api/Game";
import { ActionPhaseView } from "@/components/views/action_phase_View/ActionPhaseView";
import { useEffect, useState } from "react";
import { GameOptions } from "@/api/GameOptions";
import useWebSocket from "react-use-websocket";
import { SetupPhase } from "@/components/views/setup/SetupPhase";
import styles from "./styles.module.scss";
import { StrategyCardView } from "@/components/views/strategy_card_view/StrategyCardView";

export default function Game() {
  const [gameOptions, setGameOptions] = useState<GameOptions | null>(null);
  const [gameState, setGameState] = useState<GameState | null>(null);

  const { sendMessage, lastMessage, readyState } = useWebSocket(
    "ws://localhost:5555"
  );

  useEffect(() => {
    if (lastMessage !== null) {
      console.log("MESSAGE", lastMessage);
      const data = JSON.parse(lastMessage.data);

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

  return (
    <>
      {gameOptions && gameState && (
        <div className={styles.gamePageContainer}>
          <PlayerSidebar players={getPlayersFromGame(gameState)} />
          <PhaseView
            gameState={gameState}
            gameOptions={gameOptions}
            sendMessage={sendMsg}
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
        <StrategyCardView gameState={gameState} sendMessage={sendMessage} />
      );
    default:
      return <div>PHASE NOT YET IMPLEMENTED</div>;
  }
};

function getPlayersFromGame(gameState: GameState): Player[] {
  return getPlayerOrder(gameState).map((id) => {
    const p = gameState.players[id];
    return {
      name: p.name,
      faction: p.faction,
      color: "#000",
      isActive: gameState.currentPlayer === p.name,
      hasPassed: gameState.passedPlayers.includes(id),
      cards: Object.entries(gameState.strategyCardHolders)
        .filter(([_, playerId]) => id === playerId)
        .map(([card]) => {
          const stratCard = card as StrategyCard;
          const played = gameState.spentStrategyCards.includes(stratCard);
          const isActive = gameState.strategicAction?.card === card;
          return {
            name: stratCard,
            played: played,
            isActive: isActive,
          };
        }),
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
