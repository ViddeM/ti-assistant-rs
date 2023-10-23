"use client";

import { useEffect, useState } from "react";
import { GameOptionsForm } from "./GameOptionsForm";
import { FactionsForm, Player } from "./FactionsForm";
import styles from "./styles.module.scss";
import { GameOptions } from "@/api/GameOptions";

type Stage = "options" | "factions";

interface CreateGameFormProps {
  gameOptions: GameOptions;
}

export const CreateGameForm = ({ gameOptions }: CreateGameFormProps) => {
  const [playerCount, setPlayerCount] = useState<number>(6);
  const [victoryPoints, setVictoryPoints] = useState<number>(6);
  const [stage, setStage] = useState<Stage>("options");
  const [players, setPlayers] = useState<Player[]>([]);

  useEffect(() => {
    const newPlayers = Array.from({ length: playerCount }, (_, index) => {
      return { name: "", faction: gameOptions.factions[index].faction };
    });
    setPlayers(newPlayers);
  }, [playerCount]);

  return (
    <form className={`${styles.formContainer} card`}>
      {stage === "options" ? (
        <GameOptionsForm
          playerCounts={gameOptions.playerCounts}
          minScore={gameOptions.minScore}
          maxScore={gameOptions.maxScore}
          playerCount={playerCount}
          victoryPoints={victoryPoints}
          setPlayerCount={setPlayerCount}
          setVictoryPoints={setVictoryPoints}
          nextStage={() => setStage("factions")}
        />
      ) : (
        <FactionsForm
          factions={gameOptions.factions}
          prevStage={() => setStage("options")}
          players={players}
          setPlayer={(index, player) => {
            const newPlayers = players.map((p, i) => {
              if (index === i) {
                return player;
              } else {
                return p;
              }
            });

            setPlayers(newPlayers);
          }}
        />
      )}
    </form>
  );
};
