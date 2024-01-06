import { StrategyTechnologySecondaryView } from "./secondary_views/TechSecondary";
import { GenericSecondary } from "./secondary_views/GenericSecondary";
import { useGameContext } from "@/hooks/GameContext";

export const StrategyCardSecondary = () => {
  const { gameState } = useGameContext();

  const strategyCard = gameState.actionProgress?.Strategic?.card!!;

  if (strategyCard === "Technology") {
    return <StrategyTechnologySecondaryView />;
  }

  return <GenericSecondary />;
};
