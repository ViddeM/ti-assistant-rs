import { useEffect, useState } from "react";
import init, { run_game } from "../../../map_render/map_render";
import { Button } from "@/components/elements/button/Button";

export const MapViewMode = () => {
  const runGame = async () => {
    await init();
    run_game();
  };

  const [active, setActive] = useState(false);

  useEffect(() => {
    setActive(true);
    if (active) {
      runGame();
    }
  }, [active]);

  return (
    <div
      style={{ display: "flex", flexDirection: "column", alignItems: "center" }}
    >
      <Button onClick={() => setActive(false)}>
        View Map ({active ? "true" : "false"})
      </Button>
      <canvas id="map_render_canvas" width="1280" height="720" />
    </div>
  );
};
