import { PlanetInfo } from "@/api/GameOptions";
import { Player } from "@/api/GameState";
import { Icon, IconType } from "@/components/elements/icon/Icon";
import styles from "./PlanetViewMode.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Button } from "@/components/elements/button/Button";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowTurnUp, faTrash } from "@fortawesome/free-solid-svg-icons";
import { useGameContext } from "@/hooks/GameContext";
import { faTrashCan } from "@fortawesome/free-regular-svg-icons";

export const PlayerPlanetsGrid = () => {
  const { gameState } = useGameContext();

  return (
    <div className={styles.playerPlanetCardsContainer}>
      {Object.keys(gameState.players).map((p) => (
        <PlayerPlanetsCard key={p} playerId={p} player={gameState.players[p]} />
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
  playerId: string;
  player: Player;
}

const NUM_COLUMNS = 6;
const PlayerPlanetsCard = ({ playerId, player }: PlayerPlanetsCardProps) => {
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
          attachments.filter((a) => a.addedPlanetTraits.includes("Cultural"))
            .length > 0
            ? 1
            : 0),
        industrial:
          tot.industrial +
          (planet.planetTrait === "Industrial" ||
          attachments.filter((a) => a.addedPlanetTraits.includes("Industrial"))
            .length > 0
            ? 1
            : 0),
        hazardous:
          tot.hazardous +
          (planet.planetTrait === "Hazardous" ||
          attachments.filter((a) => a.addedPlanetTraits.includes("Hazardous"))
            .length > 0
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
                {playerTotals.cultural}
                <Icon name="cultural" />

                {playerTotals.industrial}
                <Icon name="industrial" />

                {playerTotals.hazardous}
                <Icon name="hazardous" />
              </div>
              <div>
                {playerTotals.warfare}
                <Icon name="warfare" isFilled={true} />
                {playerTotals.propulsion}
                <Icon name="propulsion" isFilled={true} />
                {playerTotals.cybernetic}
                <Icon name="cybernetic" isFilled={true} />
                {playerTotals.biotic}
                <Icon name="biotic" isFilled={true} />
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
              playerId={playerId}
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
  playerId: string;
  planetId: string;
  planet: PlanetInfo;
  attachments: string[];
}

const PlayerPlanetRow = ({
  playerId,
  planetId,
  planet,
  attachments,
}: PlayerPlanetRowProps) => {
  const { sendEvent } = useGameContext();

  return (
    <>
      <tr className={styles.planetRow}>
        <td align={NAME_COL_ALIGN}>{planet.name}</td>
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
        <AttachmentRow
          key={a}
          playerId={playerId}
          planetId={planetId}
          attachment={a}
        />
      ))}
    </>
  );
};

interface AttachmentRowProps {
  playerId: string;
  planetId: string;
  attachment: string;
}

const AttachmentRow = ({
  playerId,
  planetId,
  attachment,
}: AttachmentRowProps) => {
  const { gameOptions, sendEvent } = useGameContext();
  const info = gameOptions.planetAttachments[attachment];

  return (
    <tr className={styles.attachmentRow}>
      <td align="center">
        <FontAwesomeIcon icon={faArrowTurnUp} style={{ rotate: "90deg" }} />
      </td>
      <td className={styles.attachmentRowText} align="right">
        <AttachmentIcon attachment={attachment} />
      </td>

      <td className={styles.attachmentRowText} align={RESOURCE_COL_ALIGN}>
        {info.resources}
      </td>

      <td className={styles.attachmentRowText} align={INFLUENCE_COL_ALIGN}>
        {info.influence}
      </td>

      <td align={TECH_COL_ALIGN}>
        {info.techSpecialty && (
          <Icon
            name={info.techSpecialty.toLowerCase() as IconType}
            isFilled={true}
          />
        )}
      </td>

      <td align={DELETE_COL_ALIGN}>
        <Button
          className={styles.unclaimPlanetButton}
          onClick={() =>
            sendEvent({
              RemovePlanetAttachment: {
                player: playerId,
                planet: planetId,
                attachment: attachment,
              },
            })
          }
        >
          <FontAwesomeIcon icon={faTrashCan} />
        </Button>
      </td>
    </tr>
  );
};

interface AttachmentIconProps {
  attachment: string;
}

const AttachmentIcon = ({ attachment }: AttachmentIconProps) => {
  switch (attachment) {
    case "DemilitarizedZone":
      return <Icon name="demilitarized" />;
    case "TombOfEmphidia":
      return <Icon name="tomb_of_emphida" />;
    case "UITheProgenitor":
      return <p style={{ color: "white" }}>✹✹✹</p>;
    case "BioticResearchFacility":
    case "CyberneticResearchFacility":
    case "PropulsionResearchFacility":
    case "WarfareResearchFacility":
      return (
        <div style={{ color: "gray" }}>
          ( 1 <Icon name="resource" isFilled={true} />
          1 <Icon name="influence" isFilled={true} />)
        </div>
      );
    case "BioticResearchFacilityResources":
      return (
        <div style={{ color: "gray" }}>
          (<Icon name="biotic" isFilled={true} />)
        </div>
      );
    case "CyberneticResearchFacilityResources":
      return (
        <div style={{ color: "gray" }}>
          (<Icon name="cybernetic" isFilled={true} />)
        </div>
      );
    case "PropulsionResearchFacilityResources":
      return (
        <div style={{ color: "gray" }}>
          (<Icon name="propulsion" isFilled={true} />)
        </div>
      );
    case "WarfareResearchFacilityResources":
      return (
        <div style={{ color: "gray" }}>
          (<Icon name="warfare" isFilled={true} />)
        </div>
      );
    case "NanoForge":
      return <Icon name="legendary_planet_circled" />;
    case "Terraform":
      return (
        <div>
          <Icon name="industrial" />
          <Icon name="hazardous" />
          <Icon name="cultural" />
        </div>
      );
    default:
      return <></>;
  }
};
