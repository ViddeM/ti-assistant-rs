// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Objective } from "./Objective";
import type { SecretObjective } from "./SecretObjective";

export type StatusPhaseState = { 
/**
 * What each player scored, or didn't score, for their public objective slot.
 */
scoredPublicObjectives: { [key: string]: Objective | null }, 
/**
 * What each player scored, or didn't score, for their private objective slot.
 */
scoredSecretObjectives: { [key: string]: SecretObjective | null }, 
/**
 * What objective was revealed during this status phase.
 */
revealedObjective: Objective | null, 
/**
 * The number of objectives expected to have been revealed before we start revealing stage II cards.
 */
expectedObjectivesBeforeStageTwo: number, };