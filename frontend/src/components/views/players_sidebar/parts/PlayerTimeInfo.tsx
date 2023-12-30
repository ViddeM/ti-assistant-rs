import { useEffect, useState } from "react";
import { SidebarPlayer } from "../PlayersSidebar";

export interface PlayerTimeInfoProps {
  player: SidebarPlayer;
  isPaused: boolean;
  currentTurnStartTime: string | null;
}

export const PlayerTimeInfo = ({
  player,
  isPaused,
  currentTurnStartTime,
}: PlayerTimeInfoProps) => {
  const [extraSeconds, setExtraSeconds] = useState<number>(0);

  useEffect(() => {
    if (player.isActive && currentTurnStartTime !== null && !isPaused) {
      // To avoid the time freezing when taking actions.
      const startTime = Date.parse(currentTurnStartTime);
      const now = Date.now();
      setExtraSeconds(Math.floor((now - startTime) / 1000));

      let interval = setInterval(() => {
        const startTime = Date.parse(currentTurnStartTime);
        const now = Date.now();
        setExtraSeconds(Math.floor((now - startTime) / 1000));
      }, 1000);

      return () => {
        clearInterval(interval);
      };
    }
  }, [player, currentTurnStartTime, isPaused]);

  useEffect(() => {
    setExtraSeconds(0);
  }, [player.isActive]);

  const totalSeconds = player.playTime.secs + extraSeconds;
  const seconds = totalSeconds % 60;
  const totalMinutes = Math.floor((totalSeconds - seconds) / 60);
  const minutes = totalMinutes % 60;
  const totalHours = Math.floor((totalMinutes - minutes) / 60);

  return (
    <p>
      {pad(totalHours)}:{pad(minutes)}:{pad(seconds)}
    </p>
  );
};

function pad(number: number): string {
  if (number < 10) {
    return `0${number}`;
  }
  return number.toString();
}
