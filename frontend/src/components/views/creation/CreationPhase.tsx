"use client";

import { Color, FactionResponse, GameOptions } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";
import styles from "./CreationPhase.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import { Faction } from "@/resources/types/factions";
import { GameState, Player } from "@/api/GameState";

export interface CreationPhaseProps {
  gameOptions: GameOptions;
  gameState: GameState;
  sendMessage: (data: any) => void;
}

const FACTION_NOT_SELECTED = "not_selected";

export const CreationPhase = ({
  gameOptions,
  gameState,
  sendMessage,
}: CreationPhaseProps) => {
  const playerCount = Object.keys(gameState.players).length;
  const allowedNumberOfPlayers = gameOptions.playerCounts.includes(playerCount);

  const availableFactions = getAvailableFactions(
    gameOptions.factions,
    Object.values(gameState.players)
  );
  const takenColors = Object.values(gameState.players).map((p) => p.color);

  const addPlayer = (name: string, faction: Faction, color: Color) => {
    sendMessage({
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
    <div className={`card ${styles.setupCard}`}>
      <h2>Add players</h2>
      {Object.entries(gameState.players).map(([playerId, player]) => (
        <DisplayPlayer
          key={playerId}
          player={player}
          gameOptions={gameOptions}
        />
      ))}
      {playerCount < 8 && (
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
          sendMessage("CreationDone");
        }}
      >
        Start game
      </Button>
    </div>
  );
};

interface DisplayPlayerProps {
  gameOptions: GameOptions;
  player: Player;
}

const DisplayPlayer = ({ player, gameOptions }: DisplayPlayerProps) => {
  return (
    <div className={styles.displayPlayerContainer}>
      <h3>{player.name}</h3>
      <div className={styles.factionRow}>
        <FactionIcon faction={player.faction} />
        <p className={styles.factionName}>
          {
            gameOptions.factions.filter((f) => f.faction === player.faction)[0]
              .name
          }
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
    null
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
  players: Player[]
): FactionResponse[] {
  const takenFactions = players.map((p) => p.faction);
  return allFactions.filter((f) => takenFactions.includes(f.faction) === false);
}
