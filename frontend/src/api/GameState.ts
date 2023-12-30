import { Faction } from "@/resources/types/factions";
import { GamePhase } from "@/resources/types/gamePhase";
import { StrategyCard } from "@/resources/types/strategyCards";
import { Color } from "./GameOptions";
import { AgendaElect, AgendaElectKind, AgendaKind } from "./Agenda";

export type PlayerId = string;

export interface GameState {
  round: number;
  phase: GamePhase;
  players: { [key: PlayerId]: Player };
  speaker: PlayerId | null;
  tableOrder: PlayerId[];
  turnOrder: PlayerId[];
  strategyCardHolders: { [key: string]: PlayerId };
  currentPlayer: string | null;
  spentStrategyCards: StrategyCard[];
  passedPlayers: PlayerId[];
  actionProgress: ActionProgress | null;
  score: Score;
  statusPhaseState: StatusPhaseState | null;
  agenda: AgendaState | null;
  agendaVoteHistory: AgendaRecord[];
  laws: { [agenda: string]: AgendaElect };
  timeTrackingPaused: boolean;
  playersPlayTime: { [playerId: string]: Duration };
  currentTurnStartTime: string | null;
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
  Imperial?: {
    objective: string | null;
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

export interface Score {
  maxPoints: number;
  playerPoints: { [player: string]: number };
  revealedObjectives: { [objective: string]: string };
  secretObjectives: { [player: string]: string[] };
  supportForTheThrone: { [player: string]: string };
  extraPoints: { [player: string]: number };
  imperial: { [player: string]: number };
  custodians: string | null;
}

export interface StatusPhaseState {
  scoredPublicObjectives: { [player: string]: string | null };
  scoredSecretObjectives: { [player: string]: string | null };
  revealedObjective: string | null;
  expectedObjectivesBeforeStageTwo: number;
}

export interface AgendaState {
  round: AgendaRound;
  vote: VoteState | null;
}

export type AgendaRound = "Round1" | "Round2" | "Completed";

export interface VoteState {
  agenda: string;
  kind: AgendaKind;
  elect: AgendaElectKind;
  candidates: AgendaElect[];
  playerVotes: { [playerId: string]: Vote | null };
  outcomesByVotes: Vote[];
  expectedOutcome: AgendaElect | null;
}

export interface Vote {
  votes: number;
  outcome: AgendaElect;
}

export interface AgendaRecord {
  round: number;
  vote: VoteState;
  outcome: AgendaElect | null;
}

export interface Duration {
  secs: number;
  nanos: number;
}
