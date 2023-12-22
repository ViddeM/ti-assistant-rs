import { GameState, Player } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { useState } from "react";
import styles from "./ScoreViewMode.module.scss";

export interface SupportForTheThroneViewProps {
  gameState: GameState;
  sendEvent: (data: any) => void;
}

export const SupportForTheThroneView = ({
  gameState,
  sendEvent,
}: SupportForTheThroneViewProps) => {
  const players = Object.keys(gameState.players).map((p) => {
    return {
      ...gameState.players[p],
      id: p,
    };
  });

  return (
    <div className="card" style={{ marginBottom: "1rem" }}>
      <div className={styles.spftttImperialContainer}>
        <table className={styles.supportForTheThroneTable}>
          <thead>
            <tr>
              <th colSpan={5}>
                <h2>Support for the Throne</h2>
              </th>
            </tr>
          </thead>
          <tbody>
            {players.map((p) => (
              <PlayerSupportForTheThroneView
                key={p.id}
                gameState={gameState}
                player={p}
                allPlayers={players}
                sendEvent={sendEvent}
              />
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

interface PlayerSupportForTheThroneViewProps {
  gameState: GameState;
  sendEvent: (data: any) => void;
  player: Player & {
    id: string;
  };
  allPlayers: (Player & { id: string })[];
}

const PlayerSupportForTheThroneView = ({
  gameState,
  sendEvent,
  player,
  allPlayers,
}: PlayerSupportForTheThroneViewProps) => {
  const currentSelectedPlayer =
    gameState.score.supportForTheThrone[player.id] ?? "";
  const [selectedPlayer, setSelectedPlayer] = useState<string>(
    currentSelectedPlayer
  );

  return (
    <tr>
      <td>
        <FactionIcon faction={player.faction} />
      </td>
      <td>{player.name}</td>
      <td>{"->"}</td>
      <td>
        <Dropdown
          value={selectedPlayer}
          onChange={(e) => setSelectedPlayer(e.target.value)}
        >
          <option value="">None</option>
          {allPlayers
            .filter((p) => p.id !== player.id)
            .map((p) => (
              <option value={p.id} key={p.id}>
                {p.name}
              </option>
            ))}
        </Dropdown>
      </td>
      <td>
        <Button
          disabled={currentSelectedPlayer === selectedPlayer}
          onClick={() =>
            sendEvent({
              GiveSupportForTheThrone: {
                giver: player.id,
                receiver: selectedPlayer,
              },
            })
          }
        >
          Give
        </Button>
      </td>
    </tr>
  );
};
