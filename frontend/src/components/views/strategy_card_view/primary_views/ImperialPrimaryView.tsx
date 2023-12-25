import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import styles from "./Primary.module.scss";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import { Button } from "@/components/elements/button/Button";

export interface ImperialPrimaryViewProps {
  gameState: GameState;
  sendMessage: (data: any) => void;
}

export const ImperialPrimaryView = ({
  gameState,
  sendMessage,
}: ImperialPrimaryViewProps) => {
  const [objective, setObjective] = useState<string>("");

  const progress = gameState.actionProgress?.Strategic?.primary?.Imperial;
  const objectives = Object.keys(gameState.score.revealedObjectives).filter(
    (o) => {
      return !gameState.score.revealedObjectives[o].includes(
        gameState.currentPlayer!!
      );
    }
  );

  const performAction = (objective: string | null) => {
    sendMessage({
      StrategicActionPrimary: {
        player: gameState.currentPlayer!!,
        action: {
          Imperial: {
            scoreObjective: objective,
          },
        },
      },
    });
  };

  return (
    <div className={styles.primaryContainer}>
      {progress ? (
        <p>{progress.objective ? progress.objective : "No objective taken"}</p>
      ) : (
        <fieldset>
          <legend>Score objective</legend>
          <div className={styles.selectPrimaryContainer}>
            <Dropdown
              value={objective}
              disabled={objectives.length === 0}
              onChange={(e) => setObjective(e.target.value)}
            >
              <option value={""}>--No objective--</option>
              {objectives.map((o) => (
                <option value={o} key={o}>
                  {o}
                </option>
              ))}
            </Dropdown>
            <div>
              <Button onClick={() => performAction(null)}>Skip</Button>
              <Button
                onClick={() => performAction(objective)}
                disabled={objective === ""}
              >
                Score
              </Button>
            </div>
          </div>
        </fieldset>
      )}
    </div>
  );
};
