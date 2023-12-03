import { GameState, Player } from "@/api/GameState";
import styles from "./ScoreViewMode.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { Button } from "@/components/elements/button/Button";
import { useState } from "react";
import { GameOptions } from "@/api/GameOptions";

export interface SecretObjectivesViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const SecretObjectivesView = ({
  gameState,
  gameOptions,
  sendEvent,
}: SecretObjectivesViewProps) => {
  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });

  const playerSecrets = gameState.score.secretObjectives;

  const getPlayerSecrets = (playerId: string) => {
    if (playerSecrets[playerId]) {
      return playerSecrets[playerId];
    }

    return [];
  };

  return (
    <div className="card">
      <h2>Secret Objectives</h2>
      {players.map((p) => (
        <PlayerSecretView
          key={p.id}
          playerId={p.id}
          gameOptions={gameOptions}
          gameState={gameState}
          player={p}
          playerSecrets={getPlayerSecrets(p.id)}
          sendEvent={sendEvent}
        />
      ))}
    </div>
  );
};

interface PlayerSecretViewProps {
  playerId: string;
  player: Player;
  playerSecrets: string[];
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

const PlayerSecretView = ({
  playerId,
  player,
  playerSecrets,
  gameState,
  gameOptions,
  sendEvent,
}: PlayerSecretViewProps) => {
  const [secret, setSecret] = useState<string>("");

  const allTakenSecrets = Object.values(
    gameState.score.secretObjectives
  ).flat();

  const unrevealedSecrets = Object.keys(gameOptions.objectives)
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind !== "StageI" && o.kind !== "StageII")
    .filter((o) => !allTakenSecrets.includes(o.id));

  return (
    <div>
      <div className={styles.playerSecretTitleRow}>
        <FactionIcon faction={player.faction} />
        <h6>{player.name}</h6>
        <div />
      </div>
      {playerSecrets.map((secret) => (
        <p key={secret}>{secret}</p>
      ))}
      <Dropdown value={secret} onChange={(e) => setSecret(e.target.value)}>
        <option value="">--Select secret objective--</option>
        {unrevealedSecrets.map((o) => (
          <option value={o.id} key={o.id}>
            {o.name}
          </option>
        ))}
      </Dropdown>
      <Button
        disabled={secret === ""}
        onClick={() => {
          sendEvent({
            ScoreSecretObjective: {
              player: playerId,
              objective: secret,
            },
          });
          setSecret("");
        }}
      >
        Score
      </Button>
    </div>
  );
};
