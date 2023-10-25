import { StrategyCard } from "@/resources/types/strategyCards";

export interface StrategyCardInfoProps {
  cards: StrategyCard[];
}

export const StrategyCardInfo = ({ cards }: StrategyCardInfoProps) => {
  <div>
    {cards.map((c) => (
      <p>{c}</p>
    ))}
  </div>;
};
