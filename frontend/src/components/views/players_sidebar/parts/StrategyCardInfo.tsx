import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./StrategyCardInfo.module.scss";

export interface StrategyCardInfoProps {
  cards: {
    name: StrategyCard;
    played: boolean;
  }[];
}

export const StrategyCardInfo = ({ cards }: StrategyCardInfoProps) => {
  return (
    <div className={styles.strategyCardsContainer}>
      {cards.map((c) => (
        <div
          key={c.name}
          className={`${styles.cardContainer} style${c.name} ${
            c.played ? styles.cardPlayed : ""
          }`}
        >
          <p key={c.name}>{c.name}</p>
        </div>
      ))}
    </div>
  );
};
