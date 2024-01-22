import { Icon } from "@/components/elements/icon/Icon";
import styles from "./PlayerScoreInfo.module.scss";
import { SidebarPlayer } from "../PlayersSidebar";
import { Score } from "@/api/GameState";
import { useGameContext } from "@/hooks/GameContext";

export interface PlayerScoreInfoProps {
  player: SidebarPlayer;
  score: Score;
}

export const PlayerScoreInfo = ({ player, score }: PlayerScoreInfoProps) => {
  const isCustodian = score?.custodians === player.name;
  const currentScore = score.playerPoints[player.name];

  return (
    <div className={styles.scoreContainer}>
      <h3 className={styles.scoreText}>{currentScore}</h3>

      {isCustodian && (
        <div>
          <Icon name="custodians" width={32} height={32} />
        </div>
      )}
    </div>
  );
};
