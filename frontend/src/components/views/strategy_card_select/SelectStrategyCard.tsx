import styles from "./SelectStrategyCard.module.scss";
import { Button } from "@/components/elements/button/Button";
import { StrategyCardButton } from "./StrategyCardButton";
import { Faction } from "@/resources/types/factions";
import {
  ALL_STRATEGY_CARDS,
  StrategyCard,
} from "@/resources/types/strategyCards";
import { InfoButton } from "@/components/elements/button/InfoButton";
import { useGameContext } from "@/hooks/GameContext";
import { Dropdown } from "@/components/elements/dropdown/Dropdown";
import { FactionIcon } from "@/components/elements/factionIcon/FactionIcon";
import Image from "next/image";

export type SelectedCard = {
  card: StrategyCard;
  faction: Faction;
};

export const SelectStrategyCardView = () => {
  const { gameState, sendEvent } = useGameContext();

  const selectedCards: SelectedCard[] = Object.entries(gameState.strategyCardHolders).map(
    ([strategyCard, playerId]) => {
      return {
        card: strategyCard as StrategyCard,
        faction: gameState.players[playerId].faction,
      };
    },
  );

  const expectedStrategyCards: number = getExpectedStrategyCards(
    Object.keys(gameState.players).length,
  );

  const selectCard: (card: StrategyCard) => void = (card) => {
    sendEvent({
      TakeStrategyCard: {
        player: gameState.currentPlayer,
        card: card,
      },
    });
  };

  const startActionPhase = () => sendEvent("CompleteStrategyPhase");

  return (
    <div className={`card ${styles.selectStrategyCardCard}`}>
      <h2>Select a strategy card</h2>
      <div className={styles.strategyCardsContainer}>
        {ALL_STRATEGY_CARDS.map((card) => (
          <div key={card} className={styles.selectCardRow}>
            <InfoButton
              info={{
                Strategy: card,
              }}
            />
            <StrategyCardButton
              strategyCard={card}
              selectedByFaction={
                selectedCards.filter((c) => c.card === card)[0]?.faction ?? null
              }
              setSelected={() => selectCard(card)}
              finishedSelectingCards={
                selectedCards.length === expectedStrategyCards
              }
            />
          </div>
        ))}

        <NaaluTelepathy />

        <Button
          disabled={selectedCards.length !== expectedStrategyCards}
          onClick={startActionPhase}
        >
          Start Action Phase
        </Button>
      </div>
    </div>
  );
};


// A dropdown show and change the owner of Naalu's ability "Telepathy"/"Give of Prescience".
const NaaluTelepathy = () => {
  const { gameState, sendEvent } = useGameContext();

  const giveNaaluTelepathy = (player: String) => sendEvent({ "PlayGiftOfPrescience": { "player": player } });


  const naaluTelepathy = gameState.naaluTelepathy;
  if (naaluTelepathy === null) {
    return null; // Naalu is not in the game, hide this component.
  }

  return (
    <div className={styles.naaluTelepathy}>
      <InfoButton
        info={{
          "Custom": {
            "title": "Telepathic",
            "subtitle": "Faction Ability / Promisary Note",
            "description": {
              type: "custom",
              content: (
                <div>
                  <br />
                  <h3>TELEPATHIC:</h3>
                  <div>At the end of the strategy phase, place the Naalu 0 token on your strategy card; you are first in initiative order.</div>
                  <br />
                  <br />
                  <h3><b>GIFT OF PRESCIENCE:</b></h3>
                  <div>At the end of the strategy phase:</div>
                  <br />
                  <div>Place this card face-up in your play area and place the Naalu 0 token on your strategy card;  you are first in the initiative order.  The Naalu player cannot use their TELEPATHIC faction ability during this game round.</div>
                  <br />
                  <div>Return this card to the Naalu player at the end of the status phase.</div>
                </div>
              ),
            },
          }
        }}
      />
      <div className={styles.naaluTelepathyBody}>
        < div className={styles.naaluTelepathyHeader} >
          <Image
            src={"/icons/resources/naalu_0_token.webp"}
            alt={`Naalu '0' token`}
            width={32}
            height={32}
          />
          Telepathic / Gift of Prescience
          < FactionIcon faction={"NaaluCollective"} />
        </div >
        <Dropdown
          required
          id="naalu_telepathy_dropdown"
          value={naaluTelepathy}
          onChange={(e) => {
            const val = e.target.value;
            giveNaaluTelepathy(val);
          }}
        >
          {Object.keys(gameState.players).map((player) => (
            <option key={player} value={player}>
              {player}
            </option>
          ))}
        </Dropdown>
      </div >
    </div >
  );
}

function getExpectedStrategyCards(noPlayers: number): number {
  if (noPlayers > 4) {
    return 1 * noPlayers;
  }

  return 2 * noPlayers;
}
