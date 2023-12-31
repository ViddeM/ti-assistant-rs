import { GameOptions, TechInfo } from "@/api/GameOptions";
import { GameState, Player } from "@/api/GameState";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./TechViewMode.module.scss";
import React from "react";
import { FactionButton } from "@/components/elements/factionButton/FactionButton";
import { Icon } from "@/components/elements/icon/Icon";

export interface TechTableProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const TechTable = ({
  gameState,
  gameOptions,
  sendEvent,
}: TechTableProps) => {
  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });
  const playerCount = players.length;

  const techs = Object.keys(gameOptions.technologies)
    .map((t) => {
      return {
        id: t,
        ...gameOptions.technologies[t],
      };
    })
    .sort((a, b) => {
      let aDeps = Object.values(a.requirements).reduce(
        (prev, curr) => prev + curr,
        0
      );
      let bDeps = Object.values(b.requirements).reduce(
        (prev, curr) => prev + curr,
        0
      );

      return aDeps - bDeps;
    });

  const unitUpgrades = techs.filter(
    (t) => t.techType === "UnitUpgrade" && t.origin === "Base"
  );
  const warfare = techs.filter(
    (t) =>
      t.techType !== "UnitUpgrade" &&
      t.techType.Category === "Warfare" &&
      t.origin === "Base"
  );
  const biotic = techs.filter(
    (t) =>
      t.techType !== "UnitUpgrade" &&
      t.techType.Category === "Biotic" &&
      t.origin === "Base"
  );
  const propulsion = techs.filter(
    (t) =>
      t.techType !== "UnitUpgrade" &&
      t.techType.Category === "Propulsion" &&
      t.origin === "Base"
  );
  const cybernetic = techs.filter(
    (t) =>
      t.techType !== "UnitUpgrade" &&
      t.techType.Category === "Cybernetic" &&
      t.origin === "Base"
  );

  let playerTechs: { [player: string]: (TechInfo & { id: string })[] } = {};
  players.forEach((p) => {
    playerTechs[p.id] = techs.filter(
      (t) => t.origin !== "Base" && t.origin.Faction === p.faction
    );
  });

  const toggleTechForPlayer = (
    playerId: string,
    player: Player,
    tech: string
  ) => {
    if (player.technologies.includes(tech)) {
      sendEvent({
        RemoveTechFromPlayer: {
          player: playerId,
          tech: tech,
        },
      });
    } else {
      sendEvent({
        AddTechToPlayer: {
          player: playerId,
          tech: tech,
        },
      });
    }
  };

  return (
    <table className={`card ${styles.techViewTable}`}>
      <thead>
        <tr>
          {Object.values(players).map((p) => (
            <th key={p.faction} className={styles.techViewTableHeader}>
              <FactionIcon faction={p.faction} />
            </th>
          ))}
        </tr>
        <tr>
          {Object.values(players).map((p) => (
            <th key={p.faction}>{p.technologies.length}</th>
          ))}
        </tr>
      </thead>
      <tbody>
        {/* Unit Upgrades */}
        <TableSectionHeading
          id="UnitUpgrades"
          playerCount={playerCount}
          title={"Unit Upgrades"}
          stylingPrefix={"unitUpgrade"}
        />
        <TechRows
          techs={unitUpgrades}
          players={players}
          toggleTechForPlayer={toggleTechForPlayer}
        />

        {/* Warfare */}
        <TableSectionHeading
          id="Warfare"
          playerCount={playerCount}
          title={"Warfare"}
          stylingPrefix="warfare"
          icon={"warfare"}
        />
        <TechRows
          techs={warfare}
          players={players}
          toggleTechForPlayer={toggleTechForPlayer}
        />

        {/* Propulsion */}
        <TableSectionHeading
          id="Propulsion"
          playerCount={playerCount}
          title={"Propulsion"}
          stylingPrefix="propulsion"
          icon={"propulsion"}
        />
        <TechRows
          techs={propulsion}
          players={players}
          toggleTechForPlayer={toggleTechForPlayer}
        />

        {/* Cybernetic */}
        <TableSectionHeading
          id="Cybernetic"
          playerCount={playerCount}
          title={"Cybernetic"}
          stylingPrefix="cybernetic"
          icon={"cybernetic"}
        />
        <TechRows
          techs={cybernetic}
          players={players}
          toggleTechForPlayer={toggleTechForPlayer}
        />

        {/* Biotic */}
        <TableSectionHeading
          id="Biotic"
          playerCount={playerCount}
          title={"Biotic"}
          stylingPrefix="biotic"
          icon={"biotic"}
        />
        <TechRows
          techs={biotic}
          players={players}
          toggleTechForPlayer={toggleTechForPlayer}
        />

        {/* Players */}
        {players.map((p) => (
          <React.Fragment key={p.id}>
            <TableSectionHeading
              id={p.id}
              playerCount={playerCount}
              title={`${p.name} - ${p.faction}`}
              stylingPrefix={`player${p.color}`}
            />
            {playerTechs[p.id].length > 0 ? (
              <TechRows
                techs={playerTechs[p.id]}
                players={players}
                toggleTechForPlayer={toggleTechForPlayer}
              />
            ) : (
              <p>Faction has no unique techs</p>
            )}
          </React.Fragment>
        ))}
      </tbody>
    </table>
  );
};

interface TableSectionHeadingProps {
  id: string;
  playerCount: number;
  title: string;
  stylingPrefix:
    | "unitUpgrade"
    | "warfare"
    | "propulsion"
    | "biotic"
    | "cybernetic"
    | "playerBlack"
    | "playerBlue"
    | "playerGreen"
    | "playerRed"
    | "playerYellow"
    | "playerPurple"
    | "playerOrange"
    | "playerPink";
  icon?: "warfare" | "propulsion" | "biotic" | "cybernetic";
}

const ICON_SIZE = 22;

const TableSectionHeading = ({
  id,
  playerCount,
  title,
  stylingPrefix,
  icon,
}: TableSectionHeadingProps) => {
  let background = styles[`${stylingPrefix}BackgroundColor`];
  let color = styles[`${stylingPrefix}Color`];

  return (
    <tr id={id}>
      <th colSpan={playerCount}>
        <div className={styles.stageContainer}>
          <div className={`${background} ${styles.horizontalLine}`} />
          <h2 className={`${color} ${styles.techGroupText}`}>
            {icon && (
              <Icon
                name={icon}
                isFilled={true}
                width={ICON_SIZE}
                height={ICON_SIZE}
              />
            )}
            {title}
            {icon && (
              <Icon
                name={icon}
                isFilled={true}
                width={ICON_SIZE}
                height={ICON_SIZE}
              />
            )}
          </h2>
          <div className={`${background} ${styles.horizontalLine}`} />
        </div>
      </th>
    </tr>
  );
};

interface TechRowsProps {
  techs: ({ id: string } & TechInfo)[];
  players: ({ id: string } & Player)[];
  toggleTechForPlayer: (playerId: string, player: Player, tech: string) => void;
}

const TechRows = ({ techs, players, toggleTechForPlayer }: TechRowsProps) => {
  return (
    <>
      {techs.map((t, index) => (
        <React.Fragment key={t.id}>
          <tr key={t.id}>
            <th
              colSpan={players.length}
              className={index === 0 ? "" : styles.borderTop}
            >
              {t.name}
            </th>
          </tr>
          <tr>
            {players.map((p) => (
              <td key={p.id} align="center">
                <FactionButton
                  faction={p.faction}
                  selected={p.technologies.includes(t.id)}
                  onClick={() => toggleTechForPlayer(p.id, p, t.id)}
                />
              </td>
            ))}
          </tr>
        </React.Fragment>
      ))}
    </>
  );
};
