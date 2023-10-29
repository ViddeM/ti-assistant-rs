"use client";

import { GameState, System } from "@/api/Game";
import { Button } from "@/components/elements/button/Button";
import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./ActionPhaseView.module.scss";
import { useState } from "react";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";

type SubAction = "Tactical" | "Component" | "StrategyCard";

export interface ActionPhaseViewProps {
  gameState: GameState;
  systems: System[];
  sendMessage: (data: any) => void;
}

export const ActionPhaseView = ({
  gameState,
  systems,
  sendMessage,
}: ActionPhaseViewProps) => {
  const [subAction, setSubAction] = useState<SubAction | null>(null);

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
            // onClick={() => setSubAction("StrategyCard")}

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
          onClick={() => setSubAction("Tactical")}
        >
          Tactical
        </Button>
        <Button
          className={styles.actionButton}
          onClick={() => setSubAction("Component")}
        >
          Component
        </Button>
      </div>
      {subAction && (
        <>
          <div className={styles.actionOptions}>
            {subAction === "Tactical" ? (
              <>
                <div>
                  <label>Take control of planet</label>
                  <Dropdown className={styles.actionButton}>
                    {systems
                      .flatMap((s) => s.planets)
                      .map((s) => (
                        <option key={s}>{s}</option>
                      ))}
                  </Dropdown>
                  <Button>Take</Button>
                </div>
                <Button className={styles.actionButton}>
                  Score action phase objective
                </Button>
              </>
            ) : (
              <p>{subAction} not implemented</p>
            )}
          </div>
        </>
      )}
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
