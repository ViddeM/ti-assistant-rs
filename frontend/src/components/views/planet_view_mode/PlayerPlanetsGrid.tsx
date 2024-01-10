import { PlanetInfo } from "@/api/GameOptions";
import { Player } from "@/api/GameState";
import { Icon, IconType } from "@/components/elements/icon/Icon";
import styles from "./PlanetViewMode.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Button } from "@/components/elements/button/Button";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { useGameContext } from "@/hooks/GameContext";

export const PlayerPlanetsGrid = () => {
  const { gameState } = useGameContext();

  return (
    <div className={styles.playerPlanetCardsContainer}>
      {Object.keys(gameState.players).map((p) => (
        <PlayerPlanetsCard key={p} player={gameState.players[p]} />
      ))}
    </div>
  );
};

const NAME_COL_ALIGN = "left";
const TRAIT_COL_ALIGN = "center";
const RESOURCE_COL_ALIGN = "center";
const INFLUENCE_COL_ALIGN = "center";
const TECH_COL_ALIGN = "center";
const DELETE_COL_ALIGN = "right";

interface PlayerPlanetsCardProps {
  player: Player;
}

const NUM_COLUMNS = 6;
const PlayerPlanetsCard = ({ player }: PlayerPlanetsCardProps) => {
  const { gameOptions } = useGameContext();

  const playerTotals = Object.keys(player.planets).reduce(
    (tot, p) => {
      let planet = gameOptions.planetInfos[p];
      let attachments = player.planets[p].map(
        (a) => gameOptions.planetAttachments[a]
      );
      return {
        cultural:
          tot.cultural +
          (planet.planetTrait === "Cultural" ||
          attachments.filter((a) => a.planetTrait === "Cultural").length > 0
            ? 1
            : 0),
        industrial:
          tot.industrial +
          (planet.planetTrait === "Industrial" ||
          attachments.filter((a) => a.planetTrait === "Industrial").length > 0
            ? 1
            : 0),
        hazardous:
          tot.hazardous +
          (planet.planetTrait === "Hazardous" ||
          attachments.filter((a) => a.planetTrait === "Hazardous").length > 0
            ? 1
            : 0),
        resources:
          tot.resources +
          planet.resources +
          attachments.reduce((acc, a) => acc + a.resources, 0),
        influence:
          tot.influence +
          planet.influence +
          attachments.reduce((acc, a) => acc + a.influence, 0),
        warfare:
          tot.warfare +
          (planet.techSpecialty === "Warfare" ||
          attachments.filter((a) => a.techSpecialty === "Warfare").length > 0
            ? 1
            : 0),
        propulsion:
          tot.propulsion +
          (planet.techSpecialty === "Propulsion" ||
          attachments.filter((a) => a.techSpecialty === "Propulsion").length > 0
            ? 1
            : 0),
        cybernetic:
          tot.cybernetic +
          (planet.techSpecialty === "Cybernetic" ||
          attachments.filter((a) => a.techSpecialty === "Cybernetic").length > 0
            ? 1
            : 0),
        biotic:
          tot.biotic +
          (planet.techSpecialty === "Biotic" ||
          attachments.filter((a) => a.techSpecialty === "Biotic").length > 0
            ? 1
            : 0),
      };
    },
    {
      cultural: 0,
      industrial: 0,
      hazardous: 0,
      resources: 0,
      influence: 0,
      warfare: 0,
      propulsion: 0,
      cybernetic: 0,
      biotic: 0,
    }
  );

  return (
    <div className={`card ${styles.playerPlanetsTableContainer}`}>
      <table className={styles.playerPlanetsTable}>
        <thead>
          <tr>
            <th colSpan={NUM_COLUMNS}>
              <div className={styles.playerNameRow}>
                <FactionIcon faction={player.faction} />
                <h2>{player.name}</h2>
                <FactionIcon faction={player.faction} />
              </div>
            </th>
          </tr>
          <tr>
            <th colSpan={NUM_COLUMNS}>
              <div>
                <Icon name="cultural" />
                {playerTotals.cultural}

                <Icon name="industrial" />
                {playerTotals.industrial}

                <Icon name="hazardous" />
                {playerTotals.hazardous}
              </div>
              <div>
                <Icon name="warfare" isFilled={true} />
                {playerTotals.warfare}
                <Icon name="propulsion" isFilled={true} />
                {playerTotals.propulsion}
                <Icon name="cybernetic" isFilled={true} />
                {playerTotals.cybernetic}
                <Icon name="biotic" isFilled={true} />
                {playerTotals.biotic}
              </div>
            </th>
          </tr>
          <tr>
            <th align={NAME_COL_ALIGN} className={styles.borderBottomRow}>
              Name
            </th>
            <th align={TRAIT_COL_ALIGN} className={styles.borderBottomRow}>
              Type
            </th>
            <th align={RESOURCE_COL_ALIGN} className={styles.borderBottomRow}>
              <Icon name="resource" isFilled={true} />
            </th>
            <th align={INFLUENCE_COL_ALIGN} className={styles.borderBottomRow}>
              <Icon name="influence" isFilled={true} />
            </th>
            <th align={TECH_COL_ALIGN} className={styles.borderBottomRow}>
              <Icon name="legendary_planet" isFilled={true} />
            </th>
            <th
              align={DELETE_COL_ALIGN}
              className={styles.borderBottomRow}
            ></th>
          </tr>
          <tr>
            <th align={NAME_COL_ALIGN} className={styles.borderBottomRow}>
              Total
            </th>
            <th align={TRAIT_COL_ALIGN} className={styles.borderBottomRow}>
              {playerTotals.cultural +
                playerTotals.industrial +
                playerTotals.hazardous}
            </th>
            <th align={RESOURCE_COL_ALIGN} className={styles.borderBottomRow}>
              {playerTotals.resources}
            </th>
            <th align={INFLUENCE_COL_ALIGN} className={styles.borderBottomRow}>
              {playerTotals.influence}
            </th>
            <th align={TECH_COL_ALIGN} className={styles.borderBottomRow}>
              {playerTotals.warfare +
                playerTotals.propulsion +
                playerTotals.cybernetic +
                playerTotals.biotic}
            </th>
            <th align={DELETE_COL_ALIGN} className={styles.borderBottomRow} />
          </tr>
        </thead>
        <tbody>
          {Object.keys(player.planets).map((planet) => (
            <PlayerPlanetRow
              key={planet}
              planetId={planet}
              planet={gameOptions.planetInfos[planet]}
              attachments={player.planets[planet]}
            />
          ))}
        </tbody>
      </table>
    </div>
  );
};

interface PlayerPlanetRowProps {
  planetId: string;
  planet: PlanetInfo;
  attachments: string[];
}

const PlayerPlanetRow = ({
  planetId,
  planet,
  attachments,
}: PlayerPlanetRowProps) => {
  const { gameState, sendEvent } = useGameContext();

  return (
    <>
      <tr className={styles.planetRow}>
        <td align={NAME_COL_ALIGN}>{planetId}</td>
        <td align={TRAIT_COL_ALIGN}>
          {planet.planetTrait && (
            <Icon name={planet.planetTrait.toLowerCase() as IconType} />
          )}
        </td>
        <td align={RESOURCE_COL_ALIGN}>{planet.resources}</td>
        <td align={INFLUENCE_COL_ALIGN}>{planet.influence}</td>
        <td align={TECH_COL_ALIGN}>
          {planet.techSpecialty && (
            <Icon
              name={planet.techSpecialty.toLowerCase() as IconType}
              isFilled={true}
            />
          )}
        </td>
        <td align={DELETE_COL_ALIGN}>
          <Button
            className={styles.unclaimPlanetButton}
            onClick={() =>
              sendEvent({
                SetPlanetOwner: {
                  player: null,
                  planet: planetId,
                },
              })
            }
          >
            <FontAwesomeIcon icon={faTrash} />
          </Button>
        </td>
      </tr>
      {attachments.map((a) => (
        <AttachmentRow key={a} attachment={a} />
      ))}
    </>
  );
};

interface AttachmentRowProps {
  attachment: string;
}

const AttachmentRow = ({ attachment }: AttachmentRowProps) => {
  const { gameOptions } = useGameContext();
  const info = gameOptions.planetAttachments[attachment];

  return (
    <tr>
      <td colSpan={1}>ELLO</td>
      <td colSpan={4} align="right">
        {info.name}
      </td>
      <td align={DELETE_COL_ALIGN}>
        <Button className={styles.unclaimPlanetButton} onClick={() => {}}>
          <FontAwesomeIcon icon={faTrash} />
        </Button>
      </td>
    </tr>
  );
};
