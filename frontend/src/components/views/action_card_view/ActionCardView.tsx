import { GameOptions } from "@/api/GameOptions";
import { ActionCardProgress, GameState, Player } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../strategy_card_view/common_views/SelectTechView";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";
import styles from "./ActionCardView.module.scss";

export interface ActionCardViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const ActionCardView = ({
  gameState,
  gameOptions,
  sendMessage,
}: ActionCardViewProps) => {
  const progress = gameState.actionProgress!!.ActionCard!!;
  const card = gameOptions.actionCards[progress.card];
  return (
    <div className="card">
      <h2>{card.name}</h2>
      <ActionCardProgressView
        gameState={gameState}
        gameOptions={gameOptions}
        cardProgress={progress}
        sendMessage={sendMessage}
      />
    </div>
  );
};

interface ActionCardProgressViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  cardProgress: ActionCardProgress;
  sendMessage: (data: any) => void;
}

const ActionCardProgressView = ({
  gameState,
  gameOptions,
  cardProgress,
  sendMessage,
}: ActionCardProgressViewProps) => {
  const sendCommitMessage = (data: any) => {
    sendMessage({
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
          gameState={gameState}
          gameOptions={gameOptions}
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
    return (
      <DivertFundingView
        gameState={gameState}
        gameOptions={gameOptions}
        sendCommitMessage={sendCommitMessage}
      />
    );
  } else if (cardProgress.card === "Plagiarize") {
    return (
      <PlagiarizeView
        gameState={gameState}
        gameOptions={gameOptions}
        sendCommitMessage={sendCommitMessage}
      />
    );
  }

  return (
    <div className={styles.commitButtonRow}>
      <Button onClick={() => sendCommitMessage(null)}>Commit</Button>
    </div>
  );
};

interface DivertFundingViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendCommitMessage: (data: any) => void;
}

const DivertFundingView = ({
  gameState,
  gameOptions,
  sendCommitMessage,
}: DivertFundingViewProps) => {
  const [removeTech, setRemoveTech] = useState<string | null>(null);
  const [gainTech, setGainTech] = useState<string | null>(null);

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
    <div>
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
              setRemoveTech(v);
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
              {tech}
            </option>
          ))}
        </Dropdown>
      </fieldset>

      <fieldset className={styles.techChangeContainer}>
        <legend>Gain tech</legend>
        {gainTech === null ? (
          <SelectTechView
            gameState={gameState}
            gameOptions={gameOptions}
            playerId={gameState.currentPlayer!!}
            onSelect={(tech) => setGainTech(tech)}
          />
        ) : (
          <p>{gainTech}</p>
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
  gameState: GameState;
  gameOptions: GameOptions;
  sendCommitMessage: (data: any) => void;
}

const PlagiarizeView = ({
  gameState,
  gameOptions,
  sendCommitMessage,
}: PlagiarizeViewProps) => {
  const [selectedPlayer, setSelectedPlayer] = useState<string | null>(null);
  const [selectedTech, setSelectedTech] = useState<string | null>(null);

  return (
    <table className={styles.plagiarizeTable}>
      <thead>
        <tr>
          <th colSpan={2}>Select neighbours tech</th>
        </tr>
        <tr>
          <td align="center">Player</td>
          <td align="center">Tech</td>
        </tr>
      </thead>
      <tbody>
        {Object.keys(gameState.players)
          .filter((p) => p !== gameState.currentPlayer)
          .map((player) => (
            <PlagiarizePlayerRow
              player={player}
              selectedPlayer={selectedPlayer}
              selectedTech={selectedTech}
              setSelectedTech={setSelectedTech}
              setSelectedPlayer={setSelectedPlayer}
              gameOptions={gameOptions}
              gameState={gameState}
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
  gameOptions: GameOptions;
  gameState: GameState;
}

const PlagiarizePlayerRow = ({
  player,
  selectedPlayer,
  selectedTech,
  setSelectedPlayer,
  setSelectedTech,
  gameOptions,
  gameState,
}: PlagiarizePlayerRowProps) => {
  const availablePlayerTechs = gameState.players[player].technologies
    .filter((tech) => gameOptions.technologies[tech].origin === "Base")
    .filter(
      (tech) =>
        !gameState.players[gameState.currentPlayer!!].technologies.includes(
          tech
        )
    );

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
            <option key={tech} value={tech}>
              {tech}
            </option>
          ))}
        </Dropdown>
      </td>
    </tr>
  );
};
