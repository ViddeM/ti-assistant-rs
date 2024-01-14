import { RelicProgress } from "@/api/GameState";
import { useGameContext } from "@/hooks/GameContext";

export const RelicCardView = () => {
  const { gameState, gameOptions } = useGameContext();

  const progress = gameState.actionProgress!!.Relic!!;
  const relic = gameOptions.relics[progress.relic];

  return (
    <div className="card">
      <h2>{relic.name}</h2>
      <RelicProgressView progress={progress} />
    </div>
  );
};

interface RelicProgressViewProps {
  progress: RelicProgress;
}

const RelicProgressView = ({ progress }: RelicProgressViewProps) => {
  const { gameState, sendEvent } = useGameContext();

  switch (progress.relic) {
    case ""
  }
};
