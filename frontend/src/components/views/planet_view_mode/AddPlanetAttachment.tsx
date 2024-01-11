import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import styles from "./PlanetViewMode.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { useState } from "react";

export const AddPlanetAttachment = () => {
  const [player, setPlayer] = useState<string>("");
  const [planet, setPlanet] = useState<string>("");
  const [attachment, setAttachment] = useState<string>("");

  const { gameState, gameOptions } = useGameContext();

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
            {p.id}
          </option>
        ))}
      </Dropdown>
      <Dropdown
        value={attachment}
        disabled={planet === ""}
        onChange={(e) => setAttachment(e.target.value)}
      >
        <option value="">--Select Attachment--</option>
      </Dropdown>
      <Button>Add attachment</Button>
    </div>
  );
};
