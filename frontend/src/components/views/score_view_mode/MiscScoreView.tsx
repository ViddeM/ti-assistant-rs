import { GameOptions } from "@/api/GameOptions";
import { GameState, Player } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { useState } from "react";
import styles from "./ScoreViewMode.module.scss";

export interface MiscScoreViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const MiscScoreView = ({
  gameState,
  gameOptions,
  sendEvent,
}: MiscScoreViewProps) => {
  const [custodians, setCustodians] = useState<string>(
    gameState.score.custodians ?? ""
  );

  const players = Object.keys(gameState.players).map((p) => {
    return {
      ...gameState.players[p],
      id: p,
    };
  });

  const currentCustodians = getCustodians(gameState, gameOptions);

  return (
    <div className="card" style={{ marginBottom: "1rem" }}>
      <h2>Custodians: {currentCustodians}</h2>
      <Dropdown
        value={custodians}
        onChange={(e) => setCustodians(e.target.value)}
      >
        <option value="">--No Custodians--</option>
        {players.map((p) => (
          <option value={p.id} key={p.id}>
            {
              gameOptions.factions.filter((f) => f.faction === p.faction)[0]
                .name
            }
            - {p.name}
          </option>
        ))}
      </Dropdown>
      {/* TODO: Implement custodians event in BE */}
      <Button>Set custodians</Button>

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

        <table className={styles.imperialTable}>
          <thead>
            <tr>
              <th colSpan={5}>
                <h2>Imperial</h2>
              </th>
            </tr>
          </thead>
          <tbody>
            {players.map((p) => (
              <PlayerImperialView
                key={p.id}
                gameState={gameState}
                player={p}
                sendEvent={sendEvent}
              />
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

function getCustodians(gameState: GameState, gameOptions: GameOptions): string {
  const scoreCust = gameState.score.custodians;
  if (!scoreCust) {
    return "None";
  }

  const custPlayer = gameState.players[scoreCust];
  const faction = gameOptions.factions.filter(
    (f) => f.faction === custPlayer.faction
  )[0];
  return `${faction.name} - ${custPlayer.name}`;
}

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

interface PlayerImperialViewProps {
  gameState: GameState;
  sendEvent: (data: any) => void;
  player: Player & {
    id: string;
  };
}

const PlayerImperialView = ({
  gameState,
  sendEvent,
  player,
}: PlayerImperialViewProps) => {
  return (
    <tr>
      <td>
        <FactionIcon faction={player.faction} />
      </td>
      <td>{player.name}</td>
      <td>
        <Button>-</Button>
      </td>
      <td>
        <p>{gameState.score.imperial[player.id] ?? 0}</p>
      </td>
      <td>
        <Button>+</Button>
      </td>
    </tr>
  );
};
