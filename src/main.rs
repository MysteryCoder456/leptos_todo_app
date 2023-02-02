use leptos::*;
use leptos_meta::*;

mod components;
mod models;

use components::*;
use models::{Todo, Todos};

#[component]
fn TodoApp(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let (todos, set_todos) = create_signal(cx, Todos::default());
    provide_context(cx, set_todos);

    // TODO: Fetch todos from some storage
    set_todos.update(|t| {
        t.push(Todo::new(cx, "Hello World", false));
        t.push(Todo::new(cx, "Learn Leptos", true));
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
                    view=move |todo| view! { cx, <TodoComponent todo=todo /> }
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
