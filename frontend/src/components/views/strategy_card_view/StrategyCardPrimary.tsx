import { PoliticsPrimaryView } from "./primary_views/PoliticsPrimaryView";
import { TechnologyPrimaryView } from "./primary_views/TechPrimaryView";
import { ImperialPrimaryView } from "./primary_views/ImperialPrimaryView";
import { useGameContext } from "@/hooks/GameContext";
import { StrategyCard } from "@/api/bindings/StrategyCard";

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
      const costWarning = getCostWarning(progress.card);
      return (
        <>
          {costWarning !== null && (
            <p className={"warningText"}>Remember: {costWarning}</p>
          )}
          <p>Primary not tracked</p>
        </>
      );
  }
};

function getCostWarning(card: StrategyCard): string | null {
  switch (card) {
    case "Leadership":
      return "pay 3 influence / extra token";
    default:
      return null;
  }
}
