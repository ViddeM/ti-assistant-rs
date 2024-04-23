import { CreationPhase } from "../creation/CreationPhase";
import { SelectStrategyCardView } from "../strategy_card_select/SelectStrategyCard";
import { StrategyCard } from "@/resources/types/strategyCards";
import { ActionPhaseView } from "../action_phase_View/ActionPhaseView";
import { StrategyCardView } from "../strategy_card_view/StrategyCardView";
import { TacticalView } from "../tactical_view/TacticalView";
import { StatusPhaseView } from "../status_phase_view/StatusPhaseView";
import { ActionCardView } from "../action_card_view/ActionCardView";
import { AgendaPhaseView } from "../agenda_phase_view/AgendaPhaseView";
import { useGameContext } from "@/hooks/GameContext";
import { EndActionPhaseView } from "../end_action_phase_view/EndActionPhaseView";
import { FrontierCardView } from "../frontier_card_view/FrontierCardView";
import { RelicCardView } from "../relic_card_view/RelicCardView";
import { RelicsPhaseView } from "../relics_phase_view/RelicsPhaseView";
import { SetupPhase } from "../setup/Setup";

export const PhaseView = () => {
  const { gameState, sendEvent } = useGameContext();

  switch (gameState.phase) {
    case "Creation":
      return <CreationPhase />;
    case "Setup":
      return <SetupPhase />;
    case "Strategy":
      return (
        <SelectStrategyCardView
          selectedCards={Object.entries(gameState.strategyCardHolders).map(
            ([strategyCard, playerId]) => {
              return {
                card: strategyCard as StrategyCard,
                faction: gameState.players[playerId].faction,
              };
            },
          )}
          expectedStrategyCards={getExpectedStrategyCards(
            Object.keys(gameState.players).length,
          )}
          selectCard={(card) => {
            sendEvent({
              TakeStrategyCard: {
                player: gameState.currentPlayer,
                card: card,
              },
            });
          }}
          startActionPhase={() => sendEvent("CompleteStrategyPhase")}
        />
      );
    case "Action":
      return <ActionPhaseView />;
    case "StrategicAction":
      return <StrategyCardView />;
    case "TacticalAction":
      return <TacticalView />;
    case "ActionCardAction":
      return <ActionCardView />;
    case "FrontierCardAction":
      return <FrontierCardView />;
    case "RelicAction":
      return <RelicCardView />;
    case "EndActionTurn":
      return <EndActionPhaseView />;
    case "Status":
      return <StatusPhaseView />;
    case "Relics":
      return <RelicsPhaseView />;
    case "Agenda":
      return <AgendaPhaseView />;
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
