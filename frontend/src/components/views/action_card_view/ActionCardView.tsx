import { ActionCardProgress } from "@/api/bindings/ActionCardProgress";
import { Technology } from "@/api/bindings/Technology";
import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../select_tech_view/SelectTechView";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./ActionCardView.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort, stringSort } from "@/utils/Utils";

export const ActionCardView = () => {
  const { gameState, gameOptions, isActive } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "ActionCard") {
    return;
  }

  const card = gameOptions.actionCards[progress.card];
  return (
    <div className="card">
      <h2>{card.name}</h2>
      {isActive ? (
        <ActionCardProgressView cardProgress={progress} />
      ) : (
        <p>Not your turn, currently {gameState.currentPlayer} is playing</p>
      )}
    </div>
  );
};

interface ActionCardProgressViewProps {
  cardProgress: ActionCardProgress;
}

const ActionCardProgressView = ({
  cardProgress,
}: ActionCardProgressViewProps) => {
  const { gameState, sendEvent } = useGameContext();

  const sendCommitMessage = (data: any) => {
    sendEvent({
      ActionCardActionCommit: {
        player: gameState.currentPlayer,
        data: data,
      },
    });
  };

  if (cardProgress.card === "FocusedResearch") {
    return (
      <div>
        <SelectTechView
          playerId={gameState.currentPlayer!!}
          onSelect={(tech) =>
            sendCommitMessage({
              FocusedResearch: {
                tech: tech,
              },
            })
          }
        />
      </div>
    );
  } else if (cardProgress.card === "DivertFunding") {
    return <DivertFundingView sendCommitMessage={sendCommitMessage} />;
  } else if (cardProgress.card === "Plagiarize") {
    return <PlagiarizeView sendCommitMessage={sendCommitMessage} />;
  }

  return (
    <div className={styles.commitButtonRow}>
      <Button onClick={() => sendCommitMessage(null)}>Commit</Button>
    </div>
  );
};

interface DivertFundingViewProps {
  sendCommitMessage: (data: any) => void;
}

const DivertFundingView = ({ sendCommitMessage }: DivertFundingViewProps) => {
  const { gameState, gameOptions } = useGameContext();

  const [removeTech, setRemoveTech] = useState<Technology | null>(null);
  const [gainTech, setGainTech] = useState<Technology | null>(null);

  const deletableTechs = gameState.players[
    gameState.currentPlayer!!
  ].technologies.filter((tech) => {
    const techInfo = gameOptions.technologies[tech];
    if (techInfo.origin !== "Base") {
      return false;
    }
    if (techInfo.techType === "UnitUpgrade") {
      return false;
    }
    return true;
  });

  return (
    <div className={styles.divertFundingContainer}>
      <fieldset className={styles.techChangeContainer}>
        <legend>Remove tech</legend>
        <Dropdown
          id="delete_tech_dropdown"
          value={removeTech ?? ""}
          onChange={(e) => {
            const v = e.target.value;
            if (v === "") {
              setRemoveTech(null);
            } else {
              setRemoveTech(v as Technology);
            }
          }}
        >
          <option value={""}>
            {deletableTechs.length === 0
              ? "No available techs to remove"
              : "--Select tech to remove--"}
          </option>
          {deletableTechs.map((tech) => (
            <option key={tech} value={tech}>
              {gameOptions.technologies[tech].name}
            </option>
          ))}
        </Dropdown>
      </fieldset>

      <fieldset className={styles.techChangeContainer}>
        <legend>Gain tech</legend>
        {gainTech === null ? (
          <SelectTechView
            playerId={gameState.currentPlayer!!}
            onSelect={(tech) => setGainTech(tech as Technology)}
          />
        ) : (
          <p>{gameOptions.technologies[gainTech].name}</p>
        )}
      </fieldset>

      <Button
        disabled={!removeTech || !gainTech}
        onClick={() =>
          sendCommitMessage({
            DivertFunding: {
              removeTech: removeTech,
              takeTech: gainTech,
            },
          })
        }
      >
        Commit
      </Button>
    </div>
  );
};

interface PlagiarizeViewProps {
  sendCommitMessage: (data: any) => void;
}

const PlagiarizeView = ({ sendCommitMessage }: PlagiarizeViewProps) => {
  const { gameState } = useGameContext();

  const [selectedPlayer, setSelectedPlayer] = useState<string | null>(null);
  const [selectedTech, setSelectedTech] = useState<string | null>(null);

  const players = Object.keys(gameState.players)
    .filter((p) => p !== gameState.currentPlayer)
    .sort(stringSort);

  return (
    <table className={styles.plagiarizeTable}>
      <thead>
        <tr>
          <th colSpan={2}>
            <h6>Select neighbours tech</h6>
          </th>
        </tr>
        <tr>
          <td align="center">Player</td>
          <td align="center">Tech</td>
        </tr>
      </thead>
      <tbody>
        {players.map((player) => (
          <PlagiarizePlayerRow
            key={player}
            player={player}
            selectedPlayer={selectedPlayer}
            selectedTech={selectedTech}
            setSelectedTech={setSelectedTech}
            setSelectedPlayer={setSelectedPlayer}
          />
        ))}
      </tbody>
      <tfoot>
        <tr>
          <th colSpan={2}>
            <Button
              disabled={!selectedPlayer || !selectedTech}
              onClick={() =>
                sendCommitMessage({
                  Plagiarize: {
                    tech: selectedTech,
                  },
                })
              }
            >
              Plagiarize Tech
            </Button>
          </th>
        </tr>
      </tfoot>
    </table>
  );
};

interface PlagiarizePlayerRowProps {
  player: string;
  selectedPlayer: string | null;
  selectedTech: string | null;
  setSelectedPlayer: (player: string | null) => void;
  setSelectedTech: (tech: string | null) => void;
}

const PlagiarizePlayerRow = ({
  player,
  selectedPlayer,
  selectedTech,
  setSelectedPlayer,
  setSelectedTech,
}: PlagiarizePlayerRowProps) => {
  const { gameState, gameOptions } = useGameContext();

  const availablePlayerTechs = gameState.players[player].technologies
    .map((tech) => {
      return {
        id: tech,
        ...gameOptions.technologies[tech],
      };
    })
    .filter((tech) => tech.origin === "Base")
    .filter(
      (tech) =>
        !gameState.players[gameState.currentPlayer!!].technologies.includes(
          tech.id,
        ),
    )
    .sort(nameSort);

  return (
    <tr key={player}>
      <td align="right">{player}: </td>
      <td align="left">
        <Dropdown
          disabled={availablePlayerTechs.length === 0}
          value={selectedPlayer === player ? selectedTech ?? "" : ""}
          onChange={(e) => {
            setSelectedPlayer(player);
            let val = e.target.value;
            if (val === "") {
              setSelectedTech(null);
            } else {
              setSelectedTech(val);
            }
          }}
        >
          <option value="">--Select a tech--</option>
          {availablePlayerTechs.map((tech) => (
            <option key={tech.id} value={tech.id}>
              {tech.name}
            </option>
          ))}
        </Dropdown>
      </td>
    </tr>
  );
};
