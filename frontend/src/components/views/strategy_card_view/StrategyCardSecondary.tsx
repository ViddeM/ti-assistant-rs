import { StrategyTechnologySecondaryView } from "./secondary_views/TechSecondary";
import { GenericSecondary } from "./secondary_views/GenericSecondary";
import { useGameContext } from "@/hooks/GameContext";

export const StrategyCardSecondary = () => {
  const { gameState } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }

  if (progress.card === "Technology") {
    return <StrategyTechnologySecondaryView />;
  }

  return <GenericSecondary />;
};
