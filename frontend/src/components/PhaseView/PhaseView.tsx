import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { SetupPhase } from "../views/setup/SetupPhase";
import { SelectStrategyCardView } from "../views/strategy_card_select/SelectStrategyCard";
import { StrategyCard } from "@/resources/types/strategyCards";
import { ActionPhaseView } from "../views/action_phase_View/ActionPhaseView";
import { StrategyCardView } from "../views/strategy_card_view/StrategyCardView";
import { TacticalView } from "../views/tactical_view/TacticalView";
import { StatusPhaseView } from "../views/status_phase_view/StatusPhaseView";

export const PhaseView = ({
  gameState,
  gameOptions,
  sendMessage,
}: {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}) => {
  switch (gameState.phase) {
    case "Setup":
      return (
        <SetupPhase
          gameOptions={gameOptions}
          gameState={gameState}
          sendMessage={sendMessage}
        />
      );
    case "Strategy":
      return (
        <SelectStrategyCardView
          selectedCards={Object.entries(gameState.strategyCardHolders).map(
            ([strategyCard, playerId]) => {
              return {
                card: strategyCard as StrategyCard,
                faction: gameState.players[playerId].faction,
              };
            }
          )}
          expectedStrategyCards={getExpectedStrategyCards(
            Object.keys(gameState.players).length
          )}
          selectCard={(card) => {
            sendMessage({
              TakeStrategyCard: {
                player: gameState.currentPlayer,
                card: card,
              },
            });
          }}
          startActionPhase={() => sendMessage("CompleteStrategyPhase")}
        />
      );
    case "Action":
      return (
        <ActionPhaseView
          gameState={gameState}
          systems={gameOptions.systems}
          sendMessage={sendMessage}
        />
      );
    case "StrategicAction":
      return (
        <StrategyCardView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      );
    case "TacticalAction":
      return (
        <TacticalView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      );
    case "Status":
      return (
        <StatusPhaseView gameOptions={gameOptions} sendMessage={sendMessage} />
      );
    default:
      return <div>PHASE NOT YET IMPLEMENTED</div>;
  }
};

function getExpectedStrategyCards(noPlayers: number): number {
  if (noPlayers > 4) {
    return 1 * noPlayers;
  }

  return 2 * noPlayers;
}