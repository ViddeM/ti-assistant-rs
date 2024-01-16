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

  const tactical = gameState.actionProgress?.Tactical!!;
  const activatedSystem = tactical.activatedSystem;
  const takenPlanets = tactical.takenPlanets;
  const attachments = tactical.planetAttachments;

  const availablePlanetsInSystem = gameOptions.systems
    .filter((s) => s.id === activatedSystem)
    .flatMap((s) => s.planets)
    .filter(
      (p) =>
        !Object.keys(currentPlayerPlanets).includes(p) &&
        !Object.keys(takenPlanets).includes(p)
    );
  const allPlanetsNotOwned = Object.keys(gameOptions.planetInfos)
    .filter((p) => !Object.keys(currentPlayerPlanets).includes(p))
    .map((p) => {
      return {
        id: p,
        ...gameOptions.planetInfos[p],
      };
    });

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
      {Object.keys(takenPlanets).length > 0 ? (
        <div className={styles.column}>
          {Object.keys(takenPlanets).map((p) => (
            <fieldset key={p}>
              <legend>{p}</legend>

              <div className={styles.column}>
                <SelectPlanetAttachment
                  planet={p}
                  attachment={attachments[p] ?? null}
                  previousOwner={takenPlanets[p]}
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
                    <Button key={p} onClick={() => takePlanet(p)}>
                      {gameOptions.planetInfos[p].name}
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
              setSelectedPlanet(v === "" ? null : v);
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
    </div>
  );
};

interface SelectPlanetAttachmentProps {
  planet: string;
  attachment: string | null;
  previousOwner: string | null;
}

const SelectPlanetAttachment = ({
  planet,
  attachment,
  previousOwner,
}: SelectPlanetAttachmentProps) => {
  const { gameState, gameOptions, sendEvent } = useGameContext();
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

  const availableAttachments = Object.keys(
    gameOptions.planetAttachments
  ).filter(
    (a) =>
      gameOptions.planetAttachments[a].planetTrait === planetInfo.planetTrait
  );

  return (
    <>
      <Dropdown
        value={selectedAttachment}
        onChange={(e) => setSelectedAttachment(e.target.value)}
      >
        <option value="">--Select attachment--</option>
        {availableAttachments.map((a) => (
          <option key={a} value={a}>
            {gameOptions.planetAttachments[a].name}
          </option>
        ))}
      </Dropdown>
      <Button
        disabled={
          availableAttachments.length === 0 || selectedAttachment === ""
        }
        className={"marginTop"}
        onClick={() =>
          sendEvent({
            TacticalActionAttachPlanetAttachment: {
              player: gameState.currentPlayer,
              planet: planet,
              attachment: selectedAttachment,
            },
          })
        }
      >
        Attach
      </Button>
    </>
  );
};
