# `zoop` i.e. "Town Racer"

## Structure

`web/tauri` - `tauri` i.e. launcher and interop for `web`  
`web` - `Next.js` UI app i.e. launcher for `engine`  
`engine` - `bevy` game app  
`server` - `actix` server for networking   


## Development

- Start `server` with `cargo run --bin zoop-server`
- Start `web` with `cargo tauri dev`
