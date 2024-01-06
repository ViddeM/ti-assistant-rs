import { ButtonBase } from "@/components/elements/button/Button";
import styles from "./SelectStrategyCard.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Faction } from "@/resources/types/factions";
import {
  StrategyCard,
  StrategyCardNumber,
} from "@/resources/types/strategyCards";

interface StrategyCardButtonProps {
  strategyCard: StrategyCard;
  selectedByFaction: Faction | null;
  setSelected: () => void;
  finishedSelectingCards: boolean;
}

export const StrategyCardButton = ({
  strategyCard,
  selectedByFaction,
  setSelected,
  finishedSelectingCards,
}: StrategyCardButtonProps) => {
  return (
    <ButtonBase
      onClick={setSelected}
      disabled={selectedByFaction !== null || finishedSelectingCards}
      className={`${styles.strategyCardButton} style${strategyCard}`}
    >
      {StrategyCardNumber[strategyCard]}.<p>{strategyCard}</p>
      {selectedByFaction && <FactionIcon faction={selectedByFaction} />}
    </ButtonBase>
  );
};
