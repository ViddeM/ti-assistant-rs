// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AgendaStagePlay } from "./AgendaStagePlay";
import type { Phase } from "./Phase";

/**
 * When an action card can be played.
 */
export type ActionCardPlay = { "StartOfPhase": Phase } | { "Agenda": AgendaStagePlay } | "StatusPhaseReturnStrategyCards" | "Action" | "AfterActionCardIsPlayed" | "AfterStrategyCardIsPlayed" | "NotImplemented";