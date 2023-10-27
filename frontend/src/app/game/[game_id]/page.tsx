"use client";

import {
  Player,
  PlayerSidebar,
} from "@/components/views/players_sidebar/PlayersSidebar";
import {
  SelectStrategyCardView,
  SelectedCard,
} from "@/components/views/strategy_card_select/SelectStrategyCard";
import styles from "./styles.module.scss";
import { useState } from "react";
import { StrategyCard } from "@/resources/types/strategyCards";

export default function Game() {
  const [players, setPlayers] = useState<Player[]>(data.players);
  const selectedCards = playersToSelectedCards(players);
  const expectedStrategyCards = getExpectedStrategyCards(players.length);

  return (
    <div className={styles.gamePageContainer}>
      <PlayerSidebar players={players} />
      <SelectStrategyCardView
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
      />
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
