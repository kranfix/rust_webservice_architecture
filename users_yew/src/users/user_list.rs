use std::rc::Rc;

use yew::prelude::*;

use crate::users::{UserListAction, UserListState};

#[function_component]
pub fn UserList() -> Html {
  //let users = props.users.clone();
  let users = use_context::<UseReducerHandle<UserListState>>().unwrap();
  let dispatch = {
    let users = users.clone();
    Rc::new(move |act| users.clone().dispatch(act))
  };

  //let on_delete = props.on_delete.clone();
  let on_delete = {
    let users = users.clone();
    let dispatch = dispatch.clone();
    move |id: String| {
      let action = UserListAction::Rm(id.clone(), dispatch);
      users.dispatch(action);
    }
  };

  {
    let users = users.clone();
    let dispatch = dispatch.clone();
    use_effect(move || {
      let action = UserListAction::Fetch(dispatch);
      users.dispatch(action);
      || {}
    });
  }

  html! {
      <div class="mt-4">
        if users.list.is_empty(){
          <div id="emailHelp" class="form-text">{ "Please add users" }</div>
        } else {
          <ul class="list-group">
            {
              users.list.iter().map(|u| {
                let on_delete = on_delete.clone();
                let id = u.id.clone();
                let onclick = {
                  move |_| {
                    on_delete.clone()(id.clone());
                  }
                };
                html!(
                  <div class="container text-start">
                    <div class="row align-items-center">
                      <div class="col mt-1">
                        <li class="list-group-item">
                          {u.name.clone()}
                        </li>
                      </div>
                      <div class="col mb-1">
                        <button {onclick} type="button" class="btn btn-outline-danger">{ "X" }</button>
                      </div>
                    </div>
                  </div>
                )
              }).collect::<Html>()
            }
          </ul>
        }
      </div>
  }
}

#[function_component]
fn UserCard() -> Html {
  html! {}
}
