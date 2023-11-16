import { GameOptions } from "@/api/GameOptions";
import { ActionCardProgress, GameState } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { SelectTechView } from "../strategy_card_view/common_views/SelectTechView";

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
  }

  return <div />;
};
