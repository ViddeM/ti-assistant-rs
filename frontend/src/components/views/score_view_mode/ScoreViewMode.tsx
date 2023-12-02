import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import styles from "./ScoreViewMode.module.scss";
import { useState } from "react";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Faction } from "@/resources/types/factions";

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
  const [selectedStageI, setSelectedStageI] = useState<string>("");
  const [selectedStageII, setSelectedStageII] = useState<string>("");

  const stageOneObjectives = Object.keys(gameOptions.objectives)
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind === "StageI");
  const stageTwoObjectives = Object.keys(gameOptions.objectives)
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind === "StageII");

  const revealedStageOneObjectives = Object.keys(
    gameState.score.revealedObjectives
  )
    .map((obj) => {
      return {
        id: obj,
        ...gameOptions.objectives[obj],
      };
    })
    .filter((obj) => obj.kind === "StageI");

  const revealedStageTwoObjectives = Object.keys(
    gameState.score.revealedObjectives
  )
    .map((obj) => {
      return {
        id: obj,
        ...gameOptions.objectives[obj],
      };
    })
    .filter((obj) => obj.kind === "StageII");

  const playerCount = Object.keys(gameState.players).length;
  return (
    <div>
      <div className="card" style={{ marginBottom: "1rem" }}>
        <label>Reveal stage I</label>
        <Dropdown
          value={selectedStageI}
          onChange={(e) => setSelectedStageI(e.target.value)}
        >
          <option value="">--Select objective--</option>
          {stageOneObjectives
            .filter(
              (o) => !revealedStageOneObjectives.map((o) => o.id).includes(o.id)
            )
            .map((o) => (
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
              RevealPublicObjective: {
                objective: selectedStageI,
              },
            });
          }}
        >
          Reveal
        </Button>

        <div className={styles.revealObjectiveColumn}>
          <label>Reveal stage II</label>
          <Dropdown
            value={selectedStageII}
            onChange={(e) => setSelectedStageII(e.target.value)}
          >
            <option value="">--Select objective--</option>
            {stageTwoObjectives
              .filter(
                (o) =>
                  !revealedStageTwoObjectives.map((o) => o.id).includes(o.id)
              )
              .map((o) => (
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
                RevealPublicObjective: {
                  objective: selectedStageII,
                },
              });
            }}
          >
            Reveal
          </Button>
        </div>
      </div>

      <table className={`card ${styles.scoreViewTable}`}>
        <thead>
          <tr>
            {Object.keys(gameState.players).map((p) => (
              <th key={p} className={styles.scoreViewTableHeader}>
                <FactionIcon faction={gameState.players[p].faction} />
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          <tr>
            <th colSpan={playerCount}>---Stage I---</th>
          </tr>
          {revealedStageOneObjectives.map((obj) => (
            <>
              <tr key={obj.id}>
                <th
                  colSpan={playerCount}
                  style={{ borderTop: "1px solid black" }}
                >
                  {obj.name}
                </th>
              </tr>
              <tr>
                {Object.keys(gameState.players).map((p) => (
                  <td key={p}>
                    <input type="checkbox" />
                  </td>
                ))}
              </tr>
            </>
          ))}
          <tr>
            <th
              colSpan={playerCount}
              style={{
                display: "flex",
                flexDirection: "row",
                alignItems: "center",
                width: "100%",
              }}
            >
              <div
                className={`${styles.stageTwoBackgroundColor} ${styles.horizontalLine}`}
              />
              <h2 className={styles.stageTwoColor}>Stage II</h2>
              <div
                className={`${styles.stageTwoBackgroundColor} ${styles.horizontalLine}`}
              />
            </th>
          </tr>
          {revealedStageTwoObjectives.map((obj) => (
            <>
              <tr key={obj.id}>
                <th colSpan={playerCount}>{obj.name}</th>
              </tr>
              <tr>
                {Object.keys(gameState.players).map((p) => (
                  <td key={p}>
                    <input type="checkbox" />
                  </td>
                ))}
              </tr>
            </>
          ))}
        </tbody>
      </table>
    </div>
  );
};
