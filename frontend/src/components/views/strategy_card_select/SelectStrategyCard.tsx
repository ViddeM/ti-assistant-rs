import styles from "./SelectStrategyCard.module.scss";
import { Button } from "@/components/elements/button/Button";
import { StrategyCardButton } from "./StrategyCardButton";
import {
  ALL_STRATEGY_CARDS,
  StrategyCard,
} from "@/resources/types/strategyCards";
import { useGameContext } from "@/hooks/GameContext";

export const SelectStrategyCardView = () => {
  const { gameState, sendEvent, isActive } = useGameContext();

  const selectedCards = Object.entries(gameState.strategyCardHolders).map(
    ([strategyCard, playerId]) => {
      return {
        card: strategyCard as StrategyCard,
        faction: gameState.players[playerId].faction,
      };
    }
  );

  const expectedStrategyCards = getExpectedStrategyCards(
    Object.keys(gameState.players).length
  );

  const selectCard = (card: StrategyCard) => {
    sendEvent({
      TakeStrategyCard: {
        player: gameState.currentPlayer,
        card: card,
      },
    });
  };

  const startActionPhase = () => sendEvent("CompleteStrategyPhase");

  return (
    <div className="card">
      <h2>Select a strategy card</h2>
      {isActive ? (
        <div className={styles.strategyCardsContainer}>
          {ALL_STRATEGY_CARDS.map((card) => (
            <StrategyCardButton
              key={card}
              strategyCard={card}
              selectedByFaction={
                selectedCards.filter((c) => c.card === card)[0]?.faction ?? null
              }
              setSelected={() => selectCard(card)}
              finishedSelectingCards={
                selectedCards.length === expectedStrategyCards
              }
            />
          ))}

          <Button
            disabled={selectedCards.length !== expectedStrategyCards}
            onClick={startActionPhase}
          >
            Start Action Phase
          </Button>
        </div>
      ) : (
        <p>Not your turn, currently {gameState.currentPlayer} is choosing</p>
      )}
    </div>
  );
};

function getExpectedStrategyCards(noPlayers: number): number {
  if (noPlayers > 4) {
    return 1 * noPlayers;
  }

  return 2 * noPlayers;
}
