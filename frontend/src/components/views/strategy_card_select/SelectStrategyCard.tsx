"use client";

import { useState } from "react";
import styles from "./SelectStrategyCard.module.scss";
import { Button } from "@/components/elements/button/Button";
import { StrategyCardButton } from "./StrategyCardButton";
import { Faction } from "@/components/elements/factionIcon/FactionIcon";

type SelectedCard = {
  cardNumber: number;
  faction: Faction;
};

export const SelectStrategyCardView = () => {
  const [selectedCards, setSelectedCards] = useState<SelectedCard[]>([]);

  return (
    <div className="card">
      <h2>Select a strategy card</h2>
      <div className={styles.strategyCardsContainer}>
        {data.strategyCards.map((card) => (
          <StrategyCardButton
            key={card.number}
            cardName={card.name}
            cardNumber={card.number}
            selectedByFaction={
              selectedCards.filter((c) => c.cardNumber === card.number)[0]
                ?.faction ?? null
            }
            setSelected={() =>
              setSelectedCards([
                ...selectedCards,
                {
                  cardNumber: card.number,
                  faction:
                    data.players[selectedCards.length % data.players.length]
                      .faction,
                },
              ])
            }
          />
        ))}

        <Button>Start Action Phase</Button>
      </div>
    </div>
  );
};

const data: {
  players: { name: string; faction: Faction }[];
  strategyCards: { name: string; number: number }[];
} = {
  players: [
    {
      name: "Adam",
      faction: "SardakkNorr",
    },
    {
      name: "Eva",
      faction: "EmbersOfMuaat",
    },
    {
      name: "TestDude",
      faction: "NaaluCollective",
    },
    {
      name: "Tux",
      faction: "NekroVirus",
    },
  ],
  strategyCards: [
    {
      name: "Leadership",
      number: 1,
    },
    {
      name: "Diplomacy",
      number: 2,
    },
    {
      name: "Politics",
      number: 3,
    },
    {
      name: "Construction",
      number: 4,
    },
    {
      name: "Trade",
      number: 5,
    },
    {
      name: "Warfare",
      number: 6,
    },
    {
      name: "Technology",
      number: 7,
    },
    {
      name: "Imperial",
      number: 8,
    },
  ],
};
