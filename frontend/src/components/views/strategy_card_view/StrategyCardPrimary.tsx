import { GameState } from "@/api/Game";
import { StrategyTechnologyPrimaryView } from "./primary_views/TechPrimaryView";
import { GameOptions } from "@/api/GameOptions";

export interface StrategyCardPrimaryProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StrategyCardPrimary = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyCardPrimaryProps) => {
  const strategyProgress = gameState.actionProgress?.Strategic!!;

  switch (strategyProgress.card) {
    case "Technology":
      return (
        <StrategyTechnologyPrimaryView
          gameOptions={gameOptions}
          gameState={gameState}
          sendMessage={sendMessage}
        />
      );
    default:
      return <p>No primary</p>;
  }
};
