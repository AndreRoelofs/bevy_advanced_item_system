# bevy_advanced_item_system

The project's documentation website is managed by [mdBook](https://rust-lang.github.io/mdBook/) in [`website/`](website/). The pages are currently empty placeholders for explaining the project's purpose and usage.

## Documentation

Install mdBook with `cargo install mdbook`, then run these commands from the repository root:

```sh
# Preview locally with live reload
mdbook serve website --open

# Build the static website into website/book/
mdbook build website
```

Edit the Markdown pages in `website/src/` and update `website/src/SUMMARY.md` when adding pages.

## Special thanks

Special thanks to [@eugineerd](https://github.com/eugineerd) for their implementation of [mutually exclusive components](https://github.com/bevyengine/bevy/pull/22818). This project uses that work to keep the `OnGround`, `EquippedBy`, and `StoredIn` item-state components mutually exclusive.
