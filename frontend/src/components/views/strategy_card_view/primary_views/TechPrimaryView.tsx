import { Technology } from "@/api/bindings/Technology";
import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../../select_tech_view/SelectTechView";
import { useState } from "react";
import { useGameContext } from "@/hooks/GameContext";
import styles from "./Primary.module.scss";

export const TechnologyPrimaryView = () => {
  const { gameState, gameOptions, sendEvent, isActive } = useGameContext();

  const [firstTech, setFirstTech] = useState<Technology | null>(null);

  const playerFaction = Object.keys(gameState.players)
    .filter((p) => gameState.currentPlayer === p)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    });

  if (playerFaction.length > 0 && playerFaction[0].faction === "NekroVirus") {
    return (
      <div className="column">
        <p>-- Unable to research technologies --</p>
      </div>
    );
  }

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }

  if (progress.primary && "Technology" in progress.primary) {
    const techProgress = progress.primary.Technology!!;

    return (
      <div className="column">
        {techProgress.tech && (
          <p>{gameOptions.technologies[techProgress.tech].name}</p>
        )}
        {techProgress.extra && (
          <p>{gameOptions.technologies[techProgress.extra].name}</p>
        )}
      </div>
    );
  }

  if (!isActive) {
    return <p>Waiting for {gameState.currentPlayer} to pick</p>;
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
            <p className="warningText">Remember: pay 6 resources</p>
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
