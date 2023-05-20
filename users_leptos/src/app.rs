use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::users::{ClientStateProvider, UserForm, UserList, UserListStateProvider};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context(cx);

  view! {
      cx,

      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

      // sets the document title
      <Title text="Welcome to Leptos"/>

      // content for this welcome page
      <Router>
          <main>
              <Routes>
                  <Route path="" view=|cx| view! { cx, <HomePage/> }/>
              </Routes>
          </main>
      </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
  view! { cx,
    //<ClientStateProvider>
      <div class="container">
          <h1> { "Usuarios" } </h1>
          //<UserListStateProvider>
          //  <UserForm />
          //  <UserList />
          //</UserListStateProvider>
      </div>
    //</ClientStateProvider>
  }
}
