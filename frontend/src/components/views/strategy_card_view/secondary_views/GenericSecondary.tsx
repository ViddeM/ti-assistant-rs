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
import { StrategyCard } from "@/api/bindings/StrategyCard";

export const GenericSecondary = () => {
  const { gameState, sendEvent, playingAs, isGlobal } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }
  const card = progress.card;

  const otherPlayers = Object.keys(gameState.players)
    .filter((p) => p !== gameState.currentPlayer)
    .map((p) => {
      return {
        action: progress.otherPlayers[p],
        ...gameState.players[p]!!,
      };
    })
    .sort(nameSort);

  const sendSecondaryMessage = (player: string, val: "Skip" | StrategyCard) => {
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
            <RenderPerformedAction performed={p.action === "Skipped"} />
          ) : p.name === playingAs || isGlobal ? (
            <RenderAction
              card={card}
              sendSecondaryMessage={(val) => sendSecondaryMessage(p.name, val)}
            />
          ) : (
            <p>Has yet to choose</p>
          )}
        </fieldset>
      ))}
    </div>
  );
};

const RenderAction = ({
  card,
  sendSecondaryMessage,
}: {
  card: StrategyCard;
  sendSecondaryMessage: (val: "Skip" | StrategyCard) => void;
}) => {
  const costWarning = getCostWarning(card);

  return (
    <>
      {costWarning !== null && (
        <p className={"warningText"}>Remember: {costWarning}</p>
      )}
      <div className={styles.buttonsContainer}>
        <Button onClick={() => sendSecondaryMessage("Skip")}>Skip</Button>
        <Button onClick={() => sendSecondaryMessage(card)}>Play</Button>
      </div>
    </>
  );
};

const RenderPerformedAction = ({ performed }: { performed: boolean }) => {
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

function getCostWarning(card: StrategyCard): string | null {
  switch (card) {
    case "Leadership":
      return "pay 3 influence / token";
    case "Diplomacy":
      return "pay 1 token";
    case "Politics":
      return "pay 1 token";
    case "Construction":
      return "place token in system";
    case "Trade":
      return "pay 1 token";
    case "Warfare":
      return "pay 1 token";
    case "Imperial":
      return "pay 1 token";
    default:
      return null;
  }
}
