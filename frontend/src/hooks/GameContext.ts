import { GameOptions } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import React from "react";
import { useContext } from "react";

export interface GameContext {
  gameOptions: GameOptions;
  gameState: GameState;
  sendEvent: (data: any) => void;
  sendUndo: () => void;
}

export const GameContext = React.createContext<GameContext | undefined>(
  undefined
);

export const useGameContext = (): GameContext => {
  const ctx = useContext(GameContext);
  return ctx!!;
};
