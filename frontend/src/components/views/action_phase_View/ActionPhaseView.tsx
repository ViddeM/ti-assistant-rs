import { GameState } from "@/api/Game";
import { Button } from "@/components/elements/button/Button";
import { StrategyCard } from "@/resources/types/strategyCards";

export const ActionPhaseView = ({ gameState }: { gameState: GameState }) => {
  const currentPlayer = gameState.currentPlayer as string | null;

  if (!currentPlayer) {
    return <div>Invalid state, currentPlayer is null in action phase!</div>;
  }

  const playableStrategyCards = getPlayableStrategyCards(
    gameState,
    currentPlayer
  );

  return (
    <div>
      ACTION PHASE
      <div>
        <Button>Pass</Button>
        {playableStrategyCards.map((c) => (
          <div key={c}>
            <Button>{c}</Button>
          </div>
        ))}
      </div>
    </div>
  );
};

function getPlayableStrategyCards(
  gameState: GameState,
  currentPlayer: string
): StrategyCard[] {
  return Object.entries(gameState.strategyCardHolders)
    .map(([strategyCard, player]) => {
      return {
        card: strategyCard,
        player: player,
      };
    })
    .filter(
      (v) => !gameState.spentStrategyCards.includes(v.card as StrategyCard)
    )
    .filter((v) => v.player === `${currentPlayer}`)
    .map((v) => v.card as StrategyCard);
}
