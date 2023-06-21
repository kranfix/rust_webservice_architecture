import {
  $,
  QRL,
  Slot,
  component$,
  useComputed$,
  useSignal,
  useStore,
} from "@builder.io/qwik";
//import styles from "../users/index.module.css";
import { Form, routeAction$, routeLoader$, zod$ } from "@builder.io/qwik-city";
import { User, createUser, deleteUserById, getUsers } from "../../users-client";
import { z } from "zod";
import { style, styled } from "styled-vanilla-extract/qwik";
import { Row } from "./styles.css";

export const useGetUsers = routeLoader$(async () => {
  console.log("useGetUsers");
  return await getUsers();
});

export const useCreateUser = routeAction$(
  async (props) => {
    console.log(props);
    return await createUser(props.username);
  },
  zod$({
    username: z.string().trim(),
  })
);

export const useDeleteUserById = routeAction$(
  async (props) => {
    console.log("useDeleteUserById", props);
    return await deleteUserById(props.id);
  },
  zod$({
    id: z.string().trim(),
  })
);

interface UserListProps {
  users: User[];
}

export const UserList = component$<UserListProps>(({ users }) => {
  const deleteUserById = useDeleteUserById();
  return (
    <ul>
      {users.map((user) => (
        <li>
          <Form
            action={deleteUserById}
            onSubmitCompleted$={() => {
              console.log("submit completed", user.id);
            }}
          >
            <Row>
              <div>
                <p>{user.id}</p>
                <p>{user.username}</p>
              </div>
              <input type="hidden" name="id" value={user.id} />
              <CuykButton>X</CuykButton>
            </Row>
          </Form>
        </li>
      ))}
    </ul>
  );
});

const AddUserTextField = component$<{}>(() => {
  const name = useSignal("");
  const canAdd = useComputed$(() => name.value.trim() != "");
  const createUser = useCreateUser();

  return (
    <Row>
      <Form
        action={createUser}
        onSubmitCompleted$={() => {
          console.log("submit completed", name.value);
          name.value = "";
        }}
      >
        <input type="text" name="username" bind:value={name} />
        <CuykButton
          disabled={!canAdd.value}
          onClick$={() => {
            console.log("button onClick", name.value);
          }}
        >
          Add
        </CuykButton>
      </Form>
      <CuykButton
        onClick$={() => {
          name.value = "";
        }}
      >
        Clear
      </CuykButton>
    </Row>
  );
});

interface TryUsersAgainProps {
  onClick$: QRL<() => void>;
}

const TryUsersAgain = component$<TryUsersAgainProps>(({ onClick$ }) => {
  return (
    <>
      <p>There was an error</p>
      <CuykButton onClick$={onClick$}>Try again!</CuykButton>
    </>
  );
});

interface UsersStore {
  isLoading: boolean;
  list: undefined | User[];
  tryAgain: QRL<(this: UsersStore) => Promise<void>>;
  createOne: QRL<(this: UsersStore, username: string) => Promise<void>>;
}

const useUsersStore = (firstLoad: Error | User[]) => {
  const users = useStore<UsersStore>({
    isLoading: false,
    list: firstLoad instanceof Error ? undefined : [...firstLoad],
    tryAgain: $(async function (this: UsersStore) {
      this.isLoading = true;
      const result = await getUsers();
      this.isLoading = false;
      if (result instanceof Error) {
        console.log("Show dialog");
      } else {
        this.list = result;
      }
    }),
    createOne: $(async function (this: UsersStore, username: string) {
      this.isLoading = true;
      const result = await createUser(username);
      this.isLoading = false;
      if (result instanceof Error) {
        console.log("Show dialog");
      } else {
        this.list ??= [];
        this.list.push(result);
      }
    }),
  });
  return users;
};

export default component$(() => {
  const usersFirstLoad = useGetUsers();
  //const users = useUsersStore(usersFirstLoad.value);

  // return (
  //   <>
  //     <h1>Users</h1>
  //     {users.isLoading && <>... is loading</>}
  //     {users.list === undefined ? (
  //       <TryUsersAgain onClick$={() => users.tryAgain()} />
  //     ) : (
  //       <>
  //         <AddUserTextField />
  //         <UserList users={users.list} />
  //       </>
  //     )}
  //   </>
  // );
  return (
    <>
      <h1>Users</h1>
      {usersFirstLoad.value instanceof Error ? (
        "<TryUsersAgain onClick$={() => users.tryAgain()} />"
      ) : (
        <>
          <AddUserTextField />
          <UserList users={usersFirstLoad.value} />
        </>
      )}
    </>
  );
});

interface CuykButtonProps {
  disabled?: boolean;
  onClick$?: QRL<() => void>;
}

export const CuykButton = component$<CuykButtonProps>(
  ({ disabled, onClick$ }) => {
    return (
      <button disabled={disabled ?? false} onClick$={onClick$}>
        <Slot />
      </button>
    );
  }
);
