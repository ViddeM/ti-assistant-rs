import { Faction } from "@/resources/types/factions";
import { GamePhase } from "@/resources/types/gamePhase";
import { StrategyCard } from "@/resources/types/strategyCards";

export type PlayerId = string;

export interface Game {
  players: string[]; // TODO
  current: GameState;
  history: string[]; // TODO
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
  planets: string[];
}
