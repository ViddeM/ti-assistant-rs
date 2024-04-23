import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./Primary.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { stringSort } from "@/utils/Utils";

export const PoliticsPrimaryView = () => {
  const { gameState, sendEvent, isActive } = useGameContext();

  const [nextSpeaker, setNextSpeaker] = useState<string>("");

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }
  const primary = progress.primary;

  const nonSpeakerPlayers = Object.keys(gameState.players)
    .filter((p) => p !== gameState.speaker)
    .sort(stringSort);

  return (
    <div>
      {primary && "Politics" in primary ? (
        <p>Next speaker: {primary.Politics.newSpeaker}</p>
      ) : isActive ? (
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
      ) : (
        <p>Waiting for {gameState.currentPlayer} to pick</p>
      )}
    </div>
  );
};
