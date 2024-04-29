// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Expansion } from "./Expansion";
import type { FrontierCard } from "./FrontierCard";
import type { FrontierCardType } from "./FrontierCardType";

export type FrontierCardInfo = { 
/**
 * Which frontier card this is in regards to.
 */
card: FrontierCard, 
/**
 * The 'pretty' name of the action card.
 */
name: string, 
/**
 * Which expansion this card comes from.
 */
expansion: Expansion, 
/**
 * The number of cards that exists in a deck.
 */
numInDeck: number, 
/**
 * Weather this frontier carries with it an action or not.
 */
frontierType: FrontierCardType, };