import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./Primary.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { stringSort } from "@/utils/Utils";

export const PoliticsPrimaryView = () => {
  const { gameState, sendEvent } = useGameContext();

  const [nextSpeaker, setNextSpeaker] = useState<string>("");

  const progress = gameState.actionProgress?.Strategic?.primary;
  const nonSpeakerPlayers = Object.keys(gameState.players)
    .filter((p) => p !== gameState.speaker)
    .sort(stringSort);

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
                sendEvent({
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
