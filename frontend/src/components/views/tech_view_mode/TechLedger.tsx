import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Icon } from "@/components/elements/icon/Icon";
import styles from "./TechViewMode.module.scss";
import Link from "next/link";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const TechLedger = () => {
  const { gameState } = useGameContext();

  const players = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  return (
    <div className="card">
      <h2>Shortcuts</h2>
      <ul className={styles.techLedgerList}>
        <li>
          {/* TODO: What do we use here? */}
          <Icon name={"legendary_planet"} isFilled={true} />
          <Link href={"#UnitUpgrades"}>Unit Upgrades</Link>
        </li>
        <li>
          <Icon name={"warfare"} isFilled={true} />
          <Link href={"#Warfare"}>Warfare</Link>
        </li>
        <li>
          <Icon name={"propulsion"} isFilled={true} />
          <Link href={"#Propulsion"}>Propulsion</Link>
        </li>
        <li>
          <Icon name={"cybernetic"} isFilled={true} />
          <Link href={"#Cybernetic"}>Cybernetic</Link>
        </li>
        <li>
          <Icon name={"biotic"} isFilled={true} />
          <Link href={"#Biotic"}>Biotic</Link>
        </li>
        {players.map((p) => (
          <li key={p.id}>
            <FactionIcon faction={p.faction} width={16} height={16} />
            <Link href={`#${p.id}`}>{`${p.name} - ${p.faction}`}</Link>
          </li>
        ))}
      </ul>
    </div>
  );
};
