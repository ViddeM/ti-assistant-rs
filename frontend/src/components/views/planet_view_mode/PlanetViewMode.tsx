import { PlayerPlanetsGrid } from "./PlayerPlanetsGrid";
import styles from "./PlanetViewMode.module.scss";
import { UnclaimedPlanetsTable } from "./UnclaimedPlanetsTable";

export const PlanetViewMode = () => {
  return (
    <div className={styles.planetViewContainer}>
      <PlayerPlanetsGrid />
      <UnclaimedPlanetsTable />
    </div>
  );
};
