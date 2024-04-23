// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Objective } from "./Objective";
import type { SecretObjective } from "./SecretObjective";

export type Score = { 
/**
 * The amount of points required to win the game.
 */
maxPoints: number, 
/**
 * The amount of point that each player has.
 */
playerPoints: { [key: string]: number }, 
/**
 * Map from revealed objectives to the players that have scored them.
 */
revealedObjectives: { [key in Objective]: Array<string> }, 
/**
 * Completed secret objectives, by player.
 */
secretObjectives: { [key: string]: Array<SecretObjective> }, 
/**
 * Map from giver to receiver of Support for the Throne.
 */
supportForTheThrone: { [key: string]: string }, 
/**
 * Which (if any) player has the Shard of the Throne relic.
 */
shardOfTheThrone: string | null, 
/**
 * Which (if any) player has played the Crown of Emphidia relic.
 */
crownOfEmphidia: string | null, 
/**
 * Manually assigned points
 */
extraPoints: { [key: string]: number }, 
/**
 * Points gained by playing the Imperial strategy card action while holding Mecatol Rex.
 */
imperial: { [key: string]: number }, 
/**
 * The player who took the custodians token from Mecatol Rex.
 */
custodians: string | null, };