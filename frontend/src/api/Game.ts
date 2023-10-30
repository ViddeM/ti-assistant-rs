import { Faction } from "@/resources/types/factions";
import { GamePhase } from "@/resources/types/gamePhase";
import { StrategyCard } from "@/resources/types/strategyCards";
import { Color } from "./GameOptions";

export type PlayerId = string;

export interface Game {
  gameState: GameState;
  systems: System[];
}

export interface System {
  id: string;
  systemType: object;
  planets: string[];
  wormholes: object[];
}

export interface GameState {
  phase: GamePhase;
  players: { [key: PlayerId]: Player };
  tableOrder: PlayerId[];
  turnOrder: PlayerId[];
  strategyCardHolders: { [key: string]: PlayerId };
  currentPlayer: string | null;
  spentStrategyCards: StrategyCard[];
  passedPlayers: PlayerId[];
  strategicAction: StrategyCardProgress | null;
}

export interface StrategyCardProgress {
  card: StrategyCard;
  otherPlayers: { [key: PlayerId]: boolean };
}

export interface Player {
  name: string;
  faction: Faction;
  color: Color;
  planets: string[];
}
