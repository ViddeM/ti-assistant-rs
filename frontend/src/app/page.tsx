"use client";

import Link from "next/link";
import { useState } from "react";
import styles from "./page.module.scss";
import { useRouter } from "next/navigation";
import localFont from "next/font/local";

const sourceCodeProFont = localFont({
  src: "../resources/fonts/source-code-pro/static/SourceCodePro-Medium.ttf",
  display: "swap",
  variable: "--source-code-pro",
});

export default function Home() {
  const [gameId, setGameId] = useState<string>("");

  const router = useRouter();

  const gameUrl = `/game/${gameId}`;

  return (
    <div className={`${styles.mainMenuCard}`}>
      <div className={styles.titleText}>
        <h1>TI Helper</h1>
        <p>
          Made with ❤️ by{" "}
          <Link href={"https://radicle.vidarmagnusson.com/"}>Vidde</Link>
          {" & "}
          <Link href={"https://app.radicle.xyz/nodes/seed.nubo.sh/"}>Tux</Link>
        </p>
      </div>

      <Link href={"/game/new"} className={styles.newGameLink}>
        <h2>New Game</h2>
      </Link>

      <form
        className={styles.gameIdInputContent}
        onSubmit={(e) => {
          e.preventDefault();

          router.push(gameUrl);
        }}
      >
        <div
          className={`${sourceCodeProFont.variable} ${styles.gameIdContainer}`}
        >
          <input
            id="game_id_input"
            type="text"
            required
            maxLength={8}
            value={gameId}
            onChange={(e) => setGameId(e.target.value ?? "")}
          />
        </div>
      </form>

      {gameId.length === 8 ? (
        <Link href={gameUrl}>
          <h2>JOIN GAME</h2>
        </Link>
      ) : (
        <h2 className={styles.newGameLinkDisabled}>JOIN GAME</h2>
      )}
    </div>
  );
}
