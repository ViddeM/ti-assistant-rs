"use client";

import {
  Player,
  PlayerSidebar,
} from "@/components/views/players_sidebar/PlayersSidebar";
import { SelectStrategyCardView } from "@/components/views/strategy_card_select/SelectStrategyCard";
import { StrategyCard } from "@/resources/types/strategyCards";
import { Game as ApiGame, GameState } from "@/api/Game";
import { ActionPhaseView } from "@/components/views/action_phase_View/ActionPhaseView";
import { useEffect, useState } from "react";
import { GameOptions } from "@/api/GameOptions";
import useWebSocket from "react-use-websocket";
import { SetupPhase } from "@/components/views/setup/SetupPhase";

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
      if (gs && !gameState) {
        setGameState(gs as GameState);
      }
    }
  }, [lastMessage, gameOptions]);

  console.log("GameState", gameState);

  return (
    <>
      {gameOptions && gameState && (
        <SetupPhase
          gameOptions={gameOptions}
          gameState={gameState}
          sendMessage={sendMessage}
        />
      )}
      {/* <div className={styles.gamePageContainer}>
        <PlayerSidebar players={sidebarPlayers} />
        <PhaseView {...resp.data} />
        <div />
      </div> */}
    </>
  );
}

const PhaseView = ({ gameState, systems }: ApiGame) => {
  switch (gameState.phase) {
    case "Strategy":
      return (
        <SelectStrategyCardView
          selectedCards={[]}
          expectedStrategyCards={2}
          selectCard={() => {}}
        />
      );
    case "Action":
      return <ActionPhaseView gameState={gameState} systems={systems} />;
    default:
      return <div>PHASE NOT YET IMPLEMENTED</div>;
  }
};

function getPlayersFromGame(gameState: GameState): Player[] {
  return Object.entries(gameState.players).map(([id, p]) => {
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
          return {
            name: stratCard,
            played: played,
          };
        }),
    };
  });
}
