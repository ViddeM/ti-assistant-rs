import { Button } from "@/components/elements/button/Button";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./Secondary.module.scss";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faCircleCheck,
  faCircleXmark,
} from "@fortawesome/free-regular-svg-icons";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const GenericSecondary = () => {
  const { gameState, sendEvent, playingAs, isGlobal } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }
  const strategyCard = progress.card;

  const otherPlayers = Object.keys(gameState.players)
    .filter((p) => p !== gameState.currentPlayer)
    .map((p) => {
      return {
        action: progress.otherPlayers[p],
        ...gameState.players[p]!!,
      };
    })
    .sort(nameSort);

  const sendSecondaryMessage = (player: string, val: string) => {
    sendEvent({
      StrategicActionSecondary: {
        player: player,
        action: val,
      },
    });
  };

  return (
    <div className={styles.genericSecondaryContainer}>
      {otherPlayers.map((p) => (
        <fieldset key={p.name} className={styles.genericPlayerContainer}>
          <legend className={styles.alignedLegend}>
            <h6 className={styles.horizontalPadding}>{p.name}</h6>{" "}
            <FactionIcon faction={p.faction} />
          </legend>
          {p.action ? (
            <RenderAction performed={p.action === "Skipped"} />
          ) : p.name === playingAs || isGlobal ? (
            <div className={styles.buttonsContainer}>
              <Button onClick={() => sendSecondaryMessage(p.name, "Skip")}>
                Skip
              </Button>
              <Button
                onClick={() => sendSecondaryMessage(p.name, strategyCard)}
              >
                Play
              </Button>
            </div>
          ) : (
            <p>Has yet to choose</p>
          )}
        </fieldset>
      ))}
    </div>
  );
};

const RenderAction = ({ performed }: { performed: boolean }) => {
  return (
    <div className={styles.actionContainer}>
      {performed ? (
        <FontAwesomeIcon icon={faCircleXmark} className={styles.skipped} />
      ) : (
        <FontAwesomeIcon icon={faCircleCheck} className={styles.performed} />
      )}
    </div>
  );
};
