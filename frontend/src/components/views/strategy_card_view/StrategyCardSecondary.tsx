import { GameOptions } from "@/api/GameOptions";
import { StrategyTechnologySecondaryView } from "./secondary_views/TechSecondary";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import styles from "./StrategyCardView.module.scss";
import { Button } from "@/components/elements/button/Button";
import { GameState } from "@/api/GameState";

export interface StrategyCardSecondaryProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StrategyCardSecondary = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyCardSecondaryProps) => {
  const strategyCard = gameState.actionProgress?.Strategic?.card!!;

  if (strategyCard === "Technology") {
    return (
      <StrategyTechnologySecondaryView
        gameState={gameState}
        gameOptions={gameOptions}
        sendMessage={sendMessage}
      />
    );
  }

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
    <div>
      {otherPlayers.map((p) => (
        <div key={p.name}>
          <div>
            {p.name} <FactionIcon faction={p.faction} />
          </div>
          {p.action ? (
            <div>
              <p
                className={
                  p.action === "Skipped" ? styles.skipped : styles.performed
                }
              >
                {p.action === "Skipped" ? "X" : "V"}
              </p>
            </div>
          ) : (
            <div className={styles.buttonGroup}>
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
        </div>
      ))}
    </div>
  );
};
