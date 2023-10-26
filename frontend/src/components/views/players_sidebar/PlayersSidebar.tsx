import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategyCardInfo } from "./parts/StrategyCardInfo";
import { StrategyCard } from "@/resources/types/strategyCards";

interface Player {
  name: string;
  faction: Faction;
  color: string;
  cards: {
    name: StrategyCard;
    played: boolean;
  }[];
}

const players: Player[] = [
  {
    name: "Tux",
    faction: "Empyrean",
    color: "#000",
    cards: [
      {
        name: "Leadership",
        played: false,
      },
      {
        name: "Construction",
        played: true,
      },
    ],
  },
  {
    name: "Vidde",
    faction: "UniversitiesOfJolNar",
    color: "#F00",
    cards: [
      {
        name: "Warfare",
        played: true,
      },
      {
        name: "Imperial",
        played: false,
      },
    ],
  },
  {
    name: "Gurr",
    faction: "CouncilKeleres",
    color: "#0F0",
    cards: [
      {
        name: "Diplomacy",
        played: true,
      },
      {
        name: "Technology",
        played: true,
      },
    ],
  },
];

export const PlayerSidebar = () => {
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
      style={{ borderColor: player.color, color: player.color }}
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
