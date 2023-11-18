import { GameOptions, TechCategory, TechInfo } from "@/api/GameOptions";
import { GameState } from "@/api/GameState";
import { Button } from "@/components/elements/button/Button";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { Faction } from "@/resources/types/factions";
import { useState } from "react";
import styles from "./SelectTechView.module.scss";

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

const LABEL_COL_ALIGN = "right";
const DROPDOWN_COL_ALIGN = "left";

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
    <table className={styles.selectTechTable}>
      <tbody>
        <tr>
          <td align={LABEL_COL_ALIGN}>
            <label htmlFor="unit_upgrades">Unit Upgrades</label>
          </td>
          <td align={DROPDOWN_COL_ALIGN}>
            <Dropdown
              id="unit_upgrades"
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
          </td>
        </tr>
        {TECH_CATEGORIES.map((category) => (
          <tr key={category}>
            <td align={LABEL_COL_ALIGN}>
              <label htmlFor={category}>{category}</label>
            </td>
            <td align={DROPDOWN_COL_ALIGN}>
              <Dropdown
                id={category}
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
            </td>
          </tr>
        ))}
      </tbody>
      <tfoot>
        <tr>
          <th colSpan={2}>
            <Button
              disabled={selectedTech === ""}
              onClick={() => onSelect(selectedTech)}
            >
              Take
            </Button>
          </th>
        </tr>
      </tfoot>
    </table>
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
