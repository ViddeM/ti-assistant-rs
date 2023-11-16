"use client";

import { Button } from "@/components/elements/button/Button";
import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./ActionPhaseView.module.scss";
import { GameState } from "@/api/GameState";
import { System } from "@/api/System";
import { useEffect, useState } from "react";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { GameOptions } from "@/api/GameOptions";

export interface ActionPhaseViewProps {
  gameOptions: GameOptions;
  gameState: GameState;
  systems: System[];
  sendMessage: (data: any) => void;
}

export const ActionPhaseView = ({
  gameOptions,
  gameState,
  sendMessage,
}: ActionPhaseViewProps) => {
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
          onClick={() =>
            sendMessage({
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
      {isComponent && (
        <ComponentSelectRow
          gameState={gameState}
          sendMessage={sendMessage}
          gameOptions={gameOptions}
        />
      )}
    </div>
  );
};

type ComponentMode = "ACTION_CARD" | "";

interface ComponentSelectRowProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

const ComponentSelectRow = ({
  gameState,
  gameOptions,
  sendMessage,
}: ComponentSelectRowProps) => {
  const [componentMode, setComponentMode] = useState<ComponentMode>("");

  useEffect(() => setComponentMode(""), [gameState]);

  return (
    <>
      <div>
        <Button onClick={() => setComponentMode("ACTION_CARD")}>
          Action Card
        </Button>
      </div>
      {componentMode === "ACTION_CARD" && (
        <ActionCardSelectView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      )}
    </>
  );
};

interface ActionCardSelectViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

const ActionCardSelectView = ({
  gameState,
  gameOptions,
  sendMessage,
}: ActionCardSelectViewProps) => {
  const [card, setCard] = useState<string>("");

  useEffect(() => setCard(""), [gameState]);

  const playableActionCards = Object.keys(gameOptions.actionCards).filter(
    (card) => {
      return gameOptions.actionCards[card].play === "Action";
    }
  );

  return (
    <div>
      <Dropdown value={card} onChange={(e) => setCard(e.target.value)}>
        <option value={""}>--Select an Action Card--</option>
        {playableActionCards.map((card) => (
          <option key={card} value={card}>
            {card}
          </option>
        ))}
      </Dropdown>
      <Button
        onClick={() =>
          sendMessage({
            ActionCardActionBegin: {
              player: gameState.currentPlayer,
              card: card,
            },
          })
        }
      >
        Play
      </Button>
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
