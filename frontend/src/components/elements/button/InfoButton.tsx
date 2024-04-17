import { InfoObject } from "@/components/views/info_modal/InfoModal";
import { useGameContext } from "@/hooks/GameContext";
import { faInfo } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import styles from "./Button.module.scss";

export interface InfoButtonProps {
  info: InfoObject;
}

export const InfoButton = ({ info }: InfoButtonProps) => {
  const { showInfo } = useGameContext();

  return (
    <button onClick={() => showInfo(info)} className={styles.infoButton}>
      <FontAwesomeIcon icon={faInfo} />
    </button>
  );
};
