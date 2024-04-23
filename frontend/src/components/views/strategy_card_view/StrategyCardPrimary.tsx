import { PoliticsPrimaryView } from "./primary_views/PoliticsPrimaryView";
import { TechnologyPrimaryView } from "./primary_views/TechPrimaryView";
import { ImperialPrimaryView } from "./primary_views/ImperialPrimaryView";
import { useGameContext } from "@/hooks/GameContext";

export const StrategyCardPrimary = () => {
  const { gameState } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }

  switch (progress.card) {
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
