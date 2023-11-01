import { FC, HTMLAttributes, ImgHTMLAttributes, ReactNode } from "react";
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
  isFilled?: boolean;
  width?: number;
  height?: number;
};

export const Icon: FC<IconProps> = ({ name, isFilled, width, height }) => {
  const style = `${styles.icon}`;

  return (
    <Image
      src={`/icons/resources/${name}${isFilled ? "_filled" : ""}.png`}
      alt={`${name}${isFilled ? "_filled" : ""} icon`}
      width={width ? width : 16}
      height={height ? height : 16}
      className={style}
    />
  );
};
