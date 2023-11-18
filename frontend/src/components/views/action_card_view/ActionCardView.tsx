import { GameOptions } from "@/api/GameOptions";
import { ActionCardProgress, GameState } from "@/api/GameState";
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
