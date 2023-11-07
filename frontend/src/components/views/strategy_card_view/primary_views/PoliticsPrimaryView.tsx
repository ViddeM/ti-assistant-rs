import { GameState } from "@/api/Game";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";

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
        <>
          <label>Select next speaker</label>
          <Dropdown onChange={(e) => setNextSpeaker(e.target.value)}>
            <option value="">--Select player--</option>
            {nonSpeakerPlayers.map((p) => (
              <option key={p} value={p}>
                {p}
              </option>
            ))}
          </Dropdown>
          <Button
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
        </>
      )}
    </div>
  );
};