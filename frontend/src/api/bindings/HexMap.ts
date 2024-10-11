// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Tile } from "./Tile";

/**
 * The galactic map.
 */
export type HexMap = { 
/**
 * All tiles that are in the game (at start).
 */
tiles: Array<Tile>, 
/**
 * How many rings there are in the galactic map (does not include tiles that are 'outside the galaxy').
 */
ring_count: number, };