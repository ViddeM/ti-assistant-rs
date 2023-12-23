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
              <FactionIcon faction={p.faction} width={32} height={32} />
            </th>
          ))}
        </tr>
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              {gameState.score.playerPoints[p.id]}p
            </td>
          ))}
        </tr>
      </thead>
      <tbody>
        {/* CUSTODIANS */}
        <TableSectionHeading
          playerCount={playerCount}
          title={"Custodians"}
          stylingPrefix={"custodians"}
        />
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
        <TableSectionHeading
          playerCount={playerCount}
          title="Stage I"
          stylingPrefix="stageOne"
        />
        {revealedStageOneObjectives.map((obj, index) => (
          <React.Fragment key={obj.id}>
            <tr key={obj.id}>
              <th
                colSpan={playerCount}
                className={index === 0 ? "" : styles.borderTop}
              >
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
                      if (
                        gameState.score.revealedObjectives[obj.id]?.includes(
                          p.id
                        ) ??
                        false
                      ) {
                        sendEvent({
                          UnscoreObjective: {
                            player: p.id,
                            objective: obj.id,
                          },
                        });
                      } else {
                        sendEvent({
                          ScorePublicObjective: {
                            player: p.id,
                            objective: obj.id,
                          },
                        });
                      }
                    }}
                  />
                </td>
              ))}
            </tr>
          </React.Fragment>
        ))}

        {/* STAGE II Public Objectives */}
        <TableSectionHeading
          playerCount={playerCount}
          title="Stage II"
          stylingPrefix="stageTwo"
        />
        {revealedStageTwoObjectives.map((obj, index) => (
          <React.Fragment key={obj.id}>
            <tr key={obj.id}>
              <th
                colSpan={playerCount}
                className={index === 0 ? "" : styles.borderTop}
              >
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
                      if (
                        gameState.score.revealedObjectives[obj.id]?.includes(
                          p.id
                        ) ??
                        false
                      ) {
                        sendEvent({
                          UnscoreObjective: {
                            player: p.id,
                            objective: obj.id,
                          },
                        });
                      } else {
                        sendEvent({
                          ScorePublicObjective: {
                            player: p.id,
                            objective: obj.id,
                          },
                        });
                      }
                    }}
                  />
                </td>
              ))}
            </tr>
          </React.Fragment>
        ))}

        {/* SECRET OBJECTIVES */}
        <TableSectionHeading
          playerCount={playerCount}
          title="Secrets"
          stylingPrefix="secret"
        />
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

        {/* Support for the Throne */}
        <TableSectionHeading
          playerCount={playerCount}
          title="Support for the Throne"
          stylingPrefix="spftt"
        />
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

        {/* IMPERIAL */}
        <TableSectionHeading
          playerCount={playerCount}
          title="Imperial"
          stylingPrefix="imperial"
        />
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              <IncDecView
                points={gameState.score.imperial[p.id] ?? 0}
                changePoints={(newPoints) =>
                  sendEvent({
                    AddImperial: {
                      player: p.id,
                      value: newPoints - (gameState.score.imperial[p.id] ?? 0),
                    },
                  })
                }
              />
            </td>
          ))}
        </tr>

        {/* Extra points (manual modifications) */}
        <TableSectionHeading
          playerCount={playerCount}
          title="Extra Points"
          stylingPrefix="extra"
        />
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              <IncDecView
                points={gameState.score.extraPoints[p.id] ?? 0}
                changePoints={(newPoints) =>
                  sendEvent({
                    AddExtraPoints: {
                      player: p.id,
                      value:
                        newPoints - (gameState.score.extraPoints[p.id] ?? 0),
                    },
                  })
                }
              />
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
      <FactionIcon faction={faction} width={32} height={32} />
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

interface TableSectionHeadingProps {
  playerCount: number;
  title: string;
  stylingPrefix:
    | "stageOne"
    | "stageTwo"
    | "secret"
    | "custodians"
    | "imperial"
    | "spftt"
    | "extra";
}

const TableSectionHeading = ({
  playerCount,
  title,
  stylingPrefix,
}: TableSectionHeadingProps) => {
  let background = styles[`${stylingPrefix}BackgroundColor`];
  let color = styles[`${stylingPrefix}Color`];

  return (
    <tr>
      <th colSpan={playerCount}>
        <div className={styles.stageContainer}>
          <div className={`${background} ${styles.horizontalLine}`} />
          <h2 className={`${color} ${styles.stageText}`}>{title}</h2>
          <div className={`${background} ${styles.horizontalLine}`} />
        </div>
      </th>
    </tr>
  );
};

interface IncDecViewProps {
  points: number;
  changePoints: (newPoints: number) => void;
}

const IncDecView = ({ points, changePoints }: IncDecViewProps) => {
  return (
    <>
      <Button
        onClick={() => changePoints(points + 1)}
        className={styles.incDecButton}
      >
        ^
      </Button>
      <p>{points}</p>
      <Button
        onClick={() => changePoints(points - 1)}
        className={styles.incDecButton}
        disabled={points == 0}
      >
        v
      </Button>
    </>
  );
};
