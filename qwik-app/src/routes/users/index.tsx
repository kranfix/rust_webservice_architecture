import {
  $,
  QRL,
  Slot,
  component$,
  useComputed$,
  useSignal,
  useStore,
  useTask$,
} from "@builder.io/qwik";
import styles from "./index.module.css";
import { Form, routeAction$, routeLoader$, zod$ } from "@builder.io/qwik-city";
import {
  User,
  createUser,
  deleteUserById,
  getUsers,
  updateUserById,
} from "../../users-client";
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

export const useUpdateUserById = routeAction$(
  async (props) => {
    console.log("useUpdateUserById", props.id);
    return await updateUserById(props.id, props.newUsername);
  },
  zod$({
    id: z.string().trim(),
    newUsername: z.string().trim(),
  })
);

interface UserListProps {
  users: User[];
}

export const UserList = component$<UserListProps>(({ users }) => {
  const deleteUserById = useDeleteUserById();
  const selectedUser = useSignal<User | null>(null);
  const modalStore = useStore({
    isOpen: false,
  });
  return (
    <>
      <ul>
        {users.map((user) => (
          <li>
            <Row>
              <div>
                <p>{user.id}</p>
                <p>{user.username}</p>
              </div>

              <CuykButton
                onClick$={() => {
                  selectedUser.value = user;
                  modalStore.isOpen = true;
                }}
              >
                PC
              </CuykButton>

              <Form
                action={deleteUserById}
                onSubmitCompleted$={() => {
                  console.log("submit completed", user.id);
                }}
              >
                <input type="hidden" name="id" value={user.id} />
                <CuykButton>X</CuykButton>
              </Form>
            </Row>
          </li>
        ))}
      </ul>
      <Modal title={"Edit User"} store={modalStore}>
        {selectedUser.value && modalStore.isOpen && (
          <Row>
            <EditUser
              user={selectedUser.value}
              onUpdate={$(() => {
                modalStore.isOpen = false;
              })}
            />
            <CuykButton
                onClick$={() => {
                  modalStore.isOpen = false;
                }}
              >
                X
            </CuykButton>
          </Row>
        )}
      </Modal>
    </>
  );
});

interface EditUserProps {
  user: User;
  onUpdate: QRL<() => void>;
}

const EditUser = component$<EditUserProps>(({ user, onUpdate }) => {
  const username = useSignal(user.username);
  const updateUserById = useUpdateUserById();
  const disableUpdate = useComputed$(() => {
    const trimmed = username.value.trim();
    return trimmed.length == 0 || trimmed == user.username;
  });
  return (
    <Form action={updateUserById} onSubmit$={onUpdate}>
      <input type="hidden" name="id" value={user.id} />
      <input type="text" name="newUsername" bind:value={username} />
      <CuykButton disabled={disableUpdate.value}>Update</CuykButton>
    </Form>
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
        <input type="text" name="username" bind:value={name} autoComplete="New username"/>
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
    <div class={styles.div}>
      <h1>Users</h1>
      {usersFirstLoad.value instanceof Error ? (
        "<TryUsersAgain onClick$={() => users.tryAgain()} />"
      ) : (
        <>
          <AddUserTextField />
          <UserList users={usersFirstLoad.value} />
        </>
      )}
    </div>
  );
});

interface CuykButtonProps {
  disabled?: boolean;
  onClick$?: QRL<() => void>;
}

export const CuykButton = component$<CuykButtonProps>(
  ({ disabled, onClick$ }) => {
    return (
      <button class={styles.button} disabled={disabled} onClick$={onClick$}>
        <Slot />
      </button>
    );
  }
);

///////////

export interface ModalStore {
  isOpen: boolean;
}

export interface ModalProps {
  title: string;
  store: ModalStore;
}

export const Modal = component$(({ title, store }: ModalProps) => {
  const dialog = useSignal<HTMLDialogElement>()

  useTask$(({track}) => {
    const isOpen = track(() => store.isOpen)
    console.log("MARK A", isOpen)
    if(isOpen) {
      console.log("MARK B", dialog.value)
      dialog.value?.showModal()
    } else {
      dialog.value?.close()
    }
  })
  
  return (
    <dialog ref={dialog}>
      <div>
        {title && <h3 style="color:black">{title}</h3>}
        <Slot/>
      </div>
    </dialog>
  );
});
