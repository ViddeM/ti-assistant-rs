import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import styles from "./StatusPhaseView.module.scss";
import { GameOptions } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";
import { StatusPhaseInstructionsView } from "./StatusPhaseInstructionsView";
import { StatusPhaseActionsView } from "./StatusPhaseActionsView";
import { GameState } from "@/api/GameState";

export interface StatusPhaseViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StatusPhaseView = ({
  gameState,
  gameOptions,
  sendMessage,
}: StatusPhaseViewProps) => {
  const mappedObjectives = Object.keys(gameOptions.objectives).map((obj) => {
    let val = gameOptions.objectives[obj];

    return {
      id: obj,
      phase:
        val.kind === "StageI" || val.kind === "StageII" ? null : val.kind.phase,
      ...val,
    };
  });

  return (
    <div className={styles.statusPhaseContainer}>
      <StatusPhaseActionsView
        gameState={gameState}
        gameOptions={gameOptions}
        sendMessage={sendMessage}
      />
      <StatusPhaseInstructionsView
        gameState={gameState}
        sendMessage={sendMessage}
      />
    </div>
  );
};
