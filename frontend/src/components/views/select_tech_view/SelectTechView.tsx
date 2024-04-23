import { GameOptions } from "@/api/bindings/GameOptions";
import { Technology } from "@/api/bindings/Technology";
import { TechCategory } from "@/api/bindings/TechCategory";
import { TechInfo } from "@/api/bindings/TechInfo";
import { GameState } from "@/api/bindings/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { Faction } from "@/resources/types/factions";
import { useState } from "react";
import styles from "./SelectTechView.module.scss";
import { useGameContext } from "@/hooks/GameContext";
import { nameSort } from "@/utils/Utils";
import { Icon, IconType } from "@/components/elements/icon/Icon";

interface SelectTechViewProps {
  playerId: string;
  onSelect: (tech: Technology) => void;
  filteredTechs?: string[];
}

const TECH_CATEGORIES: string[] = [
  "Biotic",
  "Cybernetic",
  "Propulsion",
  "Warfare",
  "UnitUpgrade",
];

export const SelectTechView = ({
  playerId,
  onSelect,
  filteredTechs,
}: SelectTechViewProps) => {
  const { gameState, gameOptions } = useGameContext();

  const [selectedCategory, setSelectedCategory] = useState<string>("");
  const [selectedTech, setSelectedTech] = useState<string>("");

  const playerTechs = getPlayerTechs(gameState, gameOptions, playerId);
  const availableTechs = getAvailableTechs(
    playerTechs,
    gameOptions,
    gameState.players[playerId].faction,
    filteredTechs ?? [],
  );

  const selectedTechs = availableTechs.filter((t) => {
    const tt = t.info.techType;
    if (tt === "UnitUpgrade") {
      return selectedCategory === "UnitUpgrade";
    } else {
      return selectedCategory === tt.Category;
    }
  });

  const setCat = (category: string) => {
    setSelectedCategory(category);
    setSelectedTech("");
  };

  return (
    <div className="column">
      {TECH_CATEGORIES.map((category) => (
        <TechCategoryButton
          key={category}
          category={category}
          isSelected={category === selectedCategory}
          setSelected={() => setCat(category)}
          selectedTech={selectedTech}
          techOptions={selectedTechs}
          setSelectedTech={setSelectedTech}
        />
      ))}
      <Button
        disabled={selectedTech === ""}
        onClick={() => onSelect(selectedTech as Technology)}
      >
        Select tech
      </Button>
    </div>
  );
};

interface TechCategoryButtonProps {
  category: string;
  isSelected: boolean;
  setSelected: () => void;
  selectedTech: string;
  techOptions: {
    tech: string;
    info: TechInfo;
  }[];
  setSelectedTech: (tech: string) => void;
}

const TechCategoryButton = ({
  category,
  isSelected,
  setSelected,
  selectedTech,
  techOptions,
  setSelectedTech,
}: TechCategoryButtonProps) => {
  const rootStyles = `${styles.techCategoryButtonContainer} ${
    styles[`color${category}`]
  }`;

  if (!isSelected) {
    return (
      <Button onClick={setSelected} className={rootStyles}>
        {category}
        {category !== "UnitUpgrade" && (
          <Icon name={category.toLowerCase() as IconType} isFilled={true} />
        )}
      </Button>
    );
  }

  return (
    <fieldset className={`${rootStyles} centerRow`}>
      <legend>
        {category}
        {category !== "UnitUpgrade" && (
          <Icon name={category.toLowerCase() as IconType} isFilled={true} />
        )}
      </legend>
      <Dropdown
        value={selectedTech}
        onChange={(e) => setSelectedTech(e.target.value)}
      >
        <option value="">--Select Tech--</option>
        {techOptions.map((t) => (
          <option value={t.tech} key={t.tech}>
            {t.info.name}
          </option>
        ))}
      </Dropdown>
    </fieldset>
  );
};

function getPlayerTechs(
  gameState: GameState,
  gameOptions: GameOptions,
  player: string,
): Tech[] {
  return gameState.players[player].technologies.map((t) => {
    return {
      tech: t,
      info: gameOptions.technologies[t],
    };
  });
}

interface Tech {
  tech: string;
  info: TechInfo;
}

function getAvailableTechs(
  taken: Tech[],
  gameOptions: GameOptions,
  playerFaction: Faction,
  filterTechs: string[],
): Tech[] {
  return Object.keys(gameOptions.technologies)
    .map((t) => {
      return t as Technology;
    })
    .filter((t) => !taken.map((t) => t.tech).includes(t))
    .map((t) => {
      return {
        tech: t,
        info: gameOptions.technologies[t],
      };
    })
    .filter((t) => !filterTechs.includes(t.tech))
    .filter((t) => {
      if (t.info.origin === "Base") {
        return true;
      }

      return t.info.origin.Faction === playerFaction;
    })
    .sort((a, b) => nameSort(a.info, b.info));
}
