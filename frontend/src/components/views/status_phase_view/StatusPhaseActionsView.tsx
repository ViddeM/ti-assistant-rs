import { GameOptions } from "@/api/GameOptions";
import { GameState, Player } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import React, { useState } from "react";

export interface StatusPhaseActionsViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StatusPhaseActionsView = ({
  gameState,
  gameOptions,
  sendMessage,
}: StatusPhaseActionsViewProps) => {
  return (
    <div className="card">
      <ScoreObjectives
        gameState={gameState}
        gameOptions={gameOptions}
        sendMessage={sendMessage}
      />
    </div>
  );
};

interface ScoreObjectivesProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

const ScoreObjectives = ({
  gameState,
  gameOptions,
  sendMessage,
}: ScoreObjectivesProps) => {
  const [revealedObjective, setRevealedObjective] = useState<string>("");

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

  const allObjectives = Object.keys(gameOptions.objectives).map((o) => {
    return {
      id: o,
      ...gameOptions.objectives[o],
    };
  });

  let stageName;
  let selectableObjectives;
  // TODO: Support special cases (e.g. Incentive Program agenda)
  if (revealedObjectives.length >= 5) {
    selectableObjectives = allObjectives.filter((o) => o.kind === "StageII");
    stageName = "Stage II";
  } else {
    selectableObjectives = allObjectives.filter((o) => o.kind === "StageI");
    stageName = "Stage I";
  }

  return (
    <div>
      <h2>Score Objectives</h2>
      {players.map((p) => (
        <PlayerObjectives
          key={p.id}
          gameState={gameState}
          gameOptions={gameOptions}
          player={p}
          sendMessage={sendMessage}
        />
      ))}
      <h2>Reveal {stageName} Objective</h2>
      {state.revealedObjective !== null ? (
        <p>{state.revealedObjective}</p>
      ) : (
        <>
          <Dropdown
            disabled={!revealUnlocked}
            value={revealedObjective}
            onChange={(e) => setRevealedObjective(e.target.value)}
          >
            <option>--Select Objective to Reveal--</option>
            {selectableObjectives.map((o) => (
              <option key={o.id} value={o.id}>
                {o.name}
              </option>
            ))}
          </Dropdown>
          <Button
            onClick={() =>
              sendMessage({
                RevealPublicObjective: {
                  objective: revealedObjective,
                },
              })
            }
          >
            Reveal
          </Button>
        </>
      )}
    </div>
  );
};

interface PlayerObjectivesProps {
  gameState: GameState;
  gameOptions: GameOptions;
  player: Player & { id: string };
  sendMessage: (data: any) => void;
}

const PlayerObjectives = ({
  gameState,
  gameOptions,
  player,
  sendMessage,
}: PlayerObjectivesProps) => {
  const pub = gameState.statusPhaseState!!.scoredPublicObjectives[player.id];
  const sec = gameState.statusPhaseState!!.scoredSecretObjectives[player.id];
  const availablePubs = Object.keys(gameState.score.revealedObjectives).filter(
    (o) => !gameState.score.revealedObjectives[o].includes(player.id)
  );

  const playerScoredSecrets = gameState.score.secretObjectives[player.id] ?? [];
  const availableSecs = Object.keys(gameOptions.objectives)
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind !== "StageI" && o.kind !== "StageII")
    .filter((o) => !playerScoredSecrets.includes(o.id));

  const [selectedPub, setSelectedPub] = useState<string>("");
  const [selectedSec, setSelectedSec] = useState<string>("");

  const scorePublic = (obj: string | null) => {
    sendMessage({
      ScorePublicObjective: {
        player: player.id,
        objective: obj,
      },
    });
  };
  const scoreSecret = (obj: string | null) => {
    sendMessage({
      ScoreSecretObjective: {
        player: player.id,
        objective: obj,
      },
    });
  };

  return (
    <div>
      <h4>{player.name}</h4>
      {pub !== undefined ? (
        <p>Public: {pub ? pub : "Skipped"}</p>
      ) : (
        <div>
          <Dropdown
            disabled={availablePubs.length === 0}
            value={selectedPub}
            onChange={(e) => setSelectedPub(e.target.value)}
          >
            {availablePubs.length === 0 ? (
              <option>--No public objectives available--</option>
            ) : (
              <>
                <option value="">--Select public objective--</option>
                {availablePubs.map((o) => (
                  <option key={o} value={o}>
                    {o}
                  </option>
                ))}
              </>
            )}
          </Dropdown>
          <Button
            disabled={selectedPub === ""}
            onClick={() => scorePublic(selectedPub)}
          >
            Score
          </Button>
          <Button onClick={() => scorePublic(null)}>Skip</Button>
        </div>
      )}
      {sec !== undefined ? (
        <p>Secret: {sec ? sec : "Skipped"}</p>
      ) : (
        <div>
          <Dropdown
            value={selectedSec}
            onChange={(e) => setSelectedSec(e.target.value)}
          >
            <option value="">--Select secret objective--</option>
            {availableSecs.map((o) => (
              <option key={o.id} value={o.id}>
                {o.name}
              </option>
            ))}
          </Dropdown>
          <Button
            disabled={selectedSec === ""}
            onClick={() => scoreSecret(selectedSec)}
          >
            Score
          </Button>
          <Button onClick={() => scoreSecret(null)}>Skip</Button>
        </div>
      )}
    </div>
  );
};
