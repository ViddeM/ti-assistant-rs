import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import styles from "./ScoreViewMode.module.scss";
import { RevealObjectiveForm } from "./RevealObjectiveForm";
import React from "react";
import { SecretObjectivesView } from "./SecretObjectivesView";
import { ScoreTableView } from "./ScoreTableView";
import { SupportForTheThroneView } from "./SupportForTheThroneView";

export interface ScoreViewModeProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const ScoreViewMode = ({
  gameState,
  gameOptions,
  sendEvent,
}: ScoreViewModeProps) => {
  return (
    <div className={styles.scoreViewContainer}>
      <ScoreTableView
        gameState={gameState}
        gameOptions={gameOptions}
        sendEvent={sendEvent}
      />

      <RevealObjectiveForm
        gameState={gameState}
        gameOptions={gameOptions}
        sendEvent={sendEvent}
      />

      <SecretObjectivesView
        gameState={gameState}
        gameOptions={gameOptions}
        sendEvent={sendEvent}
      />

      <SupportForTheThroneView gameState={gameState} sendEvent={sendEvent} />
    </div>
  );
};
