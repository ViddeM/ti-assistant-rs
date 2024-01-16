import { useGameContext } from "@/hooks/GameContext";
import { SelectTechView } from "../select_tech_view/SelectTechView";
import { Button } from "@/components/elements/button/Button";

export const MawOfWorldsView = () => {
  const { gameState, sendEvent } = useGameContext();

  const owningPlayer = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .filter((p) => p.relics.includes("MawOfWorlds"))[0];

  return (
    <div className="column card">
      <h2>Play maw of worlds?</h2>
      {owningPlayer ? (
        <fieldset>
          <legend>{owningPlayer.name}</legend>
          <SelectTechView
            playerId={owningPlayer.id}
            onSelect={(tech) =>
              sendEvent({
                PlayMawOfWorlds: {
                  player: owningPlayer.id,
                  tech: tech,
                },
              })
            }
          />
        </fieldset>
      ) : (
        <p>Relic played, continue</p>
      )}
      <Button
        className="marginTop"
        onClick={() => sendEvent("CompleteMawOfWorldsPhase")}
      >
        Continue
      </Button>
    </div>
  );
};
