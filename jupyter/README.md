### Jupyter notebook for Rust!

- This project uses [evcxr](https://github.com/google/evcxr), which is an evaluation context for Rust.

### How to set up.

- `docker-compose build` in this project root.
- `docker-compose up -d` to start container as a deamon.
- `docker-compose logs` to get url + token on which Jupyter is running. It usually like `http://127.0.0.1:8888/?token=xxxxxxx`

### Meta commands

- `:dep` import crates.
- `:vars` shows all variables.

### jupyter command

- `jupyter notebook list` to show currently running servers.
- stop server: `jupyter notebook stop 8888`
