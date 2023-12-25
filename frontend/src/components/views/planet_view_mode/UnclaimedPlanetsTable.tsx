import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { FactionButton } from "@/components/elements/factionButton/FactionButton";
import React from "react";

export interface UnclaimedPlanetsTable {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const UnclaimedPlanetsTable = ({
  gameState,
  gameOptions,
  sendEvent,
}: UnclaimedPlanetsTable) => {
  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });
  const playerCount = players.length;

  const claimedPlanets = players.flatMap((p) => p.planets);
  const unclaimedPlanets = Object.keys(gameOptions.planetInfos)
    .filter((p) => !claimedPlanets.includes(p))
    .map((p) => {
      return {
        id: p,
        ...gameOptions.planetInfos[p],
      };
    });

  return (
    <div className="card">
      <table>
        <thead>
          <tr>
            <th colSpan={playerCount}>
              <h2>Unclaimed planets</h2>
            </th>
          </tr>
        </thead>
        <tbody>
          {unclaimedPlanets.map((planet) => (
            <React.Fragment key={planet.id}>
              <tr>
                <th>
                  <h4>{planet.id}</h4>
                </th>
              </tr>
              <tr>
                {players.map((player) => (
                  <FactionButton
                    key={player.faction}
                    faction={player.faction}
                    selected={false}
                    onClick={() => {
                      sendEvent({
                        SetPlanetOwner: {
                          player: player.id,
                          planet: planet.id,
                        },
                      });
                    }}
                  />
                ))}
              </tr>
            </React.Fragment>
          ))}
        </tbody>
      </table>
    </div>
  );
};
