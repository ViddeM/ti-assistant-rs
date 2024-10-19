import { AgendaState } from "@/api/bindings/AgendaState";
import { electKindToString } from "./AgendaUtils";
import { useGameContext } from "@/hooks/GameContext";
import { InfoButton } from "@/components/elements/button/InfoButton";
import styles from "./AgendaPhaseView.module.scss";

export interface AgendaInfoViewProps {
  state: AgendaState;
}

export const AgendaInfoView = ({ state }: AgendaInfoViewProps) => {
  const { gameState, gameOptions } = useGameContext();

  const previousVotesThisRound = gameState.agendaVoteHistory.filter(
    (vote) => vote.round === gameState.round,
  );

  return (
    <div className="card">
      <h2>Agenda Phase</h2>

      <fieldset>
        <legend>
          <h3>Previous agendas</h3>
        </legend>
        {previousVotesThisRound.map((vote) => (
          <div key={vote.vote.agenda}>
            <div className={styles.agendaInfoRow}>
              <h6>{gameOptions.agendas[vote.vote.agenda].name}</h6>
              <InfoButton
                info={{ Agenda: gameOptions.agendas[vote.vote.agenda] }}
              />
            </div>
            <p>
              {electKindToString(vote.vote.elect)}:
              {vote.outcome?.value ?? "Discarded"}
            </p>
          </div>
        ))}
      </fieldset>

      <fieldset>
        <legend>
          <h3>Current agenda</h3>
        </legend>

        {state.vote && (
          <div>
            <div className={styles.agendaInfoRow}>
              <h6>{gameOptions.agendas[state.vote.agenda].name}</h6>
              <InfoButton
                info={{ Agenda: gameOptions.agendas[state.vote.agenda] }}
              />
            </div>
            <p>Votes</p>
            <ol>
              {state.vote.outcomesByVotes.map((v) => (
                <li key={v.outcome.value}>
                  {v.outcome.value} - {v.votes}
                </li>
              ))}
            </ol>
          </div>
        )}
      </fieldset>
    </div>
  );
};
