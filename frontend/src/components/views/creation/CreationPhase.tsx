"use client";

import { Color } from "@/api/bindings/Color";
import { FactionResponse } from "@/api/bindings/FactionResponse";
import { GameOptions } from "@/api/bindings/GameOptions";
import { Player } from "@/api/bindings/Player";
import { Faction } from "@/api/bindings/Faction";
import { Button } from "@/components/elements/button/Button";
import styles from "./CreationPhase.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

const FACTION_NOT_SELECTED = "not_selected";

export const CreationPhase = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const playerCount = Object.keys(gameState.players).length;
  const allowedNumberOfPlayers =
    playerCount >= gameOptions.minPlayers &&
    playerCount <= gameOptions.maxPlayers;

  const availableFactions = getAvailableFactions(
    gameOptions.factions,
    Object.values(gameState.players),
  );

  const takenColors = Object.values(gameState.players).map((p) => p.color);

  const players = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  const addPlayer = (name: string, faction: Faction, color: Color) => {
    sendEvent({
      AddPlayer: {
        player: {
          name: name,
          faction: faction,
          color: color,
        },
      },
    });
  };

  return (
    <div className={`card screenContainer ${styles.setupCard}`}>
      <h2>Add players</h2>
      {players.map((p) => (
        <DisplayPlayer key={p.id} player={p} />
      ))}
      {playerCount < gameOptions.maxPlayers && (
        <AddPlayer
          colors={gameOptions.colors}
          takenColors={takenColors}
          availableFactions={availableFactions}
          addPlayer={addPlayer}
        />
      )}
      <Button
        className={styles.startGameButton}
        disabled={!allowedNumberOfPlayers}
        onClick={() => {
          sendEvent("CreationDone");
        }}
      >
        Start game
      </Button>
    </div>
  );
};

interface DisplayPlayerProps {
  player: Player;
}

const DisplayPlayer = ({ player }: DisplayPlayerProps) => {
  const { gameOptions } = useGameContext();

  const faction = gameOptions.factions.filter(
    (f) => f.faction === player.faction,
  )[0];

  return (
    <div className={styles.displayPlayerContainer}>
      <h3>{player.name}</h3>
      <div className={styles.factionRow}>
        <FactionIcon faction={player.faction} />
        <p className={styles.factionName}>{faction.name}:</p>
        <p className={`marginLeft`}>
          <span className={`playerColor${player.color}`}>{player.color}</span>
        </p>
      </div>
    </div>
  );
};

interface AddPlayerProps {
  availableFactions: FactionResponse[];
  colors: Color[];
  takenColors: Color[];
  addPlayer: (name: string, faction: Faction, color: Color) => void;
}

const AddPlayer = ({
  availableFactions,
  addPlayer,
  colors,
  takenColors,
}: AddPlayerProps) => {
  const [newPlayerName, setNewPlayerName] = useState<string>("");
  const [newPlayerFaction, setNewPlayerFaction] = useState<Faction | null>(
    null,
  );
  const [color, setColor] = useState<Color>(colors[0]);

  const resetForm = () => {
    setNewPlayerName("");
    setNewPlayerFaction(null);
    setColor(colors.filter((c) => !takenColors.includes(c) && c !== color)[0]);
  };

  return (
    <form
      className={styles.addPlayerForm}
      onSubmit={(e) => {
        e.preventDefault();

        if (newPlayerFaction !== null) {
          addPlayer(newPlayerName, newPlayerFaction, color);
          resetForm();
        }
      }}
    >
      <div>
        <label htmlFor="player_name_input">Name: </label>
        <input
          id="player_name_input"
          required
          value={newPlayerName}
          onChange={(e) => setNewPlayerName(e.target.value)}
        />
      </div>
      <div className="marginTop">
        <label htmlFor="player_faction_dropdown">Faction: </label>
        <Dropdown
          required
          id="player_faction_dropdown"
          value={newPlayerFaction ? newPlayerFaction : FACTION_NOT_SELECTED}
          onChange={(e) => {
            const val = e.target.value;
            if (val === FACTION_NOT_SELECTED) {
              setNewPlayerFaction(null);
            } else {
              setNewPlayerFaction(val as Faction);
            }
          }}
        >
          <option value={FACTION_NOT_SELECTED}>--Select a faction--</option>
          {availableFactions.map((f) => (
            <option key={f.faction} value={f.faction}>
              {f.name}
            </option>
          ))}
        </Dropdown>
      </div>
      <div className={styles.colorsContainer}>
        {colors.map((c) => (
          <div key={c} className={styles.colorContainer}>
            <label htmlFor={`id-${c}`}>
              <div
                className={`${styles.colorButton} ${
                  takenColors.includes(c)
                    ? styles.disabledColorButton
                    : `playerColorBackground${c}`
                }`}
              />
            </label>
            <input
              name="color"
              id={`id-${c}`}
              type="radio"
              value={c}
              checked={c === color}
              disabled={takenColors.includes(c)}
              onChange={() => setColor(c)}
            />
          </div>
        ))}
      </div>
      <Button
        className="marginTop"
        type="submit"
        disabled={newPlayerName === "" || newPlayerFaction === null}
      >
        Add Player
      </Button>
    </form>
  );
};

function getAvailableFactions(
  allFactions: FactionResponse[],
  players: Player[],
): FactionResponse[] {
  const takenFactions = players.map((p) => p.faction);
  return allFactions
    .filter((f) => takenFactions.includes(f.faction) === false)
    .sort(nameSort);
}
