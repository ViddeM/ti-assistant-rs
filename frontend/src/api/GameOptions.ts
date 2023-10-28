import { Faction } from "@/resources/types/factions";
import { System } from "./Game";

export interface GameOptions {
  playerCounts: number[];
  minScore: number;
  maxScore: number;
  factions: FactionResponse[];
  systems: System[];
}

export interface FactionResponse {
  faction: Faction;
  name: string;
}
