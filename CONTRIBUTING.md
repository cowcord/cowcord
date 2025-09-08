# Setting up

## Requirements

- [ ] [git](https://git-scm.com/downloads)
- [ ] Some non ancient rustc version

## Development

1. Clone the repo

```bash
git clone https://github.com/not-a-cowfr/cowcord.git
cd cowcord
```

2. Install Dioxus cli

```bash
cargo install dioxus-cli -F wasm-opt
```

3. Run

```bash
# web builds can be used for development on localhost but do not work when hosted on an actual domain.
dx serve --platform <web|desktop>
```

## Building

### Desktop

1. Run this

```bash
dx build --platform desktop
```

idk what else to do, ill fix this part later

# Consistency

<details><summary><h2>Endpoints</h2></summary>

### 1. Declaring Endpoints

If the endpoint has no changing string query fields or a part of the url is not always the same, then define it as a const, like this:

```rust
pub const SUPER_COOL: &str = "/super/cool";
```

However, with a lot of endpoints they have something that changes, like maybe a part of the url is a guild id, or it needs some string query parameters, in this case you would define it as a function, keeping the upper snake case, example:

```rust
pub fn SUPER_COOL(some_id: &SomeId, query: &SuperCoolQueryParams) -> String {
	format!("/super/{}/cool{}", some_id, query.to_string_query())
}
```

Also important, make sure to end the variable/struct/function/type name with what is format

```rust
pub const SUPER_COOL: &str = "/super/cool";

pub struct SuperCoolRequest {}

pub type SuperCoolResponse = SomeOtherThing;
```

Endpoint docs

These arent incredibly consistent since i've changed how I want them structured since starting writing them but in general they follow this format.
When in doubt you can always look at endpoints already defined.

As for grammar, I really couldnt care less if you make a mistake, mispell a word, forget a comma etc. I probably do it more than you ever will and it doesnt affect anyone.

```rust
/// meta info about the endpoint, like http method, supported headers and auth, required flags etc.
/// 
/// what the endpoint does
/// 
/// additional info like gotchas or examples
/// 
/// what the endpoint returns and/or what event(s) it fires
pub const SUPER_COOL: &str = "/super/cool";
```

Most of these will be already adhered to if copying the endpoint via the copy button from [discord userdoccers].

</details>

<details><summary><h2>Structs</h2></summary>

normal structs

```rust
#[derive(Serialize, Deserialize)]
pub struct MyCoolStruct {
    field_one: FieldOneType,
    field_one: FieldTwoType,
    field_two: FieldThreeFlags,
}
```

enums with integer as an identifier

```rust
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum FieldOneType {
    THIS_COOL_TYPE = 1,
    THIS_OTHER_COOL_TYPE = 2,
}
```

enums with the enum name as an identifier

```rust
#[derive(Serialize, Deserialize)]
pub enum FieldTwoType {
    this_thing,
    this_other_thing,
}
```

bitflags

```rust
bitflags! {
    pub struct FieldThreeFlags: u64 {
        const THIS_COOL_FLAG = 1 << 0;
        const THIS_OTHER_COOL_FLAG = 1 << 1;
    }
}
```

Again this will be done automatically for you if copying the code via the copy button on a table from [discord userdoccers].

</details>

<!-- # Troubleshooting -->

# AI

I don't mind if you use AI to help write simpler more repetitive parts, but I will not accept fully AI generated pull requests. Additionally, do not use AI at all when working with the `discord-types` crate, instead, all data there should be from your own research (make sure you document it or add your documentation to the [discord userdoccers github]) and/or [discord userdoccers].

[discord userdoccers]: https://docs.discord.food
[discord userdoccers github]: https://github.com/discord-userdoccers/discord-userdoccers