import { GameState } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./Primary.module.scss";

interface PoliticsPrimaryViewProps {
  gameState: GameState;
  sendMessage: (data: any) => void;
}

export const PoliticsPrimaryView = ({
  gameState,
  sendMessage,
}: PoliticsPrimaryViewProps) => {
  const [nextSpeaker, setNextSpeaker] = useState<string>("");

  const progress = gameState.actionProgress?.Strategic?.primary;
  const nonSpeakerPlayers = Object.keys(gameState.players).filter(
    (p) => p !== gameState.speaker
  );
  return (
    <div>
      {progress ? (
        <p>Next speaker: {progress?.Politics?.newSpeaker}</p>
      ) : (
        <fieldset>
          <legend>Select next speaker</legend>
          <div className={styles.selectPrimaryContainer}>
            <Dropdown onChange={(e) => setNextSpeaker(e.target.value)}>
              <option value="">--Select player--</option>
              {nonSpeakerPlayers.map((p) => (
                <option key={p} value={p}>
                  {p}
                </option>
              ))}
            </Dropdown>
            <Button
              className={styles.marginTop}
              disabled={nextSpeaker === ""}
              onClick={() =>
                sendMessage({
                  StrategicActionPrimary: {
                    player: gameState.currentPlayer!!,
                    action: {
                      Politics: {
                        newSpeaker: nextSpeaker,
                      },
                    },
                  },
                })
              }
            >
              Commit
            </Button>
          </div>
        </fieldset>
      )}
    </div>
  );
};
