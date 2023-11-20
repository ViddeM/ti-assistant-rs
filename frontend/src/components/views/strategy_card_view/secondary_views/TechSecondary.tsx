import { GameOptions } from "@/api/GameOptions";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { SelectTechView } from "../common_views/SelectTechView";
import { Button } from "@/components/elements/button/Button";
import { GameState } from "@/api/GameState";
import styles from "./Secondary.module.scss";

interface StrategyTechnologySecondaryViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StrategyTechnologySecondaryView = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyTechnologySecondaryViewProps) => {
  const donePlayers = gameState.actionProgress?.Strategic?.otherPlayers!!;
  const sendTechSecondaryMessage = (player: string, action: any) => {
    sendMessage({
      StrategicActionSecondary: {
        player: player,
        action: action,
      },
    });
  };

  return (
    <div>
      {Object.keys(gameState.players)
        .filter((p) => p !== gameState.currentPlayer)
        .map((p) => {
          const player = gameState.players[p]!!;
          if (donePlayers[p]) {
            const choice = donePlayers[p] as
              | "Skipped"
              | {
                  Technology: {
                    tech: string;
                  };
                };
            return (
              <div key={p}>
                {player.name} <FactionIcon faction={player.faction} />
                {choice === "Skipped" ? (
                  <p>--Skipped--</p>
                ) : (
                  <p>Tech: {choice.Technology.tech}</p>
                )}
              </div>
            );
          } else {
            return (
              <fieldset key={p}>
                <legend className={styles.alignedLegend}>
                  <h6 className={styles.horizontalPadding}>{player.name}</h6>{" "}
                  <FactionIcon faction={player.faction} />
                </legend>
                <SelectTechView
                  gameState={gameState}
                  gameOptions={gameOptions}
                  playerId={player.name}
                  onSelect={(tech) =>
                    sendTechSecondaryMessage(player.name, {
                      Technology: { tech: tech },
                    })
                  }
                />
                <Button
                  onClick={() => sendTechSecondaryMessage(player.name, "Skip")}
                >
                  Skip
                </Button>
              </fieldset>
            );
          }
        })}
    </div>
  );
};
