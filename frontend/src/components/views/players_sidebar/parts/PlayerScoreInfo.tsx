import { Icon } from "@/components/elements/icon/Icon";
import styles from "./PlayerScoreInfo.module.scss";
import { SidebarPlayer } from "../PlayersSidebar";
import { Score } from "@/api/GameState";
import { GameOptions } from "@/api/GameOptions";

export interface PlayerScoreInfoProps {
  player: SidebarPlayer;
  score: Score;
  gameOptions: GameOptions;
}

export const PlayerScoreInfo = ({
  player,
  score,
  gameOptions,
}: PlayerScoreInfoProps) => {
  const isCustodian = score?.custodians === player.name;
  const currentScore = score.playerPoints[player.name];
  const publicObjectivePoints = Object.keys(score.revealedObjectives)
    .map((obj) => {
      return {
        obj: obj,
        objInfo: gameOptions.objectives[obj],
        players: score.revealedObjectives[obj],
      };
    })
    .filter((obj) => obj.players.includes(player.name))
    .map((obj) => {
      if (obj.objInfo.kind === "StageII") {
        return 2;
      } else {
        return 1;
      }
    })
    .reduce((acc, curr) => acc + curr, 0);

  const playerObjectives = score.secretObjectives[player.name];
  const secretObjectivePoints = playerObjectives ? playerObjectives.length : 0;

  const scoreTooltip = `Score ${currentScore} due to:
 - Custodians (${isCustodian ? "1" : "0"})
 - Public objectives (${publicObjectivePoints})
 - Secret objectives (${secretObjectivePoints})`;

  return (
    <div>
      {isCustodian && <Icon name="custodians" width={32} height={32} />}
      <h3 className={styles.scoreText} title={scoreTooltip}>
        {currentScore}
      </h3>
    </div>
  );
};