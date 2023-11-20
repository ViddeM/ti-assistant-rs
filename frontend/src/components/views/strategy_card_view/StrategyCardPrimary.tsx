import { GameOptions } from "@/api/GameOptions";
import { PoliticsPrimaryView } from "./primary_views/PoliticsPrimaryView";
import { TechnologyPrimaryView } from "./primary_views/TechPrimaryView";
import { GameState } from "@/api/GameState";

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
