use crate::users::UserController;
use leptos::{html::Input, *};
//use web_sys::HtmlInputElement;

#[component]
pub fn UserForm(cx: Scope) -> impl IntoView {
  let input_node_ref = create_node_ref::<Input>(cx);
  //let name = use_state(String::default);
  let (name, set_name) = create_signal(cx, String::default());
  //let on_input = move |_| set_name.update(|_| "new_name");
  let oninput = {
    move |_| {
      let input = input_node_ref.get();

      if let Some(input) = &input {
        let text = input.value();
        let trimmed = text.trim().to_owned();
        set_name(trimmed);
      }
    }
  };

  let clear_text = {
    //let name = name.clone();
    // Callback::from(move |_: ()| {
    move || {
      set_name(String::default());
    }
  };

  view! { cx,
    <>
      <form>
        <div class="mb-3">
          <input ref={input_node_ref} type="text" on:input=oninput value=name class="form-control"/>
          <div id="emailHelp" class="form-text">{"Name"}</div>
        </div>
      </form>

      <AddUserButton name={name()} on_added={clear_text}/>
    </>
  }
}

#[component]
fn AddUserButton<A>(cx: Scope, name: String, on_added: A) -> impl IntoView
where
  A: Fn() + 'static,
{
  let users = use_context::<UserController>(cx).unwrap();

  let onclick = {
    let name = name.clone();
    let on_added = on_added;

    move |_| {
      users.add_one(name.clone());
      on_added()
    }
  };

  view!(cx,
    <button disabled=name.is_empty() on:click=onclick  type="button" class="btn btn-primary">{ "Add user" }</button>
  )
}
