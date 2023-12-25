import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { PlayerPlanetsGrid } from "./PlayerPlanetsGrid";
import styles from "./PlanetViewMode.module.scss";
import { UnclaimedPlanetsTable } from "./UnclaimedPlanetsTable";

export interface PlanetViewModeProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const PlanetViewMode = ({
  gameOptions,
  gameState,
  sendEvent,
}: PlanetViewModeProps) => {
  return (
    <div className={styles.planetViewContainer}>
      <PlayerPlanetsGrid
        gameState={gameState}
        gameOptions={gameOptions}
        sendEvent={sendEvent}
      />
      <UnclaimedPlanetsTable
        gameState={gameState}
        gameOptions={gameOptions}
        sendEvent={sendEvent}
      />
    </div>
  );
};
