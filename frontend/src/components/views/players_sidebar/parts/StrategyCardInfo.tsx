import { StrategyCard } from "@/resources/types/strategyCards";
import styles from "./StrategyCardInfo.module.scss";

export interface StrategicCardInfo {
  name: StrategyCard;
  played: boolean;
  isActive: boolean;
}

export interface StrategyCardInfoProps {
  cards: StrategicCardInfo[];
}

export const StrategyCardInfo = ({ cards }: StrategyCardInfoProps) => {
  return (
    <div className={styles.strategyCardsContainer}>
      {cards.map((c) => (
        <div
          key={c.name}
          className={`${styles.cardContainer} style${c.name} ${
            c.isActive ? styles.cardActive : c.played ? styles.cardPlayed : ""
          }`}
        >
          <p key={c.name}>{c.name}</p>
        </div>
      ))}
    </div>
  );
};
