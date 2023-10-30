import { FC, HTMLAttributes, ImgHTMLAttributes } from "react";
import styles from "./Icon.module.scss";
import Image from "next/image";

export type IconType =
  | "biotic"
  | "cultural"
  | "cybernetic"
  | "hazardous"
  | "industrial"
  | "influence"
  | "legendary"
  | "legendary_planet_circled"
  | "legendary_planet"
  | "propulsion"
  | "resource"
  | "warfare";

export type IconProps = {
  name: IconType;
  isFilled: boolean;
};

export const Icon: FC<IconProps> = ({ name, isFilled }) => {
  const style = `${styles.icon}`;

  return (
    <Image
      src={`/icons/resources/${name}${isFilled ? "_filled" : ""}.png`}
      alt={`${name}${isFilled ? "_filled" : ""} icon`}
      width={32}
      height={32}
      className={style}
    />
  );
};
