import { GameOptions } from "./GameOptions";

const BASE_URL = "http://localhost:8000/api";

export const Api = {
  gameOptions: {
    get: () => getEndpoint<GameOptions>("/game_options"),
  },
};

export type Response<T> = {
  data?: T;
  error?: string;
};

async function getEndpoint<T>(endpoint: string): Promise<Response<T>> {
  const url = `${BASE_URL}/${endpoint}`;
  const res = await fetch(url);
  if (!res.ok) {
    console.error("Failed to communicate with backend, error: ", res);
    return {
      error: "Failed to communicate with backend",
    };
  }

  const data = await res.json();

  return {
    data: data as T,
  } as Response<T>;
}
