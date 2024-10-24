// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AgentInfo } from "./AgentInfo";
import type { CommanderInfo } from "./CommanderInfo";
import type { HeroInfo } from "./HeroInfo";

export type LeaderInfo = { "type": "Agent" } & AgentInfo | { "type": "Commander" } & CommanderInfo | { "type": "Hero" } & HeroInfo;