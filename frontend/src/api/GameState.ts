import { Faction } from "@/resources/types/factions";
import { GamePhase } from "@/resources/types/gamePhase";
import { StrategyCard } from "@/resources/types/strategyCards";
import { Color } from "./GameOptions";

export type PlayerId = string;

export interface GameState {
  phase: GamePhase;
  speaker: PlayerId | null;
  players: { [key: PlayerId]: Player };
  tableOrder: PlayerId[];
  turnOrder: PlayerId[];
  strategyCardHolders: { [key: string]: PlayerId };
  currentPlayer: string | null;
  spentStrategyCards: StrategyCard[];
  passedPlayers: PlayerId[];
  actionProgress: ActionProgress | null;
}

export interface ActionProgress {
  Strategic?: StrategyCardProgress;
  Tactical?: TacticalProgress;
  ActionCard?: ActionCardProgress;
}

export interface StrategyCardProgress {
  card: StrategyCard;
  primary: StrategicPrimaryProgress | null;
  otherPlayers: { [key: PlayerId]: StrategicSecondaryProgress };
}

export interface StrategicPrimaryProgress {
  Technology?: {
    tech: string;
    extra: string | null;
  };
  Politics?: {
    newSpeaker: string;
  };
}

export interface ActionCardProgress {
  card: string;
}

export type StrategicSecondaryProgress =
  | "Leadership"
  | "Diplomacy"
  | "Politics"
  | "Construction"
  | "Trade"
  | "Warfare"
  | {
      tech: string;
    }
  | "Imperial"
  | "Skipped";

export interface TacticalProgress {
  activatedSystem: string | null;
  takenPlanets: string[];
}

export interface Player {
  name: string;
  faction: Faction;
  color: Color;
  planets: string[];
  technologies: string[];
}
