"use client";

import Link from "next/link";
import { useState } from "react";
import styles from "./page.module.scss";

export default function Home() {
  const [gameId, setGameId] = useState<string>("");

  return (
    <div className="card">
      <h1>TI-Assistant (RS)</h1>

      <div className={styles.newGameContainer}>
        <Link href={"/game/new"} className={styles.newGameLink}>
          <h2>New Game</h2>
        </Link>
      </div>

      <div>
        <label htmlFor="join_game_input">Game ID: </label>
        <input
          id="join_game_input"
          className={styles.gameIdInput}
          value={gameId}
          onChange={(e) => setGameId(e.target.value ?? "")}
          maxLength={8}
        />
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
        }}
      >
        <Link href={`/game/${gameId}`}>Join Game</Link>
      </div>
    </div>
  );
}
