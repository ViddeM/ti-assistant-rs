import { Faction } from "@/resources/types/factions";

export interface GameOptions {
  playerCounts: number[];
  minScore: number;
  maxScore: number;
  factions: FactionResponse[];
}

export interface FactionResponse {
  faction: Faction;
  name: string;
}
