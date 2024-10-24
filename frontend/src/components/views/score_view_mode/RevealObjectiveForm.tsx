import { Objective } from "@/api/bindings/Objective";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./ScoreViewMode.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort, stringSort } from "@/utils/Utils";

export const RevealObjectiveForm = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [selectedStageI, setSelectedStageI] = useState<string>("");
  const [selectedStageII, setSelectedStageII] = useState<string>("");

  const revealedObjectives = Object.keys(
    gameState.score.revealedObjectives,
  ).sort(stringSort);

  const stageOneObjectives = Object.keys(gameOptions.objectives)
    .map((o) => {
      return o as Objective;
    })
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind === "StageI")
    .filter((o) => !revealedObjectives.includes(o.id))
    .sort(nameSort);
  const stageTwoObjectives = Object.keys(gameOptions.objectives)
    .map((o) => {
      return o as Objective;
    })
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind === "StageII")
    .filter((o) => !revealedObjectives.includes(o.id))
    .sort(nameSort);

  return (
    <form
      className={`${styles.revealObjectiveForm} card`}
      onSubmit={(e) => e.preventDefault()}
    >
      <h2>Reveal Objectives</h2>
      <div className={styles.revealObjectiveRow}>
        <Dropdown
          value={selectedStageI}
          onChange={(e) => setSelectedStageI(e.target.value)}
        >
          <option value="">--Select Stage I objective--</option>
          {stageOneObjectives.map((o) => (
            <option key={o.id} value={o.id}>
              {o.name}
            </option>
          ))}
        </Dropdown>
        <Button
          disabled={selectedStageI === ""}
          onClick={() => {
            setSelectedStageI("");
            sendEvent({
              RevealExtraPublicObjective: {
                objective: selectedStageI,
              },
            });
          }}
        >
          Reveal
        </Button>
      </div>
      <div className={styles.revealObjectiveRow}>
        <Dropdown
          value={selectedStageII}
          onChange={(e) => setSelectedStageII(e.target.value)}
        >
          <option value="">--Select Stage II objective--</option>
          {stageTwoObjectives.map((o) => (
            <option key={o.id} value={o.id}>
              {o.name}
            </option>
          ))}
        </Dropdown>
        <Button
          disabled={selectedStageII === ""}
          onClick={() => {
            setSelectedStageII("");
            sendEvent({
              RevealExtraPublicObjective: {
                objective: selectedStageII,
              },
            });
          }}
        >
          Reveal
        </Button>
      </div>
    </form>
  );
};
