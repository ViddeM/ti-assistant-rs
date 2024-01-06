import { TechLedger } from "./TechLedger";
import { TechTable } from "./TechTable";
import styles from "./TechViewMode.module.scss";

export const TechViewMode = () => {
  return (
    <div className={styles.techViewContainer}>
      <TechLedger />
      <TechTable />
    </div>
  );
};
