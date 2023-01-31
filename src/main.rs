use leptos::*;
use leptos_meta::*;

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
    fn add(&mut self, new_todo: Todo) {
        self.0.insert(0, new_todo);
    }

    fn push(&mut self, new_todo: Todo) {
        self.0.push(new_todo);
    }

    fn remove(&mut self, id: &Uuid) -> Option<Todo> {
        let idx = self.0.iter().position(|t| t.id == *id)?;
        Some(self.0.remove(idx))
    }
}

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

        <main class="w-3/5 mx-auto bg-slate-100 dark:bg-zinc-800 px-12 py-8 mt-8 rounded-lg drop-shadow-xl">
            <h1 class="text-3xl text-center">"Leptos Todos!"</h1>
            <AddTodoComponent />

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
fn AddTodoComponent(cx: Scope) -> impl IntoView {
    let (new_todo, set_new_todo) = create_signal(cx, String::new());

    let set_todos = use_context::<WriteSignal<Todos>>(cx).unwrap();
    let input_ref = NodeRef::<HtmlElement<Input>>::new(cx);

    let input_changed = move |_| {
        let node = input_ref.get().expect("Add Todo input element not loaded");
        set_new_todo.set(node.value());
    };

    let add_todo = move |_| {
        // Add the new todo to todo list
        let todo = Todo::new(cx, &new_todo.get_untracked(), false);
        set_todos.update(|t| t.add(todo));

        // Set input value to empty string
        set_new_todo.set(String::new());
    };

    view! { cx,
        <div class="mt-5 grid grid-cols-5 gap-4">
            <input
                _ref=input_ref
                prop:value={move || new_todo.get()}
                type="text"
                placeholder="New Todo"
                class="add-todo-input"
                on:input=input_changed
            />
            <button class="rounded-md bg-indigo-800 hover:drop-shadow-lg" on:click=add_todo>"Add"</button>
        </div>
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
                if let Some(t) = set_todos.remove(&todo.id) {
                    set_todos.push(t);
                }
            });
        }
    };

    view! { cx,
        <div class="todo" class:completed={move || todo.completed.get()}>
            <p class="w-full text-lg">
                {move || todo.content.get()}
            </p>
            <input
                type="checkbox"
                prop:checked={move || todo.completed.get()}
                on:input=toggle_completed
                class="scale-125 mr-2"
            />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(|cx| view! { cx, <TodoApp /> });
}
