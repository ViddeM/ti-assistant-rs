import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../../select_tech_view/SelectTechView";
import { useState } from "react";
import { useGameContext } from "@/hooks/GameContext";
import styles from "./Primary.module.scss";

export const TechnologyPrimaryView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [firstTech, setFirstTech] = useState<string | null>(null);
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
    <div className={`column ${styles.primaryContainer}`}>
      {firstTech === null ? (
        <fieldset className={styles.primaryContainer}>
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
          <fieldset className={styles.primaryContainer}>
            <legend>
              <h6>Take another?</h6>
            </legend>
            <SelectTechView
              playerId={gameState.currentPlayer!!}
              onSelect={(secondTech) => {
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
              }}
              filteredTechs={[firstTech]}
            />
          </fieldset>
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
                  extra: null,
                },
              },
            },
          });
          setFirstTech(null);
        }}
      >
        Done
      </Button>
    </div>
  );
};
