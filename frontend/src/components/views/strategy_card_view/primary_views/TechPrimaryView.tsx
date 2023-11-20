import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../common_views/SelectTechView";
import { useState } from "react";
import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";

interface TechnologyPrimaryViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const TechnologyPrimaryView = ({
  gameState,
  gameOptions,
  sendMessage,
}: TechnologyPrimaryViewProps) => {
  const [firstTech, setFirstTech] = useState<string | null>(null);
  const [secondTech, setSecondTech] = useState<string | null>(null);
  const progress = gameState.actionProgress?.Strategic?.primary;

  if (progress) {
    const techProgress = progress!!.Technology!!;
    return (
      <div>
        <p>{techProgress.tech}</p>
        {techProgress.extra && <p>{techProgress.extra}</p>}{" "}
      </div>
    );
  }

  return (
    <div>
      {firstTech === null ? (
        <fieldset>
          <legend>
            <h6>Pick a tech</h6>
          </legend>
          <SelectTechView
            gameState={gameState}
            gameOptions={gameOptions}
            playerId={gameState.currentPlayer!!}
            onSelect={setFirstTech}
          />
        </fieldset>
      ) : (
        <>
          <p>{firstTech}</p>
          {secondTech === null ? (
            <fieldset>
              <legend>
                <h6>Take another?</h6>
              </legend>
              <SelectTechView
                gameState={gameState}
                gameOptions={gameOptions}
                playerId={gameState.currentPlayer!!}
                onSelect={setSecondTech}
              />
            </fieldset>
          ) : (
            <p>{secondTech}</p>
          )}
        </>
      )}
      <Button
        disabled={firstTech === null}
        onClick={() =>
          sendMessage({
            StrategicActionPrimary: {
              player: gameState.currentPlayer!!,
              action: {
                Technology: {
                  tech: firstTech,
                  extra: secondTech,
                },
              },
            },
          })
        }
      >
        Done
      </Button>
    </div>
  );
};
