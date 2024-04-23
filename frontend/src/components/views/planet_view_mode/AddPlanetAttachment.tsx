import { Planet } from "@/api/bindings/Planet";
import { PlanetAttachment } from "@/api/bindings/PlanetAttachment";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import styles from "./PlanetViewMode.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { useEffect, useMemo, useState } from "react";
import { nameSort, stringSort } from "@/utils/Utils";

/* List of attachments added only for technical purposes. */
const ADDED_ATTACHMENTS = [
  "BioticResearchFacilityResources",
  "CyberneticResearchFacilityResources",
  "PropulsionResearchFacilityResources",
  "WarfareResearchFacilityResources",
];

export const AddPlanetAttachment = () => {
  const [player, setPlayer] = useState<string>("");
  const [planet, setPlanet] = useState<Planet | "">("");
  const [attachment, setAttachment] = useState<PlanetAttachment | "">("");

  const { gameState, gameOptions, sendEvent } = useGameContext();

  const availablePlayers = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  const availablePlanets = useMemo(
    () =>
      player === ""
        ? []
        : Object.keys(
            availablePlayers.filter((p) => p.id === player)[0].planets,
          )
            .map((p) => {
              return p as Planet;
            })
            .map((p) => {
              return {
                id: p,
                ...gameOptions.planetInfos[p],
              };
            })
            .sort(nameSort),
    [availablePlayers, gameOptions.planetInfos, player],
  );

  const homePlanets = Object.values(gameOptions.systems)
    .filter(
      (s) =>
        typeof s.systemType === "object" &&
        Object.keys(s.systemType).includes("HomeSystem"),
    )
    .flatMap((s) => s.planets)
    .sort(stringSort);

  const availableAttachments = useMemo(
    () =>
      planet === ""
        ? []
        : Object.keys(gameOptions.planetAttachments)
            .map((a) => {
              return a as PlanetAttachment;
            })
            .map((a) => {
              return {
                id: a,
                ...gameOptions.planetAttachments[a],
              };
            })
            .filter((a) => !ADDED_ATTACHMENTS.includes(a.id))
            .filter(
              (a) =>
                a.planetTrait === null ||
                gameOptions.planetInfos[planet].planetTrait === a.planetTrait,
            )
            .filter(
              (a) =>
                !gameState.players[player]?.planets[planet]?.includes(a.id),
            )
            .filter(
              (a) => !(a.id === "UITheProgenitor" && planet !== "Elysium"),
            )
            .filter(
              (a) =>
                !(
                  a.id === "Terraform" &&
                  (planet === "MecatolRex" ||
                    gameOptions.planetInfos[planet].isLegendary)
                ),
            )
            .filter(
              (a) =>
                !(
                  (a.id === "Terraform" || a.id === "NanoForge") &&
                  homePlanets.includes(planet)
                ),
            )
            .sort(nameSort),
    [
      gameOptions.planetAttachments,
      gameOptions.planetInfos,
      gameState.players,
      homePlanets,
      planet,
      player,
    ],
  );

  useEffect(() => {
    if (planet !== "" && !availablePlanets.map((p) => p.id).includes(planet)) {
      setPlanet("");
    }
  }, [availablePlanets, planet, setPlanet]);

  useEffect(() => {
    if (
      attachment !== "" &&
      !availableAttachments.map((a) => a.id).includes(attachment)
    ) {
      setAttachment("");
    }
  }, [attachment, availableAttachments, setAttachment]);

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
        onChange={(e) => setPlanet(e.target.value as Planet)}
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
        onChange={(e) => setAttachment(e.target.value as PlanetAttachment)}
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
        onClick={() => {
          sendEvent({
            AddPlanetAttachment: {
              player: player,
              planet: planet,
              attachment: attachment,
            },
          });
          setAttachment("");
          setPlanet("");
          setPlayer("");
        }}
      >
        Add attachment
      </Button>
    </div>
  );
};
