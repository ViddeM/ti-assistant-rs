import { AgendaElect, AgendaElectKind } from "@/api/Agenda";
import { GameOptions } from "@/api/GameOptions";
import { AgendaState, GameState, Player, Vote } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useEffect, useState } from "react";
import styles from "./AgendaPhaseView.module.scss";

export interface AgendaActionsViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  state: AgendaState;
  sendMessage: (data: any) => void;
}

export const AgendaActionsView = ({
  gameState,
  gameOptions,
  state,
  sendMessage,
}: AgendaActionsViewProps) => {
  const [currentAgenda, setCurrentAgenda] = useState<string>("");

  const allAgendas = Object.keys(gameOptions.agendas).map((a) => {
    return {
      id: a,
      ...gameOptions.agendas[a],
    };
  });
  const usedAgendas = gameState.agendaVoteHistory.map((a) => a.vote.agenda);
  const availableAgendas = allAgendas.filter(
    (a) => !usedAgendas.includes(a.id)
  );

  const speaker = gameState.players[gameState.speaker!!];
  const players = Object.keys(gameState.players).map((p) => {
    return {
      id: p,
      ...gameState.players[p],
    };
  });

  const everyoneHasVoted =
    Object.keys(state.vote?.playerVotes ?? {}).length === players.length;

  const castVote = (
    player: string,
    outcome: AgendaElect | null,
    votes: number = 0
  ) => {
    sendMessage({
      CastAgendaVote: {
        player: player,
        outcome: outcome,
        votes: votes,
      },
    });
  };

  return (
    <div className="card">
      {state.round === "Completed" ? (
        <div>
          <h3>Ready all planets!</h3>
          <Button onClick={() => sendMessage("CompleteAgendaPhase")}>
            Complete Agenda Phase
          </Button>
        </div>
      ) : (
        <>
          {state.vote === null ? (
            <div>
              <fieldset>
                <legend>
                  <h2 className={styles.revealAgendaTitle}>{speaker.name}</h2>
                </legend>
                <div className={styles.revealAgendaBox}>
                  <label htmlFor="select-agenda-dropdown">
                    Reveal an agenda
                  </label>
                  <Dropdown
                    id="select-agenda-dropdown"
                    value={currentAgenda}
                    onChange={(e) => setCurrentAgenda(e.target.value)}
                  >
                    <option value="">--Select Agenda--</option>
                    {availableAgendas.map((a) => (
                      <option value={a.id} key={a.id}>
                        {a.name}
                      </option>
                    ))}
                  </Dropdown>
                  <Button
                    disabled={currentAgenda === ""}
                    onClick={() =>
                      sendMessage({
                        RevealAgenda: {
                          agenda: currentAgenda,
                        },
                      })
                    }
                  >
                    Reveal
                  </Button>
                </div>
              </fieldset>
            </div>
          ) : (
            <div>
              <h6>{gameOptions.agendas[state.vote.agenda].name}</h6>
              <ol>
                <li>{speaker.name}: Read the agenda</li>
                <li>When... TODO</li>
                <li>After... TODO</li>
                <li>
                  Vote:
                  {players.map((p) => (
                    <PlayerVoteView
                      key={p.id}
                      player={p}
                      castVote={castVote}
                      candidates={state.vote!!.candidates}
                      playerVote={state.vote?.playerVotes[p.id]}
                      voteKind={state.vote!!.elect}
                    />
                  ))}
                </li>
                <li>
                  <ResolveOutcome
                    sendMessage={sendMessage}
                    everyoneHasVoted={everyoneHasVoted}
                    state={state}
                  />
                </li>
              </ol>
            </div>
          )}
        </>
      )}
    </div>
  );
};

interface PlayerVoteViewProps {
  player: Player & { id: string };
  castVote: (player: string, vote: AgendaElect | null, votes?: number) => void;
  candidates: AgendaElect[];
  playerVote?: Vote | null;
  voteKind: AgendaElectKind;
}

const PlayerVoteView = ({
  player,
  castVote,
  candidates,
  playerVote,
  voteKind,
}: PlayerVoteViewProps) => {
  const [voteOption, setVoteOption] = useState<string>("");
  const [votes, setVotes] = useState<number>(0);

  return (
    <fieldset>
      <legend>{player.name}</legend>
      {playerVote === undefined ? (
        <div className={styles.castVoteContainer}>
          <Dropdown
            value={voteOption}
            onChange={(e) => setVoteOption(e.target.value)}
          >
            <option value="">--Select vote option--</option>
            {candidates.map((candidate) => (
              <option key={candidate.value} value={candidate.value}>
                {candidate.value}
              </option>
            ))}
          </Dropdown>
          {/* TODO: Restrict player votes */}
          <input
            type="number"
            min={0}
            max={1000}
            value={votes}
            onChange={(e) => {
              const val = parseInt(e.target.value, 10);
              if (!isNaN(val)) {
                setVotes(val);
              }
            }}
          />
          <div>
            <Button onClick={() => castVote(player.id, null)}>Abstain</Button>
            <Button
              disabled={votes === 0 || voteOption === ""}
              onClick={() =>
                castVote(
                  player.id,
                  {
                    electKind: voteKind,
                    value: voteOption,
                  },
                  votes
                )
              }
            >
              Vote
            </Button>
          </div>
        </div>
      ) : (
        <div>
          {playerVote === null ? (
            <p>Abstained</p>
          ) : (
            <p>
              {playerVote.outcome.value} - {playerVote.votes}
            </p>
          )}
        </div>
      )}
    </fieldset>
  );
};

interface ResolveOutcomeProps {
  everyoneHasVoted: boolean;
  state: AgendaState;
  sendMessage: (data: any) => void;
}

const ResolveOutcome = ({
  everyoneHasVoted,
  state,
  sendMessage,
}: ResolveOutcomeProps) => {
  const expectedOutcome = state.vote?.expectedOutcome?.value ?? "Discard";
  const [outcome, setOutcome] = useState<string>(expectedOutcome);

  useEffect(() => {
    setOutcome(expectedOutcome);
  }, [expectedOutcome]);

  return (
    <>
      Resolve outcome
      <fieldset>
        <legend>Resolve</legend>

        <div className={styles.resolveOutcomeContainer}>
          <label>Override outcome</label>
          <Dropdown
            value={outcome}
            onChange={(e) => setOutcome(e.target.value)}
          >
            <option value="">Discard</option>
            {state.vote?.candidates.map((candidate) => (
              <option value={candidate.value} key={candidate.value}>
                {candidate.value}
              </option>
            ))}
          </Dropdown>

          <Button
            disabled={!everyoneHasVoted}
            onClick={() =>
              sendMessage({
                ResolveAgenda: {
                  outcome: {
                    electKind: state.vote?.elect,
                    value: outcome === "Discard" ? null : outcome,
                  },
                },
              })
            }
          >
            Resolve {outcome}
          </Button>
        </div>
      </fieldset>
    </>
  );
};
