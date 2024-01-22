import { Faction } from "@/resources/types/factions";
import { GamePhase } from "@/resources/types/gamePhase";
import { System } from "./System";
import { AgendaElect, AgendaKind } from "./Agenda";

export interface GameOptions {
  minPlayers: number;
  maxPlayers: number;
  factions: FactionResponse[];
  colors: Color[];
  systems: System[];
  planetInfos: { [key: string]: PlanetInfo };
  planetAttachments: { [key: string]: PlanetAttachmentInfo };
  objectives: { [key: string]: ObjectiveInfo };
  technologies: { [key: string]: TechInfo };
  actionCards: { [key: string]: ActionCardInfo };
  agendas: { [key: string]: AgendaInfo };
  frontierCards: { [key: string]: FrontierCardInfo };
  relics: { [key: string]: RelicInfo };
}

export interface PlanetInfo {
  name: string;
  planetTrait: PlanetTrait | null;
  techSpecialty: TechSpeciality | null;
  resources: number;
  influence: number;
  isLegendary: boolean;
}

export interface PlanetAttachmentInfo {
  name: string;
  planetTrait: PlanetTrait | null;
  resources: number;
  influence: number;
  techSpecialty: TechCategory | null;
  addedPlanetTraits: PlanetTrait[];
  makeLegendary: boolean;
}

export interface TechInfo {
  name: string;
  techType: TechType;
  origin: TechOrigin;
  requirements: { [key: string]: number };
}

export type TechType =
  | "UnitUpgrade"
  | {
      Category: TechCategory;
    };

export type TechCategory = "Biotic" | "Propulsion" | "Cybernetic" | "Warfare";

export type TechOrigin =
  | "Base"
  | {
      Faction: Faction;
    };

export type PlanetTrait = "Cultural" | "Hazardous" | "Industrial";
export type TechSpeciality = "Biotic" | "Propulsion" | "Cybernetic" | "Warfare";

export interface FactionResponse {
  faction: Faction;
  name: string;
}

export interface ObjectiveInfo {
  name: string;
  condition: string;
  kind: ObjectiveKind;
  points: number;
}

export type ObjectiveKind =
  | "StageI"
  | "StageII"
  | {
      phase: GamePhase;
    };

export type Color =
  | "Blue"
  | "Green"
  | "Red"
  | "Yellow"
  | "Black"
  | "Purple"
  | "Orange"
  | "Pink";

export type ActionCardPlay =
  | "StatusPhaseReturnStrategyCards"
  | "Action"
  | "AfterActionCardIsPlayed"
  | "AfterStrategyCardIsPlayed"
  | "NotImplemented"
  | {
      Agenda: AgendaStagePlay;
    }
  | {
      StartOfPhase: GamePhase;
    };

export type AgendaStagePlay =
  | "WhenReveal"
  | "AfterReveal"
  | "AfterYouCastVotes"
  | "AfterSpeakerVotes"
  | "AfterElected"
  | "WhenOutcomeResolve";

export interface ActionCardInfo {
  card: string;
  name: string;
  expansion: string; // TODO
  playText: string;
  play: ActionCardPlay;
  effect: string;
  flavorText: string;
}

export interface AgendaInfo {
  name: string;
  description: string;
  kind: AgendaKind;
  elect: AgendaElect;
  origin: string; // TODO
}

export type FrontierCardType = "Action" | "Unhandled";

export interface FrontierCardInfo {
  card: string;
  name: string;
  expansion: string;
  numInDeck: number;
  frontierType: FrontierCardType;
}

export type RelicPlay =
  | "Action"
  | "Agenda"
  | "Possession"
  | "AfterTactical"
  | "Unhandled";

export interface RelicInfo {
  card: string;
  name: string;
  expansion: string;
  play: RelicPlay;
}
