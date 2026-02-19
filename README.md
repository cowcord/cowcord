# Cowcord

## Contributing

Prequisites (if on NixOS, use `nix-shell`)
- rust
- dioxus cli
- tailwind cli

Set up tailwind
```bash
tailwindcss -i ./crates/app/input.css -o ./crates/app/assets/tailwind.css --watch
```

Run dev app with hotreloading
```bash
dx serve
```

## Todo

### Basic features
- [ ] Logging in
	- [x] with email/phone + password
	- [x] with qr code and mobile
	- [ ] with passkey
	- [ ] mfa support
	- [ ] reset password
- [ ] view servers
- [ ] view a server's channels
- [ ] view a channel and its messages
- [ ] view a server list
- [ ] send a message
