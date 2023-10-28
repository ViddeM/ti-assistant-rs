import {
  Player,
  PlayerSidebar,
} from "@/components/views/players_sidebar/PlayersSidebar";
import { SelectStrategyCardView } from "@/components/views/strategy_card_select/SelectStrategyCard";
import styles from "./styles.module.scss";
import { StrategyCard } from "@/resources/types/strategyCards";
import { Api } from "@/api/Api";
import { Game, GameState } from "@/api/Game";
import { ActionPhaseView } from "@/components/views/action_phase_View/ActionPhaseView";
import { Suspense } from "react";

export default async function Game() {
  const resp = await Api.game.get_example();
  if (resp.error) {
    return <div>Error</div>;
  }

  if (!resp.data) {
    return <div>Failed to load data</div>;
  }

  const gameState = resp.data.current;
  const sidebarPlayers = getPlayersFromGame(gameState);

  return (
    <>
      <Suspense fallback={<div>Loading...</div>}>
        <div className={styles.gamePageContainer}>
          <PlayerSidebar players={sidebarPlayers} />
          <PhaseView {...gameState} />
          <div />
        </div>
      </Suspense>
    </>
  );
}

const PhaseView = (gameState: GameState) => {
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
      return <ActionPhaseView gameState={gameState} />;
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
