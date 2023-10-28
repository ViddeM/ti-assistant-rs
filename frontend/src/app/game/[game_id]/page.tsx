import {
  Player,
  PlayerSidebar,
} from "@/components/views/players_sidebar/PlayersSidebar";
import { SelectedCard } from "@/components/views/strategy_card_select/SelectStrategyCard";
import styles from "./styles.module.scss";
import { StrategyCard } from "@/resources/types/strategyCards";
import { Api } from "@/api/Api";
import { Game, GameState } from "@/api/Game";

export default async function Game() {
  const resp = await Api.game.get_example();
  if (resp.error) {
    return <div>Error</div>;
  }

  if (!resp.data) {
    return <div>Failed to load data</div>;
  }

  const sidebarPlayers = getPlayersFromGame(resp.data.current);

  return (
    <div className={styles.gamePageContainer}>
      <PlayerSidebar players={sidebarPlayers} />
      {/* <SelectStrategyCardView
        selectedCards={selectedCards}
        selectCard={(card) => {
          const playerIndex = selectedCards.length % players.length;
          const newPlayers = players.map((player, index) => {
            if (index === playerIndex) {
              return addCardToPlayer(player, card);
            } else {
              return player;
            }
          });
          setPlayers(newPlayers);
        }}
        expectedStrategyCards={expectedStrategyCards}
      /> */}
      <div />
    </div>
  );
}

function getExpectedStrategyCards(numPlayers: number): number {
  if (numPlayers > 4) {
    return 1 * numPlayers;
  }
  return 2 * numPlayers;
}

function playersToSelectedCards(players: Player[]): SelectedCard[] {
  return players.flatMap((p) => {
    return p.cards.map((c) => {
      return {
        card: c.name,
        faction: p.faction,
      };
    });
  });
}

function addCardToPlayer(player: Player, card: StrategyCard): Player {
  return {
    ...player,
    cards: [
      ...player.cards,
      {
        name: card,
        played: false,
      },
    ],
  };
}

const data: {
  players: Player[];
} = {
  players: [
    {
      name: "Adam",
      color: "#F0F",
      faction: "SardakkNorr",
      cards: [],
    },
    {
      name: "Eva",
      color: "#FFF",
      faction: "EmbersOfMuaat",
      cards: [],
    },
    // {
    //   name: "TestDude",
    //   color: "#FF0",
    //   faction: "NaaluCollective",
    //   cards: [],
    // },
    {
      name: "Tux",
      color: "#00F",
      faction: "NekroVirus",
      cards: [],
    },
  ],
};

function getPlayersFromGame(gameState: GameState): Player[] {
  return Object.entries(gameState.players).map(([id, p]) => {
    return {
      name: p.name,
      faction: p.faction,
      color: "#000",
      cards: Object.entries(gameState.strategyCardHolders)
        .filter(([_, playerId]) => id === playerId)
        .map(([card]) => {
          let stratCard = card as StrategyCard;
          return {
            name: stratCard,
            played: gameState.spentStrategyCards.includes(stratCard),
          };
        }),
    };
  });
}
