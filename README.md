# Axum

Seems promising, especially as built of the Tokio/Tower stack.

## Things To Tick Off

In no particular order:

- [x] path params
- [x] response with JSON (with param based input) -- correct mime type
- [ ] peceive a struct and de-serialize it (POST body)
- [ ] use some form of app state in handler .. like a cache etc
- [ ] errors on de-serialize
- [ ] Option?? in Params and structs
- [ ] serve static files
- [ ] read / add headers
- [ ] cookies 
- [ ] TLS / HTTPS
- [ ] user based security 
- [ ] file upload

## JSON

Using the Struct "Json" as return value for the handler function.  This also sets MIME type.  But how does it have accept a Param as T ?? .. like a constructor ??

```rust
async fn hello_with_struct(Path(NameNumber{name, number}): Path<NameNumber>) -> Json<NameNumber >{
    Json(NameNumber{ name, number })
}
```

Seems to be the Struct definition, from Axum source code:

```rust
pub struct Json<T>(pub T);
```

Duh! - all explained in "The Book" -- known as a _tuple struct_.

# Learning

(Tokio, Tower and async)

Rust async functions de-sugar into futures.

## Serde

[https://serde.rs/](Serde book)

| What | How |
|--|--|
| Suppressing Optoin::None output | See Atttribues/Field atts in Serde book, and this sample code |

