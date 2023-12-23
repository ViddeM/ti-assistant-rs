import { Faction } from "@/resources/types/factions";
import styles from "./FactionButton.module.scss";
import { FactionIcon } from "../factionIcon/FactionIcon";

export interface FactionButtonProps {
  faction: Faction;
  selected: boolean;
  onClick: () => void;
}

export const FactionButton = ({
  faction,
  selected,
  onClick,
}: FactionButtonProps) => {
  return (
    <button
      className={`${selected ? styles.factionButtonSelected : ""} ${
        styles.factionButton
      }`}
      onClick={onClick}
    >
      <FactionIcon faction={faction} width={32} height={32} />
    </button>
  );
};
