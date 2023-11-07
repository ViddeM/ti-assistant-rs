import { GameState } from "@/api/Game";
import { Button } from "@/components/elements/button/Button";
import styles from "./StrategyCardView.module.scss";
import { GameOptions } from "@/api/GameOptions";
import { TechnologyPrimaryView } from "./primary_views/TechPrimaryView";
import { StrategyCardSecondary } from "./StrategyCardSecondary";
import { PoliticsPrimaryView } from "./primary_views/PoliticsPrimaryView";

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

interface StrategyCardPrimaryProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

const StrategyCardPrimary = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyCardPrimaryProps) => {
  const strategyProgress = gameState.actionProgress?.Strategic!!;

  switch (strategyProgress.card) {
    case "Technology":
      return (
        <TechnologyPrimaryView
          gameOptions={gameOptions}
          gameState={gameState}
          sendMessage={sendMessage}
        />
      );
    case "Politics":
      return (
        <PoliticsPrimaryView gameState={gameState} sendMessage={sendMessage} />
      );
    default:
      return <p>No primary</p>;
  }
};

function isPrimaryDone(gameState: GameState): boolean {
  const strategic = gameState.actionProgress?.Strategic!!;

  if (strategic.card === "Technology" || strategic.card === "Politics") {
    return strategic.primary !== null;
  }

  return true;
}
