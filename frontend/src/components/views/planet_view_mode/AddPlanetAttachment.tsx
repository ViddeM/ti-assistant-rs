import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import styles from "./PlanetViewMode.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { useState } from "react";

/* List of attachments added only for technical purposes. */
const ADDED_ATTACHMENTS = [
  "BioticResearchFacilityResources",
  "CyberneticResearchFacilityResources",
  "PropulsionResearchFacilityResources",
  "WarfareResearchFacilityResources",
];

export const AddPlanetAttachment = () => {
  const [player, setPlayer] = useState<string>("");
  const [planet, setPlanet] = useState<string>("");
  const [attachment, setAttachment] = useState<string>("");

  const { gameState, gameOptions, sendEvent } = useGameContext();

  const availablePlayers = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });
  const availablePlanets =
    player === ""
      ? []
      : Object.keys(
          availablePlayers.filter((p) => p.id === player)[0].planets
        ).map((p) => {
          return {
            id: p,
            ...gameOptions.planetInfos[p],
          };
        });

  // TODO: Maybe filter away home planets for relevant cards?
  const availableAttachments =
    planet === ""
      ? []
      : Object.keys(gameOptions.planetAttachments)
          .map((a) => {
            return {
              id: a,
              ...gameOptions.planetAttachments[a],
            };
          })
          .filter((a) => !ADDED_ATTACHMENTS.includes(a.id))
          .filter(
            (a) => gameOptions.planetInfos[planet].planetTrait === a.planetTrait
          )
          .filter(
            (a) => !gameState.players[player].planets[planet].includes(a.id)
          )
          .filter((a) => !(a.id === "UITheProgenitor" && planet !== "Elysium"))
          .filter(
            (a) =>
              !(
                a.id === "Terraform" &&
                (planet === "MecatolRex" ||
                  gameOptions.planetInfos[planet].isLegendary)
              )
          );

  return (
    <div className={`card ${styles.addPlanetAttachmentContainer}`}>
      <h2>Attach to planet</h2>
      <Dropdown value={player} onChange={(e) => setPlayer(e.target.value)}>
        <option value="">--Select Player--</option>
        {availablePlayers.map((p) => (
          <option id={p.id} key={p.id}>
            {p.name}
          </option>
        ))}
      </Dropdown>
      <Dropdown
        value={planet}
        disabled={player === ""}
        onChange={(e) => setPlanet(e.target.value)}
      >
        <option value="">--Select Planet--</option>
        {availablePlanets.map((p) => (
          <option key={p.id} value={p.id}>
            {p.name}
          </option>
        ))}
      </Dropdown>
      <Dropdown
        value={attachment}
        disabled={planet === "" || availableAttachments.length === 0}
        onChange={(e) => setAttachment(e.target.value)}
      >
        {availableAttachments.length === 0 ? (
          <option value="">No attachments available</option>
        ) : (
          <>
            <option value="">--Select Attachment--</option>
            {availableAttachments.map((a) => (
              <option key={a.id} value={a.id}>
                {a.name}
              </option>
            ))}
          </>
        )}
      </Dropdown>
      <Button
        disabled={player === "" || planet === "" || attachment === ""}
        onClick={() =>
          sendEvent({
            AddPlanetAttachment: {
              player: player,
              planet: planet,
              attachment: attachment,
            },
          })
        }
      >
        Add attachment
      </Button>
    </div>
  );
};
