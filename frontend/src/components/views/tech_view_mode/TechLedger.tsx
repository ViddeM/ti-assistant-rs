import { GameState } from "@/api/GameState";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Icon } from "@/components/elements/icon/Icon";
import styles from "./TechViewMode.module.scss";
import Link from "next/link";

export interface TechLedgerProps {
  gameState: GameState;
}

export const TechLedger = ({ gameState }: TechLedgerProps) => {
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
        {Object.keys(gameState.players).map((pId) => (
          <li key={pId}>
            <FactionIcon
              faction={gameState.players[pId].faction}
              width={16}
              height={16}
            />
            <Link
              href={`#${pId}`}
            >{`${gameState.players[pId].name} - ${gameState.players[pId].faction}`}</Link>
          </li>
        ))}
      </ul>
    </div>
  );
};
