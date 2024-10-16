import { useGameContext } from "@/hooks/GameContext";

export const MapViewMode = ({ wsUri }: { wsUri: string }) => {
  const { gameId, gameState } = useGameContext();

  if (!gameState.mapData) {
    return (
      <div>
        No milty string was provided at creation, map will not be available for
        this game
      </div>
    );
  }

  return (
    <div
      style={{ display: "flex", flexDirection: "column", alignItems: "center" }}
    >
      <iframe
        id="map_render_game_id"
        src={`/map_render/index.html?gameId=${gameId}&serverUrl=${wsUri}`}
        width="1280"
        height="720"
      ></iframe>
    </div>
  );
};
