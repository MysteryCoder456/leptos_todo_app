use leptos::*;
use uuid::Uuid;

#[derive(Clone)]
struct Todo {
    content: String,
    completed: bool,
}

impl Todo {
    fn new(content: &str, completed: bool) -> Self {
        Self {
            content: content.to_owned(),
            completed,
        }
    }
}

#[derive(Default, Clone)]
struct Todos(Vec<(Uuid, Todo)>);

impl Todos {
    fn insert(&mut self, new_todo: Todo) {
        self.0.push((Uuid::new_v4(), new_todo));
    }
}

#[component]
fn TodoApp(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, Todos::default());

    // TODO: Fetch todos from some storage
    set_todos.update(|t| {
        t.insert(Todo::new("Hello World", false));
        t.insert(Todo::new("Learn Leptos", true));
    });

    let todos_iter = move || todos.get().0;

    view! { cx,
        <main class="w-3/5 mx-auto bg-slate-100 dark:bg-zinc-800 px-12 py-8 mt-8 rounded-lg drop-shadow-xl">
            <h1 class="text-3xl text-center">"Leptos Todos!"</h1>

            <div class="pt-3">
                <For
                    each=todos_iter
                    key=|(id, _todo)| id.clone()
                    view=move |(_id, todo)| view! { cx, <TodoComponent todo /> }
                />
            </div>
        </main>
    }
}

#[component]
fn TodoComponent(cx: Scope, todo: Todo) -> impl IntoView {
    view! { cx,
        <div class="p-4 my-3 rounded-xl drop-shadow-md bg-neutral-700 flex">
            <p class="w-full text-lg">
                {
                    if todo.completed {
                        view! { cx, <span><s>{todo.content}</s></span> }
                    } else {
                        view! { cx, <span>{todo.content}</span> }
                    }
                }
            </p>
            <input type="checkbox" checked={todo.completed} class="scale-125 mr-2" />
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <TodoApp /> });
}
