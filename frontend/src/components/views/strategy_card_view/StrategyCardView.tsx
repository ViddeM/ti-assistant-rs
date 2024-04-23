import { Button } from "@/components/elements/button/Button";
import styles from "./StrategyCardView.module.scss";
import { StrategyCardSecondary } from "./StrategyCardSecondary";
import { StrategyCardPrimary } from "./StrategyCardPrimary";
import { useGameContext } from "@/hooks/GameContext";
import { GameState } from "@/api/bindings/GameState";
import { StrategicProgress } from "@/api/bindings/StrategicProgress";

export const StrategyCardView = () => {
  const { gameState, sendEvent, isActive } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }
  const strategicAction = progress;

  const expectedSecondaries = Object.keys(gameState.players)
    .filter((p) => p !== gameState.currentPlayer)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .filter(
      (p) =>
        !(p.faction === "NekroVirus" && strategicAction.card === "Technology"),
    ).length;

  const secondaryDone =
    Object.keys(strategicAction.otherPlayers).length === expectedSecondaries;
  const primaryDone = isPrimaryDone(strategicAction);

  return (
    <div className={`card ${styles.strategyCardView}`}>
      <h2>{strategicAction.card}</h2>

      <>
        <div className={styles.partDivider} />
        <h6>Primary</h6>
        <StrategyCardPrimary />
      </>

      <>
        <div className={styles.partDivider} />
        <h6>Secondary</h6>
        <StrategyCardSecondary />
      </>

      {isActive && (
        <Button
          className="marginTop"
          disabled={!primaryDone || !secondaryDone || !isActive}
          onClick={() => sendEvent("StrategicActionCommit")}
        >
          Submit
        </Button>
      )}
    </div>
  );
};

function isPrimaryDone(strategic: StrategicProgress): boolean {
  if (
    strategic.card === "Technology" ||
    strategic.card === "Politics" ||
    strategic.card === "Imperial"
  ) {
    return strategic.primary !== null;
  }

  return true;
}
