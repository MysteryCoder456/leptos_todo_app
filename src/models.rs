use leptos::{create_rw_signal, RwSignal, Scope};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub content: RwSignal<String>,
    pub completed: RwSignal<bool>,
}

impl Todo {
    pub fn new(cx: Scope, content: &str, completed: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: create_rw_signal(cx, content.to_owned()),
            completed: create_rw_signal(cx, completed),
        }
    }
}

#[derive(Default, Clone)]
pub struct Todos(Vec<Todo>);

impl Todos {
    pub fn get_todos(&self) -> Vec<Todo> {
        self.0.clone()
    }

    pub fn add(&mut self, new_todo: Todo) {
        self.0.insert(0, new_todo);
    }

    pub fn push(&mut self, new_todo: Todo) {
        self.0.push(new_todo);
    }

    pub fn remove(&mut self, id: &Uuid) -> Option<Todo> {
        let idx = self.0.iter().position(|t| t.id == *id)?;
        Some(self.0.remove(idx))
    }
}
