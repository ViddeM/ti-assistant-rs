import { Button } from "@/components/elements/button/Button";
import { useGameContext } from "@/hooks/GameContext";
import styles from "./EndActionPhaseView.module.scss";

export const EndActionPhaseView = () => {
  const { gameState, sendEvent, isActive } = useGameContext();

  return (
    <div className={`card ${styles.endActionPhaseViewContainer}`}>
      <h2>End player turn?</h2>
      {isActive ? (
        <>
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
        </>
      ) : (
        <p>Not your turn, currently {gameState.currentPlayer} is playing</p>
      )}
    </div>
  );
};
