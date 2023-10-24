import { ButtonHTMLAttributes, FC } from "react";
import styles from "./Button.module.scss";

export type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement>;

export const Button: FC<ButtonProps> = ({ className, ...props }) => {
  const style = `${className} ${styles.buttonBase}`;
  return <button {...props} className={style} />;
};
