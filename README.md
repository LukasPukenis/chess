# Chess
Experiments on chess

This project is dedicated for experiments in chess to make a small engine.

# Architecture
Server-Client architecture is used where Rust code serves it's state as well as listen for commands via HTTP API.
HTML UI client is implemented to speak with the Rust backend and display the state.

Server also serves local static files so no extra server is necessary.

# Run
- `cargo run`
- open `http://localhost:3030/static/ui/index.html`
