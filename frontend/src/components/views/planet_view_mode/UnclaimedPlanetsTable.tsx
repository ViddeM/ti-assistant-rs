import { FactionButton } from "@/components/elements/factionButton/FactionButton";
import React, { useState } from "react";
import styles from "./PlanetViewMode.module.scss";
import { useGameContext } from "@/hooks/GameContext";

export const UnclaimedPlanetsTable = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [planetFilter, setPlanetFilter] = useState<string>("");
  const filter = planetFilter.toLowerCase();

  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });
  const playerCount = players.length;

  const claimedPlanets = players.flatMap((p) => Object.keys(p.planets));
  const unclaimedPlanets = Object.keys(gameOptions.planetInfos)
    .filter((p) => !claimedPlanets.includes(p))
    .map((p) => {
      return {
        id: p,
        ...gameOptions.planetInfos[p],
      };
    })
    .filter((p) => p.id.toLowerCase().includes(filter));

  return (
    <div className={styles.unclaimedPlanetsContainer}>
      <div className="card">
        <table>
          <thead>
            <tr>
              <th colSpan={playerCount}>
                <h2>Unclaimed planets</h2>
              </th>
            </tr>
            <tr>
              <th colSpan={playerCount}>
                <input
                  placeholder="Filter planets"
                  value={planetFilter}
                  onChange={(e) => setPlanetFilter(e.target.value)}
                  className={styles.planetFilter}
                />
              </th>
            </tr>
          </thead>
          <tbody>
            {unclaimedPlanets.map((planet) => (
              <React.Fragment key={planet.id}>
                <tr>
                  <th colSpan={playerCount}>
                    <h4>{planet.id}</h4>
                  </th>
                </tr>
                <tr>
                  {players.map((player) => (
                    <td key={player.faction} align="center">
                      <FactionButton
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
                    </td>
                  ))}
                </tr>
              </React.Fragment>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};
