import { Faction } from "@/resources/types/factions";
import Image from "next/image";

export interface FactionIconProps {
  faction: Faction;
  width?: number;
  height?: number;
}

export const FactionIcon = ({ faction, width, height }: FactionIconProps) => (
  <Image
    src={`/icons/factions/${faction}.png`}
    alt={`Faction Icon ${faction}`}
    width={width ? width : 32}
    height={height ? height : 32}
  />
);
