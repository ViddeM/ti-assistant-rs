import { Button } from "@/components/elements/button/Button";
import { useGameContext } from "@/hooks/GameContext";
import styles from "./EndActionPhaseView.module.scss";

export const EndActionPhaseView = () => {
  const { gameState, sendEvent } = useGameContext();

  return (
    <div className={`card ${styles.endActionPhaseViewContainer}`}>
      <h2>End player turn?</h2>{" "}
      <Button
        onClick={() =>
          sendEvent({
            TakeAnotherTurn: {
              player: gameState.currentPlayer,
            },
          })
        }
      >
        Take another turn
      </Button>
      <Button
        onClick={() =>
          sendEvent({
            EndTurn: {
              player: gameState.currentPlayer,
            },
          })
        }
      >
        End turn
      </Button>
    </div>
  );
};
