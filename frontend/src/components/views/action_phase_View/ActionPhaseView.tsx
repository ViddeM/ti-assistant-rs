"use client";

import { Button } from "@/components/elements/button/Button";
import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./ActionPhaseView.module.scss";
import { GameState } from "@/api/GameState";
import { useEffect, useState } from "react";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useGameContext } from "@/hooks/GameContext";

export const ActionPhaseView = () => {
  const { gameState, sendEvent } = useGameContext();

  const [isComponent, setIsComponent] = useState<boolean>(false);

  useEffect(() => setIsComponent(false), [gameState]);

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
            sendEvent({
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
              sendEvent({
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
          onClick={() =>
            sendEvent({
              TacticalActionBegin: {
                player: gameState.currentPlayer,
              },
            })
          }
        >
          Tactical
        </Button>
        <Button
          className={styles.actionButton}
          onClick={() => setIsComponent(true)}
        >
          Component
        </Button>
      </div>
      {isComponent && <ComponentSelectRow />}
    </div>
  );
};

type ComponentMode = "ACTION_CARD" | "";

const ComponentSelectRow = () => {
  const { gameState } = useGameContext();

  const [componentMode, setComponentMode] = useState<ComponentMode>("");

  useEffect(() => setComponentMode(""), [gameState]);

  return (
    <>
      <div>
        <Button
          disabled={componentMode === "ACTION_CARD"}
          onClick={() => setComponentMode("ACTION_CARD")}
        >
          Action Card
        </Button>
      </div>
      {componentMode === "ACTION_CARD" && <ActionCardSelectView />}
    </>
  );
};

const ActionCardSelectView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [card, setCard] = useState<string>("");

  useEffect(() => setCard(""), [gameState]);

  const playableActionCards = Object.keys(gameOptions.actionCards)
    .map((card) => gameOptions.actionCards[card])
    .filter((card) => {
      return card.play === "Action";
    });

  return (
    <fieldset className={styles.playActionCardContainer}>
      <legend>Play Action Card</legend>
      <Dropdown value={card} onChange={(e) => setCard(e.target.value)}>
        <option value={""}>--Select an Action Card--</option>
        {playableActionCards.map((card) => (
          <option key={card.card} value={card.card}>
            {card.name}
          </option>
        ))}
      </Dropdown>
      <Button
        onClick={() =>
          sendEvent({
            ActionCardActionBegin: {
              player: gameState.currentPlayer,
              card: card,
            },
          })
        }
      >
        Play
      </Button>
    </fieldset>
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
