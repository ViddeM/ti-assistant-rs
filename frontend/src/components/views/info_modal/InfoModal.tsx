import { AgendaInfo, ObjectiveInfo, ObjectiveKind, TechInfo } from "@/api/GameOptions";
import { useGameContext } from "@/hooks/GameContext";
import styles from "./InfoModal.module.scss";

export type InfoObject = { Agenda: AgendaInfo } 
    | { Objective: ObjectiveInfo }
    | { Strategy: TechInfo }
    | { Tech: TechInfo };

interface InfoModalProps {
  infoObject: InfoObject | null;
}

interface InfoFields {
  title: string;
  subtitle: string;
  description: string;
}

export const InfoModal = ({ infoObject }: InfoModalProps) => {
  const { showInfo } = useGameContext();

  if (!infoObject) {
    return null;
  }

  const info = getInfo(infoObject);

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
        <h1>{info.title}</h1>
        <hr />
        <h2>{info.subtitle}</h2>
        <p>{info.description}</p>
      </div>
    </div>
  );
};

function getInfo(info: InfoObject): InfoFields {
  if ("Agenda" in info) {
    const agenda = info["Agenda"];
    return {
      title: agenda.name,
      subtitle: agenda.kind, // TODO: show elect kind
      description: agenda.description,
    };
  }

  if ("Objective" in info) {
    const objective = info["Objective"];
    return {
      title: objective.name,
      subtitle: objectiveKindToString(objective.kind),
      description: objective.condition,
    };
  }

  if ("Tech" in info) {
    const tech = info["Tech"];
    return {
        title: tech.name,
        subtitle: techTypeToString(tech.techType),
        description: tech.effects.join("\n"),
    };
  }

  throw "Type error, invalid object!";
}

function objectiveKindToString(kind: ObjectiveKind): string {
  switch (kind) {
    case "StageI":
      return "Objective, Stage I";
    case "StageII":
      return "Objective, Stage II";
    default:
      return "Secret Objective";
  }
}

function techTypeToString(techType: TechType) {
    if (techType === "UnitUpgrade") {
        return "Unit Upgrade";
    }

    return `Technology, ${techType.Category}`;
}
