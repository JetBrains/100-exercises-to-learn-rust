The final exercise is open-ended: build an **asynchronous REST API** for the
ticket management system you've grown throughout the course.

Your API should let a client:

- **create** a ticket — `POST /tickets`
- **list** every ticket — `GET /tickets`
- **retrieve** a single ticket — `GET /tickets/{id}`
- **update** a ticket — `PATCH /tickets/{id}`

`Cargo.toml` is visible and yours to edit: add any crates you need from
[crates.io](https://crates.io). We suggest [`axum`](https://docs.rs/axum) — it's
an industry standard and is nicely supported by RustRover — together with
[`tokio`](https://docs.rs/tokio) and [`serde`](https://docs.rs/serde), but you're
free to pick a different stack.

> 💡 **Note**
>
> Many Rust crates are feature-gated: some parts compile only if you opt in through
> Cargo features. The recommended stack needs two of them.
>
> Add serde with its `derive` feature:
> `serde = { version = "1", features = ["derive"] }`.
> This enables `#[derive(Serialize, Deserialize)]`.
>
> Add tokio with its `full` feature:
> `tokio = { version = "1", features = ["full"] }`.
> This turns on the runtime, `#[tokio::main]`, networking, and `RwLock`/`Mutex`.
>
> Without these features you'll get "cannot find" errors.
> `full` is the simplest choice, but you can also list your own set of features for
> finer-grained control—each crate's docs.rs page documents what every feature
> enables.

Store the tickets in memory behind some shared state. That state is touched by
many requests at once, so it needs a lock: think about whether a
[`Mutex`](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html) or an
[`RwLock`](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html) is the
better fit for an API that reads far more often than it writes.

## What each endpoint should do

These are exactly the requests `requests.http` sends and the responses it expects,
so treat them as the spec for your handlers. A ticket is represented in JSON like
this:

```json
{
  "id": 0,
  "title": "Learn async Rust",
  "description": "Finish the last exercise",
  "status": "ToDo"
}
```

`status` is one of `"ToDo"`, `"InProgress"`, or `"Done"`.

### `POST /tickets` — create a ticket

Request body (no `id` or `status`—the server assigns those):

```json
{
  "title": "Learn async Rust",
  "description": "Finish the last exercise"
}
```

Assign a fresh `id`, default the `status` to `"ToDo"`, store the ticket, and
respond with `201 Created` and the newly created ticket as JSON.

### `GET /tickets` — list every ticket

No request body. Respond with `200 OK` and a JSON array in which **each element is
a complete ticket object** (`id`, `title`, `description`, and `status`—the same
shape shown above), not just the ids or titles:

```json
[
  {
    "id": 0,
    "title": "Learn async Rust",
    "description": "Finish the last exercise",
    "status": "ToDo"
  },
  {
    "id": 1,
    "title": "Ship the API",
    "description": "Wire up the remaining handlers",
    "status": "InProgress"
  }
]
```

Return an empty array `[]` when there are no tickets.

### `GET /tickets/{id}` — retrieve one ticket

No request body. Respond with `200 OK` and the ticket when `id` exists, or
`404 Not Found` when it doesn't.

### `PATCH /tickets/{id}` — update a ticket

Request body carrying only the fields to change—any of `title`, `description`,
`status`:

```json
{
  "status": "InProgress"
}
```

Apply the provided fields to the stored ticket and respond with `200 OK` and the
updated ticket, or `404 Not Found` when `id` doesn't exist.

## Testing it

There are no automated tests for this exercise—the design is yours, so you drive
the API by hand with real HTTP requests. The exercise ships with a
[`requests.http`](requests.http) file that you can use to test your solution. It is written
in RustRover's built-in [HTTP Client](https://www.jetbrains.com/help/rust/http-client-in-product-code-editor.html)
format.

> 💡 **Note**
>
> The requests in `requests.http` target `http://127.0.0.1:3000`, so they assume
> your server is listening on port `3000`. If you bind to a different address or
> port, update the URLs in `requests.http` to match.

**How to run it:**

1. Start the server first (green **▶** icon next to main)—the requests hit `http://127.0.0.1:3000`,
   so the app has to be listening.
2. Open `requests.http` in the editor. Each request has a green **▶** icon in the
   left gutter—click it to send that single request, or use **Run All Requests in
   File** to fire them top to bottom.
3. The response (status line, headers, body) opens in the **Services** tool window,
   along with a green/red result for each check.

**Run them in order.** The requests share state through a variable: the
`### Create a ticket` request saves the new ticket's `id` into `ticket_id`
(the `client.global.set("ticket_id", response.body.id)` line), and the later
"retrieve" and "patch" requests reuse it as `/tickets/{{ticket_id}}`. If you run
those before creating a ticket, `{{ticket_id}}` will be empty.

Each request also carries a small response handler (the `> {% ... %}` block) that
asserts the outcome, so you get a pass/fail without reading the JSON yourself. In
order, they:

1. **create** a ticket and expect `201 Created`;
2. **list** all tickets and expect `200 OK`;
3. **retrieve** the created ticket and expect `200` with `status == "ToDo"`;
4. **patch** its status and expect `200` with `status == "InProgress"`;
5. **request a missing ticket** (`/tickets/9999`) and expect `404 Not Found`.

## Hints

This exercise is open-ended, so if you're not sure where to begin, work through
the hints below **one at a time**—click a hint to expand it. Each one nudges you
toward a design close to the reference solution without writing it for you, so
try one, take it as far as you can, and only open the next when you're stuck.
Reveal the complete reference solution only once you've given the hints a genuine
attempt.

<div class="hint">

Mark `main` with `#[tokio::main]` so it runs as an asynchronous function on the
Tokio runtime. Inside, build an `axum::Router`, bind a
`tokio::net::TcpListener` to `127.0.0.1:3000`, and hand both to
`axum::serve(listener, app).await`.

</div>

<div class="hint">

Register one route per endpoint on the router. `axum::routing::{get, post}` attach
handlers to a path, and you can chain methods on the same path—e.g.
`.route("/tickets", post(create).get(list))`. Capture the id with a path
parameter: `.route("/tickets/{id}", get(get_one).patch(patch))`.

</div>

<div class="hint">

Every handler needs to reach the same in-memory store, and axum clones the state
for each request—so wrap it in an `Arc`. Because this API reads far more often
than it writes, a `tokio::sync::RwLock` is a better fit than a `Mutex`. Attach the
state with `.with_state(...)` and receive it in each handler through the `State`
extractor.

</div>

<div class="hint">

Model a `Ticket` (id, title, description, and a `Status` enum) and
`#[derive(Serialize, Deserialize)]` on it so axum's `Json` can convert to and from
JSON. Separate request structs for *create* and *patch* keep things tidy—and if
the *patch* fields are `Option<...>`, a client can update only the fields it
actually sends.

</div>

<div class="hint">

Each handler is an `async fn` whose parameters are just the extractors it needs
(`State<...>`, `Json<Payload>`, `Path<u64>`) and whose return type implements
`IntoResponse`. Return `StatusCode::CREATED` after inserting a new ticket, and
surface a missing id as `StatusCode::NOT_FOUND`—for example by returning
`Result<Json<Ticket>, StatusCode>`.

</div>
