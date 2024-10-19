// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ActionPhaseProgress } from "./ActionPhaseProgress";
import type { Agenda } from "./Agenda";
import type { AgendaElect } from "./AgendaElect";
import type { AgendaRecord } from "./AgendaRecord";
import type { AgendaState } from "./AgendaState";
import type { GameSettings } from "./GameSettings";
import type { Leader } from "./Leader";
import type { MapData } from "./MapData";
import type { Phase } from "./Phase";
import type { Player } from "./Player";
import type { Score } from "./Score";
import type { StatusPhaseState } from "./StatusPhaseState";
import type { StrategyCard } from "./StrategyCard";

export type GameState = { 
/**
 * The current round number of the game.
 */
round: number, 
/**
 * The settings for the game.
 */
gameSettings: GameSettings, 
/**
 * The current phase of the game.
 */
phase: Phase, 
/**
 * Information about the map, only available on games imported from milty.
 */
mapData: MapData, 
/**
 * Which players are in the game.
 */
players: { [key: string]: Player }, 
/**
 * The current speaker, if any.
 */
speaker: string | null, 
/**
 * The order that players are sitting around the table, starting with the speaker.
 */
tableOrder: Array<string>, 
/**
 * The current turn order, either based on table order or strategy cards (initiative).
 */
turnOrder: Array<string>, 
/**
 * Which players hold which strategy cards.
 */
strategyCardHolders: { [key in StrategyCard]: string }, 
/**
 * The current player, if any.
 */
currentPlayer: string | null, 
/**
 * Which strategy cards have been spent this action phase.
 */
spentStrategyCards: Array<StrategyCard>, 
/**
 * Which players have passed this phase.
 */
passedPlayers: Array<string>, 
/**
 * Tracks progress of the current action (if any) that is being taken.
 */
actionProgress: ActionPhaseProgress | null, 
/**
 * All things that concern scoring for the game.
 */
score: Score, 
/**
 * State for agenda phase.
 */
agenda: AgendaState | null, 
/**
 * List of past things voted on in the agenda phase.
 */
agendaVoteHistory: Array<AgendaRecord>, 
/**
 * Laws in play.
 */
laws: { [key in Agenda]: AgendaElect }, 
/**
 * State for the status phase.
 */
statusPhaseState: StatusPhaseState | null, 
/**
 * Leaders available for play for each player.
 */
availableLeaders: { [key: string]: Array<Leader> }, 
/**
 * Weather or not time should be tracked.
 */
timeTrackingPaused: boolean, 
/**
 * Time taken by each player to complete their rounds during the action phase.
 *
 * This does not include the time taken for the current round, that will be calculated and
 * included when the current player ends their turn.
 */
playersPlayTime: { [playerId: string]: { secs: number, nanos: number } }, 
/**
 * When the current player started their turn.
 */
currentTurnStartTime: string | null, 
/**
 * The player (if any) currently using [Faction::NaaluCollective]s faction ability:
 * "Telepathic", or the Naalu promisary note "Gift of Prescience".
 *
 * This player has initiative 0 in the action and status phase.
 */
naaluTelepathy: string | null, };