const PLAYER_COUNTS = [3, 4, 5, 6, 7, 8];

export const NewGame = () => {
  return (
    <form>
      <div>
        <label htmlFor="player-count">Player count</label>
        <div>
          {PLAYER_COUNTS.map((n) => (
            <button
              type="button"
              disabled={n === playerCount}
              onClick={() => setPlayerCount(n)}
            >
              {n}
            </button>
          ))}
        </div>
      </div>
      <div>
        <label>Victory Points</label>
        <input type="number" min="4" max="20" value={victoryPoints} />
      </div>
    </form>
  );
};
