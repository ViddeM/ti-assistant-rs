import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React from "react";
import styles from "./LawsViewMode.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";

export const LawsViewMode = () => {
  const { gameState, gameOptions, sendEvent } = useGameContext();

  const laws = Object.keys(gameState.laws)
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
            <th colSpan={2} align="center">
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
                  </tr>
                </React.Fragment>
              ))}
            </>
          ) : (
            <tr>
              <td colSpan={2} align="center">
                No laws in play
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
};
