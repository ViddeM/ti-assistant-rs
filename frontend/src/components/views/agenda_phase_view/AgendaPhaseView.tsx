import { AgendaActionsView } from "./AgendaActionsView";
import { AgendaInfoView } from "./AgendaInfoView";
import styles from "./AgendaPhaseView.module.scss";
import { useGameContext } from "@/hooks/GameContext";

export const AgendaPhaseView = () => {
  const { gameState } = useGameContext();
  const state = gameState.agenda;

  if (!state) {
    return <p>Agenda state not set during agenda phase?</p>;
  }

  return (
    <div className={styles.agendaPhaseContainer}>
      <AgendaInfoView state={state} />
      <AgendaActionsView state={state} />
    </div>
  );
};
