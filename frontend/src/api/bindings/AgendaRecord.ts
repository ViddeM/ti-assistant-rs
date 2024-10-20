// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AgendaElect } from "./AgendaElect";
import type { VoteState } from "./VoteState";

export type AgendaRecord = { 
/**
 * The round number at the time.
 */
round: number, 
/**
 * The [VoteState] at the time of resolution.
 * Note that if the outcome was forced, the [VoteState] will not line up with it.
 */
vote: VoteState, 
/**
 * The outcome of the vote or None if it was discarded.
 */
outcome: AgendaElect | null, };