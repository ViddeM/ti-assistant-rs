import styles from "./SelectStrategyCard.module.scss";
import { Button } from "@/components/elements/button/Button";
import { StrategyCardButton } from "./StrategyCardButton";
import { Faction } from "@/resources/types/factions";
import {
  ALL_STRATEGY_CARDS,
  StrategyCard,
} from "@/resources/types/strategyCards";
import { InfoButton } from "@/components/elements/button/InfoButton";

export type SelectedCard = {
  card: StrategyCard;
  faction: Faction;
};

interface SelectedCardProps {
  selectedCards: SelectedCard[];
  selectCard: (card: StrategyCard) => void;
  expectedStrategyCards: number;
  startActionPhase: () => void;
}

export const SelectStrategyCardView = ({
  selectedCards,
  selectCard,
  expectedStrategyCards,
  startActionPhase,
}: SelectedCardProps) => {
  return (
    <div className={`card ${styles.selectStrategyCardCard}`}>
      <h2>Select a strategy card</h2>
      <div className={styles.strategyCardsContainer}>
        {ALL_STRATEGY_CARDS.map((card) => (
          <div key={card} className={styles.selectCardRow}>
            <InfoButton
              info={{
                Strategy: card,
              }}
            />
            <StrategyCardButton
              strategyCard={card}
              selectedByFaction={
                selectedCards.filter((c) => c.card === card)[0]?.faction ?? null
              }
              setSelected={() => selectCard(card)}
              finishedSelectingCards={
                selectedCards.length === expectedStrategyCards
              }
            />
          </div>
        ))}

        <Button
          disabled={selectedCards.length !== expectedStrategyCards}
          onClick={startActionPhase}
        >
          Start Action Phase
        </Button>
      </div>
    </div>
  );
};
