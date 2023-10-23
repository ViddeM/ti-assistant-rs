export interface GameOptions {
  playerCounts: number[];
  minScore: number;
  maxScore: number;
  factions: Faction[];
}

export interface Faction {
  faction: string;
  name: string;
}
