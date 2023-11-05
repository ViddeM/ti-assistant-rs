import { GameState } from "@/api/Game";
import { GameOptions } from "@/api/GameOptions";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import { SelectTechView } from "../common_views/SelectTechView";

interface StrategyTechnologySecondaryViewProps {
  gameState: GameState;
  gameOptions: GameOptions;
  sendMessage: (data: any) => void;
}

export const StrategyTechnologySecondaryView = ({
  gameState,
  gameOptions,
  sendMessage,
}: StrategyTechnologySecondaryViewProps) => {
  const donePlayers = gameState.actionProgress?.Strategic?.otherPlayers!!;

  return (
    <div>
      {Object.keys(gameState.players)
        .filter((p) => p !== gameState.currentPlayer)
        .map((p) => {
          const player = gameState.players[p]!!;
          if (donePlayers[p]) {
            return (
              <div>
                {player.name} <FactionIcon faction={player.faction} />
                <p>{(donePlayers[p] as { tech: string }).tech}</p>
              </div>
            );
          } else {
            return (
              <div>
                {player.name} <FactionIcon faction={player.faction} />
                <SelectTechView
                  gameState={gameState}
                  gameOptions={gameOptions}
                  playerId={player.name}
                  onSelect={(tech) =>
                    sendMessage({
                      StrategicActionSecondary: {
                        player: player.name,
                        action: {
                          Technology: {
                            tech: tech,
                          },
                        },
                      },
                    })
                  }
                />
              </div>
            );
          }
        })}
    </div>
  );
};
