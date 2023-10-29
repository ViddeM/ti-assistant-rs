"use client";

import { GameState, Player } from "@/api/Game";
import { FactionResponse, GameOptions } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";
import styles from "./SetupPhase.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import { Faction } from "@/resources/types/factions";

export interface SetupPhaseProps {
  gameOptions: GameOptions;
  gameState: GameState;
  sendMessage: (data: any) => void;
}

const FACTION_NOT_SELECTED = "not_selected";

export const SetupPhase = ({
  gameOptions,
  gameState,
  sendMessage,
}: SetupPhaseProps) => {
  const playerCount = Object.keys(gameState.players).length;
  const allowedNumberOfPlayers = gameOptions.playerCounts.includes(playerCount);

  const availableFactions = getAvailableFactions(
    gameOptions.factions,
    Object.values(gameState.players)
  );

  const addPlayer = (name: string, faction: Faction) => {
    sendMessage({
      AddPlayer: {
        player: {
          name: name,
          faction: faction,
          planets: [],
        },
      },
    });
  };

  return (
    <div className={`card ${styles.setupCard}`}>
      <h2>Setup</h2>
      {Object.entries(gameState.players).map(([playerId, player]) => (
        <DisplayPlayer key={playerId} {...player} />
      ))}
      {playerCount < 8 && (
        <AddPlayer
          availableFactions={availableFactions}
          addPlayer={addPlayer}
        />
      )}
      <Button
        disabled={!allowedNumberOfPlayers}
        onClick={() => {
          sendMessage("StartGame");
        }}
      >
        Start game
      </Button>
    </div>
  );
};

const DisplayPlayer = (player: Player) => {
  return (
    <div className={styles.displayPlayerContainer}>
      <h3>{player.name}</h3>
      <div className={styles.factionRow}>
        <FactionIcon faction={player.faction} />
        <p className={styles.factionName}>{player.faction}</p>
      </div>
    </div>
  );
};

interface AddPlayerProps {
  availableFactions: FactionResponse[];
  addPlayer: (name: string, faction: Faction) => void;
}

const AddPlayer = ({ availableFactions, addPlayer }: AddPlayerProps) => {
  const [newPlayerName, setNewPlayerName] = useState<string>("");
  const [newPlayerFaction, setNewPlayerFaction] = useState<Faction | null>(
    null
  );

  const resetForm = () => {
    setNewPlayerName("");
    setNewPlayerFaction(null);
  };

  return (
    <form
      className={styles.addPlayerForm}
      onSubmit={(e) => {
        e.preventDefault();

        if (newPlayerFaction !== null) {
          addPlayer(newPlayerName, newPlayerFaction);
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
      <div>
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
      <Button
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
