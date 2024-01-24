import { FrontierCardProgress } from "@/api/GameState";
import { useGameContext } from "@/hooks/GameContext";
import { SelectTechView } from "../select_tech_view/SelectTechView";
import { Button } from "@/components/elements/button/Button";

export const FrontierCardView = () => {
  const { gameState, gameOptions } = useGameContext();

  const progress = gameState.actionProgress!!.FrontierCard!!;
  const card = gameOptions.frontierCards[progress.card];

  return (
    <div className="card column">
      <h2>{card.name}</h2>
      <FrontierCardProgressView cardProgress={progress} />
    </div>
  );
};

interface FrontierCardProgressViewProps {
  cardProgress: FrontierCardProgress;
}

const FrontierCardProgressView = ({
  cardProgress,
}: FrontierCardProgressViewProps) => {
  const { gameState, sendEvent } = useGameContext();

  switch (cardProgress.card) {
    case "EnigmaticDevice":
      return (
        <div className={"screenContainer"}>
          <SelectTechView
            playerId={gameState.currentPlayer!!}
            onSelect={(tech) =>
              sendEvent({
                FrontierCardActionCommit: {
                  player: gameState.currentPlayer,
                  data: {
                    EnigmaticDevice: {
                      tech: tech,
                    },
                  },
                },
              })
            }
          />
        </div>
      );
    default:
      return (
        <Button
          onClick={() =>
            sendEvent({
              FrontierCardActionCommit: {
                player: gameState.currentPlayer,
              },
            })
          }
        >
          Commit
        </Button>
      );
  }
};
