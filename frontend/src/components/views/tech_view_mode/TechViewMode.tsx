import { GameState } from "@/api/GameState";
import { TechLedger } from "./TechLedger";
import { GameOptions } from "@/api/GameOptions";
import { TechTable } from "./TechTable";
import styles from "./TechViewMode.module.scss";

export interface TechViewModeProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const TechViewMode = ({
  gameState,
  gameOptions,
  sendEvent,
}: TechViewModeProps) => {
  return (
    <div className={styles.techViewContainer}>
      <TechLedger gameState={gameState} />
      <TechTable
        gameOptions={gameOptions}
        gameState={gameState}
        sendEvent={sendEvent}
      />
    </div>
  );
};
