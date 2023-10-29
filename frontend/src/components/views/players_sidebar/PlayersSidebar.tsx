import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategicCardInfo, StrategyCardInfo } from "./parts/StrategyCardInfo";

export interface Player {
  name: string;
  faction: Faction;
  color: string;
  isActive: boolean;
  hasPassed: boolean;
  cards: StrategicCardInfo[];
}

export const PlayerSidebar = ({ players }: { players: Player[] }) => {
  return (
    <div className={`${styles.playerSideBarCard} card`}>
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
      className={`${styles.playerBoxContainer} ${
        player.isActive
          ? styles.activePlayer
          : player.hasPassed
          ? styles.passedPlayer
          : ""
      }`}
    >
      <div className={styles.playerTitleRow}>
        <FactionIcon faction={player.faction} />
        <p className={styles.playerName}>
          {player.name}
          {player.isActive ? " - ACTIVE" : player.hasPassed ? " - PASSED" : ""}
        </p>
      </div>
      <div className={styles.content}>
        <StrategyCardInfo cards={player.cards} />
      </div>
    </div>
  );
};
