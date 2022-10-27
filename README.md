<div align="center">
  <h1><code>Azure Functions 4.0 - Custom Handler</code></h1>
  <strong>An Azure Function (HttpTrigger) using<code> Rust</code> 
</div>

## Toolchain

- Actix-Web 4
- Rust 1.64.0
- Azure Function Core Tools (4.0.4785) Runtime (4.10.4.19213)

##  Building 🚧

```
cargo build --release && cp target/release/handler .
```

## Running 🏎️
```
func start
```

Navigate to 

- ```http://localhost:7071/api/hello```

- ```http://localhost:7071/api/{param}```





## License

Licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.


