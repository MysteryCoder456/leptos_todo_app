use leptos::*;
use leptos_meta::*;

mod components;
mod models;

use components::*;
use models::*;

#[component]
fn TodoApp(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let (todos, set_todos) = create_signal(cx, Todos::default());
    provide_context(cx, set_todos);

    // Fetch todos from local storage
    set_todos.update(|t| {
        if let Ok(Some(storage)) = window().local_storage() {
            if let Ok(Some(items_str)) = storage.get_item("TODOS") {
                if let Ok(items) = ron::from_str::<Vec<TodoSerialized>>(&items_str) {
                    for item in items {
                        t.push(Todo {
                            id: uuid::Uuid::parse_str(&item.id).unwrap(),
                            content: create_rw_signal(cx, item.content),
                            completed: create_rw_signal(cx, item.completed),
                        });
                    }
                }
            }
        }
    });

    // Listen for changes to Todos and update in local storage
    create_effect(cx, move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            let items = todos
                .get()
                .get_todos()
                .iter()
                .map(|t| TodoSerialized {
                    id: t.id.to_string(),
                    content: t.content.get(),
                    completed: t.completed.get(),
                })
                .collect::<Vec<_>>();

            let items_serialized = ron::to_string(&items).unwrap();
            storage.set_item("TODOS", &items_serialized).unwrap();
        }
    });

    view! { cx,
        <Title text="Leptos Todos" />

        <main>
            <h1 class="text-3xl text-center">"Leptos Todos!"</h1>
            <AddTodoComponent />

            <div class="pt-3">
                <For
                    each=move || todos.get().get_todos()
                    key=|todo| todo.id
                    view=move |todo| view! { cx, <TodoComponent todo /> }
                />
            </div>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|cx| view! { cx, <TodoApp /> });
}
