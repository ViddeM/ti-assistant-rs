import { GameState } from "@/api/Game";
import { GameOptions, TechCategory, TechInfo } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { Faction } from "@/resources/types/factions";
import { useState } from "react";

interface SelectTechViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  playerId: string;
  onSelect: (tech: string) => void;
}

const TECH_CATEGORIES: TechCategory[] = [
  "Biotic",
  "Cybernetic",
  "Propulsion",
  "Warfare",
];

export const SelectTechView = ({
  gameState,
  gameOptions,
  playerId,
  onSelect,
}: SelectTechViewProps) => {
  const [selectedTech, setSelectedTech] = useState<string>("");
  const playerTechs = getPlayerTechs(gameState, gameOptions, playerId);
  const availableTechs = getAvailableTechs(
    playerTechs,
    gameOptions,
    gameState.players[playerId].faction
  );

  return (
    <div>
      <div>
        <label>Unit Upgrades</label>
        <Dropdown
          value={selectedTech}
          onChange={(e) => setSelectedTech(e.target.value)}
        >
          <option value="">-- Select --</option>
          {availableTechs
            .filter((t) => t.info.techType === "UnitUpgrade")
            .map((t) => (
              <option key={t.tech} value={t.tech}>
                {t.tech}
              </option>
            ))}
        </Dropdown>
      </div>
      {TECH_CATEGORIES.map((category) => (
        <div key={category}>
          <label>{category}</label>
          <Dropdown
            value={selectedTech}
            onChange={(e) => setSelectedTech(e.target.value)}
          >
            <option value="">-- Select --</option>

            {availableTechs
              .filter((t) => t.info.techType !== "UnitUpgrade")
              .filter((t) => isOfCategory(t.info, category))
              .map((t) => (
                <option key={t.tech} value={t.tech}>
                  {t.tech}
                </option>
              ))}
          </Dropdown>
        </div>
      ))}
      <Button
        disabled={selectedTech === ""}
        onClick={() => onSelect(selectedTech)}
      >
        Take
      </Button>
    </div>
  );
};

function isOfCategory(techInfo: TechInfo, category: TechCategory): boolean {
  if (techInfo.techType === "UnitUpgrade") {
    return false;
  }

  return techInfo.techType.Category === category;
}

function getPlayerTechs(
  gameState: GameState,
  gameOptions: GameOptions,
  player: string
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
  playerFaction: Faction
): Tech[] {
  return Object.keys(gameOptions.technologies)
    .filter((t) => !taken.map((t) => t.tech).includes(t))
    .filter((t) => {
      const origin = gameOptions.technologies[t].origin;
      if (origin === "Base") {
        return true;
      }

      return origin.Faction === playerFaction;
    })
    .map((t) => {
      return {
        tech: t,
        info: gameOptions.technologies[t],
      };
    });
}
