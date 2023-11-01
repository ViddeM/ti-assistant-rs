"use client";

import { GameState, System } from "@/api/Game";
import { Button } from "@/components/elements/button/Button";
import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./ActionPhaseView.module.scss";

export interface ActionPhaseViewProps {
  gameState: GameState;
  systems: System[];
  sendMessage: (data: any) => void;
}

export const ActionPhaseView = ({
  gameState,
  sendMessage,
}: ActionPhaseViewProps) => {
  const currentPlayer = gameState.currentPlayer as string | null;

  if (!currentPlayer) {
    return <div>Invalid state, currentPlayer is null in action phase!</div>;
  }

  const playableStrategyCards = getPlayableStrategyCards(
    gameState,
    currentPlayer
  );

  return (
    <div className={`card ${styles.actionPhaseViewContainer}`}>
      ACTION PHASE
      <div className={styles.actionsContainer}>
        <Button
          className={styles.actionButton}
          disabled={playableStrategyCards.length > 0}
          onClick={() =>
            sendMessage({
              PassAction: {
                player: gameState.currentPlayer,
              },
            })
          }
        >
          Pass
        </Button>
        {playableStrategyCards.map((c) => (
          <Button
            key={c}
            className={styles.actionButton}
            onClick={() =>
              sendMessage({
                StrategicActionBegin: {
                  card: c,
                  player: currentPlayer,
                },
              })
            }
          >
            {c}
          </Button>
        ))}
        <Button
          className={styles.actionButton}
          onClick={() => sendMessage("TacticalAction")}
        >
          Tactical
        </Button>
      </div>
    </div>
  );
};

function getPlayableStrategyCards(
  gameState: GameState,
  currentPlayer: string
): StrategyCard[] {
  return Object.entries(gameState.strategyCardHolders)
    .map(([strategyCard, player]) => {
      return {
        card: strategyCard,
        player: player,
      };
    })
    .filter(
      (v) => !gameState.spentStrategyCards.includes(v.card as StrategyCard)
    )
    .filter((v) => v.player === `${currentPlayer}`)
    .map((v) => v.card as StrategyCard);
}
