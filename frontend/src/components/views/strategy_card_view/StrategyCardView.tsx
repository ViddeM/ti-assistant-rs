import { GameState, PlayerId } from "@/api/Game";
import { Button } from "@/components/elements/button/Button";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";

export interface StrategyCardView {
  gameState: GameState;
  sendMessage: (data: any) => void;
}

export const StrategyCardView = ({
  gameState,
  sendMessage,
}: StrategyCardView) => {
  const strategicAction = gameState.strategicAction!!;
  const doAction = (playerId: PlayerId, didSecondary: boolean) => {
    sendMessage({
      StrategicActionSecondary: {
        player: playerId,
        didSecondary: didSecondary,
      },
    });
  };

  return (
    <div className={`card`}>
      <h2>{gameState.strategicAction?.card}</h2>
      <div>
        {Object.keys(gameState.players)
          .filter((p) => p !== gameState.currentPlayer)
          .map((p) => gameState.players[p])
          .map((p) => (
            <div key={p.name}>
              {p.name} <FactionIcon faction={p.faction} />
              {Object.keys(strategicAction.otherPlayers).includes(p.name) ? (
                <p>{strategicAction.otherPlayers[p.name] ? "V" : "X"}</p>
              ) : (
                <>
                  <Button onClick={() => doAction(p.name, true)}>Play</Button>
                  <Button onClick={() => doAction(p.name, false)}>Skip</Button>
                </>
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
    </div>
  );
};
