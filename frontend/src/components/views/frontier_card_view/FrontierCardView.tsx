import { FrontierCardProgress } from "@/api/bindings/FrontierCardProgress";
import { useGameContext } from "@/hooks/GameContext";
import { SelectTechView } from "../select_tech_view/SelectTechView";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";

export const FrontierCardView = () => {
  const { gameState, gameOptions, isActive } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "FrontierCard") {
    return;
  }

  const card = gameOptions.frontierCards[progress.card];
  return (
    <div className="card column">
      <h2>{card.name}</h2>
      {isActive ? (
        <FrontierCardProgressView cardProgress={progress} />
      ) : (
        <p>Not your turn, currently {gameState.currentPlayer} is playing</p>
      )}
    </div>
  );
};

interface FrontierCardProgressViewProps {
  cardProgress: FrontierCardProgress;
}

const FrontierCardProgressView = ({
  cardProgress,
}: FrontierCardProgressViewProps) => {
  const { gameState, sendEvent } = useGameContext();

  switch (cardProgress.card) {
    case "EnigmaticDevice":
      return (
        <div className={"screenContainer"}>
          <SelectTechView
            playerId={gameState.currentPlayer!!}
            onSelect={(tech) =>
              sendEvent({
                FrontierCardActionCommit: {
                  player: gameState.currentPlayer,
                  data: {
                    EnigmaticDevice: {
                      tech: tech,
                    },
                  },
                },
              })
            }
          />
        </div>
      );
    case "Mirage":
      return <MirageView />;
    default:
      return (
        <Button
          onClick={() =>
            sendEvent({
              FrontierCardActionCommit: {
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

const MirageView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();
  const [selectedSystem, setSelectedSystem] = useState<string>("");

  if (!gameState.hexMap) {
    return (
      <Button
        onClick={() =>
          // Send event to take the planet.
          console.log("TODO")
        }
      >
        Take planet
      </Button>
    );
  }

  const systemsWithoutPlanets = Object.values(gameOptions.systems)
    .filter((s) => s.planets.length === 0)
    .map((s) => s.id);

  const usedSystemsWithoutPlanets = gameState.hexMap.tiles
    .filter((t) => systemsWithoutPlanets.includes(t.system))
    .filter((t) => gameOptions.systems[t.system].systemType !== "Hyperlane");

  return (
    <div>
      <fieldset className={`centerRow`}>
        <legend>Pick system</legend>
        <Dropdown
          onChange={(e) => {
            setSelectedSystem(e.target.value);
          }}
        >
          <option value="">--Select System--</option>
          {usedSystemsWithoutPlanets.map((sys) => (
            <option key={sys.system} value={sys.system}>
              System {sys.system}
            </option>
          ))}
        </Dropdown>
      </fieldset>
      <Button
        disabled={selectedSystem === ""}
        onClick={() => {
          sendEvent({
            FrontierCardActionCommit: {
              player: gameState.currentPlayer,
              data: {
                system: selectedSystem,
              },
            },
          });
        }}
      >
        Commit
      </Button>
    </div>
  );
};
