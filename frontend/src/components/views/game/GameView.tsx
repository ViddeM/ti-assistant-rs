"use client";

import { GameOptions } from "@/api/bindings/GameOptions";
import { GameState } from "@/api/bindings/GameState";
import { StrategyCard } from "@/api/bindings/StrategyCard";
import { Planet } from "@/api/bindings/Planet";
import { useEffect, useRef, useState } from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";
import {
  PlayerSidebar,
  SidebarPlayer,
  PlayerPlanetInfo,
} from "../players_sidebar/PlayersSidebar";
import styles from "./GameView.module.scss";
import { useRouter } from "next/navigation";
import { PhaseView } from "../phase_view/PhaseView";
import { Button } from "@/components/elements/button/Button";
import { InfoModal, InfoObject } from "../info_modal/InfoModal";
import { ScoreViewMode } from "../score_view_mode/ScoreViewMode";
import { TechViewMode } from "../tech_view_mode/TechViewMode";
import { PlanetViewMode } from "../planet_view_mode/PlanetViewMode";
import { LawsViewMode } from "../laws_view_mode/LawsViewMode";
import { GameContext, useGameContext } from "@/hooks/GameContext";
import Link from "next/link";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPause, faPlay } from "@fortawesome/free-solid-svg-icons";
import { StringParam, useQueryParam, withDefault } from "use-query-params";
import { Spinner } from "@/components/elements/spinner/Spinner";

const NEW_GAME_ID = "new";

interface GameViewProps {
  wsUri: string;
  gameId: string;
}

type View = "Game" | "Score" | "Planets" | "Techs" | "Laws";

export const GameView = ({ gameId, wsUri }: GameViewProps) => {
  const [error, setError] = useState<string | null>(null);
  const [gameOptions, setGameOptions] = useState<GameOptions | null>(null);
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [currentViewMode, setCurrentViewMode] = useState<View>("Game");
  const [notFound, setNotFound] = useState<string | null>(null);
  const [infoObject, showInfo] = useState<InfoObject | null>(null);

  const [playingAs, setPlayingAs] = useQueryParam(
    "playing_as",
    withDefault(StringParam, null),
  );

  const router = useRouter();

  const { sendMessage, lastMessage, readyState } = useWebSocket(wsUri, {
    shouldReconnect: (closeEvent) => error === null,
  });

  const droppedConnection =
    readyState === ReadyState.CLOSED || readyState === ReadyState.CLOSING;

  const initialized = useRef(false);
  const isNewGame = gameId === NEW_GAME_ID;

  /* General message handling */
  useEffect(() => {
    if (lastMessage !== null) {
      console.log("MESSAGE", lastMessage);
      const data = JSON.parse(lastMessage.data);

      const error = data["HandleEventError"];
      if (error) {
        setError(error as string);
      } else {
        setError(null);
      }

      const notFoundGameId = data["NotFound"];
      if (notFoundGameId) {
        setNotFound(notFoundGameId as string);
      }

      const joinedGameId = data["JoinedGame"];
      if (joinedGameId && gameId !== joinedGameId) {
        router.replace(`/game/${joinedGameId}`);
      }

      const msgOpts = data["GameOptions"];
      if (msgOpts && !gameOptions) {
        setGameOptions(msgOpts as GameOptions);
      }

      const gs = data["GameState"];
      if (gs) {
        setGameState(gs as GameState);
      }
    }
  }, [lastMessage, gameOptions, gameId, router, setNotFound]);

  const sendMsg = (data: any) => sendMessage(JSON.stringify(data));
  const sendEvent = (data: any) => sendMsg({ Event: data });
  const sendUndo = () => sendMsg("Undo");
  const sendTimekeeping = (paused: boolean) =>
    sendEvent({
      TrackTime: {
        paused: paused,
      },
    });

  /* Send join game message */
  useEffect(() => {
    if (!initialized.current) {
      initialized.current = true;
      if (!isNewGame) {
        sendMessage(
          JSON.stringify({
            JoinGame: gameId,
          }),
        );
      }
    } else if (droppedConnection) {
      initialized.current = false;
    }
  }, [gameId, isNewGame, sendMessage, droppedConnection]);

  useEffect(() => {
    if (gameId.length !== 8) {
      setNotFound(gameId);
    }
  }, [gameId, setNotFound]);

  /* Ensure playingAs is valid */
  useEffect(() => {
    if (playingAs !== null && !gameState?.players[playingAs]) {
      setPlayingAs(null);
    }
  }, [playingAs, gameState?.players, setPlayingAs]);

  if (error) {
    return (
      <div className="card column">
        <h2>Invalid event</h2>
        <p>Failed to apply error due to:</p>
        <p> {error}</p>
        <Button onClick={() => window.location.reload()}>Reload game</Button>
      </div>
    );
  }

  if (droppedConnection) {
    return (
      <div className={styles.loadingContainer}>
        <p>Connection lost, retrying...</p>
        <Spinner />
      </div>
    );
  }

  if (isNewGame) {
    return (
      <CreateGameView
        startGame={(data) =>
          sendMsg({
            NewGame: data,
          })
        }
      />
    );
  }

  if (notFound) {
    return (
      <div className="card">
        Game {notFound} does not exist.
        <br />
        <Link href="/">Back</Link>
      </div>
    );
  }

  if (!gameOptions || !gameState) {
    return (
      <div className={styles.loadingContainer}>
        <p>Awaiting response from server</p>
        <Spinner />
      </div>
    );
  }

  const currentPlayer = gameState.currentPlayer
    ? gameState.players[gameState.currentPlayer]
    : null;

  return (
    <GameContext.Provider
      value={{
        gameOptions: gameOptions,
        gameState: gameState,
        sendEvent: sendEvent,
        sendUndo: sendUndo,
        showInfo: showInfo,
        isCurrentPlayer: playingAs === currentPlayer?.name,
        isGlobal: playingAs === null,
        isActive: playingAs === null || playingAs === currentPlayer?.name,
        isSpeaker: playingAs === gameState.speaker,
        playingAs: playingAs,
        setPlayingAs: setPlayingAs,
      }}
    >
      <InfoModal infoObject={infoObject} />
      <div className={`card ${styles.gameInfoCard}`}>
        <h4>Game: {gameId}</h4>
        <div className={styles.viewModeButtonGroup}>
          <Button
            onClick={() => setCurrentViewMode("Game")}
            disabled={currentViewMode === "Game"}
          >
            Game
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Score")}
            disabled={currentViewMode === "Score"}
          >
            Score
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Techs")}
            disabled={currentViewMode === "Techs"}
          >
            Techs
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Planets")}
            disabled={currentViewMode === "Planets"}
          >
            Planets
          </Button>
          <Button
            onClick={() => setCurrentViewMode("Laws")}
            disabled={currentViewMode === "Laws"}
          >
            Laws
          </Button>
        </div>
        <p className="marginTop">Round {gameState?.round}</p>
        {currentPlayer && (
          <p>
            Current player:{" "}
            {playingAs === currentPlayer.name ? "You!" : currentPlayer.name}
          </p>
        )}
        Currently viewing: {playingAs ?? "Global"}
        <div>
          <Button
            className="marginRight"
            onClick={() => sendTimekeeping(!gameState.timeTrackingPaused)}
          >
            <FontAwesomeIcon
              icon={gameState.timeTrackingPaused ? faPlay : faPause}
            />
          </Button>
          <Button onClick={() => sendUndo()}>Undo</Button>
        </div>
      </div>
      {gameOptions && gameState && (
        <DisplayViewMode viewMode={currentViewMode} />
      )}
    </GameContext.Provider>
  );
};

const CreateGameView = ({ startGame }: { startGame: (data: any) => void }) => {
  const [pok, setPok] = useState<boolean>(false);
  const [cod1, setCod1] = useState<boolean>(false);
  const [cod2, setCod2] = useState<boolean>(false);
  const [cod3, setCod3] = useState<boolean>(false);
  const [points, setPoints] = useState<number>(10);

  return (
    <div className={`card ${styles.createGameContainer}`}>
      <h2>Create Game</h2>
      <div className={styles.createGameRow}>
        <input
          type="checkbox"
          id="pok-checkbox"
          checked={pok}
          onChange={() => setPok(!pok)}
        />
        <label htmlFor="pok-checkbox">Prophecy of Kings</label>
      </div>
      <div className={styles.createGameRow}>
        <input
          type="checkbox"
          id="cod1"
          checked={cod1}
          onChange={() => setCod1(!cod1)}
        />
        <label htmlFor="cod1">Codex I</label>
      </div>
      <div className={styles.createGameRow}>
        <input
          type="checkbox"
          id="cod2"
          checked={cod2}
          onChange={() => setCod2(!cod2)}
        />
        <label htmlFor="cod2">Codex II</label>
      </div>
      <div className={styles.createGameRow}>
        <input
          type="checkbox"
          id="cod3"
          checked={cod3}
          onChange={() => setCod3(!cod3)}
        />
        <label htmlFor="cod3">Codex III</label>
      </div>
      <label htmlFor="points-slider">Winning Score</label>
      <div className={styles.createGameRow}>
        <input
          type="range"
          min={4}
          max={16}
          id="points-slider"
          value={points}
          onChange={(e) => setPoints(parseInt(e.target.value))}
        />
        <p>{points}</p>
      </div>
      <Button
        onClick={() =>
          startGame({
            points: points,
            pok: pok,
            cod1: cod1,
            cod2: cod2,
            cod3: cod3,
          })
        }
      >
        Create Game
      </Button>
    </div>
  );
};

interface DisplayViewModeProps {
  viewMode: View;
}

const DisplayViewMode = ({ viewMode }: DisplayViewModeProps) => {
  const { gameState, gameOptions } = useGameContext();

  switch (viewMode) {
    case "Game":
      return (
        <div className={styles.gamePageContainer}>
          {!(gameState.phase === "Setup" || gameState.phase === "Creation") && (
            <PlayerSidebar
              players={getPlayersFromGame(gameState, gameOptions)}
              score={gameState.score}
              currentTurnStartTime={gameState.currentTurnStartTime}
              isPaused={gameState.timeTrackingPaused}
            />
          )}
          <div className={styles.phaseContainer}>
            <PhaseView />
          </div>
        </div>
      );
    case "Score":
      return <ScoreViewMode />;
    case "Techs":
      return <TechViewMode />;
    case "Planets":
      return <PlanetViewMode />;
    case "Laws":
      return <LawsViewMode />;
    default:
      return <p>Unknown view mode ({viewMode})?</p>;
  }
};

function getPlayersFromGame(
  gameState: GameState,
  gameOptions: GameOptions,
): SidebarPlayer[] {
  return gameState.turnOrder.map((id) => {
    const p = gameState.players[id];
    return {
      name: p.name,
      faction: gameOptions.factions.filter((f) => f.faction === p.faction)[0],
      color: p.color,
      isActive: gameState.currentPlayer === p.name,
      hasPassed: gameState.passedPlayers.includes(id),
      cards: Object.entries(gameState.strategyCardHolders)
        .filter(([_, playerId]) => id === playerId)
        .map(([card]) => {
          const stratCard = card as StrategyCard;
          const played = gameState.spentStrategyCards.includes(stratCard);
          const isActive =
            gameState.actionProgress?.t === "Strategic" &&
            gameState.actionProgress?.card === card;
          return {
            name: stratCard,
            played: played,
            isActive: isActive,
          };
        }),
      planets: Object.keys(p.planets)
        .map((planet) => {
          return planet as Planet;
        })
        .map((planet) => {
          return {
            planet: planet,
            info: gameOptions.planetInfos[planet],
            attachments: p.planets[planet].map(
              (attachment) => gameOptions.planetAttachments[attachment],
            ),
          } as PlayerPlanetInfo;
        }),
      technologies: p.technologies.map((t) => {
        return {
          tech: t,
          info: gameOptions.technologies[t],
        };
      }),
      isSpeaker: gameState.speaker === p.name,
      playTime: {
        secs: gameState.playersPlayTime[id]?.secs ?? 0,
        nanos: gameState.playersPlayTime[id]?.nanos ?? 0,
      },
    };
  });
}
