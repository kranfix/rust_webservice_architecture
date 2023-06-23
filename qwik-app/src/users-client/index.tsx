export type User = {
  id: string;
  username: string;
};

export const getUsers = async (): Promise<User[] | Error> => {
  let response: Response;
  try {
    console.log("[getUsers] before");
    response = await fetch(`http://127.0.0.1:3000/users`);
    console.log("[getUsers] After");
  } catch (err) {
    console.log("[getUsers] err on fetch:", err);
    return new Error("Server did not response");
  }
  console.log("[getUsers] validation OK");
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
    response = await fetch(`http://127.0.0.1:3000/users`, {
      method: "POST",
      body: JSON.stringify({ username }),
      headers: { "content-type": "application/json" },
    });
  } catch {
    return new Error("Server did not response");
  }

  if (!response.ok) {
    console.log(`[createUser] ${response.status} ${response.body}`);
    return new Error("");
  }

  const result = (await response.json()) as {
    data: User;
  };
  return result.data;
};

export const deleteUserById = async (id: string): Promise<User | Error> => {
  let response: Response;
  try {
    response = await fetch(`http://127.0.0.1:3000/users/` + id, {
      method: "DELETE",
      //body: JSON.stringify({ username }),
      headers: { "content-type": "application/json" },
    });
  } catch {
    return new Error("Server did not response");
  }

  if (!response.ok) {
    console.log(`[deleteUserById] ${response.status} ${response.body}`);
    return new Error("");
  }

  const result = (await response.json()) as {
    data: User;
  };
  return result.data;
};

export const updateUserById = async (
  id: string,
  username: string
): Promise<User | Error> => {
  let response: Response;
  try {
    response = await fetch(`http://127.0.0.1:3000/users/` + id, {
      method: "PUT",
      body: JSON.stringify({ username }),
      headers: { "content-type": "application/json" },
    });
  } catch {
    return new Error("Server did not response");
  }

  if (!response.ok) {
    console.log(`[updateUserById] ${response.status} ${response.body}`);
    return new Error("");
  }

  const result = (await response.json()) as {
    data: User;
  };
  return result.data;
};
