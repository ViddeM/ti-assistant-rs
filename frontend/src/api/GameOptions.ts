import { Faction } from "@/resources/types/factions";
import { System } from "./Game";
import { GamePhase } from "@/resources/types/gamePhase";

export interface GameOptions {
  playerCounts: number[];
  minScore: number;
  maxScore: number;
  factions: FactionResponse[];
  systems: System[];
  secretObjectives: SecretObjectiveInfo[];
  publicObjectives: PublicObjectiveInfo[];
}

export interface FactionResponse {
  faction: Faction;
  name: string;
}

export interface SecretObjectiveInfo {
  id: string;
  phase: GamePhase;
  name: string;
  condition: string;
}

export interface PublicObjectiveInfo {
  id: string;
  points: number;
  name: string;
  condition: string;
}
