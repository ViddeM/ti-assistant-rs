import { useGameContext } from "@/hooks/GameContext";
import { SelectTechView } from "../select_tech_view/SelectTechView";
import { Button } from "@/components/elements/button/Button";

export const RelicsPhaseView = () => {
  const { gameState, sendEvent } = useGameContext();

  const allPlayers = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });

  const mawOfWorldsHolder =
    allPlayers.filter((p) => p.relics.includes("MawOfWorlds"))[0] ?? null;

  const crownOfEmphidiaHolder =
    allPlayers
      .filter((p) => p.relics.includes("TheCrownOfEmphidia"))
      .filter(
        (p) =>
          Object.values(p.planets).filter((attachments) =>
            attachments.includes("TombOfEmphidia")
          ).length > 0
      )[0] ?? null;

  return (
    <div className="column card">
      {crownOfEmphidiaHolder && (
        <>
          <h2>Play the Crown of Emphidia?</h2>
          <fieldset>
            <legend>{crownOfEmphidiaHolder.name}</legend>
            <Button
              onClick={() =>
                sendEvent({
                  PlayCrownOfEmphidia: {
                    player: crownOfEmphidiaHolder.id,
                  },
                })
              }
            >
              Play
            </Button>
          </fieldset>
        </>
      )}
      {mawOfWorldsHolder && (
        <>
          <h2>Play maw of worlds?</h2>
          <fieldset>
            <legend>{mawOfWorldsHolder.name}</legend>
            <SelectTechView
              playerId={mawOfWorldsHolder.id}
              onSelect={(tech) =>
                sendEvent({
                  PlayMawOfWorlds: {
                    player: mawOfWorldsHolder.id,
                    tech: tech,
                  },
                })
              }
            />
          </fieldset>
        </>
      )}
      {crownOfEmphidiaHolder === null && mawOfWorldsHolder === null && (
        <h2>Continue to status phase</h2>
      )}
      <Button
        className="marginTop"
        onClick={() => sendEvent("CompleteRelicsPhase")}
      >
        Continue
      </Button>
    </div>
  );
};
