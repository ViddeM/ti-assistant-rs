import { AgendaState } from "@/api/GameState";
import { electKindToString } from "./AgendaUtils";
import { useGameContext } from "@/hooks/GameContext";

export interface AgendaInfoViewProps {
  state: AgendaState;
}

export const AgendaInfoView = ({ state }: AgendaInfoViewProps) => {
  const { gameState, gameOptions, showInfo } = useGameContext();

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
        {previousVotesThisRound.map((vote) => (
          <div key={vote.vote.agenda}
               onClick = {() => showInfo({Agenda: gameOptions.agendas[vote.vote.agenda]})}
          >
            <h6>{gameOptions.agendas[vote.vote.agenda].name}</h6>
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
            <h6 onClick = {() => showInfo({Agenda: gameOptions.agendas[state.vote.agenda]})}
            >{gameOptions.agendas[state.vote.agenda].name}</h6>
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
