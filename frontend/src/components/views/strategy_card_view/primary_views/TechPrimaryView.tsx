import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../../select_tech_view/SelectTechView";
import { useState } from "react";
import { useGameContext } from "@/hooks/GameContext";

export const TechnologyPrimaryView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [firstTech, setFirstTech] = useState<string | null>(null);
  const [secondTech, setSecondTech] = useState<string | null>(null);
  const progress = gameState.actionProgress?.Strategic?.primary;

  if (progress) {
    const techProgress = progress!!.Technology!!;
    return (
      <div className="column">
        <p>{gameOptions.technologies[techProgress.tech].name}</p>
        {techProgress.extra && (
          <p>{gameOptions.technologies[techProgress.extra].name}</p>
        )}
      </div>
    );
  }

  return (
    <div className="column">
      {firstTech === null ? (
        <fieldset>
          <legend>
            <h6>Pick a tech</h6>
          </legend>
          <SelectTechView
            playerId={gameState.currentPlayer!!}
            onSelect={setFirstTech}
          />
        </fieldset>
      ) : (
        <>
          <p>{gameOptions.technologies[firstTech].name}</p>
          {secondTech === null ? (
            <fieldset>
              <legend>
                <h6>Take another?</h6>
              </legend>
              <SelectTechView
                playerId={gameState.currentPlayer!!}
                onSelect={setSecondTech}
                filteredTechs={[firstTech]}
              />
            </fieldset>
          ) : (
            <p>{gameOptions.technologies[secondTech].name}</p>
          )}
        </>
      )}
      <Button
        disabled={firstTech === null}
        onClick={() => {
          sendEvent({
            StrategicActionPrimary: {
              player: gameState.currentPlayer!!,
              action: {
                Technology: {
                  tech: firstTech,
                  extra: secondTech,
                },
              },
            },
          });
          setFirstTech(null);
          setSecondTech(null);
        }}
      >
        Done
      </Button>
    </div>
  );
};
