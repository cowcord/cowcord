# Hex

This crate provides a custom `Hex` type which can be serialized and deserialized ina  specific format

## as number

```rust
#[derive(Deserialize, Serialize)]
struct MyStruct {
    #[serde(with = "hex::as_num")]
    foo: Hex
    #[serde(with = "hex::as_num")]
    bar: Option<Hex>
    #[serde(with = "hex::as_num")]
    buzz: Vec<Hex>
}

let data = MyStruct {
    foo: "123abc".into(),
    bar: Some(16777215.into()),
    buzz: vec![vec![1, 10, 100].into()],
};

let json = serde_json::to_string(&thing).unwrap();
```

which serializes to

```json
{
    "foo": 1194684,
    "bar": 16777215,
    "buzz": [68196]
}
```

## as string

```rust
#[derive(Deserialize, Serialize)]
struct MyStruct {
    #[serde(with = "hex::as_str")]
    foo: Hex
    #[serde(with = "hex::as_str")]
    bar: Option<Hex>
    #[serde(with = "hex::as_str")]
    buzz: Vec<Hex>
}

let data = MyStruct {
    foo: "123abc".into(),
    bar: Some(16777215.into()),
    buzz: vec![vec![1, 10, 100].into()],
};

let json = serde_json::to_string(&thing).unwrap();
```

which serializes to

```json
{
    "foo": "123abc",
    "bar": "ffffff",
    "buzz": ["010a64"]
}
```