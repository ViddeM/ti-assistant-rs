// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Expansion } from "./Expansion";
import type { PlanetTrait } from "./PlanetTrait";
import type { TechCategory } from "./TechCategory";

export type PlanetAttachmentInfo = { 
/**
 * The 'pretty' name of the attachment.
 */
name: string, 
/**
 * The expansion this attachment comes from.
 */
expansion: Expansion, 
/**
 * The planet trait required (if any) for this attachment to be attached to it.
 */
planetTrait: PlanetTrait | null, 
/**
 * The amount of resources gained from the attachment.
 */
resources: number, 
/**
 * The amount of influence gained from the attachment.
 */
influence: number, 
/**
 * Tech specialty (if any) gained from this attachment.
 */
techSpecialty: TechCategory | null, 
/**
 * The planet traits provided from this attachment.
 */
addedPlanetTraits: Array<PlanetTrait>, 
/**
 * Weather this attachment makes the planet a legendary planet or not.
 */
setLegendary: boolean, };