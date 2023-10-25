import { StrategyCard } from "@/resources/types/strategyCards";

export interface StrategyCardInfoProps {
  cards: StrategyCard[];
}

export const StrategyCardInfo = ({ cards }: StrategyCardInfoProps) => {
  return (
    <div>
      {cards.map((c) => (
        <p key={c}>{c}</p>
      ))}
    </div>
  );
};
