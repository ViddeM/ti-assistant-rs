import { GameState } from "@/api/Game";
import { GameOptions } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./TacticalView.module.scss";

export interface TacticalViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const TacticalView = ({
  gameState,
  gameOptions,
  sendMessage,
}: TacticalViewProps) => {
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
    sendMessage({
      TacticalActionTakePlanet: {
        player: gameState.currentPlayer,
        planet: planet,
      },
    });
  };

  return (
    <div className={`${styles.column} card`}>
      <h2>Tactical</h2>
      {takenPlanets?.length > 0 ? (
        <div className={styles.column}>
          <div>
            {takenPlanets.map((p) => (
              <p>{p}</p>
            ))}
          </div>
          {availablePlanetsInSystem.length > 0 && (
            <div>
              <h6>Take another planet</h6>
              <div className={styles.selectAnotherPlanetContainer}>
                {availablePlanetsInSystem.map((p) => (
                  <Button onClick={() => takePlanet(p)}>{p}</Button>
                ))}
              </div>
            </div>
          )}
        </div>
      ) : (
        <div>
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
          sendMessage({
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
