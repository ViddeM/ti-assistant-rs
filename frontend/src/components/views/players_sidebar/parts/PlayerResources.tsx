import { TechCategory } from "@/api/bindings/TechCategory";
import { PlanetTrait } from "@/api/bindings/PlanetTrait";
import { PlayerPlanetInfo, SidebarPlayer, Tech } from "../PlayersSidebar";
import { Icon } from "@/components/elements/icon/Icon";
import styles from "./PlayerResources.module.scss";

export const PlayerResources = ({ player }: { player: SidebarPlayer }) => {
  const numPlanets = player.planets.length;
  const resources = player.planets.reduce(
    (acc, curr) =>
      acc +
      curr.info.resources +
      curr.attachments.reduce((acc, curr) => acc + curr.resources, 0),
    0,
  );
  const influence = player.planets.reduce(
    (acc, curr) =>
      acc +
      curr.info.influence +
      curr.attachments.reduce((acc, curr) => acc + curr.influence, 0),
    0,
  );
  const numCultural = getPlanetTraitCount(player.planets, "Cultural");
  const numHazardous = getPlanetTraitCount(player.planets, "Hazardous");
  const numIndustrial = getPlanetTraitCount(player.planets, "Industrial");

  const numTechs = player.technologies.length;
  const numBiotic = getTechCategoryCount(player.technologies, "Biotic");
  const numCybernetic = getTechCategoryCount(player.technologies, "Cybernetic");
  const numPropulsion = getTechCategoryCount(player.technologies, "Propulsion");
  const numWarfare = getTechCategoryCount(player.technologies, "Warfare");

  return (
    <div className={styles.planetTechContainer}>
      <div className={styles.planetContent}>
        <div className={styles.resourceRow}>
          <div className={styles.planetsCount}>
            <p>{numPlanets}</p>
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
      <div className={styles.techContent}>
        <div className={styles.resourceRow}>
          <p>{numTechs}</p>
          <p className={styles.techIcon}>T</p>
        </div>
        <div className={styles.resourceRow}>
          {numBiotic}
          <Icon name="biotic" isFilled={true} />
          {numCybernetic}
          <Icon name="cybernetic" isFilled={true} />
          {numPropulsion}
          <Icon name="propulsion" isFilled={true} />
          {numWarfare}
          <Icon name="warfare" isFilled={true} />
        </div>
      </div>
    </div>
  );
};

function getTechCategoryCount(
  technologies: Tech[],
  category: TechCategory,
): number {
  console.log("foo", technologies);
  return technologies.filter((t) => {
    if (t.info.techType === "UnitUpgrade") {
      return false;
    }

    if (t.info.techType.Category !== category) {
      return false;
    }

    return true;
  }).length;
}

function getPlanetTraitCount(
  planets: PlayerPlanetInfo[],
  trait: PlanetTrait,
): number {
  return planets.filter(
    (p) =>
      p.info.planetTrait === trait ||
      p.attachments.filter((p) => p.addedPlanetTraits.includes(trait)).length >
        0,
  ).length;
}
