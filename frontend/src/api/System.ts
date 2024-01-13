import { Faction } from "@/resources/types/factions";

export interface System {
  id: string;
  systemType: SystemType;
  planets: string[];
  wormholes: WormHoleType[];
}

export type SystemType = AnomalySystem | "Hyperlane" | "Normal" | HomeSystem;

export type HomeSystem = {
  HomeSystem: Faction;
};

export type AnomalySystem = {
  Anomaly: AnomalyType;
};

export type AnomalyType =
  | "AsteroidField"
  | "Nebula"
  | "Supernova"
  | "MuaatSupernova"
  | "GravityRift";

export type WormHoleType = "Alpha" | "Beta" | "Gamma" | "Delta";
