import { GameOptions } from "@/api/GameOptions";
import { AgendaState, GameState } from "@/api/GameState";
import { electKindToString } from "./AgendaUtils";
import styles from "./AgendaPhaseView.module.scss";

export interface AgendaInfoViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  state: AgendaState;
}

export const AgendaInfoView = ({
  gameState,
  gameOptions,
  state,
}: AgendaInfoViewProps) => {
  const previousVotesThisRound = gameState.agendaVoteHistory.filter(
    (vote) => vote.round === gameState.round
  );

  return (
    <div className="card">
      <h2>Agenda Phase</h2>

      <fieldset>
        <legend>
          <h3>Previous agendas</h3>
        </legend>

        {previousVotesThisRound.length > 0 ? (
          <>
            {previousVotesThisRound.map((vote) => (
              <div key={vote.vote.agenda}>
                <h6>{gameOptions.agendas[vote.vote.agenda].name}</h6>
                <p>
                  {electKindToString(vote.vote.elect)}:
                  {vote.outcome?.value ?? "Discarded"}
                </p>
              </div>
            ))}
          </>
        ) : (
          <p>None</p>
        )}
      </fieldset>

      <fieldset>
        <legend>
          <h3>Current agenda</h3>
        </legend>

        {state.vote && (
          <div>
            <h6>{gameOptions.agendas[state.vote.agenda].name}</h6>
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
