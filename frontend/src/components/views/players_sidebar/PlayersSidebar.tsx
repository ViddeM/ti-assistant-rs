import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/api/bindings/Faction";
import { Planet } from "@/api/bindings/Planet";
import { PlanetInfo } from "@/api/bindings/PlanetInfo";
import { PlanetAttachmentInfo } from "@/api/bindings/PlanetAttachmentInfo";
import { TechInfo } from "@/api/bindings/TechInfo";
import { Score } from "@/api/bindings/Score";
import { Color } from "@/api/bindings/Color";
import { Duration } from "@/api/bindings/Duration";
import { StrategicCardInfo, StrategyCardInfo } from "./parts/StrategyCardInfo";
import { PlayerResources } from "./parts/PlayerResources";
import { PlayerScoreInfo } from "./parts/PlayerScoreInfo";
import { PlayerTimeInfo } from "./parts/PlayerTimeInfo";
import { factionIconName } from "@/components/elements/factionIcon/FactionIcon";
import { useGameContext } from "@/hooks/GameContext";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";

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
  planet: Planet;
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
    <div className={`card`}>
      <h2>Players</h2>
      <label htmlFor="playing-as-dropdown">Select player view: </label>
      <Dropdown
        id="playing-as-dropdown"
        value={playingAs ?? ""}
        onChange={(e) => {
          const val = e.target.value;
          setPlayingAs(val === "" ? null : val);
        }}
      >
        <option value="">Global view</option>
        {players.map((p) => (
          <option key={p.name} value={p.name}>
            {p.name}
          </option>
        ))}
      </Dropdown>
      <div className={styles.playerSideBarCard}>
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

const HIGH_NUMBER = 800;
function getNextPlayer(
  players: SidebarPlayer[],
  currentPlayer: SidebarPlayer | undefined,
): string | undefined {
  if (!currentPlayer) {
    return undefined;
  }

  const activePlayers = players.filter((p) => !p.hasPassed);
  const currentPlayerIndex = activePlayers.indexOf(currentPlayer);

  const indexedPlayers = activePlayers
    .filter((p) => !p.isActive)
    .map((p, index) => {
      return {
        index:
          (index + activePlayers.length - currentPlayerIndex) %
          activePlayers.length,
        ...p,
      };
    });

  const nextPlayerIndex = indexedPlayers.reduce((acc, p) => {
    if (p.index < acc) {
      return p.index;
    }
    return acc;
  }, HIGH_NUMBER);

  return nextPlayerIndex < HIGH_NUMBER
    ? indexedPlayers.filter((p) => p.index === nextPlayerIndex)[0].name
    : undefined;
}
