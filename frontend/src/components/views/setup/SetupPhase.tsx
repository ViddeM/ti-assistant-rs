import { GameState } from "@/api/Game";
import { GameOptions } from "@/api/GameOptions";
import { Button } from "@/components/elements/button/Button";

export interface SetupPhaseProps {
  gameOptions: GameOptions;
  gameState: GameState;
  sendMessage: (data: any) => void;
}

export const SetupPhase = ({
  gameOptions,
  gameState,
  sendMessage,
}: SetupPhaseProps) => {
  console.log("GAME STATE PLAYERS", gameState.players);

  const playerCount = Object.keys(gameState.players).length;
  const allowedNumberOfPlayers = gameOptions.playerCounts.includes(playerCount);
  return (
    <div className="card">
      Setup
      {playerCount < 8 && <Button>Add Player</Button>}
      <Button disabled={!allowedNumberOfPlayers}>Start game</Button>
    </div>
  );
};
