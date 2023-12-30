import { GameState } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./Secondary.module.scss";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faCircleCheck,
  faCircleXmark,
} from "@fortawesome/free-regular-svg-icons";

export interface GenericSecondaryProps {
  gameState: GameState;
  sendMessage: (data: any) => void;
}

export const GenericSecondary = ({
  gameState,
  sendMessage,
}: GenericSecondaryProps) => {
  const strategyCard = gameState.actionProgress?.Strategic?.card!!;

  const otherPlayers = Object.keys(gameState.players)
    .filter((p) => p !== gameState.currentPlayer)
    .map((p) => {
      return {
        action: gameState.actionProgress?.Strategic?.otherPlayers[p],
        ...gameState.players[p]!!,
      };
    });

  const sendSecondaryMessage = (player: string, val: string) => {
    sendMessage({
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
          ) : (
            <div className={styles.buttonsContainer}>
              <Button
                onClick={() => sendSecondaryMessage(p.name, strategyCard)}
              >
                Play
              </Button>
              <Button onClick={() => sendSecondaryMessage(p.name, "Skip")}>
                Skip
              </Button>
            </div>
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
