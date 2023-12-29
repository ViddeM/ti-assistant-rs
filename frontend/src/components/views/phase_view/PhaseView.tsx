import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { SetupPhase } from "../setup/SetupPhase";
import { SelectStrategyCardView } from "../strategy_card_select/SelectStrategyCard";
import { StrategyCard } from "@/resources/types/strategyCards";
import { ActionPhaseView } from "../action_phase_View/ActionPhaseView";
import { StrategyCardView } from "../strategy_card_view/StrategyCardView";
import { TacticalView } from "../tactical_view/TacticalView";
import { StatusPhaseView } from "../status_phase_view/StatusPhaseView";
import { ActionCardView } from "../action_card_view/ActionCardView";
import { AgendaPhaseView } from "../agenda_phase_view/AgendaPhaseView";

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
          gameOptions={gameOptions}
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
    case "ActionCardAction":
      return (
        <ActionCardView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      );
    case "Status":
      return (
        <StatusPhaseView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      );
    case "Agenda":
      return (
        <AgendaPhaseView
          gameState={gameState}
          gameOptions={gameOptions}
          sendMessage={sendMessage}
        />
      );
    default:
      return (
        <div>
          PHASE {"'"}
          {gameState.phase}
          {"'"} NOT YET IMPLEMENTED
        </div>
      );
  }
};

function getExpectedStrategyCards(noPlayers: number): number {
  if (noPlayers > 4) {
    return 1 * noPlayers;
  }

  return 2 * noPlayers;
}
