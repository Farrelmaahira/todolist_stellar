#![cfg(test)]

use super::*;
use soroban_sdk::Env;

#[test]
fn test_create_todo() {
    let env = Env::default();
    let contract_id = env.register(TodoContract, ());
    let client = TodoContractClient::new(&env, &contract_id);

    let result = client.create_todo(
        &soroban_sdk::String::from_str(&env, "Buy groceries"),
        &soroban_sdk::String::from_str(&env, "Milk, eggs, bread"),
    );

    assert_eq!(result, soroban_sdk::String::from_str(&env, "Todo successfully created"));

    let todos = client.get_todos();
    assert_eq!(todos.len(), 1);
}

#[test]
fn test_complete_todo() {
    let env = Env::default();
    let contract_id = env.register(TodoContract, ());
    let client = TodoContractClient::new(&env, &contract_id);

    client.create_todo(
        &soroban_sdk::String::from_str(&env, "Study Soroban"),
        &soroban_sdk::String::from_str(&env, "Learn smart contract development"),
    );

    let todos = client.get_todos();
    let id = todos.get(0).unwrap().id;

    let result = client.complete_todo(&id);
    assert_eq!(result, soroban_sdk::String::from_str(&env, "Todo marked as completed"));

    let updated_todos = client.get_todos();
    assert!(matches!(updated_todos.get(0).unwrap().status, TodoStatus::Completed));
}

#[test]
fn test_update_todo() {
    let env = Env::default();
    let contract_id = env.register(TodoContract, ());
    let client = TodoContractClient::new(&env, &contract_id);

    client.create_todo(
        &soroban_sdk::String::from_str(&env, "Old Title"),
        &soroban_sdk::String::from_str(&env, "Old Description"),
    );

    let todos = client.get_todos();
    let id = todos.get(0).unwrap().id;

    let result = client.update_todo(
        &id,
        &soroban_sdk::String::from_str(&env, "New Title"),
        &soroban_sdk::String::from_str(&env, "New Description"),
    );

    assert_eq!(result, soroban_sdk::String::from_str(&env, "Todo successfully updated"));

    let updated_todos = client.get_todos();
    assert_eq!(
        updated_todos.get(0).unwrap().title,
        soroban_sdk::String::from_str(&env, "New Title")
    );
}

#[test]
fn test_delete_todo() {
    let env = Env::default();
    let contract_id = env.register(TodoContract, ());
    let client = TodoContractClient::new(&env, &contract_id);

    client.create_todo(
        &soroban_sdk::String::from_str(&env, "Temporary Task"),
        &soroban_sdk::String::from_str(&env, "This will be deleted"),
    );

    let todos = client.get_todos();
    let id = todos.get(0).unwrap().id;

    let result = client.delete_todo(&id);
    assert_eq!(result, soroban_sdk::String::from_str(&env, "Todo successfully deleted"));

    let updated_todos = client.get_todos();
    assert_eq!(updated_todos.len(), 0);
}

#[test]
fn test_clear_completed() {
    let env = Env::default();
    let contract_id = env.register(TodoContract, ());
    let client = TodoContractClient::new(&env, &contract_id);

    client.create_todo(
        &soroban_sdk::String::from_str(&env, "Task 1"),
        &soroban_sdk::String::from_str(&env, "Pending task"),
    );
    client.create_todo(
        &soroban_sdk::String::from_str(&env, "Task 2"),
        &soroban_sdk::String::from_str(&env, "Will be completed"),
    );

    let todos = client.get_todos();
    let id = todos.get(1).unwrap().id;
    client.complete_todo(&id);

    client.clear_completed();

    let remaining = client.get_todos();
    assert_eq!(remaining.len(), 1);
    assert_eq!(
        remaining.get(0).unwrap().title,
        soroban_sdk::String::from_str(&env, "Task 1")
    );
}