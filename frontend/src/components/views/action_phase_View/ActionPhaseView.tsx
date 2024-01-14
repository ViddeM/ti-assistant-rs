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
          disabled={isComponent}
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

type ComponentMode = "ACTION_CARD" | "RELICS" | "FRONTIER_CARD" | "";

const ComponentSelectRow = () => {
  const { gameOptions, gameState } = useGameContext();

  const [componentMode, setComponentMode] = useState<ComponentMode>("");

  useEffect(() => setComponentMode(""), [gameState]);

  return (
    <>
      <div className={styles.componentSelectContainer}>
        <Button
          disabled={componentMode === "ACTION_CARD"}
          onClick={() => setComponentMode("ACTION_CARD")}
        >
          Action Card
        </Button>
        <Button
          disabled={
            componentMode === "RELICS" ||
            Object.keys(gameOptions.relics).length === 0
          }
          onClick={() => setComponentMode("RELICS")}
        >
          Relics
        </Button>
        <Button
          disabled={
            componentMode === "FRONTIER_CARD" ||
            Object.keys(gameOptions.frontierCards).length === 0
          }
          onClick={() => setComponentMode("FRONTIER_CARD")}
        >
          Frontier Card
        </Button>
      </div>
      {componentMode !== "" && <DisplayComponentMode mode={componentMode} />}
    </>
  );
};

interface DisplayComponentModeProps {
  mode: ComponentMode;
}

const DisplayComponentMode = ({ mode }: DisplayComponentModeProps) => {
  switch (mode) {
    case "ACTION_CARD":
      return <ActionCardSelectView />;
    case "RELICS":
      return <RelicCardView />;
    case "FRONTIER_CARD":
      return <FrontierCardView />;
    default:
      return <p>Invalid display mode {mode}</p>;
  }
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
        disabled={card === ""}
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

const RelicCardView = () => {
  const { gameOptions } = useGameContext();

  const [selected, setSelected] = useState<string>("");

  const availableRelics = Object.values(gameOptions.relics).filter(
    (r) => r.play === "Action"
  );

  return (
    <div>
      <fieldset className={styles.playActionCardContainer}>
        <legend>Play Relic</legend>
        <Dropdown>
          <option value="">--Select relic--</option>
          {availableRelics.map((r) => (
            <option key={r.card} value={r.card}>
              {r.name}
            </option>
          ))}
        </Dropdown>
        <Button>Play</Button>
      </fieldset>
    </div>
  );
};

const FrontierCardView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [selected, setSelected] = useState<string>("");

  const availableCards = Object.values(gameOptions.frontierCards).filter(
    (f) => f.frontierType === "Action"
  );

  return (
    <div>
      <fieldset className={styles.playActionCardContainer}>
        <legend>Play Frontier Card</legend>
        <Dropdown
          value={selected}
          onChange={(e) => setSelected(e.target.value)}
        >
          <option>--Select frontier card--</option>
          {availableCards.map((f) => (
            <option key={f.card} value={f.card}>
              {f.name}
            </option>
          ))}
        </Dropdown>
        <Button
          disabled={selected === ""}
          onClick={() =>
            sendEvent({
              FrontierCardBegin: {
                player: gameState.currentPlayer,
                card: selected,
              },
            })
          }
        >
          Play
        </Button>
      </fieldset>
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
