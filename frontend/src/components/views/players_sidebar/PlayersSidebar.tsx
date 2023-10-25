import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./PlayersSidebar.module.scss";
import { Faction } from "@/resources/types/factions";
import { StrategyCardInfo } from "./parts/StrategyCardInfo";

const players: { name: string; faction: Faction; color: string }[] = [
  {
    name: "Tux",
    faction: "Empyrean",
    color: "#000",
  },
  {
    name: "Vidde",
    faction: "UniversitiesOfJolNar",
    color: "#F00",
  },
  {
    name: "Gurr",
    faction: "CouncilKeleres",
    color: "#0F0",
  },
];

export const PlayerSidebar = () => {
  return (
    <div className="card">
      {players.map((p) => (
        <PlayerBox key={p.name} player={p}>
          <StrategyCardInfo strategyCards={["Leadership", "Technology"]} />
        </PlayerBox>
      ))}
    </div>
  );
};

interface PlayerProps {
  name: string;
  faction: Faction;
  color: string;
}

const PlayerBox = ({
  player,
  children,
}: {
  player: PlayerProps;
  children: React.ReactNode;
}) => {
  return (
    <div
      style={{ borderColor: player.color, color: player.color }}
      className={styles.playerBoxContainer}
    >
      <div className={styles.playerTitleText}>
        <FactionIcon faction={player.faction} /> {player.name}
      </div>
      {children}
    </div>
  );
};
