import { Planet } from "@/api/bindings/Planet";
import { RelicProgress } from "@/api/bindings/RelicProgress";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";
import { useState } from "react";

export const RelicCardView = () => {
  const { gameState, gameOptions, isActive } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Relic") {
    return;
  }
  const relic = gameOptions.relics[progress.relic];

  return (
    <div className="card column">
      <h2>{relic.name}</h2>
      {isActive ? (
        <RelicProgressView progress={progress} />
      ) : (
        <p>Not your turn, currently {gameState.currentPlayer} is playing</p>
      )}
    </div>
  );
};

interface RelicProgressViewProps {
  progress: RelicProgress;
}

const RelicProgressView = ({ progress }: RelicProgressViewProps) => {
  const { gameState, sendEvent } = useGameContext();

  switch (progress.relic) {
    case "StellarConverter":
      return <StellarConvertersView />;
    case "NanoForge":
      return <NanoForgeView />;
    default:
      return (
        <Button
          onClick={() =>
            sendEvent({
              RelicActionCommit: {
                player: gameState.currentPlayer,
              },
            })
          }
        >
          Commit
        </Button>
      );
  }
};

const StellarConvertersView = () => {
  const { gameOptions, gameState, sendEvent } = useGameContext();

  const [planet, setPlanet] = useState<string>("");

  const availablePlanets = Object.keys(gameOptions.planetInfos)
    .map((p) => {
      return p as Planet;
    })
    .filter((p) => p !== "MecatolRex")
    .map((p) => {
      return {
        id: p,
        ...gameOptions.planetInfos[p],
      };
    })
    .filter((p) => !p.isLegendary)
    .filter(
      // remove planets that have an attachment that makes them legendary.
      (p) =>
        Object.values(gameState.players)
          .map((player) => player.planets[p.id])
          .filter((p) => p !== undefined)
          .flat()
          .filter((a) => gameOptions.planetAttachments[a].setLegendary)
          .length === 0,
    )
    .filter((p) => {
      let type = Object.values(gameOptions.systems).filter((s) =>
        s.planets.includes(p.id),
      )[0].systemType;
      return (
        typeof type !== "object" || !Object.keys(type).includes("HomeSystem")
      );
    })
    .sort(nameSort);

  return (
    <fieldset>
      <legend>Stellar Converters</legend>
      <div className="column">
        <Dropdown value={planet} onChange={(e) => setPlanet(e.target.value)}>
          <option value="">--Select planet--</option>
          {availablePlanets.map((p) => (
            <option key={p.id} value={p.id}>
              {p.name}
            </option>
          ))}
        </Dropdown>
        <Button
          className="marginTop"
          disabled={planet === ""}
          onClick={() =>
            sendEvent({
              RelicActionCommit: {
                player: gameState.currentPlayer,
                data: {
                  StellarConverter: {
                    planet: planet,
                  },
                },
              },
            })
          }
        >
          Commit
        </Button>
      </div>
    </fieldset>
  );
};

const NanoForgeView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [planet, setPlanet] = useState<string>("");

  const availablePlanets = Object.keys(
    gameState.players[gameState.currentPlayer!!].planets,
  )
    .map((p) => {
      return p as Planet;
    })
    .map((p) => {
      return {
        id: p,
        attachments: gameState.players[gameState.currentPlayer!!].planets[p],
        ...gameOptions.planetInfos[p],
      };
    })
    .filter((p) => !p.isLegendary)
    .filter((p) => !p.attachments.includes("NanoForge"))
    .filter((p) => {
      let type = Object.values(gameOptions.systems).filter((s) =>
        s.planets.includes(p.id),
      )[0].systemType;
      return (
        typeof type !== "object" || !Object.keys(type).includes("HomeSystem")
      );
    })
    .sort(nameSort);

  return (
    <fieldset>
      <legend>Nano-Forge</legend>
      <div className="column">
        <Dropdown value={planet} onChange={(e) => setPlanet(e.target.value)}>
          <option value="">--Select planet--</option>
          {availablePlanets.map((p) => (
            <option key={p.id} value={p.id}>
              {p.name}
            </option>
          ))}
        </Dropdown>
        <Button
          className="marginTop"
          disabled={planet === ""}
          onClick={() =>
            sendEvent({
              RelicActionCommit: {
                player: gameState.currentPlayer,
                data: {
                  NanoForge: {
                    planet: planet,
                  },
                },
              },
            })
          }
        >
          Commit
        </Button>
      </div>
    </fieldset>
  );
};
