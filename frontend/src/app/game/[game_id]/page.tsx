import { PlayerSidebar } from "@/components/views/players_sidebar/PlayersSidebar";
import { SelectStrategyCardView } from "@/components/views/strategy_card_select/SelectStrategyCard";
import styles from "./styles.module.scss";

export default function Game() {
  return (
    <div className={styles.gamePageContainer}>
      <PlayerSidebar />
      <SelectStrategyCardView />
      <div />
    </div>
  );
}
