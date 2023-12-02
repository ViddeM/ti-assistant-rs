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
    src={`/icons/factions/${faction}.png`}
    alt={`Faction Icon ${faction}`}
    width={width ? width : 32}
    height={height ? height : 32}
    className={className}
  />
);
