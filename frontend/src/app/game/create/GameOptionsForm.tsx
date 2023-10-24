import { Button } from "@/components/elements/button/Button";

export interface GameOptionsFormProps {
  playerCounts: number[];
  minScore: number;
  maxScore: number;

  playerCount: number;
  victoryPoints: number;

  setPlayerCount: (n: number) => void;
  setVictoryPoints: (n: number) => void;

  nextStage: () => void;
}

export const GameOptionsForm = ({
  playerCounts,
  minScore,
  maxScore,
  playerCount,
  victoryPoints,
  setPlayerCount,
  setVictoryPoints,
  nextStage,
}: GameOptionsFormProps) => (
  <>
    <div>
      <label htmlFor="player-count">Player count</label>
      <div>
        {playerCounts.map((n) => (
          <Button
            key={n}
            type="button"
            disabled={n === playerCount}
            onClick={() => setPlayerCount(n)}
          >
            {n}
          </Button>
        ))}
      </div>
    </div>
    <div>
      <label>Victory Points</label>
      <input
        type="number"
        min={minScore}
        max={maxScore}
        value={victoryPoints}
        onChange={(e) => {
          const parsed = parseInt(e.target.value);
          setVictoryPoints(parsed);
        }}
      />
    </div>
    <Button onClick={nextStage}>Next</Button>
  </>
);
