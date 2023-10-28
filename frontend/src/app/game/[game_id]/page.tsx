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

export default function Game() {
  const [gameOptions, setGameOptions] = useState<GameOptions | null>(null);

  const { sendMessage, lastMessage, readyState } = useWebSocket(
    "ws://localhost:5555"
  );

  useEffect(() => {
    if (lastMessage !== null) {
      console.log("MESSAGE", lastMessage);
    }
  }, [lastMessage]);

  // useEffect(() => {
  //   const ws = new WebSocket("ws://localhost:5555");
  //   ws.onopen = () => {
  //     console.log("Connected to websocket");
  //   };
  //   ws.onmessage = (event) => {
  //     console.log("Received ", event.data);
  //     const data = event.data;
  //     if (data.GameOptions) {
  //       setGameOptions(data.GameOptions as GameOptions);
  //     }
  //   };
  //   ws.onclose = () => {
  //     console.log("Disconnected from websocket");
  //   };
  //   return () => {
  //     ws.close();
  //   };
  // }, []);

  console.log("GAME OPTIONS", gameOptions);

  return (
    <>
      <div />
      {gameOptions && <div>{gameOptions.maxScore}</div>}
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
