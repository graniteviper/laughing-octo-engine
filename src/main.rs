#[macro_use]
extern crate rocket;

mod client;
use client::connect_to_db;
use rocket::serde::json::Json;
use tokio_postgres::Row;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct User {
    username: String,
}

#[derive(Deserialize)]
struct ChangeTodo {
    new_task: String,
    id: i32,
}

#[derive(Deserialize)]
struct NewTodo {
    task: String,
    user_id: String,
}

#[derive(Deserialize)]
struct DelTodo {
    id: i32,
}

#[derive(Deserialize)]
#[derive(Serialize)]
struct Todo {
    id: i32,
    task: String,
    user_id: String,
    is_completed: bool
}

#[derive(Deserialize)]
struct ChangeTodoState {
    id: i32,
    new_state: bool
}

// #[get("/create")]
// async fn create() -> String {
//     match connect_to_db().await {
//         Ok(client) => {
//             let query = "
//                 CREATE TABLE IF NOT EXISTS users (
//                     username TEXT PRIMARY KEY,
//                     data BYTEA
//                 );
//                 Create TABLE IF NOT EXISTS todos (
//                     id SERIAL PRIMARY KEY,
//                     task TEXT NOT NULL,
//                     is_completed BOOLEAN NOT NULL DEFAULT FALSE,
//                     user_id TEXT REFERENCES users(username) ON DELETE CASCADE
//                 );
//             ";
//             if let Err(e) = client.batch_execute(query).await {
//                 format!("Query error: {}", e)
//             } else {
//                 "Table created successfully ✅".to_string()
//             }
//         }
//         Err(e) => format!("Connection error: {}", e),
//     }
// }

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[post("/createuser", format = "json", data = "<user>")]
async fn createuser(user: Json<User>) -> String {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                INSERT INTO users (username) VALUES ($1)
            ";
            if let Err(e) = client.execute(query, &[&user.username]).await {
                format!("Query error: {}", e)
            } else {
                "User created".to_string()
            }
        }
        Err(e) => format!("Connection error: {}", e),
    }
}

#[delete("/deleteuser", format = "json", data = "<user>")]
async fn deleteuser(user: Json<User>) -> String {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                DELETE FROM users WHERE username = $1;
            ";
            if let Err(e) = client.execute(query, &[&user.username]).await {
                format!("Query error: {}", e)
            } else {
                "User Deleted ".to_string()
            }
        }
        Err(e) => format!("Connection error: {}", e),
    }
}

#[post("/createtodo", format = "json", data = "<todo>")]
async fn createtodo(todo: Json<NewTodo>) -> String {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                INSERT INTO todos (task, user_id) VALUES ($1,$2)
            ";
            if let Err(e) = client.execute(query, &[&todo.task, &todo.user_id]).await {
                format!("Query Error: {}", e)
            } else {
                "Todo Created".to_string()
            }
        }
        Err(e) => format!("Connection Error: {}", e),
    }
}

#[delete("/deletetodo", format = "json", data = "<todo>")]
async fn deletetodo(todo: Json<DelTodo>) -> String {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                DELETE FROM todos WHERE id = $1;
            ";
            if let Err(e) = client.execute(query, &[&todo.id]).await {
                format!("Query error: {}", e)
            } else {
                "Todo Deleted ".to_string()
            }
        }
        Err(e) => format!("Connection error: {}", e),
    }
}

#[get("/todos/<username>")]
async fn todos(username: &str) -> Json<Vec<Todo>> {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                SELECT * FROM todos WHERE user_id = $1;
            ";
            match client.query(query, &[&username]).await {
                Ok(rows) => {
                    let todos: Vec<Todo> = rows
                        .into_iter()
                        .map(|row: Row| Todo {
                            id: row.get("id"),
                            user_id: row.get("user_id"),
                            task: row.get("task"),
                            is_completed: row.get("is_completed"),
                        })
                        .collect();

                    return Json(todos);
                }
                Err(e) => {
                    eprintln!("Query Error: {}", e);
                    Json(vec![])
                }
            }
        }
        Err(e) => {
            eprintln!("Connection Error: {}", e);
            Json(vec![])
        }
    }
}

#[put("/changetodostate", format="json", data="<todo>")]
async fn changetodostate(todo: Json<ChangeTodoState>) -> String {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                UPDATE todos SET is_completed = $1 WHERE id = $2;
            ";
            if let Err(e) = client.execute(query, &[&todo.new_state, &todo.id]).await {
                format!("Query error: {}", e)
            } else {
                "Todo state changed ".to_string()
            }
        }
        Err(e) => {
            format!("Error: {}", e)
        }
    }
}

#[put("/changetodo", format="json", data="<todo>")]
async fn changetodo(todo: Json<ChangeTodo>) -> String {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                UPDATE todos SET task = $1 WHERE id = $2;
            ";
            if let Err(e) = client.execute(query, &[&todo.new_task, &todo.id]).await {
                format!("Query error: {}", e)
            } else {
                "Todo changed ".to_string()
            }
        }
        Err(e) => {
            format!("Error: {}", e)
        }
    }
}

#[get("/completed/<username>")]
async fn completed(username: &str) -> Json<Vec<Todo>> {
    match connect_to_db().await {
        Ok(client) => {
            let query = "
                SELECT * FROM todos WHERE user_id = $1 AND is_completed = TRUE;
            ";
            match client.query(query, &[&username]).await {
                Ok(rows) => {
                    let todos: Vec<Todo> = rows
                        .into_iter()
                        .map(|row: Row| Todo {
                            id: row.get("id"),
                            user_id: row.get("user_id"),
                            task: row.get("task"),
                            is_completed: row.get("is_completed"),
                        })
                        .collect();

                    return Json(todos);
                }
                Err(e) => {
                    eprintln!("Query Error: {}", e);
                    Json(vec![])
                }
            }
        }
        Err(e) => {
            eprintln!("Connection Error: {}", e);
            Json(vec![])
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![hello, createuser, deleteuser, createtodo, deletetodo, todos, changetodostate, changetodo, completed],
    )
}
