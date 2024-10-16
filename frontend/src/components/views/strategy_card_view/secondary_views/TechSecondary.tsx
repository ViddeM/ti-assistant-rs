import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { SelectTechView } from "../../select_tech_view/SelectTechView";
import { Button } from "@/components/elements/button/Button";
import styles from "./Secondary.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";
import { Player } from "@/api/bindings/Player";
import { Technology } from "@/api/bindings/Technology";
import { useState } from "react";

type Choice =
  | "NekroVirus"
  | "Skipped"
  | "OtherPlayer"
  | { Technology: TechnologyChoice }
  | {
      TechnologyJolNar: JolNarChoice;
    }
  | "YetToChoose";

type TechnologyChoice = {
  tech: Technology;
};

type JolNarChoice = {
  firstTech: Technology;
  secondTech: Technology | null;
};

export const StrategyTechnologySecondaryView = () => {
  const { gameState, playingAs, isGlobal } = useGameContext();

  const progress = gameState.actionProgress!!;
  if (progress.t !== "Strategic") {
    return;
  }

  const donePlayers = progress.otherPlayers;

  const players = Object.keys(gameState.players)
    .filter((p) => p !== gameState.currentPlayer)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  const getChoice: (player: Player & { id: string }) => Choice = (
    player: Player & { id: string },
  ) => {
    if (player.faction === "NekroVirus") {
      return "NekroVirus";
    }

    const choice = donePlayers[player.id];
    if (choice !== undefined) {
      return choice as Choice;
    }

    if (player.id !== playingAs && !isGlobal) {
      return "OtherPlayer";
    }

    return "YetToChoose";
  };

  return (
    <div className={`column ${styles.genericSecondaryContainer}`}>
      {players.map((p) => {
        const choice = getChoice(p);

        return (
          <fieldset key={p.id} className={styles.techSecondaryFieldset}>
            <legend className={styles.alignedLegend}>
              <h6 className={styles.horizontalPadding}>{p.name}</h6>
              <FactionIcon faction={p.faction} />
            </legend>
            <RenderChoice player={p} choice={choice} />
          </fieldset>
        );
      })}
    </div>
  );
};

const RenderChoice = ({
  choice,
  player,
}: {
  choice: Choice;
  player: Player & { id: string };
}) => {
  const { gameOptions, sendEvent } = useGameContext();

  const sendTechSecondaryMessage = (player: string, action: any) => {
    sendEvent({
      StrategicActionSecondary: {
        player: player,
        action: action,
      },
    });
  };

  if (choice === "NekroVirus") {
    return <p>--Nekro Virus cannot research technologies--</p>;
  }

  if (choice === "OtherPlayer") {
    return <p>Yet to choose</p>;
  }

  if (choice === "YetToChoose") {
    if (player.faction === "UniversitiesOfJolNar") {
      return (
        <JolNarTechSecondary
          player={player}
          sendTechSecondaryMessage={sendTechSecondaryMessage}
        />
      );
    }

    return (
      <>
        <p className="warningText">Remember: pay 1 token and 4 resources</p>
        <SelectTechView
          playerId={player.name}
          onSelect={(tech) =>
            sendTechSecondaryMessage(player.name, {
              Technology: { tech: tech },
            })
          }
        />
        <div className={styles.skipDivider} />
        <div className={styles.techSkipButton}>
          <Button onClick={() => sendTechSecondaryMessage(player.name, "Skip")}>
            Skip
          </Button>
        </div>
      </>
    );
  }

  if (choice === "Skipped") {
    return <p>--Skipped--</p>;
  }

  console.log("CHOICE", choice);
  if ("TechnologyJolNar" in choice) {
    const c = choice.TechnologyJolNar as JolNarChoice;
    return (
      <>
        <p>First Tech: {gameOptions.technologies[c.firstTech].name}</p>
        {c.secondTech === null ? (
          <p>Skipped second tech</p>
        ) : (
          <p>Second Tech: {gameOptions.technologies[c.secondTech].name}</p>
        )}
      </>
    );
  }

  return <p>Tech: {gameOptions.technologies[choice.Technology.tech].name}</p>;
};

const JolNarTechSecondary = ({
  player,
  sendTechSecondaryMessage,
}: {
  player: Player & { id: string };
  sendTechSecondaryMessage: (player: string, action: any) => void;
}) => {
  const [firstTech, setFirstTech] = useState<Technology | null>(null);

  return (
    <>
      <p className="warningText">
        Remember Jol-Nar special either:
        <br />
        Pay 1 token and 4 resources for 1 tech OR
        <br />
        Pay 1 token and 6 resources for 2 techs.
      </p>
      {firstTech === null ? (
        <SelectTechView
          playerId={player.name}
          onSelect={(tech) => setFirstTech(tech)}
        />
      ) : (
        <>
          <p>First tech: {firstTech}</p>
          <SelectTechView
            playerId={player.name}
            onSelect={(tech) => {
              sendTechSecondaryMessage(player.name, {
                TechnologyJolNar: {
                  firstTech: firstTech,
                  secondTech: tech,
                },
              });
            }}
          />
          <Button
            className={"marginTop"}
            onClick={() =>
              sendTechSecondaryMessage(player.name, {
                TechnologyJolNar: {
                  firstTech: firstTech,
                  secondTech: null,
                },
              })
            }
          >
            Done
          </Button>
        </>
      )}
      <div className={styles.skipDivider} />
      <div className={styles.techSkipButton}>
        <Button onClick={() => sendTechSecondaryMessage(player.name, "Skip")}>
          Skip
        </Button>
      </div>
    </>
  );
};
