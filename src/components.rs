use leptos::*;

use crate::models::{Todo, Todos};

#[component]
pub fn AddTodoComponent(cx: Scope) -> impl IntoView {
    let (new_todo, set_new_todo) = create_signal(cx, String::new());

    let set_todos = use_context::<WriteSignal<Todos>>(cx).unwrap();
    let input_ref = NodeRef::<HtmlElement<Input>>::new(cx);

    let input_changed = move |_| {
        let node = input_ref.get().expect("Add Todo input element not loaded");
        set_new_todo.set(node.value());
    };

    let add_todo = move |_| {
        let content = new_todo.get_untracked();
        let content = content.trim();

        if !content.is_empty() {
            // Add the new todo to todo list
            let todo = Todo::new(cx, &content, false);
            set_todos.update(|t| t.add(todo));

            // Set input value to empty string
            set_new_todo.set(String::new());
        }
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
            <button class="add-todo-submit" on:click=add_todo>"Add"</button>
        </div>
    }
}

#[component]
pub fn TodoComponent(cx: Scope, todo: Todo) -> impl IntoView {
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
            <p class="w-full text-lg text-clip overflow-hidden">
                {move || todo.content.get()}
            </p>
            <input
                type="checkbox"
                prop:checked={move || todo.completed.get()}
                on:input=toggle_completed
                class="scale-125 ml-5 mr-2"
            />
        </div>
    }
}
