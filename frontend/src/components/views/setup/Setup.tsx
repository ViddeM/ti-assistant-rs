import { GameOptions } from "@/api/GameOptions";
import { GameState, Player } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import styles from "./Setup.module.scss";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import {
  FactionIcon,
  factionIconName,
} from "@/components/elements/factionIcon/FactionIcon";

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
          <fieldset
            className={`playerColorBorder${p.color} ${styles.setupPlayerFieldset}`}
          >
            <legend
              className={`playerColorBorder${p.color} ${styles.setupPlayerLegend}`}
            >
              {p.name}
            </legend>
            <div className={styles.setupRow}>
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
              <FactionIcon faction={p.faction} />
            </div>
            {
              gameOptions.factions.filter((f) => f.faction === p.faction)[0]
                .name
            }
            <FactionSpecificSetup
              player={p}
              gameState={gameState}
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
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

const FactionSpecificSetup = ({
  player,
  gameState,
  gameOptions,
  sendEvent,
}: FactionSpecificSetupProps) => {
  switch (player.faction) {
    case "Winnu":
      return (
        <WinnuSetup
          player={player}
          gameState={gameState}
          gameOptions={gameOptions}
          sendEvent={sendEvent}
        />
      );
    case "ArgentFlight":
      return (
        <ArgentFlightSetup
          player={player}
          gameState={gameState}
          gameOptions={gameOptions}
          sendEvent={sendEvent}
        />
      );
    case "CouncilKeleres":
      return (
        <CouncilKeleresSetup
          player={player}
          gameState={gameState}
          gameOptions={gameOptions}
          sendEvent={sendEvent}
        />
      );
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

const ArgentFlightSetup = ({
  player,
  gameOptions,
  sendEvent,
}: FactionSpecificSetupProps) => {
  const possibleTechs = [
    "NeuralMotivator",
    "SarweenTools",
    "PlasmaScoring",
  ].map((t) => {
    return { id: t, ...gameOptions.technologies[t] };
  });

  const [firstTech, setFirstTech] = useState<string>("");
  const [secondTech, setSecondTech] = useState<string>("");

  return (
    <div className={styles.setupColumn}>
      {player.technologies.length > 0 ? (
        <>
          {player.technologies.map((t) => (
            <p>{gameOptions.technologies[t].name}</p>
          ))}
        </>
      ) : (
        <>
          <Dropdown
            value={firstTech}
            onChange={(e) => setFirstTech(e.target.value)}
          >
            <option value="">--Select technology--</option>
            {possibleTechs
              .filter((t) => t.id !== secondTech)
              .map((t) => (
                <option key={t.id} value={t.id}>
                  {t.name}
                </option>
              ))}
          </Dropdown>
          <Dropdown
            value={secondTech}
            onChange={(e) => setSecondTech(e.target.value)}
          >
            <option value="">--Select technology--</option>
            {possibleTechs
              .filter((t) => t.id !== firstTech)
              .map((t) => (
                <option key={t.id} value={t.id}>
                  {t.name}
                </option>
              ))}
          </Dropdown>

          <Button
            disabled={firstTech === "" || secondTech === ""}
            onClick={() =>
              sendEvent({
                SetupPlayerTechs: {
                  player: player.id,
                  technologies: [firstTech, secondTech],
                },
              })
            }
          >
            Choose
          </Button>
        </>
      )}
    </div>
  );
};

const CouncilKeleresSetup = ({
  player,
  gameState,
  gameOptions,
  sendEvent,
}: FactionSpecificSetupProps) => {
  const [firstTech, setFirstTech] = useState<string>("");
  const [secondTech, setSecondTech] = useState<string>("");

  const possibleTechs = Object.keys(gameState.players)
    .filter((p) => p !== player.name)
    .map((p) => gameState.players[p])
    .flatMap((p) => p.technologies)
    .map((t) => {
      return {
        id: t,
        ...gameOptions.technologies[t],
      };
    })
    .filter((t) => t.origin === "Base");

  return (
    <div className={styles.setupColumn}>
      {player.technologies.length === 0 ? (
        <>
          <Dropdown onChange={(e) => setFirstTech(e.target.value)}>
            <option value="">--Select technology--</option>
            {possibleTechs
              .filter((t) => t.id !== secondTech)
              .map((t) => (
                <option key={t.id} value={t.id}>
                  {t.name}
                </option>
              ))}
          </Dropdown>
          <Dropdown onChange={(e) => setSecondTech(e.target.value)}>
            <option value="">--Select technology--</option>
            {possibleTechs
              .filter((t) => t.id !== firstTech)
              .map((t) => (
                <option key={t.id} value={t.id}>
                  {t.name}
                </option>
              ))}
          </Dropdown>
          <Button
            disabled={firstTech === "" || secondTech === ""}
            onClick={() =>
              sendEvent({
                SetupPlayerTechs: {
                  player: player.id,
                  technologies: [firstTech, secondTech],
                },
              })
            }
          >
            Select technology
          </Button>
        </>
      ) : (
        <>
          <p>{gameOptions.technologies[player.technologies[0]].name}</p>
          <p>{gameOptions.technologies[player.technologies[1]].name}</p>
        </>
      )}
    </div>
  );
};
