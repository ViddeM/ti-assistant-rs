import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { SelectTechView } from "../../select_tech_view/SelectTechView";
import { Button } from "@/components/elements/button/Button";
import styles from "./Secondary.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const StrategyTechnologySecondaryView = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const donePlayers = gameState.actionProgress?.Strategic?.otherPlayers!!;
  const sendTechSecondaryMessage = (player: string, action: any) => {
    sendEvent({
      StrategicActionSecondary: {
        player: player,
        action: action,
      },
    });
  };

  const players = Object.keys(gameState.players)
    .filter((p) => p !== gameState.currentPlayer)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  return (
    <div>
      {players.map((p) => {
        if (donePlayers[p.id]) {
          const choice = donePlayers[p.id] as
            | "Skipped"
            | {
                Technology: {
                  tech: string;
                };
              };
          return (
            <div key={p.id}>
              {p.name} <FactionIcon faction={p.faction} />
              {choice === "Skipped" ? (
                <p>--Skipped--</p>
              ) : (
                <p>
                  Tech: {gameOptions.technologies[choice.Technology.tech].name}
                </p>
              )}
            </div>
          );
        } else {
          return (
            <fieldset key={p.id}>
              <legend className={styles.alignedLegend}>
                <h6 className={styles.horizontalPadding}>{p.name}</h6>
                <FactionIcon faction={p.faction} />
              </legend>
              <SelectTechView
                playerId={p.name}
                onSelect={(tech) =>
                  sendTechSecondaryMessage(p.name, {
                    Technology: { tech: tech },
                  })
                }
              />
              <Button onClick={() => sendTechSecondaryMessage(p.name, "Skip")}>
                Skip
              </Button>
            </fieldset>
          );
        }
      })}
    </div>
  );
};
