import { GameView } from "@/components/views/game/GameView";

export default function Game({ params }: { params: { gameId: string } }) {
  const wsUri = process.env.SERVER_WS_URI;
  if (!wsUri) {
    console.error(
      "No WS uri has been specified! Please ensure to set the 'SERVER_WS_URI' environment variable"
    );
  }
  return <GameView gameId={params.gameId} wsUri={wsUri!!} />;
}
