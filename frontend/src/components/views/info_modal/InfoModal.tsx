import { AgendaInfo } from "@/api/bindings/AgendaInfo";
import { LeaderInfo } from "@/api/bindings/LeaderInfo";
import { ObjectiveInfo } from "@/api/bindings/ObjectiveInfo";
import { ObjectiveKind } from "@/api/bindings/ObjectiveKind";
import { TechInfo } from "@/api/bindings/TechInfo";
import { TechType } from "@/api/bindings/TechType";
import { useGameContext } from "@/hooks/GameContext";
import styles from "./InfoModal.module.scss";
import { StrategyCard } from "@/resources/types/strategyCards";
import { ReactNode } from "react";
import Image from "next/image";

export type InfoObject =
  | { Agenda: AgendaInfo }
  | { Leader: LeaderInfo }
  | { Objective: ObjectiveInfo }
  | { Strategy: StrategyCard }
  | { Tech: TechInfo };

interface InfoModalProps {
  infoObject: InfoObject | null;
}

interface InfoFields {
  title: string;
  subtitle: string;
  description:
    | { type: "description"; description: string }
    | { type: "custom"; content: ReactNode };
}

export const InfoModal = ({ infoObject }: InfoModalProps) => {
  const { showInfo } = useGameContext();

  if (!infoObject) {
    return null;
  }

  const info = getInfo(infoObject);
  const description = info.description;
  const renderDescription = () => {
    switch (description.type) {
      case "description":
        return <p>{description.description}</p>;
      case "custom":
        return description.content;
    }
  };

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
        {renderDescription()}
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
      description: {
        type: "description",
        description: agenda.description,
      },
    };
  }

  if ("Leader" in info) {
    const leader = info["Leader"];
    return {
      title: leader.name,
      subtitle: leader.type,
      description: {
        type: "description",
        description: leader.description,
      },
    };
  }

  if ("Objective" in info) {
    const objective = info["Objective"];
    return {
      title: objective.name,
      subtitle: objectiveKindToString(objective.kind),
      description: {
        type: "description",
        description: objective.condition,
      },
    };
  }

  if ("Strategy" in info) {
    const strategy = info["Strategy"];
    const imagePath = getStrategyCardImagePath(strategy);

    return {
      title: strategy,
      subtitle: "",
      description: {
        type: "custom",
        content: (
          <Image
            alt={`Strategy card ${strategy}`}
            src={imagePath}
            style={{ objectFit: "contain" }}
            width={300}
            height={300}
          />
        ),
      },
    };
  }

  if ("Tech" in info) {
    const tech = info["Tech"];
    return {
      title: tech.name,
      subtitle: techTypeToString(tech.techType),
      description: {
        type: "description",
        description: tech.effects.join("\n"),
      },
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

function getStrategyCardImagePath(card: StrategyCard): string {
  const basePath = "/images/strat_cards";

  switch (card) {
    case "Leadership":
      return `${basePath}/leadership.webp`;
    case "Diplomacy":
      return `${basePath}/diplomacy_codex.webp`;
    case "Politics":
      return `${basePath}/politics.webp`;
    case "Construction":
      return `${basePath}/construction_pok.webp`;
    case "Trade":
      return `${basePath}/trade.webp`;
    case "Warfare":
      return `${basePath}/warfare.webp`;
    case "Technology":
      return `${basePath}/technology.webp`;
    case "Imperial":
      return `${basePath}/imperial.webp`;
  }
}
