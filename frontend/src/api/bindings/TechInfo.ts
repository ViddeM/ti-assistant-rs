// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Expansion } from "./Expansion";
import type { TechCategory } from "./TechCategory";
import type { TechOrigin } from "./TechOrigin";
import type { TechType } from "./TechType";

export type TechInfo = { 
/**
 * The name of the tech in 'pretty' format.
 */
name: string, 
/**
 * What type of tech this is.
 */
techType: TechType, 
/**
 * Weather the tech is general or belongs to a faction.
 */
origin: TechOrigin, 
/**
 * What requirements there are for the technology.
 */
requirements: { [key in TechCategory]: number }, 
/**
 * Which expansion this tech belongs to.
 */
expansion: Expansion, 
/**
 * The effects of the technology. Each element corresponds to a s
 */
effects: Array<string>, };