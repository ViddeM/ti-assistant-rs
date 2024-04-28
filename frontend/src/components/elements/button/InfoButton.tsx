import { InfoObject } from "@/components/views/info_modal/InfoModal";
import { useGameContext } from "@/hooks/GameContext";
import { faInfo } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import styles from "./Button.module.scss";
import { ButtonHTMLAttributes, FC } from "react";

export type InfoButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  info: InfoObject;
};

export const InfoButton: FC<InfoButtonProps> = ({
  info,
  className,
  ...props
}) => {
  const { showInfo } = useGameContext();

  const style = `${className} ${styles.infoButton}`;

  return (
    <button onClick={() => showInfo(info)} className={style} {...props}>
      <FontAwesomeIcon icon={faInfo} />
    </button>
  );
};
