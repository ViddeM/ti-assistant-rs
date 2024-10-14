import { FrontierCardProgress } from "@/api/bindings/FrontierCardProgress";
import { useGameContext } from "@/hooks/GameContext";
import { SelectTechView } from "../select_tech_view/SelectTechView";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import { SelectPlanetAttachment } from "../tactical_view/TacticalView";
import { PlanetAttachment } from "@/api/bindings/PlanetAttachment";

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
  const [selectedAttachment, setSelectedAttachment] =
    useState<PlanetAttachment | null>(null);

  const systemsWithoutPlanets = Object.values(gameOptions.systems)
    .filter((s) => s.planets.length === 0)
    .map((s) => s.id);

  const usedSystemsWithoutPlanets =
    gameState.mapData.miltyInformation?.hexMap.tiles
      .filter((t) => systemsWithoutPlanets.includes(t.system))
      .filter((t) => gameOptions.systems[t.system].systemType !== "Hyperlane");

  return (
    <div className="column">
      {usedSystemsWithoutPlanets ? (
        <fieldset className={`centerRow fullWidth`}>
          <legend>Pick system</legend>
          <Dropdown
            value={selectedSystem}
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
      ) : (
        <p>Can only select system when game is imported from milty draft.</p>
      )}

      <fieldset>
        <legend>Select planet attachment</legend>
        <div className="column">
          {selectedSystem === "" ? (
            <p>Select system first...</p>
          ) : (
            <SelectPlanetAttachment
              planet={"Mirage"}
              attachment={selectedAttachment}
              previousOwner={null}
              selectAttachment={(attachment) =>
                setSelectedAttachment(attachment)
              }
            />
          )}
        </div>
      </fieldset>

      <Button
        disabled={selectedSystem === ""}
        onClick={() => {
          sendEvent({
            FrontierCardActionCommit: {
              player: gameState.currentPlayer,
              data: {
                Mirage: {
                  system: selectedSystem,
                  attachment: selectedAttachment,
                },
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
