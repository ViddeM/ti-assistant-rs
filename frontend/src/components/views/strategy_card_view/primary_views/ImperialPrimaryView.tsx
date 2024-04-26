import styles from "./Primary.module.scss";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import { Button } from "@/components/elements/button/Button";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const ImperialPrimaryView = () => {
  const { gameState, gameOptions, sendEvent, isActive } = useGameContext();

  const [objective, setObjective] = useState<string>("");

  const progress = gameState.actionProgress?.Strategic?.primary?.Imperial;
  const objectives = Object.keys(gameState.score.revealedObjectives)
    .filter((o) => {
      return !gameState.score.revealedObjectives[o].includes(
        gameState.currentPlayer!!
      );
    })
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .sort(nameSort);

  const performAction = (objective: string | null) => {
    sendEvent({
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
        <div className={styles.primaryChoiceContainer}>
          <p>
            {progress.objective
              ? gameOptions.objectives[progress.objective].name
              : "No objective taken"}
          </p>
        </div>
      ) : (
        <fieldset>
          <legend>Score objective</legend>
          {isActive ? (
            <div className={styles.selectPrimaryContainer}>
              <Dropdown
                value={objective}
                disabled={objectives.length === 0}
                onChange={(e) => setObjective(e.target.value)}
              >
                <option value={""}>--No objective--</option>
                {objectives.map((o) => (
                  <option value={o.id} key={o.id}>
                    {o.name}
                  </option>
                ))}
              </Dropdown>
              <div className={styles.actionButtonsContainer}>
                <Button onClick={() => performAction(null)}>Skip</Button>
                <Button
                  onClick={() => performAction(objective)}
                  disabled={objective === ""}
                >
                  Score
                </Button>
              </div>
            </div>
          ) : (
            <p>Has yet to choose</p>
          )}
        </fieldset>
      )}
    </div>
  );
};
