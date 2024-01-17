import styles from "./StatusPhaseView.module.scss";
import { StatusPhaseInstructionsView } from "./StatusPhaseInstructionsView";
import { StatusPhaseActionsView } from "./StatusPhaseActionsView";

export const StatusPhaseView = () => {
  return (
    <div className={styles.statusPhaseContainer}>
      <StatusPhaseActionsView />
      <StatusPhaseInstructionsView />
    </div>
  );
};
