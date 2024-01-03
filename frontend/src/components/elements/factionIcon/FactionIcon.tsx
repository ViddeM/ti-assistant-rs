import { Faction } from "@/resources/types/factions";
import Image from "next/image";

export interface FactionIconProps {
  faction: Faction;
  width?: number;
  height?: number;
  className?: string;
}

export const FactionIcon = ({
  faction,
  width,
  height,
  className,
}: FactionIconProps) => (
  <Image
    src={factionIconName(faction)}
    alt={`Faction Icon ${faction}`}
    width={width ? width : 32}
    height={height ? height : 32}
    className={className}
  />
);

export function factionIconName(faction: Faction): string {
  return `/icons/factions/${faction}.png`;
}
