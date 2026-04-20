#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Enum untuk status todo
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum TodoStatus {
    Pending,
    Completed,
}

// Struktur data yang akan menyimpan todo
#[contracttype]
#[derive(Clone, Debug)]
pub struct Todo {
    id: u64,
    title: String,
    description: String,
    status: TodoStatus,
}

// Storage key untuk data todos
const TODO_DATA: Symbol = symbol_short!("TODO_DATA");

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {
    // Fungsi untuk mengambil semua todo
    pub fn get_todos(env: Env) -> Vec<Todo> {
        return env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat todo baru
    pub fn create_todo(env: Env, title: String, description: String) -> String {
        // 1. Ambil data todos dari storage
        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Buat object todo baru dengan status Pending
        let todo = Todo {
            id: env.prng().gen::<u64>(),
            title,
            description,
            status: TodoStatus::Pending,
        };

        // 3. Tambahkan todo baru ke list todos
        todos.push_back(todo);

        // 4. Simpan todos ke storage
        env.storage().instance().set(&TODO_DATA, &todos);

        return String::from_str(&env, "Todo successfully created");
    }

    // Fungsi untuk menandai todo sebagai selesai (completed)
    pub fn complete_todo(env: Env, id: u64) -> String {
        // 1. Ambil data todos dari storage
        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari todo berdasarkan id, lalu ubah statusnya menjadi Completed
        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();
            if todo.id == id {
                todo.status = TodoStatus::Completed;
                todos.set(i, todo);
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Todo marked as completed");
            }
        }

        return String::from_str(&env, "Todo not found");
    }

    // Fungsi untuk mengupdate judul dan deskripsi todo
    pub fn update_todo(env: Env, id: u64, title: String, description: String) -> String {
        // 1. Ambil data todos dari storage
        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari todo berdasarkan id, lalu update title dan description-nya
        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();
            if todo.id == id {
                todo.title = title;
                todo.description = description;
                todos.set(i, todo);
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Todo successfully updated");
            }
        }

        return String::from_str(&env, "Todo not found");
    }

    // Fungsi untuk menghapus todo berdasarkan id
    pub fn delete_todo(env: Env, id: u64) -> String {
        // 1. Ambil data todos dari storage
        let mut todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Cari index todo yang akan dihapus menggunakan perulangan
        for i in 0..todos.len() {
            if todos.get(i).unwrap().id == id {
                todos.remove(i);
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Todo successfully deleted");
            }
        }

        return String::from_str(&env, "Todo not found");
    }

    // Fungsi untuk menghapus semua todo yang sudah selesai
    pub fn clear_completed(env: Env) -> String {
        // 1. Ambil data todos dari storage
        let todos: Vec<Todo> = env
            .storage()
            .instance()
            .get(&TODO_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. Filter hanya todo yang masih Pending
        let mut pending_todos: Vec<Todo> = Vec::new(&env);
        for i in 0..todos.len() {
            let todo = todos.get(i).unwrap();
            if todo.status == TodoStatus::Pending {
                pending_todos.push_back(todo);
            }
        }

        // 3. Simpan kembali hanya todos yang pending
        env.storage().instance().set(&TODO_DATA, &pending_todos);

        return String::from_str(&env, "Completed todos cleared");
    }
}

mod test;