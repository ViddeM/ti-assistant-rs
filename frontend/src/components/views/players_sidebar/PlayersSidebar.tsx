import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategyCardInfo } from "./parts/StrategyCardInfo";
import { StrategyCard } from "@/resources/types/strategyCards";

export interface Player {
  name: string;
  faction: Faction;
  color: string;
  cards: {
    name: StrategyCard;
    played: boolean;
  }[];
}

export const PlayerSidebar = ({ players }: { players: Player[] }) => {
  return (
    <div className="card">
      {players.map((p) => (
        <PlayerBox key={p.name} player={p} />
      ))}
    </div>
  );
};

const PlayerBox = ({ player }: { player: Player }) => {
  return (
    <div
      style={{ borderColor: player.color }}
      className={styles.playerBoxContainer}
    >
      <div className={styles.playerTitleRow}>
        <FactionIcon faction={player.faction} />
        <p className={styles.playerName}>{player.name}</p>
      </div>
      <div className={styles.content}>
        <StrategyCardInfo cards={player.cards} />
      </div>
    </div>
  );
};
