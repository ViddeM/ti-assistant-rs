import { ButtonHTMLAttributes, FC } from "react";
import styles from "./Button.module.scss";

export type ButtonBaseProps = ButtonHTMLAttributes<HTMLButtonElement>;

export const ButtonBase: FC<ButtonBaseProps> = ({ className, ...props }) => {
  const style = `${className} ${styles.buttonBase}`;
  return <button className={style} {...props} />;
};

export type ButtonProps = ButtonBaseProps;

export const Button: FC<ButtonProps> = ({ className, ...props }) => {
  const style = `${className?className:""} ${styles.button}`;
  return <ButtonBase className={style} {...props} />;
};
