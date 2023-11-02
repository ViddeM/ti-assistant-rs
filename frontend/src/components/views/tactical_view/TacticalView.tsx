import { GameState } from "@/api/Game";
import { GameOptions } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";

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

  return (
    <div className="card">
      <h2>Tactical</h2>
      {takenPlanets.length > 0 ? (
        <div>
          <div>
            {takenPlanets.map((p) => (
              <p>{p}</p>
            ))}
          </div>
          <div>
            {gameOptions.systems
              .filter((s) => s.id === activatedSystem)
              .flatMap((s) => s.planets)
              .filter(
                (p) =>
                  !currentPlayerPlanets.includes(p) && !takenPlanets.includes(p)
              )
              .map((p) => (
                <Button>{p}</Button>
              ))}
          </div>
        </div>
      ) : (
        <div>
          <label>Take planet:</label>
          <Dropdown onChange={(e) => setSelectedPlanet(e.target.value)}>
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
            onClick={() =>
              sendMessage({
                TacticalActionTakePlanet: {
                  player: gameState.currentPlayer,
                  planet: selectedPlanet,
                },
              })
            }
          >
            Take
          </Button>
        </div>
      )}
    </div>
  );
};
