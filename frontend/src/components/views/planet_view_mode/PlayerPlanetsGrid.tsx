import { GameOptions, PlanetInfo } from "@/api/GameOptions";
import { GameState, Player } from "@/api/GameState";
import { Icon, IconType } from "@/components/elements/icon/Icon";
import styles from "./PlanetViewMode.module.scss";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { Button } from "@/components/elements/button/Button";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrash } from "@fortawesome/free-solid-svg-icons";

export interface PlayerPlanetsGridProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendEvent: (data: any) => void;
}

export const PlayerPlanetsGrid = ({
  gameState,
  gameOptions,
  sendEvent,
}: PlayerPlanetsGridProps) => {
  return (
    <div className={styles.playerPlanetCardsContainer}>
      {Object.keys(gameState.players).map((p) => (
        <PlayerPlanetsCard
          key={p}
          player={gameState.players[p]}
          planets={gameOptions.planetInfos}
          sendEvent={sendEvent}
        />
      ))}
    </div>
  );
};

const DELETE_COL_ALIGN = "center";
const NAME_COL_ALIGN = "left";
const TRAIT_COL_ALIGN = "center";
const RESOURCE_COL_ALIGN = "center";
const INFLUENCE_COL_ALIGN = "center";
const TECH_COL_ALIGN = "center";

interface PlayerPlanetsCardProps {
  player: Player;
  planets: { [id: string]: PlanetInfo };
  sendEvent: (data: any) => void;
}

const NUM_COLUMNS = 6;
const PlayerPlanetsCard = ({
  player,
  planets,
  sendEvent,
}: PlayerPlanetsCardProps) => {
  const playerTotals = player.planets.reduce(
    (tot, p) => {
      let planet = planets[p];
      return {
        cultural: tot.cultural + (planet.planetTrait === "Cultural" ? 1 : 0),
        industrial:
          tot.industrial + (planet.planetTrait === "Industrial" ? 1 : 0),
        hazardous: tot.hazardous + (planet.planetTrait === "Hazardous" ? 1 : 0),
        resources: tot.resources + planet.resources,
        influence: tot.influence + planet.influence,
        warfare: tot.warfare + (planet.techSpeciality === "Warfare" ? 1 : 0),
        propulsion:
          tot.propulsion + (planet.techSpeciality === "Propulsion" ? 1 : 0),
        cybernetic:
          tot.cybernetic + (planet.techSpeciality === "Cybernetic" ? 1 : 0),
        biotic: tot.biotic + (planet.techSpeciality === "Biotic" ? 1 : 0),
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
            <th
              align={DELETE_COL_ALIGN}
              className={styles.borderBottomRow}
            ></th>
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
          </tr>
          <tr>
            <th align={DELETE_COL_ALIGN} className={styles.borderBottomRow} />
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
          </tr>
        </thead>
        <tbody>
          {player.planets.map((planet) => (
            <PlayerPlanetRow
              key={planet}
              planetId={planet}
              planet={planets[planet]}
              sendEvent={sendEvent}
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
  sendEvent: (data: any) => void;
}

const PlayerPlanetRow = ({
  planetId,
  planet,
  sendEvent,
}: PlayerPlanetRowProps) => {
  return (
    <tr>
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
      <td align={NAME_COL_ALIGN}>{planetId}</td>
      <td align={TRAIT_COL_ALIGN}>
        {planet.planetTrait && (
          <Icon name={planet.planetTrait.toLowerCase() as IconType} />
        )}
      </td>
      <td align={RESOURCE_COL_ALIGN}>{planet.resources}</td>
      <td align={INFLUENCE_COL_ALIGN}>{planet.influence}</td>
      <td align={TECH_COL_ALIGN}>
        {planet.techSpeciality && (
          <Icon
            name={planet.techSpeciality.toLowerCase() as IconType}
            isFilled={true}
          />
        )}
      </td>
    </tr>
  );
};
