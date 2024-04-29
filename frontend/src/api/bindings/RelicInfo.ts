// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Expansion } from "./Expansion";
import type { Relic } from "./Relic";
import type { RelicPlay } from "./RelicPlay";

/**
 * All relevant information for a relic card.
 */
export type RelicInfo = { 
/**
 * Which card this refers to.
 */
card: Relic, 
/**
 * The 'pretty' name of this relic.
 */
name: string, 
/**
 * Which expansion this relic came with.
 */
expansion: Expansion, 
/**
 * When the relic can be played / is relevant.
 */
play: RelicPlay, };