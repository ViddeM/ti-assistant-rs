"use client";

import { useState } from "react";
import styles from "./SelectStrategyCard.module.scss";
import { Button } from "@/components/elements/button/Button";
import { StrategyCardButton } from "./StrategyCardButton";
import { Faction } from "@/resources/types/factions";
import { StrategyCard } from "@/resources/types/strategyCards";

type SelectedCard = {
  card: StrategyCard;
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
            key={card}
            strategyCard={card}
            selectedByFaction={
              selectedCards.filter((c) => c.card === card)[0]?.faction ?? null
            }
            setSelected={() =>
              setSelectedCards([
                ...selectedCards,
                {
                  card: card,
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
  strategyCards: StrategyCard[];
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
    "Leadership",
    "Diplomacy",
    "Politics",
    "Construction",
    "Trade",
    "Warfare",
    "Technology",
    "Imperial",
  ],
};
