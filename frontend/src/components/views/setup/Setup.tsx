import { Player } from "@/api/bindings/Player";
import { Objective } from "@/api/bindings/Objective";
import { Technology } from "@/api/bindings/Technology";
import { PublicObjective } from "@/api/bindings/PublicObjective";
import { Button } from "@/components/elements/button/Button";
import styles from "./Setup.module.scss";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const SetupPhase = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [firstObjective, setFirstObjective] = useState<string>("");
  const [secondObjective, setSecondObjective] = useState<string>("");

  const players = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(
      (a, b) =>
        gameState.tableOrder.indexOf(a.id) - gameState.tableOrder.indexOf(b.id),
    );

  const availableObjectives = Object.keys(gameOptions.objectives)
    .map((o) => {
      return o as Objective;
    })
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .filter((o) => o.kind === "StageI")
    .sort(nameSort);

  const revealedObjectives = Object.keys(gameState.score.revealedObjectives)
    .map((o) => {
      return o as PublicObjective;
    })
    .map((o) => {
      return {
        id: o,
        ...gameOptions.objectives[o],
      };
    })
    .sort(nameSort);

  return (
    <div className={`card ${styles.setupContainer}`}>
      <h2>Setup</h2>

      <div className={styles.playerSpecificSetupContainer}>
        <h3>Player specific setup</h3>

        <div className={styles.playersSetupContainer}>
          {players.map((p) => (
            <fieldset
              key={p.id}
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
              <FactionSpecificSetup player={p} />
            </fieldset>
          ))}
        </div>
      </div>

      <div className={styles.revealObjectivesContainer}>
        <h3>Select initial objectives</h3>
        {revealedObjectives.length === 0 ? (
          <>
            <Dropdown
              value={firstObjective}
              onChange={(e) => setFirstObjective(e.target.value)}
            >
              <option value="">--Select Objective--</option>
              {availableObjectives
                .filter((o) => o.id !== secondObjective)
                .map((o) => (
                  <option key={o.id} value={o.id}>
                    {o.name}
                  </option>
                ))}
            </Dropdown>
            <Dropdown
              value={secondObjective}
              onChange={(e) => setSecondObjective(e.target.value)}
            >
              <option value="">--Select Objective--</option>
              {availableObjectives
                .filter((o) => o.id !== firstObjective)
                .map((o) => (
                  <option key={o.id} value={o.id}>
                    {o.name}
                  </option>
                ))}
            </Dropdown>
            <Button
              disabled={firstObjective === "" || secondObjective === ""}
              onClick={() =>
                sendEvent({
                  RevealInitialObjectives: {
                    firstObjective: firstObjective,
                    secondObjective: secondObjective,
                  },
                })
              }
            >
              Select Objectives
            </Button>
          </>
        ) : (
          <>
            {revealedObjectives.map((o) => (
              <p key={o.id}>{o.name}</p>
            ))}
          </>
        )}
      </div>
      {/* TODO: Disable this button if faction-specific setup is not done */}
      <Button
        disabled={
          !gameState.speaker ||
          Object.keys(gameState.score.revealedObjectives).length === 0
        }
        onClick={() => sendEvent("StartGame")}
      >
        Start Game
      </Button>
    </div>
  );
};

interface FactionSpecificSetupProps {
  player: Player & { id: string };
}

const FactionSpecificSetup = ({ player }: FactionSpecificSetupProps) => {
  switch (player.faction) {
    case "Winnu":
      return <WinnuSetup player={player} />;
    case "ArgentFlight":
      return <ArgentFlightSetup player={player} />;
    case "CouncilKeleres":
      return <CouncilKeleresSetup player={player} />;
    default:
      return <div>No faction specific setup</div>;
  }
};

const WinnuSetup = ({ player }: FactionSpecificSetupProps) => {
  const { gameOptions, sendEvent } = useGameContext();

  const [selectedTech, setSelectedTech] = useState<string>("");

  const availableTechs = Object.keys(gameOptions.technologies)
    .map((t) => {
      return t as Technology;
    })
    .map((t) => {
      return {
        id: t,
        ...gameOptions.technologies[t],
      };
    })
    .filter((t) => t.origin === "Base")
    .filter((t) => Object.keys(t.requirements).length === 0)
    .sort(nameSort);

  return (
    <div>
      {player.technologies.length === 0 ? (
        <div className={styles.setupColumn}>
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

const ArgentFlightSetup = ({ player }: FactionSpecificSetupProps) => {
  const { gameOptions, sendEvent } = useGameContext();

  const [firstTech, setFirstTech] = useState<Technology | "">("");
  const [secondTech, setSecondTech] = useState<Technology | "">("");

  const takenTechs = player.technologies
    .map((t) => {
      return {
        id: t,
        ...gameOptions.technologies[t],
      };
    })
    .sort(nameSort);
  const possibleTechs = ["NeuralMotivator", "SarweenTools", "PlasmaScoring"]
    .map((t) => {
      return t as Technology;
    })
    .map((t) => {
      return { id: t, ...gameOptions.technologies[t] };
    })
    .sort(nameSort);

  return (
    <div className={styles.setupColumn}>
      {player.technologies.length > 0 ? (
        <>
          {takenTechs.map((t) => (
            <p key={t.id}>{t.name}</p>
          ))}
        </>
      ) : (
        <>
          <Dropdown
            value={firstTech}
            onChange={(e) => setFirstTech(e.target.value as Technology | "")}
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
            onChange={(e) => setSecondTech(e.target.value as Technology | "")}
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

const CouncilKeleresSetup = ({ player }: FactionSpecificSetupProps) => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [firstTech, setFirstTech] = useState<string>("");
  const [secondTech, setSecondTech] = useState<string>("");
  const [selectedFaction, setSelectedFaction] = useState<string>("");

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
    .filter((t) => t.origin === "Base")
    .sort(nameSort);

  const takenFactions = Object.values(gameState.players).map((p) => p.faction);
  const possibleFactions = gameOptions.factions
    .filter(
      (f) =>
        f.faction === "MentakCoalition" ||
        f.faction === "XxchaKingdom" ||
        f.faction === "ArgentFlight",
    )
    .filter((f) => !takenFactions.includes(f.faction))
    .sort(nameSort);

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

      {Object.keys(player.planets).length === 0 ? (
        <>
          <Dropdown
            value={selectedFaction}
            onChange={(e) => setSelectedFaction(e.target.value)}
          >
            <option value="">--Select faction--</option>
            {possibleFactions.map((f) => (
              <option value={f.faction} key={f.faction}>
                {f.name}
              </option>
            ))}
          </Dropdown>
          <Button
            disabled={selectedFaction === ""}
            onClick={() =>
              sendEvent({
                SetupTheTribunii: {
                  player: player.id,
                  faction: selectedFaction,
                },
              })
            }
          >
            Select faction
          </Button>
        </>
      ) : (
        <>
          {Object.keys(player.planets).map((p) => (
            <p key={p}>{p}</p>
          ))}
        </>
      )}
    </div>
  );
};
