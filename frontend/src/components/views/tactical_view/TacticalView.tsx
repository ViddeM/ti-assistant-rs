import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./TacticalView.module.scss";
import { useGameContext } from "@/hooks/GameContext";

export const TacticalView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [selectedPlanet, setSelectedPlanet] = useState<string | null>(null);
  const currentPlayerPlanets =
    gameState.players[gameState.currentPlayer!!].planets;

  const activatedSystem = gameState.actionProgress?.Tactical?.activatedSystem;
  const takenPlanets = gameState.actionProgress?.Tactical?.takenPlanets!!;

  const availablePlanetsInSystem = gameOptions.systems
    .filter((s) => s.id === activatedSystem)
    .flatMap((s) => s.planets)
    .filter(
      (p) => !currentPlayerPlanets.includes(p) && !takenPlanets.includes(p)
    );

  const takePlanet = (planet: string) => {
    sendEvent({
      TacticalActionTakePlanet: {
        player: gameState.currentPlayer,
        planet: planet,
      },
    });
  };

  return (
    <div className={`${styles.tacticalContainer} card`}>
      <h2>Tactical</h2>
      {takenPlanets?.length > 0 ? (
        <div className={styles.column}>
          <div>
            {takenPlanets.map((p) => (
              <p key={p}>{p}</p>
            ))}
          </div>
          {availablePlanetsInSystem.length > 0 && (
            <fieldset>
              <legend>Take another planet</legend>
              <div className={styles.selectAnotherPlanetContainer}>
                {availablePlanetsInSystem.map((p) => (
                  <Button key={p} onClick={() => takePlanet(p)}>
                    {p}
                  </Button>
                ))}
              </div>
            </fieldset>
          )}
        </div>
      ) : (
        <div className={styles.takePlanetContainer}>
          <label>Take planet:</label>
          <Dropdown
            onChange={(e) => {
              const v = e.target.value;
              setSelectedPlanet(v === "" ? null : v);
            }}
          >
            <option value={""}>--select a planet--</option>
            {gameOptions.systems
              .flatMap((s) =>
                s.planets.filter((p) => !currentPlayerPlanets.includes(p))
              )
              .map((p) => (
                <option key={p} value={p}>
                  {p}
                </option>
              ))}
          </Dropdown>
          <Button
            disabled={!selectedPlanet}
            onClick={() => takePlanet(selectedPlanet!!)}
          >
            Take
          </Button>
        </div>
      )}
      <Button
        onClick={() =>
          sendEvent({
            TacticalActionCommit: {
              player: gameState.currentPlayer,
            },
          })
        }
      >
        End Tactical
      </Button>
    </div>
  );
};
