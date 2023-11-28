import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategicCardInfo, StrategyCardInfo } from "./parts/StrategyCardInfo";
import { Color, GameOptions, PlanetInfo, TechInfo } from "@/api/GameOptions";
import { PlayerResources } from "./parts/PlayerResources";
import { PlayerScoreInfo } from "./parts/PlayerScoreInfo";
import { Score } from "@/api/GameState";

export interface SidebarPlayer {
  name: string;
  faction: {
    faction: Faction;
    name: string;
  };
  color: Color;
  isActive: boolean;
  hasPassed: boolean;
  isSpeaker: boolean;
  cards: StrategicCardInfo[];
  planets: PlayerPlanetInfo[];
  technologies: Tech[];
}

export interface Tech {
  tech: string;
  info: TechInfo;
}

export interface PlayerPlanetInfo {
  planet: string;
  info: PlanetInfo;
}

export interface PlayerSidebarProps {
  players: SidebarPlayer[];
  score: Score;
  gameOptions: GameOptions;
}

export const PlayerSidebar = ({
  players,
  score,
  gameOptions,
}: PlayerSidebarProps) => {
  return (
    <div className={`${styles.playerSideBarCard} card`}>
      {players.map((p) => (
        <PlayerBox
          key={p.name}
          player={p}
          score={score}
          gameOptions={gameOptions}
        />
      ))}
    </div>
  );
};

interface PlayerBoxProps {
  player: SidebarPlayer;
  score: Score;
  gameOptions: GameOptions;
}

const PlayerBox = ({ player, score, gameOptions }: PlayerBoxProps) => {
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
        <h6 className={styles.playerName}>
          {player.name}
          {player.isSpeaker && " - Speaker"}
        </h6>
      </legend>
      <div className={styles.contentRow}>
        <StrategyCardInfo cards={player.cards} />
        <PlayerScoreInfo
          player={player}
          score={score}
          gameOptions={gameOptions}
        />
      </div>
      <PlayerResources player={player} />
    </fieldset>
  );
};
