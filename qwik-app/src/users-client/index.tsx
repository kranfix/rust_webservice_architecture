export type User = {
  id: string;
  username: string;
};

export const getUsers = async (): Promise<User[] | Error> => {
  let response: Response;
  try {
    response = await fetch(`http://localhost:3000/users`, {
      // headers: { Accept: "application/json" },
    });
  } catch {
    return new Error("Server did not response");
  }

  if (!response.ok) {
    console.log(`[getUsers] ${response.status} ${response.body}`);
    return new Error("");
  }

  const result = (await response.json()) as {
    data: User[];
  };
  return result.data;
};

export const createUser = async (username: string): Promise<User | Error> => {
  let response: Response;
  try {
    response = await fetch(`http://localhost:3000/users`, {
      method: "POST",
      body: JSON.stringify({ username }),
      // headers: { Accept: "application/json" },
    });
  } catch {
    return new Error("Server did not response");
  }

  if (!response.ok) {
    console.log(`[getUsers] ${response.status} ${response.body}`);
    return new Error("");
  }

  const result = (await response.json()) as {
    data: User;
  };
  return result.data;
};
