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

export const LawsViewMode = () => {
  return (
    <div className={styles.lawsViewContainer}>
      <ActiveLawsTable />
      <AddLawForm />
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
              <h2>Laws</h2>
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

  const voteState = gameState.agendaOverrideState?.voteState;

  return (
    <form className="card column">
      <h2>Add Agenda</h2>
      {voteState ? (
        <>
          <p>Agenda: {voteState.agenda}</p>
        </>
      ) : (
        <>
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
        </>
      )}
    </form>
  );
};
