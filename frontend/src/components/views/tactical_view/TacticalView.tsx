import { Planet } from "@/api/bindings/Planet";
import { PlanetAttachment } from "@/api/bindings/PlanetAttachment";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./TacticalView.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const TacticalView = () => {
  const { gameState, gameOptions, sendEvent, isActive } = useGameContext();

  const [selectedPlanet, setSelectedPlanet] = useState<Planet | null>(null);
  const currentPlayerPlanets =
    gameState.players[gameState.currentPlayer!!].planets;

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Tactical") {
    return;
  }
  const tactical = progress;
  const activatedSystem = tactical.activatedSystem;
  const takenPlanets = tactical.takenPlanets;
  const attachments = tactical.planetAttachments;
  const anyPlayerOwnsMirage =
    Object.values(gameState.players)
      .flatMap((p) => Object.keys(p.planets) as Planet[])
      .find((p) => p === "Mirage") !== undefined;

  const availablePlanetsInSystem = Object.values(gameOptions.systems)
    .filter((s) => s.id === activatedSystem)
    .flatMap((s) => s.planets)
    .filter(
      (p) =>
        !Object.keys(currentPlayerPlanets).includes(p) &&
        !Object.keys(takenPlanets).includes(p),
    )
    .map((p) => {
      return {
        id: p,
        ...gameOptions.planetInfos[p],
      };
    })
    .sort(nameSort);

  const allPlanetsNotOwned = Object.keys(gameOptions.planetInfos)
    .map((p) => {
      return p as Planet;
    })
    .filter((p) => p !== "Mirage" || anyPlayerOwnsMirage) // Only allow players to take mirage if someone else owns it, otherwise they'll need to use a frontier action.
    .filter((p) => !Object.keys(currentPlayerPlanets).includes(p))
    .map((p) => {
      return {
        id: p,
        ...gameOptions.planetInfos[p],
      };
    })
    .sort(nameSort);

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
      {isActive ? (
        <>
          {Object.keys(takenPlanets).length > 0 ? (
            <div className={styles.column}>
              {Object.keys(takenPlanets)

                .map((p) => {
                  return p as Planet;
                })
                .map((p) => (
                  <fieldset key={p}>
                    <legend>{p}</legend>

                    <div className={styles.column}>
                      <SelectPlanetAttachment
                        planet={p}
                        attachment={attachments[p] ?? null}
                        previousOwner={takenPlanets[p]}
                        selectAttachment={(attachment) => {
                          sendEvent({
                            TacticalActionAttachPlanetAttachment: {
                              player: gameState.currentPlayer,
                              planet: p,
                              attachment: attachment,
                            },
                          });
                        }}
                      />
                    </div>
                  </fieldset>
                ))}
              {availablePlanetsInSystem.length > 0 && (
                <>
                  <fieldset>
                    <legend>Take another planet</legend>
                    <div className={styles.selectAnotherPlanetContainer}>
                      {availablePlanetsInSystem.map((p) => (
                        <Button key={p.id} onClick={() => takePlanet(p.id)}>
                          {p.name}
                        </Button>
                      ))}
                    </div>
                  </fieldset>
                </>
              )}
            </div>
          ) : (
            <div className={styles.takePlanetContainer}>
              <label>Take planet:</label>
              <Dropdown
                onChange={(e) => {
                  const v = e.target.value;
                  setSelectedPlanet(v === "" ? null : (v as Planet));
                }}
              >
                <option value={""}>--select a planet--</option>
                {allPlanetsNotOwned.map((p) => (
                  <option key={p.id} value={p.id}>
                    {p.name}
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
        </>
      ) : (
        <p>Not your turn, waiting for {gameState.currentPlayer}</p>
      )}
    </div>
  );
};

interface SelectPlanetAttachmentProps {
  planet: Planet;
  attachment: PlanetAttachment | null;
  previousOwner: string | null;
  selectAttachment: (attachment: PlanetAttachment) => void;
}

export const SelectPlanetAttachment = ({
  planet,
  attachment,
  previousOwner,
  selectAttachment,
}: SelectPlanetAttachmentProps) => {
  const { gameState, gameOptions } = useGameContext();
  const [selectedAttachment, setSelectedAttachment] = useState<string>("");

  const planetInfo = gameOptions.planetInfos[planet];

  if (planetInfo.planetTrait === null) {
    return <div>Cannot explore</div>;
  }

  if (previousOwner !== null) {
    return <p>Taken from {gameState.players[previousOwner].name}</p>;
  }

  if (attachment !== null) {
    return <p>{gameOptions.planetAttachments[attachment].name}</p>;
  }

  const availableAttachments = Object.keys(gameOptions.planetAttachments)
    .map((a) => {
      return a as PlanetAttachment;
    })
    .filter(
      (a) =>
        gameOptions.planetAttachments[a].planetTrait === planetInfo.planetTrait,
    )
    .filter((a) => !a.toLocaleLowerCase().endsWith("resources"))
    .map((a) => {
      return {
        id: a,
        ...gameOptions.planetAttachments[a],
      };
    })
    .sort(nameSort);

  return (
    <>
      <Dropdown
        value={selectedAttachment}
        onChange={(e) => setSelectedAttachment(e.target.value)}
      >
        <option value="">--Select attachment--</option>
        {availableAttachments.map((a) => (
          <option key={a.id} value={a.id}>
            {a.name}
          </option>
        ))}
      </Dropdown>
      <Button
        disabled={
          availableAttachments.length === 0 || selectedAttachment === ""
        }
        className={"marginTop"}
        onClick={() => selectAttachment(selectedAttachment as PlanetAttachment)}
      >
        Attach
      </Button>
    </>
  );
};
