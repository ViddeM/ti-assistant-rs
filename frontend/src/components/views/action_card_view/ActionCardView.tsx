import { GameOptions } from "@/api/GameOptions";
import { ActionCardProgress, GameState } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../strategy_card_view/common_views/SelectTechView";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useState } from "react";

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
      <h2>{card.card}</h2>
      <ActionCardProgressView
        gameState={gameState}
        gameOptions={gameOptions}
        cardProgress={progress}
        sendMessage={sendMessage}
      />
      <Button
        onClick={() =>
          sendMessage({
            ActionCardActionCommit: {
              player: gameState.currentPlayer,
            },
          })
        }
      >
        Commit
      </Button>
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
  const sendPerformMessage = (data: any) => {
    sendMessage({
      ActionCardActionPerform: {
        player: gameState.currentPlayer,
        data: data,
      },
    });
  };

  if (cardProgress.card === "FocusedResearch") {
    if (cardProgress.state !== null) {
      return <div>{cardProgress.state!!.FocusedResearch.tech}</div>;
    }

    return (
      <div>
        <SelectTechView
          gameState={gameState}
          gameOptions={gameOptions}
          playerId={gameState.currentPlayer!!}
          onSelect={(tech) =>
            sendPerformMessage({
              FocusedResearch: {
                tech: tech,
              },
            })
          }
        />
      </div>
    );
  } else if (cardProgress.card === "DivertFunding") {
    if (cardProgress.state !== null) {
      return (
        <div>
          {cardProgress.state!!.DivertFunding.removed} {" -> "}
          {cardProgress.state.DivertFunding.gained}
        </div>
      );
    }

    return (
      <DivertFundingView
        gameState={gameState}
        gameOptions={gameOptions}
        sendPerformMessage={sendPerformMessage}
      />
    );
  }

  return <div />;
};

interface DivertFundingViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendPerformMessage: (data: any) => void;
}

const DivertFundingView = ({
  gameState,
  gameOptions,
  sendPerformMessage,
}: DivertFundingViewProps) => {
  const [removeTech, setRemoveTech] = useState<string | null>(null);
  const [gainTech, setGainTech] = useState<string | null>(null);

  const deleteableTechs = gameState.players[
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
      <Dropdown
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
          {deleteableTechs.length === 0
            ? "No available techs to remove"
            : "--Select tech to remove--"}
        </option>
        {deleteableTechs.map((tech) => (
          <option key={tech} value={tech}>
            {tech}
          </option>
        ))}
      </Dropdown>
      <SelectTechView
        gameState={gameState}
        gameOptions={gameOptions}
        playerId={gameState.currentPlayer!!}
        onSelect={(tech) => setGainTech(tech)}
      />
      <Button
        disabled={!removeTech || !gainTech}
        onClick={() =>
          sendPerformMessage({
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
