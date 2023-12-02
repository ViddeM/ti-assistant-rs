import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import styles from "./ScoreViewMode.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { RevealObjectiveForm } from "./RevealObjectiveForm";
import React from "react";

export interface ScoreViewModeProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const ScoreViewMode = ({
  gameState,
  gameOptions,
  sendEvent,
}: ScoreViewModeProps) => {
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
    <div className={styles.scoreViewContainer}>
      <RevealObjectiveForm
        gameState={gameState}
        gameOptions={gameOptions}
        sendEvent={sendEvent}
      />

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
                    <FactionIcon
                      className={styles.unselected}
                      faction={p.faction}
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
                    <FactionIcon
                      className={styles.unselected}
                      faction={p.faction}
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
    </div>
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
