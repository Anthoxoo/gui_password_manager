# Password Manager GUI

This password manager is very minimalistic one, it uses bcrypt to hash the master password an then base64 to encrypt the passwords. You can add an entry that contains an url as the "primary key", a username and a password associated.

## Stack

Rust is used as the backend and we got typescript with svelte for the frontend, to connect them we have [tauri](https://github.com/tauri-apps/tauri) that makes the bridge.

### Legacy

This password manager is the continuation of my older [CLI password manager](https://github.com/Anthoxoo/password_manager). Some backend functions may have changed because the handling of errors have to be different so it does not crash on some errors and the state management was a bit harsh.

#### Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

#### License

[MIT](https://choosealicense.com/licenses/mit/)
