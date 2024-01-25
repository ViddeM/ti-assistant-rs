import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategicCardInfo, StrategyCardInfo } from "./parts/StrategyCardInfo";
import {
  Color,
  PlanetAttachmentInfo,
  PlanetInfo,
  TechInfo,
} from "@/api/GameOptions";
import { PlayerResources } from "./parts/PlayerResources";
import { PlayerScoreInfo } from "./parts/PlayerScoreInfo";
import { Duration, Score } from "@/api/GameState";
import { PlayerTimeInfo } from "./parts/PlayerTimeInfo";
import { factionIconName } from "@/components/elements/factionIcon/FactionIcon";
import { useGameContext } from "@/hooks/GameContext";

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
  playTime: Duration;
}

export interface Tech {
  tech: string;
  info: TechInfo;
}

export interface PlayerPlanetInfo {
  planet: string;
  info: PlanetInfo;
  attachments: PlanetAttachmentInfo[];
}

export interface PlayerSidebarProps {
  players: SidebarPlayer[];
  score: Score;
  isPaused: boolean;
  currentTurnStartTime: string | null;
}

export const PlayerSidebar = ({
  players,
  score,
  isPaused,
  currentTurnStartTime,
}: PlayerSidebarProps) => {
  const { playingAs, setPlayingAs } = useGameContext();

  const currentPlayers = players.filter((p) => p.isActive);
  const currentPlayer =
    currentPlayers.length > 0 ? currentPlayers[0] : undefined;

  const nextPlayer = getNextPlayer(players, currentPlayer);

  return (
    <div className={`${styles.playerSideBarCard} card`}>
      {currentPlayer && (
        <>
          <fieldset className={styles.playerBoxContainer}>
            <legend>Current player</legend>
            {currentPlayer.name}
          </fieldset>
          <fieldset className={styles.playerBoxContainer}>
            <legend>Next up</legend>
            {nextPlayer ? nextPlayer : "None"}
          </fieldset>
        </>
      )}
      {players.map((p) => (
        <PlayerBox
          key={p.name}
          player={p}
          score={score}
          isPaused={isPaused}
          currentTurnStartTime={currentTurnStartTime}
        />
      ))}
    </div>
  );
};

interface PlayerBoxProps {
  player: SidebarPlayer;
  score: Score;
  isPaused: boolean;
  currentTurnStartTime: string | null;
}

const PlayerBox = ({
  player,
  score,
  currentTurnStartTime,
  isPaused,
}: PlayerBoxProps) => {
  return (
    <fieldset
      style={{
        backgroundImage: `url("${factionIconName(player.faction.faction)}")`,
        backgroundSize: "contain",
        backgroundRepeat: "no-repeat",
        backgroundPosition: "center",
      }}
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
        className={`${styles.playerBoxLegend} playerColorBorder${player.color}`}
      >
        <h6 className={styles.playerName}>
          {player.name}
          {player.isSpeaker && " - Speaker"}
        </h6>
      </legend>
      <div className={styles.contentRow}>
        <StrategyCardInfo cards={player.cards} />
        <div className={styles.scoreTimeContainer}>
          <PlayerScoreInfo player={player} score={score} />
          <PlayerTimeInfo
            player={player}
            isPaused={isPaused}
            currentTurnStartTime={currentTurnStartTime}
          />
        </div>
      </div>
      <PlayerResources player={player} />
    </fieldset>
  );
};

function getNextPlayer(players: SidebarPlayer[]): string | undefined {
  if (!currentPlayer) {
    return undefined;
  }

  const filteredPlayers = players.filter((p) => !p.hasPassed);

  const updatedIndexedPlayers = players
    .filter((p) => !p.isActive)
    .map((p, index) => {
      return {
        newIndex: (index + currentPlayer!!.index) % players.length,
        ...p,
      };
    });

  console.log(
    "Updated indexed players",
    updatedIndexedPlayers.map((p) => {
      return {
        n: p.name,
        i: p.index,
        ni: p.newIndex,
      };
    })
  );

  console.log("C", currentPlayer!!.index);

  const nextPlayerIndex = updatedIndexedPlayers.reduce((acc, p) => {
    if (p.newIndex < acc) {
      return p.newIndex;
    }
    return acc;
  }, 10);

  return nextPlayerIndex !== undefined
    ? updatedIndexedPlayers.filter((p) => p.newIndex === nextPlayerIndex)[0]
        .name
    : undefined;
}
