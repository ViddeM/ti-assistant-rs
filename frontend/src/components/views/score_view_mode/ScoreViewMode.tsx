import styles from "./ScoreViewMode.module.scss";
import { RevealObjectiveForm } from "./RevealObjectiveForm";
import React from "react";
import { SecretObjectivesView } from "./SecretObjectivesView";
import { ScoreTableView } from "./ScoreTableView";
import { SupportForTheThroneView } from "./SupportForTheThroneView";

export const ScoreViewMode = () => {
  return (
    <div className={styles.scoreViewContainer}>
      <ScoreTableView />

      <RevealObjectiveForm />

      <SecretObjectivesView />

      <SupportForTheThroneView />
    </div>
  );
};
