import { PlayerPlanetsGrid } from "./PlayerPlanetsGrid";
import styles from "./PlanetViewMode.module.scss";
import { UnclaimedPlanetsTable } from "./UnclaimedPlanetsTable";
import { AddPlanetAttachment } from "./AddPlanetAttachment";

export const PlanetViewMode = () => {
  return (
    <div className={styles.planetViewContainer}>
      <PlayerPlanetsGrid />
      <AddPlanetAttachment />
      <UnclaimedPlanetsTable />
    </div>
  );
};
