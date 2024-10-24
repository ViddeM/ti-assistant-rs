"use client";

import { Button } from "@/components/elements/button/Button";
import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./ActionPhaseView.module.scss";
import { GameState } from "@/api/bindings/GameState";
import { ActionCard } from "@/api/bindings/ActionCard";
import { Leader } from "@/api/bindings/Leader";
import { useEffect, useState } from "react";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { InfoButton } from "@/components/elements/button/InfoButton";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort, stringSort } from "@/utils/Utils";

export const ActionPhaseView = () => {
  const { gameState, sendEvent, isActive } = useGameContext();

  const [isComponent, setIsComponent] = useState<boolean>(false);

  useEffect(() => setIsComponent(false), [gameState]);

  const currentPlayer = gameState.currentPlayer as string | null;

  if (!currentPlayer) {
    return <div>Invalid state, currentPlayer is null in action phase!</div>;
  }

  const playableStrategyCards = getPlayableStrategyCards(
    gameState,
    currentPlayer,
  );

  const playerName = `${currentPlayer} ${playerEmoji(currentPlayer)}`;

  return (
    <div className={`card ${styles.actionPhaseViewContainer}`}>
      <h2>ACTION PHASE</h2>
      {isActive ? (
        <fieldset
          className={`playerColorBorder${
            gameState.players[currentPlayer!!].color
          } ${styles.actionPlayerContainer}`}
        >
          <legend
            className={`playerColorBorder${
              gameState.players[currentPlayer!!].color
            } ${styles.actionPlayerContainer}`}
          >
            <h4>{playerName}</h4>
          </legend>
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
        </fieldset>
      ) : (
        <p>Not your turn, currently {currentPlayer} is playing</p>
      )}
    </div>
  );
};

type ComponentMode =
  | "ACTION_CARD"
  | "PLAY_RELIC"
  | "GAIN_RELIC"
  | "FRONTIER_CARD"
  | "PLAY_LEADER"
  | "";

const ComponentSelectRow = () => {
  const { gameOptions, gameState } = useGameContext();

  const [componentMode, setComponentMode] = useState<ComponentMode>("");

  useEffect(() => setComponentMode(""), [gameState]);

  const currentPlayer = gameState.currentPlayer;
  const availableLeaders = currentPlayer
    ? gameState.availableLeaders[currentPlayer]
    : [];
  const actionLeaders = availableLeaders.filter(
    (l) => gameOptions.leaders[l].kind === "Action",
  );
  const canPlayLeaders = actionLeaders.length > 0;

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
            componentMode === "GAIN_RELIC" ||
            Object.keys(gameOptions.relics).length === 0
          }
          onClick={() => setComponentMode("GAIN_RELIC")}
        >
          Gain Relic
        </Button>
        <Button
          disabled={
            componentMode === "PLAY_RELIC" ||
            Object.keys(gameOptions.relics).length === 0
          }
          onClick={() => setComponentMode("PLAY_RELIC")}
        >
          Play Relic
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
        <Button
          disabled={componentMode === "PLAY_LEADER" || !canPlayLeaders}
          onClick={() => setComponentMode("PLAY_LEADER")}
        >
          Play Leader
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
    case "GAIN_RELIC":
      return <GainRelicCardView />;
    case "PLAY_RELIC":
      return <PlayRelicCardView />;
    case "FRONTIER_CARD":
      return <FrontierCardView />;
    case "PLAY_LEADER":
      return <PlayLeaderView />;
    default:
      return <p>Invalid display mode {mode}</p>;
  }
};

const ActionCardSelectView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [card, setCard] = useState<ActionCard | "">("");

  useEffect(() => setCard(""), [gameState]);

  const playableActionCards = Object.keys(gameOptions.actionCards)
    .map((card) => gameOptions.actionCards[card as ActionCard])
    .filter((card) => {
      return card.play === "Action";
    })
    .sort(nameSort);

  return (
    <fieldset className={styles.playActionCardContainer}>
      <legend>Play Action Card</legend>
      <Dropdown
        value={card}
        onChange={(e) => setCard(e.target.value as ActionCard)}
      >
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

const GainRelicCardView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [selected, setSelected] = useState<string>("");

  const takenRelics = Object.values(gameState.players).flatMap((p) => p.relics);
  const availableRelics = Object.values(gameOptions.relics)
    .filter((r) => !takenRelics.includes(r.card))
    .sort(nameSort);

  return (
    <div>
      <fieldset className={styles.playActionCardContainer}>
        <legend>Gain Relic</legend>
        <Dropdown onChange={(e) => setSelected(e.target.value)}>
          <option value="">--Select relic--</option>
          {availableRelics.map((r) => (
            <option key={r.card} value={r.card}>
              {r.name}
            </option>
          ))}
        </Dropdown>
        <Button
          disabled={selected === ""}
          onClick={() =>
            sendEvent({
              GainRelicAction: {
                player: gameState.currentPlayer,
                relic: selected,
              },
            })
          }
        >
          Gain
        </Button>
      </fieldset>
    </div>
  );
};

const PlayRelicCardView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [selected, setSelected] = useState<string>("");

  const availableRelics = gameState.players[gameState.currentPlayer!!].relics
    .map((r) => gameOptions.relics[r])
    .filter((r) => r.play === "Action");

  return (
    <div>
      <fieldset className={styles.playActionCardContainer}>
        <legend>Play Relic</legend>
        <Dropdown
          onChange={(e) => setSelected(e.target.value)}
          disabled={availableRelics.length === 0}
        >
          {availableRelics.length === 0 ? (
            <option>--No relics available--</option>
          ) : (
            <>
              <option value="">--Select relic--</option>
              {availableRelics.map((r) => (
                <option key={r.card} value={r.card}>
                  {r.name}
                </option>
              ))}
            </>
          )}
        </Dropdown>
        <Button
          disabled={selected === ""}
          onClick={() =>
            sendEvent({
              RelicActionBegin: {
                player: gameState.currentPlayer,
                relic: selected,
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

const FrontierCardView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [selected, setSelected] = useState<string>("");

  const availableCards = Object.values(gameOptions.frontierCards)
    .filter((f) => f.frontierType === "Action")
    .sort(nameSort);

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
              FrontierCardActionBegin: {
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

const PlayLeaderView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [selected, setSelected] = useState<Leader | "">("");

  const currentPlayer = gameState.currentPlayer;
  const availableLeaders = currentPlayer
    ? gameState.availableLeaders[currentPlayer]
    : [];
  const actionLeaders = availableLeaders
    .map((l) => ({ id: l, info: gameOptions.leaders[l] }))
    .filter((leader) => leader.info.kind === "Action");

  return (
    <div>
      <fieldset className={styles.playActionCardContainer}>
        <legend>Play Leader</legend>
        <table>
          {actionLeaders.map((leader) => (
            <tr key={leader.id}>
              <Button
                onClick={() =>
                  sendEvent({
                    LeaderActionBegin: {
                      player: gameState.currentPlayer,
                      leader: leader.id,
                    },
                  })
                }
              >
                {leader.info.name}
              </Button>
              <InfoButton info={{ Leader: leader.info }} />
            </tr>
          ))}
        </table>
      </fieldset>
    </div>
  );
};

function getPlayableStrategyCards(
  gameState: GameState,
  currentPlayer: string,
): StrategyCard[] {
  return Object.entries(gameState.strategyCardHolders)
    .map(([strategyCard, player]) => {
      return {
        card: strategyCard,
        player: player,
      };
    })
    .filter(
      (v) => !gameState.spentStrategyCards.includes(v.card as StrategyCard),
    )
    .filter((v) => v.player === `${currentPlayer}`)
    .map((v) => v.card as StrategyCard)
    .sort(stringSort);
}

function playerEmoji(name: string): string {
  switch (name.toLocaleLowerCase()) {
    case "portals":
      return "👑";
    case "potholes":
      return "❤️";
    case "tux":
      return "🐢";
    case "sponken":
      return "👺";
    case "vidde":
      return "🛸";
    case "swexbe":
      return "💥";
    case "håll":
      return "🧬";
    case "hoidi":
      return "🐦";
    case "gurr":
      return "⛴️️";
    default:
      return "";
  }
}
