import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { electKindToString } from "./AgendaUtils";

export interface AgendaInfoViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
}

export const AgendaInfoView = ({
  gameState,
  gameOptions,
}: AgendaInfoViewProps) => {
  const previousVotesThisRound = gameState.agendaVoteHistory.filter(
    (vote) => vote.round === gameState.round
  );

  return (
    <div className="card">
      <h2>Agenda Phase</h2>

      {previousVotesThisRound.map((vote) => (
        <div key={vote.vote.agenda}>
          <h6>{gameOptions.agendas[vote.vote.agenda].name}</h6>
          <p>
            {electKindToString(vote.outcome.electKind)}: {vote.outcome.value}
          </p>
        </div>
      ))}
    </div>
  );
};
