import { Button } from "@/components/elements/button/Button";
import styles from "./SelectStrategyCard.module.scss";
import Image from "next/image";
import {
  Faction,
  FactionIcon,
} from "@/components/elements/factionIcon/FactionIcon";

interface StrategyCardButtonProps {
  cardName: string;
  cardNumber: number;
  selectedByFaction: Faction | null;
  setSelected: () => void;
}

export const StrategyCardButton = ({
  cardName,
  cardNumber,
  selectedByFaction,
  setSelected,
}: StrategyCardButtonProps) => {
  return (
    <Button
      onClick={setSelected}
      disabled={selectedByFaction !== null}
      className={`${styles[`strategyCard${cardName}`]} ${
        styles.strategyCardButton
      }`}
    >
      {cardNumber}.<p>{cardName}</p>
      {selectedByFaction && <FactionIcon faction={selectedByFaction} />}
    </Button>
  );
};
