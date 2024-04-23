// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AgendaRound } from "./AgendaRound";
import type { VoteState } from "./VoteState";

export type AgendaState = { 
/**
 * Round number (e.g.. 1 or 2)
 */
round: AgendaRound, 
/**
 * State of the current agenda vote. This is `None` until an agenda is revealed.
 */
vote: VoteState | null, };