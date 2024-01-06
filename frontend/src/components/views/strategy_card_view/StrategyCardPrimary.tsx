import { PoliticsPrimaryView } from "./primary_views/PoliticsPrimaryView";
import { TechnologyPrimaryView } from "./primary_views/TechPrimaryView";
import { ImperialPrimaryView } from "./primary_views/ImperialPrimaryView";
import { useGameContext } from "@/hooks/GameContext";

export const StrategyCardPrimary = () => {
  const { gameState } = useGameContext();

  const strategyProgress = gameState.actionProgress?.Strategic!!;

  switch (strategyProgress.card) {
    case "Technology":
      return <TechnologyPrimaryView />;
    case "Politics":
      return <PoliticsPrimaryView />;
    case "Imperial":
      return <ImperialPrimaryView />;
    default:
      return <p>No primary</p>;
  }
};
