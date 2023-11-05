import { GameState, PlayerId } from "@/api/Game";
import { Button } from "@/components/elements/button/Button";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./StrategyCardView.module.scss";
import { GameOptions } from "@/api/GameOptions";
import { StrategyTechnologyPrimaryView } from "./primary_views/TechPrimaryView";

export interface StrategyCardViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StrategyCardView = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyCardViewProps) => {
  const strategicAction = gameState.actionProgress?.Strategic!!;
  const doAction = (playerId: PlayerId, didSecondary: boolean) => {
    sendMessage({
      StrategicActionSecondary: {
        player: playerId,
        didSecondary: didSecondary,
      },
    });
  };

  return (
    <div className={`card ${styles.strategyCardView}`}>
      <h2>{gameState.actionProgress?.Strategic?.card}</h2>

      <h6>Primary</h6>
      <StrategyCardPrimary
        gameState={gameState}
        gameOptions={gameOptions}
        sendMessage={sendMessage}
      />

      <h6>Secondary</h6>
      {Object.keys(gameState.players)
        .filter((p) => p !== gameState.currentPlayer)
        .map((p) => gameState.players[p])
        .map((p) => (
          <div key={p.name} className={styles.playerRow}>
            <div>
              {p.name} <FactionIcon faction={p.faction} />
            </div>
            {Object.keys(strategicAction.otherPlayers).includes(p.name) ? (
              <div className={styles.actionResult}>
                <p
                  className={
                    strategicAction.otherPlayers[p.name]
                      ? styles.performed
                      : styles.skipped
                  }
                >
                  {strategicAction.otherPlayers[p.name] ? "V" : "X"}
                </p>
              </div>
            ) : (
              <div className={styles.buttonGroup}>
                <Button onClick={() => doAction(p.name, true)}>Play</Button>
                <Button onClick={() => doAction(p.name, false)}>Skip</Button>
              </div>
            )}
          </div>
        ))}
      <Button
        disabled={
          Object.keys(strategicAction.otherPlayers).length <
          Object.keys(gameState.players).length - 1
        }
        onClick={() => sendMessage("StrategicActionCommit")}
      >
        Submit
      </Button>
    </div>
  );
};

interface StrategyCardPrimaryProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

const StrategyCardPrimary = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyCardPrimaryProps) => {
  const strategyProgress = gameState.actionProgress?.Strategic!!;

  switch (strategyProgress.card) {
    case "Technology":
      return (
        <StrategyTechnologyPrimaryView
          gameOptions={gameOptions}
          gameState={gameState}
          sendMessage={sendMessage}
        />
      );
    default:
      return <p>No primary</p>;
  }
};
