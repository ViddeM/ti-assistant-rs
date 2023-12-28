import { Button } from "@/components/elements/button/Button";
import styles from "./StatusPhaseView.module.scss";
import { GameState } from "@/api/GameState";

export interface StatusPhaseInstructionsViewProps {
  gameState: GameState;
  sendMessage: (data: any) => void;
}

export const StatusPhaseInstructionsView = ({
  gameState,
  sendMessage,
}: StatusPhaseInstructionsViewProps) => {
  // Relies on this only being true iff objectives have already been scored which should be handled in the BE.
  const statusPhaseComplete =
    gameState.statusPhaseState!!.revealedObjective !== null;

  return (
    <div className={`card ${styles.statusViewCard}`}>
      <h2>Status Phase</h2>
      <ol className={styles.statusStepList}>
        <li>Score Objectives</li>
        <li>Reveal Public Objective</li>
        <li>Draw Action Cards</li>
        <li>Remove Command Tokens</li>
        <li>Gain and Redistribute Tokens</li>
        <li>Ready Cards</li>
        <li>Repair Units</li>
        <li>Return Strategy Cards</li>
      </ol>
      <Button
        disabled={!statusPhaseComplete}
        onClick={() => sendMessage("CompleteStatusPhase")}
      >
        Next phase
      </Button>
    </div>
  );
};
