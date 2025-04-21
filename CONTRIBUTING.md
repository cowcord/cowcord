# Setting up

## Requirements

- [ ] git
- [ ] Some non ancient rustc version

## Testing

1. Clone the repo `git clone https://github.com/not-a-cowfr/cowcord.git && cd cowcord`
2. Install Dioxus cli `cargo install dioxus-cli`
3. Run `dx serve --platform <web|desktop>`

## Building

cowcord currently has 3 main cargo profiles, dev, release and beta, release is optimized for binary size and beta is optimized for speed, at the expensive of a larger binary and more of a possibility to crash, and then of course dev is just for the fastest build times with performance out the window

### Web

1. Run thiss
```bash
dx build --release --profile <beta|release|dev> # no needs to specify platform, web is the default
cp -r ./target/dx/Cowcord/release/web/public ./dist
cp ./dist/index.html ./dist/404.html
```
(yes --release is necessary even when using beta profile)
<!--
2. Optimize wasm, run this
```bash
sudo apt-get update
sudo apt-get install -y binaryen
wasm-opt dist/assets/dioxus/Cowcord_bg.wasm -o dist/assets/dioxus/Cowcord_bg.wasm -O4 # O4 for speed OZ for binary size
```
-->
2. Done! everything should now be in the `/Dist` directory

### Desktop

1. Run this
```bash
dx build --release --profile <beta|release|dev> --platform desktop
```
idk what else to do, ill fix this part later

# Consistency

<details><summary><h2>Endpoints</h2></summary>

### 1. Declaring Endpoints

If the endpoint has no changing string query fields or a part of the url is not always the same, then define it as a const, like this:
```rust
pub const SUPER_COOL_ENDPOINT: &str = "/super/cool";
```

However, with a lot of endpoints they have something that changes, like maybe a part of the url is a guild id, or it needs some string query parameters, in this case you would define it as a function, keeping the upper snake case, example:
```rust
pub fn SUPER_COOL_ENDPOINT_ENDPOINT(some_id: Snowflake, query: QueryStringParamsStruct) -> String {
	format!("/super/{}/cool{}", some_id, to_string_query(query))
}
```

Also important, make sure to end the variable/struct/function/type name with what is format
```rust
/// notice the ENDPOINT at the end
pub const SUPER_COOL_ENDPOINT: &str = "/super/cool";

pub struct SuperCoolRequest {}

pub type SuperCoolResponse = SomeOtherThing;
```

And finally, make sure to include important info with the endpoint, for example:
```rust
/// Type: post
///
/// supports Super-Cool-Header header
///
/// requires SUPER_COOL permission
pub const SUPER_COOL_ENDPOINT: &str = "/super/cool";
```

<!-- ### 2. Keep request and response structs seperate, even if they're the same
This is because if in the future if one changes its very easy to edit them and it just -->

</details>

<details><summary><h2>Structs</h2></summary>

Discord loves using integers to represent certain types or flags for things, and it can get pretty confusing without looking at the docs, so just make sure to include what the int type is referencing
```rust
pub struct MyCoolStruct {
    /// link to documentation for whatever this is, if no link is there, its assumed that this is just a regular number
    field_one: u8,
}
```

And make sure to include an enum that repesents it
```rust
pub enum FieldOneType {
    THIS_COOL_TYPE = 1,
    THIS_OTHER_COOL_TYPE = 2,
}
```
```rust
pub enum FieldOneFlags {
    THIS_COOL_FLAG = 1 << 0,
    THIS_OTHER_COOL_FLAG = 1 << 1,
}
```
soon these will be used rather than just defining it as an int, and then the docs comment can be removed

</details>

<details><summary><h2>Components</h2></summary>

This ones pretty easy, all it is is that if theres some ui element thats used more than once, in more than one place, make it into a seperate component instead of something built into the page

</details>

# Troubleshooting
