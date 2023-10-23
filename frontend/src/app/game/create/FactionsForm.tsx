import { Faction } from "@/api/GameOptions";

export interface Player {
  name: string;
  faction: string;
}

export interface FactionsForm {
  factions: Faction[];
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
    <div>
      <button type="button" onClick={prevStage}>
        Back
      </button>
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
      <button type="submit">Submit</button>
    </div>
  );
};

interface PlayerSelectProps {
  factions: Faction[];
  player: Player;
  setPlayer: (player: Player) => void;
}

const PlayerSelect = ({ factions, player, setPlayer }: PlayerSelectProps) => {
  return (
    <div>
      <div>
        <label>Player Name: </label>
        <input
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
        <label>Player Faction</label>
        <select
          value={player.faction}
          onChange={(e) => {
            setPlayer({
              ...player,
              faction: e.target.value,
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
