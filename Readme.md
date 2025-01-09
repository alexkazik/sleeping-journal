# Unofficial Sleeping Gods Journal

Use the tool [here](https://alexkazik.github.io/sleeping-journal/).

## Translation

If you want to help with the translation of the website, update the `msg.lrc` file.
And then either create a pull request, an issue or contact me (see below).

For translations of the game (quests) please contact me (via [email](mailto:sleeping-journal+6437@tx0.eu)
or [BGG](https://boardgamegeek.com/geekmail/compose?touser=txnull)).

## Running it yourself

### Requirements

- https://rustup.rs/
- `rustup target add wasm32-unknown-unknown`
- https://trunkrs.dev/#install

### Running

Run this application with the trunk development server:

```bash
trunk serve --features=debug --open
```

### Building

```bash
trunk build
```

If the application will not be in the domain root (e.g. `https://example.com/sleeping-journal`):

```bash
trunk build --no-default-features --public-url /sleeping-journal
```

## Running and Building inside a Docker Container

### Requirements

You need Docker to build the image and git to clone the repository

 - https://docs.docker.com/get-started/get-docker/
 - https://git-scm.com/downloads

### Dockerfile

```bash
#Latest Rust image
FROM rust:latest AS builder
#Set working directory in the container
WORKDIR /usr/src/sleeping-journal

#copy the source repository files from local disk to the container
COPY /sleeping-journal .

#build the application
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build --release

#Small running image for the application
FROM debian:bookworm-slim
# Install a simple HTTP server
RUN apt-get update && apt-get install -y python3

#Set working directory in the container
WORKDIR /usr/src/sleeping-journal
COPY --from=builder /usr/src/sleeping-journal/dist ./

RUN chmod -R 755 /usr/src/sleeping-journal

CMD ["python3", "-m", "http.server", "8080"]
```

### Dockercompose file for running the container
```bash
services:
  sleeping-journal:
    image: sleeping-journal
    ports:
      - 54321:8080
```      