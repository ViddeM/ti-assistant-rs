import { GameOptions } from "@/api/GameOptions";
import { StrategyTechnologySecondaryView } from "./secondary_views/TechSecondary";
import { GameState } from "@/api/GameState";
import { GenericSecondary } from "./secondary_views/GenericSecondary";

export interface StrategyCardSecondaryProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StrategyCardSecondary = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyCardSecondaryProps) => {
  const strategyCard = gameState.actionProgress?.Strategic?.card!!;

  if (strategyCard === "Technology") {
    return (
      <StrategyTechnologySecondaryView
        gameState={gameState}
        gameOptions={gameOptions}
        sendMessage={sendMessage}
      />
    );
  }

  return <GenericSecondary gameState={gameState} sendMessage={sendMessage} />;
};
