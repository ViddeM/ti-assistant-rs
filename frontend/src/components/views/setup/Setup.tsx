import { GameOptions } from "@/api/GameOptions";
import { GameState, Player } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import styles from "./Setup.module.scss";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";

export interface SetupProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const SetupPhase = ({
  gameState,
  gameOptions,
  sendEvent,
}: SetupProps) => {
  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });

  return (
    <div className={`card ${styles.setupContainer}`}>
      <h2>Setup</h2>
      <div className={styles.playersSetupContainer}>
        {players.map((p) => (
          <fieldset className={`playerColorBorder${p.color}`}>
            <legend>{p.name}</legend>
            {gameState.speaker === p.id ? (
              <p>Speaker</p>
            ) : (
              <Button
                disabled={gameState.speaker === p.id}
                onClick={() =>
                  sendEvent({
                    SetupSpeaker: {
                      player: p.id,
                    },
                  })
                }
              >
                Set Speaker
              </Button>
            )}
            <FactionSpecificSetup
              player={p}
              gameOptions={gameOptions}
              sendEvent={sendEvent}
            />
          </fieldset>
        ))}
      </div>
      <Button
        disabled={!gameState.speaker}
        onClick={() => sendEvent("StartGame")}
      >
        Start Game
      </Button>
    </div>
  );
};

interface FactionSpecificSetupProps {
  player: Player & { id: string };
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

const FactionSpecificSetup = ({
  player,
  gameOptions,
  sendEvent,
}: FactionSpecificSetupProps) => {
  switch (player.faction) {
    case "Winnu":
      return (
        <WinnuSetup
          player={player}
          gameOptions={gameOptions}
          sendEvent={sendEvent}
        />
      );
    case "ArgentFlight":
      return <div>Argent</div>;
    case "CouncilKeleres":
      return <div>Council</div>;
    default:
      return <div>Pelle</div>;
  }
};

const WinnuSetup = ({
  player,
  gameOptions,
  sendEvent,
}: FactionSpecificSetupProps) => {
  const [selectedTech, setSelectedTech] = useState<string>("");

  const availableTechs = Object.keys(gameOptions.technologies)
    .map((t) => {
      return {
        id: t,
        ...gameOptions.technologies[t],
      };
    })
    .filter((t) => t.origin === "Base")
    .filter((t) => Object.keys(t.requirements).length === 0);

  return (
    <div>
      {player.technologies.length === 0 ? (
        <div>
          <Dropdown
            value={selectedTech}
            onChange={(e) => setSelectedTech(e.target.value)}
          >
            <option value="">--Select technology--</option>
            {availableTechs.map((t) => (
              <option key={t.id} value={t.id}>
                {t.name}
              </option>
            ))}
          </Dropdown>
          <Button
            disabled={selectedTech === ""}
            onClick={() =>
              sendEvent({
                SetupPlayerTechs: {
                  player: player.id,
                  technologies: [selectedTech],
                },
              })
            }
          >
            Select
          </Button>
        </div>
      ) : (
        <p>{gameOptions.technologies[player.technologies[0]].name}</p>
      )}
    </div>
  );
};
