import styles from "./SelectStrategyCard.module.scss";
import { Button } from "@/components/elements/button/Button";
import { StrategyCardButton } from "./StrategyCardButton";
import { Faction } from "@/resources/types/factions";
import {
  ALL_STRATEGY_CARDS,
  StrategyCard,
} from "@/resources/types/strategyCards";

export type SelectedCard = {
  card: StrategyCard;
  faction: Faction;
};

interface SelectedCardProps {
  selectedCards: SelectedCard[];
  selectCard: (card: StrategyCard) => void;
}

export const SelectStrategyCardView = ({
  selectedCards,
  selectCard,
}: SelectedCardProps) => {
  return (
    <div className="card">
      <h2>Select a strategy card</h2>
      <div className={styles.strategyCardsContainer}>
        {ALL_STRATEGY_CARDS.map((card) => (
          <StrategyCardButton
            key={card}
            strategyCard={card}
            selectedByFaction={
              selectedCards.filter((c) => c.card === card)[0]?.faction ?? null
            }
            setSelected={() => selectCard(card)}
          />
        ))}

        <Button>Start Action Phase</Button>
      </div>
    </div>
  );
};
