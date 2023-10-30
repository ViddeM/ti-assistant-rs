import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategicCardInfo, StrategyCardInfo } from "./parts/StrategyCardInfo";
import { Color } from "@/api/GameOptions";

export interface Player {
  name: string;
  faction: Faction;
  color: Color;
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
    <fieldset
      className={`playerColorBorder${player.color} ${
        styles.playerBoxContainer
      } playerColor${player.color} ${
        player.isActive
          ? styles.activePlayer
          : player.hasPassed
          ? styles.passedPlayer
          : ""
      }`}
    >
      <legend
        className={`${styles.playerBoxLegend} playerColorBorder${
          player.color
        } ${
          player.color === "Black"
            ? styles.whiteBackground
            : styles.grayBackground
        }`}
      >
        {player.name}
      </legend>

      <div className={styles.playerTitleRow}>
        <FactionIcon faction={player.faction} />
      </div>
      <div className={styles.content}>
        <StrategyCardInfo cards={player.cards} />
      </div>
    </fieldset>
  );
};
