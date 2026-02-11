## Contributing

Prequisites (if on NixOS, use `nix-shell`)
- rust
- dioxus-cli
- tailwind cli

Set up tailwind
```bash
tailwindcss -i ./crates/app/input.css -o ./crates/app/assets/tailwind.css --watch
```

Run dev app with hotreloading
```bash
dx serve
```
