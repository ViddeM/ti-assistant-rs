import { AgendaElect, AgendaElectKind } from "@/api/Agenda";
import { AgendaState, Player, Vote } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { useEffect, useState } from "react";
import styles from "./AgendaPhaseView.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { GameOptions } from "@/api/GameOptions";
import { nameSort } from "@/utils/Utils";

export interface AgendaActionsViewProps {
  state: AgendaState;
}

export const AgendaActionsView = ({ state }: AgendaActionsViewProps) => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [currentAgenda, setCurrentAgenda] = useState<string>("");

  const allAgendas = Object.keys(gameOptions.agendas).map((a) => {
    return {
      id: a,
      ...gameOptions.agendas[a],
    };
  });
  const usedAgendas = gameState.agendaVoteHistory.map((a) => a.vote.agenda);
  const availableAgendas = allAgendas
    .filter((a) => !usedAgendas.includes(a.id))
    .sort((a, b) =>
      a.name.toLocaleLowerCase().localeCompare(b.name.toLocaleLowerCase())
    );

  const speaker = gameState.players[gameState.speaker!!];
  const players = Object.keys(gameState.players)
    .map((p) => {
      return {
        id: p,
        ...gameState.players[p],
      };
    })
    .sort(nameSort);

  const everyoneHasVoted =
    Object.keys(state.vote?.playerVotes ?? {}).length === players.length;

  const castVote = (
    player: string,
    outcome: AgendaElect | null,
    votes: number = 0
  ) => {
    sendEvent({
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
        <div className={styles.agendaPhaseCompleteContainer}>
          <h3>Ready all planets!</h3>
          <Button onClick={() => sendEvent("CompleteAgendaPhase")}>
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
                    onClick={() => {
                      sendEvent({
                        RevealAgenda: {
                          agenda: currentAgenda,
                        },
                      });
                      setCurrentAgenda("");
                    }}
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
                <li>
                  When agenda is revealed
                  <br />
                  <Button onClick={() => sendEvent("VetoAgenda")}>Veto</Button>
                </li>
                <li>
                  After agenda is revealed <br />
                  TODO
                </li>
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
  const { gameOptions } = useGameContext();
  const [voteOption, setVoteOption] = useState<string>("");
  const [votes, setVotes] = useState<string>("");

  const voteCount = votes === "" ? 0 : parseInt(votes);

  return (
    <fieldset className={styles.agendaActionsContainer}>
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
                {getElectDisplayValue(candidate, gameOptions)}
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
              const s = e.target.value;
              const val = parseInt(e.target.value, 10);
              if (!isNaN(val) || s === "") {
                setVotes(s);
              }
            }}
          />
          <div>
            <Button onClick={() => castVote(player.id, null)}>Abstain</Button>
            <Button
              disabled={voteCount === 0 || voteOption === ""}
              onClick={() =>
                castVote(
                  player.id,
                  {
                    electKind: voteKind,
                    value: voteOption,
                  },
                  voteCount
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
}

const ResolveOutcome = ({ everyoneHasVoted, state }: ResolveOutcomeProps) => {
  const { sendEvent, gameOptions } = useGameContext();

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
            <option value="Discard">Discard</option>
            {state.vote?.candidates.map((candidate) => (
              <option value={candidate.value} key={candidate.value}>
                {getElectDisplayValue(candidate, gameOptions)}
              </option>
            ))}
          </Dropdown>

          <Button
            disabled={!everyoneHasVoted}
            onClick={() =>
              sendEvent({
                ResolveAgenda: {
                  outcome:
                    outcome === "Discard"
                      ? null
                      : {
                          electKind: state.vote?.elect,
                          value: outcome,
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

function getElectDisplayValue(
  candidate: AgendaElect,
  gameOptions: GameOptions
): string {
  switch (candidate.electKind) {
    case "CulturalPlanet":
    case "HazardousPlanet":
    case "IndustrialPlanet":
    case "PlanetWithTrait":
    case "Planet":
      return gameOptions.planetInfos[candidate.value].name;
    case "SecretObjective":
      return gameOptions.objectives[candidate.value].name;
    case "Law":
      return gameOptions.agendas[candidate.value].name;
    default:
      return candidate.value;
  }
}
