import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategicCardInfo, StrategyCardInfo } from "./parts/StrategyCardInfo";
import { Color, PlanetInfo } from "@/api/GameOptions";
import { Icon } from "@/components/elements/icon/Icon";

export interface Player {
  name: string;
  faction: {
    faction: Faction;
    name: string;
  };
  color: Color;
  isActive: boolean;
  hasPassed: boolean;
  cards: StrategicCardInfo[];
  planets: PlayerPlanetInfo[];
}

export interface PlayerPlanetInfo {
  planet: string;
  info: PlanetInfo;
}

export const PlayerSidebar = ({ players }: { players: Player[] }) => {
  return (
    <div className={`${styles.playerSideBarCard} card`}>
      {players.map((p) => (
        <PlayerBox key={p.name} player={p} />
      ))}
    </div>
  );
};

const PlayerBox = ({ player }: { player: Player }) => {
  const numPlanets = player.planets.length;
  const resources = player.planets.reduce(
    (acc, curr) => acc + curr.info.resources,
    0
  );
  const influence = player.planets.reduce(
    (acc, curr) => acc + curr.info.influence,
    0
  );
  const numCultural = player.planets.filter(
    (p) => p.info.planetTrait === "Cultural"
  ).length;
  const numHazardous = player.planets.filter(
    (p) => p.info.planetTrait === "Hazardous"
  ).length;
  const numIndustrial = player.planets.filter(
    (p) => p.info.planetTrait === "Industrial"
  ).length;

  return (
    <fieldset
      className={`playerColorBorder${player.color} ${
        styles.playerBoxContainer
      } playerColor${player.color} ${
        player.isActive
          ? styles.activePlayer
          : player.hasPassed
          ? styles.passedPlayer
          : ""
      }`}
    >
      <legend
        className={`${styles.playerBoxLegend} playerColorBorder${
          player.color
        } ${
          player.color === "Black"
            ? styles.whiteBackground
            : styles.grayBackground
        }`}
      >
        <h6 style={{ textOverflow: "ellipsis" }}>{player.name} </h6>
      </legend>
      <div className={styles.content}>
        <StrategyCardInfo cards={player.cards} />
      </div>
      <div>
        <div className={styles.planetContent}>
          <div className={styles.resourceRow}>
            <div className={styles.planetsCount}>
              <p>{player.planets.length}</p>
            </div>
            <p>{resources}</p>
            <Icon name="resource" isFilled />
            <Icon name="influence" isFilled />
            <p>{influence}</p>
          </div>
          <div className={styles.resourceRow}>
            {numCultural}
            <Icon name="cultural" />
            {numIndustrial}
            <Icon name="industrial" />
            {numHazardous}
            <Icon name="hazardous" />
          </div>
        </div>
      </div>
    </fieldset>
  );
};
