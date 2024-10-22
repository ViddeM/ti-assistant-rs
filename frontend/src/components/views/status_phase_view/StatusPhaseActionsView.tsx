import { Player } from "@/api/bindings/Player";
import { Objective } from "@/api/bindings/Objective";
import { PublicObjective } from "@/api/bindings/PublicObjective";
import { SecretObjective } from "@/api/bindings/SecretObjective";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { InfoButton } from "@/components/elements/button/InfoButton";
import React, { useState } from "react";
import styles from "./StatusPhaseView.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const StatusPhaseActionsView = () => {
  return (
    <div className="card">
      <ScoreObjectives />
    </div>
  );
};

const ScoreObjectives = () => {
  const { gameState, gameOptions, sendEvent, playingAs, isGlobal, isSpeaker } =
    useGameContext();

  const [revealedObjective, setRevealedObjective] = useState<Objective | "">(
    "",
  );

  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });

  const state = gameState.statusPhaseState;
  if (!state) {
    return (
      <div>
        <p>Error: Status Phase State is not set!</p>
      </div>
    );
  }
  const numPublics = Object.keys(state.scoredPublicObjectives).length;
  const numSecrets = Object.keys(state.scoredSecretObjectives).length;
  const numPlayers = Object.keys(gameState.players).length;
  const revealUnlocked = numPlayers === numPublics && numPlayers === numSecrets;

  const revealedObjectives = Object.keys(gameState.score.revealedObjectives);

  const allObjectives = Object.keys(gameOptions.objectives)
    .map((o) => {
      return o as Objective;
    })
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    });
  const unrevealedObjectives = allObjectives.filter(
    (o) => !revealedObjectives.includes(o.id),
  );

  const revealStageII =
    revealedObjectives.length - (state.revealedObjective === null ? 0 : 1) >=
    state.expectedObjectivesBeforeStageTwo;
  const selectableObjectives = revealStageII
    ? unrevealedObjectives.filter((o) => o.kind === "StageII")
    : unrevealedObjectives.filter((o) => o.kind === "StageI");

  const revealedObjectiveInfo = state.revealedObjective
    ? gameOptions.objectives[state.revealedObjective]
    : null;

  return (
    <div>
      <h2>Score Objectives</h2>
      {players
        .filter((p) => p.name === playingAs || isGlobal)
        .map((p) => (
          <PlayerObjectives key={p.id} player={p} />
        ))}
      <fieldset className={styles.revealObjectiveContainer}>
        <legend>
          <h3>Reveal Stage {revealStageII ? "II" : "I"} Objective</h3>
        </legend>
        {revealedObjectiveInfo ? (
          <div className="row">
            <p>{revealedObjectiveInfo.name}</p>
            <InfoButton info={{ Objective: revealedObjectiveInfo }} />
          </div>
        ) : isGlobal || isSpeaker ? (
          <>
            <Dropdown
              disabled={!revealUnlocked}
              value={revealedObjective}
              onChange={(e) =>
                setRevealedObjective(e.target.value as Objective)
              }
            >
              <option value="">--Select Objective to Reveal--</option>
              {selectableObjectives.map((o) => (
                <option key={o.id} value={o.id}>
                  {o.name}
                </option>
              ))}
            </Dropdown>
            <Button
              disabled={revealedObjective === ""}
              onClick={() =>
                sendEvent({
                  RevealPublicObjective: {
                    objective: revealedObjective,
                  },
                })
              }
            >
              Reveal
            </Button>
          </>
        ) : (
          <p>
            Waiting for {gameState.speaker} to reveal a stage{" "}
            {revealStageII ? "II" : "I"} objective
          </p>
        )}
      </fieldset>
    </div>
  );
};

interface PlayerObjectivesProps {
  player: Player & { id: string };
}

const PlayerObjectives = ({ player }: PlayerObjectivesProps) => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const pub = gameState.statusPhaseState!!.scoredPublicObjectives[player.id];
  const sec = gameState.statusPhaseState!!.scoredSecretObjectives[player.id];
  const availablePubs = Object.keys(gameState.score.revealedObjectives)
    .map((o) => {
      return o as PublicObjective;
    })
    .filter((o) => !gameState.score.revealedObjectives[o].includes(player.id))
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .sort(nameSort);

  const playerScoredSecrets = gameState.score.secretObjectives[player.id] ?? [];
  const availableSecs = Object.keys(gameOptions.objectives)
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
    .filter((o) => !playerScoredSecrets.includes(o.id as SecretObjective))
    .sort(nameSort);

  const [selectedPub, setSelectedPub] = useState<PublicObjective | "">("");
  const [selectedSec, setSelectedSec] = useState<SecretObjective | "">("");

  const scorePublic = (obj: string | null) => {
    sendEvent({
      ScorePublicObjective: {
        player: player.id,
        objective: obj,
      },
    });
  };
  const scoreSecret = (obj: string | null) => {
    sendEvent({
      ScoreSecretObjective: {
        player: player.id,
        objective: obj,
      },
    });
  };

  const pubInfo = pub ? gameOptions.objectives[pub] : null;
  const secInfo = sec ? gameOptions.objectives[sec] : null;

  return (
    <fieldset>
      <legend>
        <h4>{player.name}</h4>
      </legend>
      {pub !== undefined ? (
        <p>
          Public:{" "}
          {pubInfo ? (
            <>
              {pubInfo.name}
              <InfoButton info={{ Objective: pubInfo }} />
            </>
          ) : (
            "Skipped"
          )}
        </p>
      ) : (
        <div className={styles.scoreObjectivesContainer}>
          <Dropdown
            disabled={availablePubs.length === 0}
            value={selectedPub}
            onChange={(e) =>
              setSelectedPub(e.target.value as PublicObjective | "")
            }
          >
            {availablePubs.length === 0 ? (
              <option>--No public objectives available--</option>
            ) : (
              <>
                <option value="">--Select public objective--</option>
                {availablePubs.map((o) => (
                  <option key={o.id} value={o.id}>
                    {o.name}
                  </option>
                ))}
              </>
            )}
          </Dropdown>
          <div className={styles.scoreObjectiveButtonsContainer}>
            <Button onClick={() => scorePublic(null)}>Skip</Button>
            <Button
              disabled={selectedPub === ""}
              onClick={() => scorePublic(selectedPub)}
            >
              Score
            </Button>
          </div>
        </div>
      )}
      {sec !== undefined ? (
        <p>
          Secret:{" "}
          {secInfo ? (
            <>
              {secInfo.name}
              <InfoButton info={{ Objective: secInfo }} />
            </>
          ) : (
            "Skipped"
          )}
        </p>
      ) : (
        <div className={styles.scoreObjectivesContainer}>
          <Dropdown
            value={selectedSec}
            onChange={(e) =>
              setSelectedSec(e.target.value as SecretObjective | "")
            }
          >
            <option value="">--Select secret objective--</option>
            {availableSecs.map((o) => (
              <option key={o.id} value={o.id}>
                {o.name}
              </option>
            ))}
          </Dropdown>
          <div className={styles.scoreObjectiveButtonsContainer}>
            <Button onClick={() => scoreSecret(null)}>Skip</Button>
            <Button
              disabled={selectedSec === ""}
              onClick={() => scoreSecret(selectedSec)}
            >
              Score
            </Button>
          </div>
        </div>
      )}
    </fieldset>
  );
};
