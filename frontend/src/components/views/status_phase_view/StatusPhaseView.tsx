import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import styles from "./StatusPhaseView.module.scss";
import { GameOptions } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";

export interface StatusPhaseViewProps {
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StatusPhaseView = ({
  gameOptions,
  sendMessage,
}: StatusPhaseViewProps) => {
  return (
    <div className={`card ${styles.statusViewCard}`}>
      <h2>Status Phase</h2>
      <ol className={styles.statusStepList}>
        <li>
          Score Objectives
          <div>
            <label>Secret Objectives</label>
            <Dropdown>
              {gameOptions.secretObjectives.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.name}
                </option>
              ))}
            </Dropdown>
          </div>
          <div>
            <label>Public Objectives</label>
            <Dropdown>
              {gameOptions.publicObjectives.map((p) => (
                <option key={p.id} value={p.id}>
                  {p.name}
                </option>
              ))}
            </Dropdown>
          </div>
        </li>
        <li>Reveal Public Objective</li>
        <li>Draw Action Cards</li>
        <li>Remove Command Tokens</li>
        <li>Gain and Redistribute Tokens</li>
        <li>Ready Cards</li>
        <li>Repair Units</li>
        <li>Return Strategy Cards</li>
      </ol>
      {/* TODO: Agenda phase */}
      <Button onClick={() => sendMessage("CompleteStatusPhase")}>
        Start next turn
      </Button>
    </div>
  );
};
