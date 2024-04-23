// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Faction } from "./Faction";
import type { Hero } from "./Hero";

/**
 * Information about a hero leader.
 */
export type HeroInfo = { 
/**
 * [Hero] variant for this hero.
 */
tag: Hero, 
/**
 * Faction that this hero belongs to.
 */
faction: Faction, 
/**
 * Name of the hero.
 */
name: string, 
/**
 * Name of the heros ability.
 */
ability: string, 
/**
 * Description of the heros ability.
 */
description: string, };