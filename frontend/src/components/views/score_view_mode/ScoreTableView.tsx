import { GameState } from "@/api/GameState";
import styles from "./ScoreViewMode.module.scss";
import { GameOptions } from "@/api/GameOptions";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Faction } from "@/resources/types/factions";
import React from "react";
import { Button } from "@/components/elements/button/Button";

interface ScoreTableViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const ScoreTableView = ({
  gameState,
  gameOptions,
  sendEvent,
}: ScoreTableViewProps) => {
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
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              <Button onClick={() => sendEvent()}>-</Button>
              {gameState.score.playerPoints[p.id]}p<Button>+</Button>
            </td>
          ))}
        </tr>
      </thead>
      <tbody>
        {/* CUSTODIANS */}
        <tr>
          <th colSpan={playerCount}>
            <div className={styles.stageContainer}>
              <div
                className={`${styles.stageOneBackgroundColor} ${styles.horizontalLine}`}
              />
              <h2 className={`${styles.stageOneColor} ${styles.stageText}`}>
                Custodians
              </h2>
              <div
                className={`${styles.stageOneBackgroundColor} ${styles.horizontalLine}`}
              />
            </div>
          </th>
        </tr>
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              <FactionButton
                faction={p.faction}
                selected={gameState.score.custodians === p.id}
                onClick={() => {
                  sendEvent({
                    SetCustodians: {
                      player: gameState.score.custodians === p.id ? null : p.id,
                    },
                  });
                }}
              />
            </td>
          ))}
        </tr>

        {/* STAGE I Public Objectives */}
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
                    selected={
                      gameState.score.revealedObjectives[obj.id]?.includes(
                        p.id
                      ) ?? false
                    }
                    onClick={() => {
                      sendEvent({
                        ScorePublicObjective: {
                          player: p.id,
                          objective: obj.id,
                        },
                      });
                    }}
                  />
                </td>
              ))}
            </tr>
          </React.Fragment>
        ))}

        {/* STAGE II Public Objectives */}
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
                    selected={
                      gameState.score.revealedObjectives[obj.id]?.includes(
                        p.id
                      ) ?? false
                    }
                    onClick={() => {
                      sendEvent({
                        ScorePublicObjective: {
                          player: p.id,
                          objective: obj.id,
                        },
                      });
                    }}
                  />
                </td>
              ))}
            </tr>
          </React.Fragment>
        ))}

        {/* SECRET OBJECTIVES */}
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

        {/* IMPERIAL */}
        <tr>
          <th colSpan={playerCount} className={styles.borderTop}>
            <div className={styles.stageContainer}>
              <div
                className={`${styles.secretBackgroundColor} ${styles.horizontalLine}`}
              />
              <h2 className={`${styles.secretColor} ${styles.stageText}`}>
                Imperial
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
              <Button
                onClick={() => {
                  let curr = gameState.score.imperial[p.id] ?? 0;
                  sendEvent({
                    SetImperial: {
                      player: p.id,
                      value: curr > 0 ? curr - 1 : 0,
                    },
                  });
                }}
              >
                -
              </Button>
              {gameState.score.imperial[p.id] ?? 0}
              <Button
                onClick={() => {
                  let curr = gameState.score.imperial[p.id] ?? 0;
                  sendEvent({
                    SetImperial: {
                      player: p.id,
                      value: curr + 1,
                    },
                  });
                }}
              >
                +
              </Button>
            </td>
          ))}
        </tr>

        {/* Support for the Throne */}
        <tr>
          <th colSpan={playerCount} className={styles.borderTop}>
            <div className={styles.stageContainer}>
              <div
                className={`${styles.secretBackgroundColor} ${styles.horizontalLine}`}
              />
              <h2 className={`${styles.secretColor} ${styles.stageText}`}>
                Support for the Throne
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
              {
                Object.values(gameState.score.supportForTheThrone).filter(
                  (rec) => rec == p.id
                ).length
              }
            </td>
          ))}
        </tr>
      </tbody>
    </table>
  );
};

interface FactionButtonProps {
  faction: Faction;
  selected: boolean;
  onClick: () => void;
}

const FactionButton = ({ faction, selected, onClick }: FactionButtonProps) => {
  return (
    <button
      className={`${selected ? styles.factionButtonSelected : ""} ${
        styles.factionButton
      }`}
      onClick={onClick}
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
