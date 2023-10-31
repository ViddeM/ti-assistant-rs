import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategicCardInfo, StrategyCardInfo } from "./parts/StrategyCardInfo";
import { Color } from "@/api/GameOptions";
import { Icon } from "@/components/elements/icon/Icon";

export interface Player {
  name: string;
  faction: {
    faction: Faction;
    name: string;
  };
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
        <h6 style={{ textOverflow: "ellipsis" }}>{player.name} </h6>
      </legend>
      <div className={styles.content}>
        <StrategyCardInfo cards={player.cards} />
      </div>
      <div>
        <div className={styles.planetContent}>
          <div className={styles.resourceRow}>
            <div className={styles.planetsCount}>
              <p>18</p>
            </div>
            <p>14</p>
            <Icon name="resource" isFilled />
            <Icon name="influence" isFilled />
            <p>19</p>
          </div>
          <div className={styles.resourceRow}>
            2
            <Icon name="cultural" />
            3<Icon name="industrial" />
            8<Icon name="hazardous" />
          </div>
        </div>
      </div>
    </fieldset>
  );
};
