import { Button } from "@/components/elements/button/Button";
import styles from "./StrategyCardView.module.scss";
import { StrategyCardSecondary } from "./StrategyCardSecondary";
import { StrategyCardPrimary } from "./StrategyCardPrimary";
import { useGameContext } from "@/hooks/GameContext";
import { GameState } from "@/api/GameState";

export const StrategyCardView = () => {
  const { gameState, sendEvent, isActive, isCurrentPlayer } = useGameContext();

  const strategicAction = gameState.actionProgress?.Strategic!!;
  const secondaryDone =
    Object.keys(strategicAction.otherPlayers).length ===
    Object.keys(gameState.players).length - 1;
  const primaryDone = isPrimaryDone(gameState);

  return (
    <div className={`card ${styles.strategyCardView}`}>
      <h2>{gameState.actionProgress?.Strategic?.card}</h2>

      {isActive && (
        <>
          <div className={styles.partDivider} />
          <h6>Primary</h6>
          <StrategyCardPrimary />
        </>
      )}

      {!isCurrentPlayer && (
        <>
          <div className={styles.partDivider} />
          <h6>Secondary</h6>
          <StrategyCardSecondary />
        </>
      )}

      <Button
        className="marginTop"
        disabled={!primaryDone || !secondaryDone}
        onClick={() => sendEvent("StrategicActionCommit")}
      >
        Submit
      </Button>
    </div>
  );
};

function isPrimaryDone(gameState: GameState): boolean {
  const strategic = gameState.actionProgress?.Strategic!!;

  if (
    strategic.card === "Technology" ||
    strategic.card === "Politics" ||
    strategic.card === "Imperial"
  ) {
    return strategic.primary !== null;
  }

  return true;
}
