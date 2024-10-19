import { Agenda } from "@/api/bindings/Agenda";
import { Button } from "@/components/elements/button/Button";
import { InfoButton } from "@/components/elements/button/InfoButton";
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, { useState } from "react";
import styles from "./LawsViewMode.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { getElectDisplayValue } from "../agenda_phase_view/AgendaActionsView";
import { AgendaElect } from "@/api/bindings/AgendaElect";

export const LawsViewMode = () => {
  return (
    <div className={styles.lawsViewContainer}>
      <ActiveLawsTable />
      <AddLawForm />
      <AgendaHistoryView />
    </div>
  );
};

const ActiveLawsTable = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const laws = Object.keys(gameState.laws)
    .map((l) => {
      return l as Agenda;
    })
    .map((l) => {
      return {
        id: l,
        ...gameOptions.agendas[l],
      };
    })
    .sort(nameSort);

  return (
    <div className="card">
      <table className={styles.lawsTable}>
        <thead>
          <tr>
            <th colSpan={3} align="center">
              <h2>Active laws</h2>
            </th>
          </tr>
        </thead>
        <tbody>
          {laws.length > 0 ? (
            <>
              {laws.map((l) => (
                <React.Fragment key={l.id}>
                  <tr>
                    <td>
                      <Button
                        className={styles.deleteLawButton}
                        onClick={() =>
                          sendEvent({
                            RepealLaw: {
                              law: l.id,
                            },
                          })
                        }
                      >
                        <FontAwesomeIcon icon={faTrash} />
                      </Button>
                    </td>
                    <td>{l.name}</td>
                    <td>
                      <InfoButton info={{ Agenda: l }} />
                    </td>
                  </tr>
                </React.Fragment>
              ))}
            </>
          ) : (
            <tr>
              <td colSpan={3} align="center">
                No laws in play
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};

const AddLawForm = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const [agenda, setAgenda] = useState<string>("");

  const [player, setPlayer] = useState<string>("");
  const [votes, setVotes] = useState<string>("");
  const [voteOption, setVoteOption] = useState<string>("");

  const voteState = gameState.agendaOverrideState?.voteState;
  const [outcome, setOutcome] = useState<string>(
    voteState?.expectedOutcome ? voteState.expectedOutcome.value : "",
  );

  const allAgendas = Object.keys(gameOptions.agendas)
    .map((a) => {
      return a as Agenda;
    })
    .map((a) => {
      return {
        id: a,
        ...gameOptions.agendas[a],
      };
    });
  const usedAgendas = gameState.agendaVoteHistory.map((a) => a.vote.agenda);
  const availableAgendas = allAgendas
    .filter((a) => !usedAgendas.includes(a.id))
    .sort((a, b) =>
      a.name.toLocaleLowerCase().localeCompare(b.name.toLocaleLowerCase()),
    );

  const playersThatHaveVoted = voteState
    ? Object.keys(voteState.playerVotes)
    : [];
  const playersLeftToVote = Object.keys(gameState.players).filter(
    (p) => !playersThatHaveVoted.includes(p),
  );

  const voteCount = votes === "" ? 0 : parseInt(votes);

  return (
    <div className="card column">
      <h2>Add Agenda</h2>
      {voteState ? (
        <div className={styles.formContainer}>
          <h3>
            {gameOptions.agendas[voteState.agenda].name}
            <InfoButton
              info={{
                Agenda: gameOptions.agendas[voteState.agenda],
              }}
            />
          </h3>

          <Button onClick={() => sendEvent("AddAgendaCancel")}>
            Cancel Adding Agenda
          </Button>

          {Object.keys(voteState.playerVotes).map((player) => (
            <p key={player}>
              {player}:{" "}
              {getElectDisplayValue(
                voteState.playerVotes[player]!!.outcome,
                gameOptions,
              )}{" "}
              ({voteState.playerVotes[player]!!.votes})
            </p>
          ))}

          <fieldset>
            <legend>Cast Votes</legend>

            <form
              className={styles.formContainer}
              onSubmit={() =>
                sendEvent({
                  AddAgendaPlayerVote: {
                    player: player,
                  },
                })
              }
            >
              <Dropdown
                value={player}
                onChange={(e) => setPlayer(e.target.value)}
              >
                <option value={""}>--Select Player--</option>
                {playersLeftToVote.map((p) => (
                  <option value={p} key={p}>
                    {p}
                  </option>
                ))}
              </Dropdown>
              <input
                type="number"
                className={"marginLeft"}
                placeholder="Votes (0)"
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
              <Dropdown
                value={voteOption}
                onChange={(e) => setVoteOption(e.target.value)}
              >
                <option value="">--Select vote option--</option>
                {voteState.candidates.map((candidate) => (
                  <option key={candidate.value} value={candidate.value}>
                    {getElectDisplayValue(candidate, gameOptions)}
                  </option>
                ))}
              </Dropdown>
              <Button
                type="submit"
                onClick={() =>
                  sendEvent({
                    AddAgendaPlayerVote: {
                      player: player,
                      outcome: {
                        electKind: voteState.elect,
                        value: voteOption,
                      } as AgendaElect,
                      votes: voteCount,
                    },
                  })
                }
              >
                Cast Votes
              </Button>
            </form>
          </fieldset>

          <fieldset>
            <legend>Resolve</legend>

            <div className={styles.resolveOutcomeContainer}>
              <label htmlFor="outcome-override-dropdown">
                Override outcome
              </label>

              <Dropdown
                id="outcome-override-dropdown"
                value={outcome}
                onChange={(e) => setOutcome(e.target.value)}
              >
                <option value="">--Select Outcome--</option>
                {voteState.candidates.map((candidate) => (
                  <option value={candidate.value} key={candidate.value}>
                    {getElectDisplayValue(candidate, gameOptions)}
                  </option>
                ))}
              </Dropdown>

              <Button
                disabled={outcome === ""}
                onClick={() =>
                  sendEvent({
                    AddAgendaResolve: {
                      electedOutcome: {
                        electKind: voteState.elect,
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
        </div>
      ) : (
        <form className={styles.formContainer}>
          <Dropdown
            id="select-agenda-dropdown"
            value={agenda}
            onChange={(e) => setAgenda(e.target.value)}
          >
            <option value="">--Select Agenda--</option>
            {availableAgendas.map((a) => (
              <option value={a.id} key={a.id}>
                {a.name}
              </option>
            ))}
          </Dropdown>
          <Button
            className={"marginTop"}
            disabled={agenda === ""}
            onClick={() =>
              sendEvent({
                AddAgendaBegin: {
                  agenda: agenda,
                },
              })
            }
          >
            Add Agenda
          </Button>
        </form>
      )}
    </div>
  );
};

const AgendaHistoryView = () => {
  const { gameState, gameOptions } = useGameContext();

  return (
    <div className="card">
      <h2>Agenda history</h2>
      {gameState.agendaVoteHistory.map((record) => (
        <div key={record.vote.agenda}>
          <h3>{gameOptions.agendas[record.vote.agenda].name}</h3>
          Result: {record.outcome?.value}
        </div>
      ))}
    </div>
  );
};
