"use client";

import Link from "next/link";
import styles from "./page.module.scss";
import { useState } from "react";
import { Button } from "@/components/elements/button/Button";
import { useRouter } from "next/navigation";

export default function Home() {
  const [gameId, setGameId] = useState<string>("");
  const router = useRouter();

  return (
    <main className={styles.main}>
      <div className="card">
        <h1>TI-Assistant</h1>
        <ul className={styles.gameLinks}>
          <li>
            <Link href={"/game/new"}>New Game</Link>
          </li>
          <li>
            <Link href={"/game/c561033b"}>Three Player Demo</Link>
          </li>
          <li>
            <Link href={"/game/7d96c760"}>Five Player Demo</Link>
          </li>
        </ul>
        <div>
          <label htmlFor="join_game_input">Game ID: </label>
          <input
            value={gameId}
            onChange={(e) => setGameId(e.target.value ?? "")}
          />
        </div>
        <Button onClick={() => router.push(`/game/${gameId}`)}>
          Join game
        </Button>
      </div>
    </main>
  );
}
