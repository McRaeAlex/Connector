# Notes

## Routing captures

Routing first then finish the implementations of the methods on Connection.

How do we path route captures to the functions? For example

```rust
route!(conn, Method::GET, "/user/:id", |conn: Connection| {
    conn.send_resp(StatusCode::OK, format!("{}", id));
})
```

## Possible solution

```rust
route!(conn, Method::GET, "/user/:id", |conn: Connection, id: usize, comment_id: usize| {
    conn.send_resp(StatusCode::OK, format!("{} {}", id, comment_id));
})
```

becomes 

```rust
if conn.match_method(Method::GET) {
    if let Some(vars) = conn.match_path("/user/:id/comments/:comment_id") { // returns a iterator of strings
        // we need to pollute the namespace s.t the vars become local variables
        let id = vars.next()?.into(); // convert it into the correct type
        let comment_id = vars.next()?.into(); // convert it into the correct type

        |conn: Connection, id: usize| {
            conn.send_resp(StatusCode::OK, format!("{} {}", id, comment_id));
        }(conn, id, comment_id);
        return;
    }
}
```

The issue with this is that although generating the inital part is easy and match
path is easy to generate. We have to have a macro which at compile time parses
the string and produces the `let` statements. It also has to pass them into the 
function correctly.

Another issue is typing, id and comment_id must be must be FromStr. If the 
conversion fails we simply do not match.

Rust doesn't seem to have compile time string parsing which is sorta a blocker.
It might be the case the we accept the path as a non string but some DSL.
ie. 

```rust
route!(conn, Method::GET, /user/:id, |conn: Connection, id: usize, comment_id: usize| {
    conn.send_resp(StatusCode::OK, format!("{} {}", id, comment_id));
});
```

Unfortunately I believe this forces the macro to be a proc macro.

Alternatively we could base the generation off of the function signature. This
should produce the same generated code and may be the better option.


## Resource handling

This is an issue for later/

I really like how phoenix does resources with the elixir modules. Im thinking of
a macro which does basically the same thing on a struct with asociated funcs or
maybe just allow them to pass in all the functions for it.