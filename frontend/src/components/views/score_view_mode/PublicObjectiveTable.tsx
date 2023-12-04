import { GameState } from "@/api/GameState";
import styles from "./ScoreViewMode.module.scss";
import { GameOptions } from "@/api/GameOptions";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Faction } from "@/resources/types/factions";
import React from "react";

interface PublicObjectiveTableProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const PublicObjectiveTable = ({
  gameState,
  gameOptions,
  sendEvent,
}: PublicObjectiveTableProps) => {
  const revealedStageOneObjectives = Object.keys(
    gameState.score.revealedObjectives
  )
    .map((obj) => {
      return {
        id: obj,
        ...gameOptions.objectives[obj],
      };
    })
    .filter((obj) => obj.kind === "StageI");

  const revealedStageTwoObjectives = Object.keys(
    gameState.score.revealedObjectives
  )
    .map((obj) => {
      return {
        id: obj,
        ...gameOptions.objectives[obj],
      };
    })
    .filter((obj) => obj.kind === "StageII");

  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });
  const playerCount = players.length;

  return (
    <table className={`card ${styles.scoreViewTable}`}>
      <thead>
        <tr>
          {players.map((p) => (
            <th key={p.id} className={styles.scoreViewTableHeader}>
              <FactionIcon faction={p.faction} />
            </th>
          ))}
        </tr>
      </thead>
      <tbody>
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              {gameState.score.playerPoints[p.id]}p
            </td>
          ))}
        </tr>
        <tr>
          <th colSpan={playerCount}>
            <div className={styles.stageContainer}>
              <div
                className={`${styles.stageOneBackgroundColor} ${styles.horizontalLine}`}
              />
              <h2 className={`${styles.stageOneColor} ${styles.stageText}`}>
                Stage I
              </h2>
              <div
                className={`${styles.stageOneBackgroundColor} ${styles.horizontalLine}`}
              />
            </div>
          </th>
        </tr>
        {revealedStageOneObjectives.map((obj) => (
          <React.Fragment key={obj.id}>
            <tr key={obj.id}>
              <th colSpan={playerCount} className={styles.borderTop}>
                {obj.name}
              </th>
            </tr>
            <tr>
              {players.map((p) => (
                <td key={p.id} align="center">
                  <FactionButton
                    faction={p.faction}
                    playerId={p.id}
                    objective={obj.id}
                    selected={
                      gameState.score.revealedObjectives[obj.id]?.includes(
                        p.id
                      ) ?? false
                    }
                    sendEvent={sendEvent}
                  />
                </td>
              ))}
            </tr>
          </React.Fragment>
        ))}
        <tr>
          <th colSpan={playerCount} className={styles.borderTop}>
            <div className={styles.stageContainer}>
              <div
                className={`${styles.stageTwoBackgroundColor} ${styles.horizontalLine}`}
              />
              <h2 className={`${styles.stageTwoColor} ${styles.stageText}`}>
                Stage II
              </h2>
              <div
                className={`${styles.stageTwoBackgroundColor} ${styles.horizontalLine}`}
              />
            </div>
          </th>
        </tr>
        {revealedStageTwoObjectives.map((obj) => (
          <React.Fragment key={obj.id}>
            <tr key={obj.id}>
              <th colSpan={playerCount} className={styles.borderTop}>
                {obj.name}
              </th>
            </tr>
            <tr>
              {players.map((p) => (
                <td key={p.id} align="center">
                  <FactionButton
                    faction={p.faction}
                    playerId={p.id}
                    objective={obj.id}
                    selected={
                      gameState.score.revealedObjectives[obj.id]?.includes(
                        p.id
                      ) ?? false
                    }
                    sendEvent={sendEvent}
                  />
                </td>
              ))}
            </tr>
          </React.Fragment>
        ))}
        <tr>
          <th colSpan={playerCount} className={styles.borderTop}>
            <div className={styles.stageContainer}>
              <div
                className={`${styles.secretBackgroundColor} ${styles.horizontalLine}`}
              />
              <h2 className={`${styles.secretColor} ${styles.stageText}`}>
                Secrets
              </h2>
              <div
                className={`${styles.secretBackgroundColor} ${styles.horizontalLine}`}
              />
            </div>
          </th>
        </tr>
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              <PlayerSecretObjectivesScore
                playerId={p.id}
                gameState={gameState}
              />
            </td>
          ))}
        </tr>
      </tbody>
    </table>
  );
};

interface FactionButtonProps {
  playerId: string;
  faction: Faction;
  selected: boolean;
  objective: string;
  sendEvent: (data: any) => void;
}

const FactionButton = ({
  faction,
  playerId,
  selected,
  objective,
  sendEvent,
}: FactionButtonProps) => {
  return (
    <button
      className={`${selected ? styles.factionButtonSelected : ""} ${
        styles.factionButton
      }`}
      onClick={() =>
        sendEvent({
          ScorePublicObjective: {
            player: playerId,
            objective: objective,
          },
        })
      }
    >
      <FactionIcon faction={faction} />
    </button>
  );
};

interface PlayerSecretObjectivesScore {
  playerId: string;
  gameState: GameState;
}

const PlayerSecretObjectivesScore = ({
  playerId,
  gameState,
}: PlayerSecretObjectivesScore) => {
  const playerSecrets = gameState.score.secretObjectives[playerId];

  if (!playerSecrets) {
    return <p>0</p>;
  }

  return <p>{playerSecrets.length}</p>;
};
