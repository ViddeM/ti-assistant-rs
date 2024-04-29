import { Player } from "@/api/bindings/Player";
import { Objective } from "@/api/bindings/Objective";
import { SecretObjective } from "@/api/bindings/SecretObjective";
import styles from "./ScoreViewMode.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { Button } from "@/components/elements/button/Button";
import { InfoButton } from "@/components/elements/button/InfoButton";
import { useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort, stringSort } from "@/utils/Utils";

export const SecretObjectivesView = () => {
  const { gameState, showInfo } = useGameContext();

  const players = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  const playerSecrets = gameState.score.secretObjectives;

  const getPlayerSecrets = (playerId: string) => {
    if (playerSecrets[playerId]) {
      return playerSecrets[playerId].sort(stringSort);
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
          player={p}
          playerSecrets={getPlayerSecrets(p.id)}
        />
      ))}
    </div>
  );
};

interface PlayerSecretViewProps {
  playerId: string;
  player: Player;
  playerSecrets: SecretObjective[];
}

const PlayerSecretView = ({
  playerId,
  player,
  playerSecrets,
}: PlayerSecretViewProps) => {
  const { gameState, gameOptions, sendEvent, showInfo } = useGameContext();

  const [secret, setSecret] = useState<SecretObjective | "">("");

  const allTakenSecrets = Object.values(
    gameState.score.secretObjectives,
  ).flat();

  const unrevealedSecrets = Object.keys(gameOptions.objectives)
    .map((o) => {
      return o as Objective;
    })
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind !== "StageI" && o.kind !== "StageII")
    .filter((o) => !allTakenSecrets.includes(o.id as SecretObjective))
    .sort(nameSort);

  return (
    <div>
      <div className={styles.playerSecretTitleRow}>
        <FactionIcon faction={player.faction} />
        <h6>{player.name}</h6>
        <div />
      </div>
      {playerSecrets.map((secret) => (
        <div key={secret} className={styles.secretObjectiveRow}>
          {gameOptions.objectives[secret].name}
          <InfoButton info={{ Objective: gameOptions.objectives[secret] }} />
          <Button
            className={styles.deleteSecretObjectiveButton}
            onClick={() =>
              sendEvent({
                UnscoreSecretObjective: {
                  player: playerId,
                  objective: secret,
                },
              })
            }
          >
            <FontAwesomeIcon icon={faTrash} />
          </Button>
        </div>
      ))}
      <Dropdown
        value={secret}
        onChange={(e) => setSecret(e.target.value as SecretObjective | "")}
      >
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
            ScoreExtraSecretObjective: {
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
