import { Game } from "./Game";
import { GameOptions } from "./GameOptions";

const BASE_URL = "http://localhost:8000/api";

export const Api = {
  gameOptions: {
    get: () => getRequest<GameOptions>("/game_options"),
  },
  game: {
    get_example: () => getRequest<Game>("/game/example"),
  },
};

export type Response<T> = {
  data?: T;
  error?: string;
};

async function postRequest<T>(
  endpoint: string,
  data: any
): Promise<Response<T>> {
  let body = JSON.stringify(data);

  return await sendRequest(endpoint, {
    method: "POST",
    body: body,
  });
}

async function getRequest<T>(endpoint: string): Promise<Response<T>> {
  return await sendRequest(endpoint);
}

async function sendRequest<T>(
  endpoint: string,
  requestOptions?: RequestInit
): Promise<Response<T>> {
  const url = `${BASE_URL}/${endpoint}`;
  const res = await fetch(url, requestOptions);
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
