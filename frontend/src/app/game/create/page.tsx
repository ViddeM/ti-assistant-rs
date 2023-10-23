import { Api } from "@/api/Api";
import { CreateGameForm } from "./CreateGameForm";

export default async function NewGame() {
  const response = await Api.gameOptions.get();

  if (response.error) {
    return (
      <div>
        <p>{response.error}</p>
      </div>
    );
  }

  if (!response.data) {
    console.error("No response data?");
    return (
      <div>
        <p>Unknown error</p>
      </div>
    );
  }

  return <CreateGameForm gameOptions={response.data!!} />;
}
