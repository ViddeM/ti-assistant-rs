import { ObjectiveInfo } from "@/api/GameOptions";
import { GameContext, useGameContext } from "@/hooks/GameContext";
import styles from "./InfoModal.module.scss";

type InfoObject = { Agenda: AgendaInfo }
                | { Objective: ObjectiveInfo }
                | null;

export const InfoModal = ({infoObject}) => {
  const { showInfo } = useGameContext();

  if (!infoObject) {
    return null;
  }
  let [infoKind, info] = Object.entries(infoObject)[0];

  let title = "";
  let subtitle = "";
  let description = "";

  switch (infoKind) {
    case "Agenda":
      title = info.name;
      subtitle = info.kind; // TODO: show elect kind
      description = info.description;
      break;

    case "Objective":
      title = info.name;
      description = info.condition;
      switch (info.kind) {
        case "StageI":  subtitle = "Objective, Stage I"; break;
        case "StageII": subtitle = "Objective, Stage II"; break;
        default:        subtitle = "Secret Objective"; break;
      }
      break;

    default: // TODO: error?
  }

  return (
    <div // transparent background
      className={`${styles.infoModalBg}`}
      onClick={() => showInfo(null)} // dismiss the modal when clicked
    >
      <div // modal
        className={`${styles.infoModal}`}
        // don't dismiss the modal when the modal itself is clicked
        onClick={(e) => e.stopPropagation()}
       >
        <h1>{title}</h1>
        <hr />
        <h2>{subtitle}</h2>
        <p>{description}</p>
      </div>
    </div>
  );
};


