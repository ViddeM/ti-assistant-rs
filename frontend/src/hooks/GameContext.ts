import { GameOptions } from "@/api/bindings/GameOptions";
import { GameState } from "@/api/bindings/GameState";
import React from "react";
import { useContext } from "react";
import { InfoObject } from "@/components/views/info_modal/InfoModal";

export interface GameContext {
  gameOptions: GameOptions;
  gameState: GameState;
  sendEvent: (data: any) => void;
  sendUndo: () => void;
  showInfo: (object: InfoObject | null) => void;
  isActive: boolean;
  isCurrentPlayer: boolean;
  isGlobal: boolean;
  isSpeaker: boolean;
  playingAs: string | null;
  setPlayingAs: (player: string | null) => void;
}

export const GameContext = React.createContext<GameContext | undefined>(
  undefined,
);

export const useGameContext = (): GameContext => {
  const ctx = useContext(GameContext);
  return ctx!!;
};
