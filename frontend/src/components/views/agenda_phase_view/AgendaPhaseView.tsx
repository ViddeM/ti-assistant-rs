import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { AgendaActionsView } from "./AgendaActionsView";
import { AgendaInfoView } from "./AgendaInfoView";
import styles from "./AgendaPhaseView.module.scss";

export interface AgendaPhaseViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const AgendaPhaseView = ({
  gameState,
  gameOptions,
  sendMessage,
}: AgendaPhaseViewProps) => {
  const state = gameState.agenda;

  if (!state) {
    return <p>Agenda state not set during agenda phase?</p>;
  }

  return (
    <div className={styles.agendaPhaseContainer}>
      <AgendaInfoView
        gameState={gameState}
        gameOptions={gameOptions}
        state={state}
      />
      <AgendaActionsView
        state={state}
        sendMessage={sendMessage}
        gameState={gameState}
        gameOptions={gameOptions}
      />
    </div>
  );
};
