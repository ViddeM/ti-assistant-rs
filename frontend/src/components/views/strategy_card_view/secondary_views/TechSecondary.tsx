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
    <div className={`column ${styles.genericSecondaryContainer}`}>
      {players.map((p) => {
        const choice = donePlayers[p.id] as
          | "Skipped"
          | { Technology: { tech: string } }
          | undefined;

        return (
          <fieldset key={p.id} className={styles.techSecondaryFieldset}>
            <legend className={styles.alignedLegend}>
              <h6 className={styles.horizontalPadding}>{p.name}</h6>
              <FactionIcon faction={p.faction} />
            </legend>
            {choice === undefined ? (
              <>
                <SelectTechView
                  playerId={p.name}
                  onSelect={(tech) =>
                    sendTechSecondaryMessage(p.name, {
                      Technology: { tech: tech },
                    })
                  }
                />
                <div className={styles.skipDivider} />
                <div className={styles.techSkipButton}>
                  <Button
                    onClick={() => sendTechSecondaryMessage(p.name, "Skip")}
                  >
                    Skip
                  </Button>
                </div>
              </>
            ) : (
              <>
                {choice === "Skipped" ? (
                  <p>--Skipped--</p>
                ) : (
                  <p>
                    Tech:{" "}
                    {gameOptions.technologies[choice.Technology.tech].name}
                  </p>
                )}
              </>
            )}
          </fieldset>
        );
      })}
    </div>
  );
};
