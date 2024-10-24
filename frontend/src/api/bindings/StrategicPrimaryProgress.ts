// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Objective } from "./Objective";
import type { Technology } from "./Technology";

/**
 * The progress of the primary section of a strategy card.
 */
export type StrategicPrimaryProgress = { "Technology": { 
/**
 * What main technology was taken.
 */
tech: Technology | null, 
/**
 * What, if any, extra tech was taken (and paid for).
 */
extra: Technology | null, } } | { "Politics": { 
/**
 * Who the new speaker should be.
 */
newSpeaker: string, } } | { "Imperial": { 
/**
 * What objective, if any, was scored.
 */
objective: Objective | null, } };