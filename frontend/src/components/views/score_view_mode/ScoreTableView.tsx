import { Objective } from "@/api/bindings/Objective";
import styles from "./ScoreViewMode.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import React from "react";
import { Button } from "@/components/elements/button/Button";
import { FactionButton } from "@/components/elements/factionButton/FactionButton";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";
import { InfoButton } from "@/components/elements/button/InfoButton";
import { InfoObject } from "../info_modal/InfoModal";
import { ScorableAgenda } from "@/api/bindings/ScorableAgenda";
import { Player } from "@/api/bindings/Player";

export const ScoreTableView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const revealedStageOneObjectives = Object.keys(
    gameState.score.revealedObjectives,
  )
    .map((obj) => {
      return obj as Objective;
    })
    .map((obj) => {
      return {
        id: obj,
        ...gameOptions.objectives[obj],
      };
    })
    .filter((obj) => obj.kind === "StageI")
    .sort(nameSort);

  const revealedStageTwoObjectives = Object.keys(
    gameState.score.revealedObjectives,
  )
    .map((obj) => {
      return obj as Objective;
    })
    .map((obj) => {
      return {
        id: obj,
        ...gameOptions.objectives[obj],
      };
    })
    .filter((obj) => obj.kind === "StageII")
    .sort(nameSort);

  const players = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  const crownOfEmphidia = gameOptions.relics.TheCrownOfEmphidia;
  const shardOfTheThrone = gameOptions.relics.ShardOfTheThrone;

  const agendaScores = gameState.score.agendaScores;

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
            <SubSectionHeading
              key={obj.id}
              playerCount={playerCount}
              topBorder={index > 0}
              name={obj.name}
              info={{ Objective: obj }}
            />
            <tr>
              {players.map((p) => (
                <td key={p.id} align="center">
                  <FactionButton
                    faction={p.faction}
                    selected={
                      gameState.score.revealedObjectives[obj.id]?.includes(
                        p.id,
                      ) ?? false
                    }
                    onClick={() => {
                      if (
                        gameState.score.revealedObjectives[obj.id]?.includes(
                          p.id,
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
            <SubSectionHeading
              key={obj.id}
              playerCount={playerCount}
              topBorder={index > 0}
              name={obj.name}
              info={{ Objective: obj }}
            />
            <tr>
              {players.map((p) => (
                <td key={p.id} align="center">
                  <FactionButton
                    faction={p.faction}
                    selected={
                      gameState.score.revealedObjectives[obj.id]?.includes(
                        p.id,
                      ) ?? false
                    }
                    onClick={() => {
                      if (
                        gameState.score.revealedObjectives[obj.id]?.includes(
                          p.id,
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
                          ScoreExtraPublicObjective: {
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
              <PlayerSecretObjectivesScore playerId={p.id} />
            </td>
          ))}
        </tr>

        {/* AGENDAS */}
        <TableSectionHeading
          playerCount={playerCount}
          title="Agendas"
          stylingPrefix="agenda"
        />
        {agendaScores.map((score) => (
          <AgendaScoreRow
            key={score.electableAgendaKind}
            agenda={score}
            players={players}
          />
        ))}

        {/* Relics */}
        <TableSectionHeading
          playerCount={playerCount}
          title="Relics"
          stylingPrefix="relic"
        />

        {/* The Crown of Emphidia, check that it is enabled for this game */}
        {crownOfEmphidia && (
          <>
            <SubSectionHeading
              playerCount={playerCount}
              topBorder={false}
              name={"The Crown of Emphidia"}
              info={{ Relic: gameOptions.relics.TheCrownOfEmphidia }}
            />
            <tr>
              {players.map((p) => (
                <td key={p.id} align="center">
                  <FactionButton
                    faction={p.faction}
                    selected={gameState.score.crownOfEmphidia === p.id}
                    onClick={() => {
                      let newOwner =
                        gameState.score.crownOfEmphidia === p.id ? null : p.id;
                      sendEvent({
                        SetCrownOfEmphidiaOwner: {
                          player: newOwner,
                        },
                      });
                    }}
                  />
                </td>
              ))}
            </tr>
          </>
        )}

        {/* Shard of the Throne relic, check that it is enabled for this game */}
        {shardOfTheThrone && (
          <>
            <SubSectionHeading
              playerCount={playerCount}
              topBorder={true}
              name={"Shard of the Throne"}
              info={{ Relic: gameOptions.relics.ShardOfTheThrone }}
            />
            <tr>
              {players.map((p) => (
                <td key={p.id} align="center">
                  <FactionButton
                    faction={p.faction}
                    selected={gameState.score.shardOfTheThrone === p.id}
                    onClick={() => {
                      let newOwner =
                        gameState.score.shardOfTheThrone === p.id ? null : p.id;
                      sendEvent({
                        SetShardForTheThroneOwner: {
                          player: newOwner,
                        },
                      });
                    }}
                  />
                </td>
              ))}
            </tr>
          </>
        )}

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
                  (rec) => rec == p.id,
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

interface PlayerSecretObjectivesScore {
  playerId: string;
}

const PlayerSecretObjectivesScore = ({
  playerId,
}: PlayerSecretObjectivesScore) => {
  const { gameState } = useGameContext();

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
    | "agenda"
    | "relic"
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

interface SubSectionHeadingProps {
  playerCount: number;
  topBorder: boolean;
  name: string;
  info: InfoObject;
}

const SubSectionHeading = ({
  playerCount,
  topBorder,
  name,
  info,
}: SubSectionHeadingProps) => {
  return (
    <tr>
      <th colSpan={playerCount} className={topBorder ? styles.borderTop : ""}>
        <InfoButton info={info} style={{ visibility: "hidden" }} />
        {name}
        <InfoButton info={info} />
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

interface AgendaScoreRowProps {
  agenda: ScorableAgenda;
  players: (Player & { id: string })[];
}

const AgendaScoreRow = ({ agenda, players }: AgendaScoreRowProps) => {
  switch (agenda.electableAgendaKind) {
    case "HolyPlanetOfIxth":
      let owner = players.find((p) => agenda.value.planet in p.planets)!!.id;

      return (
        <tr>
          {players.map((p) => (
            <td key={p.id} align="center">
              {owner === p.id ? `1` : `0`}
            </td>
          ))}
        </tr>
      );
  }
};
