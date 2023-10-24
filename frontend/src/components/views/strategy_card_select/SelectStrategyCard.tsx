"use client";

import { useState } from "react";
import styles from "./SelectStrategyCard.module.scss";
import Image from "next/image";
import { Button } from "@/components/elements/button/Button";

type SelectedCard = {
  cardNumber: number;
  faction: string;
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

interface StrategyCardButtonProps {
  cardName: string;
  cardNumber: number;
  selectedByFaction: string | null;
  setSelected: () => void;
}

const StrategyCardButton = ({
  cardName,
  cardNumber,
  selectedByFaction,
  setSelected,
}: StrategyCardButtonProps) => {
  const [selectedCards, setSelectedCards] = useState<SelectedCard[]>([]);

  return (
    <Button
      onClick={setSelected}
      disabled={selectedByFaction !== null}
      className={`${styles[`strategyCard${cardName}`]} ${
        styles.strategyCardButton
      }`}
    >
      {cardNumber}.<p>{cardName}</p>
      {selectedByFaction && (
        <Image
          src={`/icons/factions/${selectedByFaction}.png`}
          alt={`Faction Icon ${selectedByFaction}`}
          width={32}
          height={32}
        />
      )}
    </Button>
  );
};

const data = {
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
