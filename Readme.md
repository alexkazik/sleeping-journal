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
