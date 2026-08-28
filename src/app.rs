use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[cfg(feature = "ssr")]
mod ssr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

#[server]
pub async fn list_todos() -> Result<Vec<Todo>, ServerFnError> {
    #[cfg(feature = "ssr")]
    return ssr::list().await;

    #[cfg(not(feature = "ssr"))]
    unreachable!("list_todos executes on the server")
}

#[server]
pub async fn create_todo(title: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    return ssr::create(title).await;

    #[cfg(not(feature = "ssr"))]
    unreachable!("create_todo executes on the server")
}

#[server]
pub async fn toggle_todo(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    return ssr::toggle(id).await;

    #[cfg(not(feature = "ssr"))]
    unreachable!("toggle_todo executes on the server")
}

#[server]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    return ssr::delete(id).await;

    #[cfg(not(feature = "ssr"))]
    unreachable!("delete_todo executes on the server")
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-todo.css"/>
        <Title text="Leptos Todo"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("") view=TodoPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn TodoPage() -> impl IntoView {
    let create = ServerAction::<CreateTodo>::new();
    let toggle = ServerAction::<ToggleTodo>::new();
    let delete = ServerAction::<DeleteTodo>::new();

    let todos = Resource::new(
        move || {
            (
                create.version().get(),
                toggle.version().get(),
                delete.version().get(),
            )
        },
        move |_| list_todos(),
    );

    let todo_list = move || {
        Suspend::new(async move {
            todos.await.map(|todos| {
                if todos.is_empty() {
                    Either::Left(view! {
                        <p class="rounded-lg border border-dashed border-slate-300 bg-white px-4 py-8 text-center text-sm text-slate-500">
                            "No todos yet. Add one above."
                        </p>
                    })
                } else {
                    Either::Right(view! {
                        <ul class="space-y-2">
                            {todos.into_iter().map(|todo| {
                                let title_class = if todo.completed {
                                    "min-w-0 flex-1 truncate text-slate-400 line-through"
                                } else {
                                    "min-w-0 flex-1 truncate text-slate-900"
                                };
                                let toggle_label = if todo.completed { "✔" } else { "✖" };

                                view! {
                                    <li class="flex items-center gap-3 rounded-lg border border-slate-200 bg-white p-3 shadow-sm">
                                        <ActionForm action=toggle>
                                            <input type="hidden" name="id" value=todo.id/>
                                            <button
                                                type="submit"
                                                class="rounded-md border border-slate-300 px-3 py-1.5 text-sm font-medium text-slate-700 hover:bg-slate-50 focus:outline-none focus:ring-2 focus:ring-slate-400"
                                            >
                                                {toggle_label}
                                            </button>
                                        </ActionForm>

                                        <span class=title_class>{todo.title}</span>

                                        <ActionForm action=delete>
                                            <input type="hidden" name="id" value=todo.id/>
                                            <button
                                                type="submit"
                                                class="rounded-md px-2 py-1.5 text-sm font-medium text-rose-600 hover:bg-rose-50 focus:outline-none focus:ring-2 focus:ring-rose-300"
                                                aria-label="Delete todo"
                                            >
                                                "Delete"
                                            </button>
                                        </ActionForm>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    })
                }
            })
        })
    };

    view! {
        <section class="mx-auto min-h-screen max-w-2xl px-4 py-12 sm:px-6">
            <div class="mb-8">
                <p class="mb-2 text-xs font-semibold uppercase tracking-[0.2em] text-slate-500">
                    "Rust full stack"
                </p>
                <h1 class="text-3xl font-bold tracking-tight text-slate-950">"Todo"</h1>
                <p class="mt-2 text-sm text-slate-600">
                    "Leptos + Axum + SQLx + SQLite + Tailwind CSS"
                </p>
            </div>

            <ActionForm action=create>
                <div class="mb-6 flex gap-2">
                    <label class="sr-only" for="new-todo">"New todo"</label>
                    <input
                        id="new-todo"
                        name="title"
                        type="text"
                        required
                        maxlength="200"
                        autocomplete="off"
                        placeholder="What needs doing?"
                        class="min-w-0 flex-1 rounded-lg border border-slate-300 bg-white px-3 py-2.5 text-sm shadow-sm outline-none placeholder:text-slate-400 focus:border-slate-500 focus:ring-2 focus:ring-slate-200"
                    />
                    <button
                        type="submit"
                        class="rounded-lg bg-slate-900 px-4 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-slate-400"
                    >
                        "Add"
                    </button>
                </div>
            </ActionForm>

            <Transition fallback=move || view! {
                <p class="text-sm text-slate-500">"Loading todos…"</p>
            }>
                <ErrorBoundary fallback=|_| view! {
                    <p class="rounded-lg border border-rose-200 bg-rose-50 p-4 text-sm text-rose-700">
                        "Could not load todos. Check the server logs."
                    </p>
                }>
                    {todo_list}
                </ErrorBoundary>
            </Transition>
        </section>
    }
}
