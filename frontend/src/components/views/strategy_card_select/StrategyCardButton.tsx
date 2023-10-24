import { Button } from "@/components/elements/button/Button";
import { StrategyCardButtonProps } from "./SelectStrategyCard";

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
      {selectedByFaction && (
        <Image
          src={`/icons/factions/${selectedByFaction}.png`}
          alt={`Faction Icon ${selectedByFaction}`}
          width={32}
          height={32}
        />
      )}
    </Button>
  );
};
