use leptos::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
struct Todo {
    id: Uuid,
    content: RwSignal<String>,
    completed: RwSignal<bool>,
}

impl Todo {
    fn new(cx: Scope, content: &str, completed: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: create_rw_signal(cx, content.to_owned()),
            completed: create_rw_signal(cx, completed),
        }
    }
}

#[derive(Default, Clone)]
struct Todos(Vec<Todo>);

impl Todos {
    fn insert(&mut self, new_todo: Todo) {
        self.0.push(new_todo);
    }
}

#[component]
fn TodoApp(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, Todos::default());
    provide_context(cx, set_todos);

    // TODO: Fetch todos from some storage
    set_todos.update(|t| {
        t.insert(Todo::new(cx, "Hello World", false));
        t.insert(Todo::new(cx, "Learn Leptos", true));
    });

    view! { cx,
        <main class="w-3/5 mx-auto bg-slate-100 dark:bg-zinc-800 px-12 py-8 mt-8 rounded-lg drop-shadow-xl">
            <h1 class="text-3xl text-center">"Leptos Todos!"</h1>

            <div class="pt-3">
                <For
                    each=move || todos.get().0
                    key=|todo| todo.id
                    view=move |todo| view! { cx, <TodoComponent todo /> }
                />
            </div>
        </main>
    }
}

#[component]
fn TodoComponent(cx: Scope, todo: Todo) -> impl IntoView {
    let set_todos = use_context::<WriteSignal<Todos>>(cx).unwrap();

    let toggle_completed = move |_| {
        todo.completed.update(|c| *c = !(*c));

        // Move the todo to the end of the list
        if todo.completed.get_untracked() {
            set_todos.update(|set_todos| {
                let idx = set_todos.0.iter().position(|t| t.id == todo.id).unwrap();
                let todo = set_todos.0.remove(idx);
                set_todos.0.push(todo);
            });
        }
    };

    view! { cx,
        <div class="p-4 my-3 rounded-xl drop-shadow-md bg-neutral-700 flex">
            <p class="w-full text-lg" class:completed={move || todo.completed.get()}>
                {move || todo.content.get()}
            </p>
            <input type="checkbox" prop:checked={move || todo.completed.get()} on:input=toggle_completed class="scale-125 mr-2" />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|cx| view! { cx, <TodoApp /> });
}
