import { FactionResponse } from "@/api/GameOptions";
import styles from "./styles.module.scss";
import { Button } from "@/components/elements/button/Button";
import { Faction } from "@/resources/types/factions";

export interface Player {
  name: string;
  faction: Faction;
}

export interface FactionsForm {
  factions: FactionResponse[];
  players: Player[];

  setPlayer: (index: number, player: Player) => void;
  prevStage: () => void;
}

export const FactionsForm = ({
  factions,
  players,
  setPlayer,
  prevStage,
}: FactionsForm) => {
  const selectedFactions = players.map((p) => p.faction);

  return (
    <>
      <Button type="button" onClick={prevStage}>
        Back
      </Button>
      {players.map((p, index) => (
        <PlayerSelect
          key={p.faction}
          player={p}
          setPlayer={(p) => setPlayer(index, p)}
          factions={factions.filter(
            (f) =>
              !selectedFactions.includes(f.faction) || f.faction === p.faction
          )}
        />
      ))}
      <Button type="submit">Submit</Button>
    </>
  );
};

interface PlayerSelectProps {
  factions: FactionResponse[];
  player: Player;
  setPlayer: (player: Player) => void;
}

const PlayerSelect = ({ factions, player, setPlayer }: PlayerSelectProps) => {
  return (
    <div className={styles.playerSelect}>
      <div>
        <label>Player Name: </label>
        <input
          required={true}
          value={player.name}
          onChange={(e) => {
            setPlayer({
              ...player,
              name: e.target.value,
            });
          }}
        />
      </div>

      <div>
        <label>Player Faction: </label>
        <select
          value={player.faction}
          onChange={(e) => {
            setPlayer({
              ...player,
              faction: e.target.value as Faction,
            });
          }}
        >
          {factions.map((f) => (
            <option key={f.faction} value={f.faction}>
              {f.name}
            </option>
          ))}
        </select>
      </div>

      <div>
        <select>
          <option>BLUE</option>
          <option>GREEN</option>
        </select>
      </div>
    </div>
  );
};
