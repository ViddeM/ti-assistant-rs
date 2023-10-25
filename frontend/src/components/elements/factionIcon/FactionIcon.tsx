import { Faction } from "@/resources/types/factions";
import Image from "next/image";

export interface FactionIconProps {
  faction: Faction;
}

export const FactionIcon = ({ faction }: FactionIconProps) => (
  <Image
    src={`/icons/factions/${faction}.png`}
    alt={`Faction Icon ${faction}`}
    width={32}
    height={32}
  />
);
