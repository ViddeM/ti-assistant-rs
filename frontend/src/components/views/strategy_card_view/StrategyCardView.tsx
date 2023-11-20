import { Button } from "@/components/elements/button/Button";
import styles from "./StrategyCardView.module.scss";
import { GameOptions } from "@/api/GameOptions";
import { StrategyCardSecondary } from "./StrategyCardSecondary";
import { GameState } from "@/api/GameState";
import { StrategyCardPrimary } from "./StrategyCardPrimary";

export interface StrategyCardViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StrategyCardView = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyCardViewProps) => {
  const strategicAction = gameState.actionProgress?.Strategic!!;
  const secondaryDone =
    Object.keys(strategicAction.otherPlayers).length ===
    Object.keys(gameState.players).length - 1;
  const primaryDone = isPrimaryDone(gameState);

  return (
    <div className={`card ${styles.strategyCardView}`}>
      <h2>{gameState.actionProgress?.Strategic?.card}</h2>

      <h6>Primary</h6>
      <StrategyCardPrimary
        gameState={gameState}
        gameOptions={gameOptions}
        sendMessage={sendMessage}
      />

      <h6>Secondary</h6>
      <StrategyCardSecondary
        gameState={gameState}
        gameOptions={gameOptions}
        sendMessage={sendMessage}
      />

      <Button
        disabled={!primaryDone || !secondaryDone}
        onClick={() => sendMessage("StrategicActionCommit")}
      >
        Submit
      </Button>
    </div>
  );
};

function isPrimaryDone(gameState: GameState): boolean {
  const strategic = gameState.actionProgress?.Strategic!!;

  if (strategic.card === "Technology" || strategic.card === "Politics") {
    return strategic.primary !== null;
  }

  return true;
}
