# Catppuccin for mdBook

Vendored from [catppuccin/mdBook](https://github.com/catppuccin/mdBook),
commit `75180d529d5e77d69f69cde442f34a4c58c7ee66` (theme version 4.0.0),
using the local checkout at `~/Projects/mdBook`.
See `LICENSE.catppuccin` for the upstream license.

- `catppuccin.css` is compiled from upstream `src/catppuccin.scss` with its
  locked dependencies. It includes all four flavors and syntax highlighting.
- `index.hbs` is the upstream example template for mdBook 0.5.4, with the theme
  button IDs prefixed by `mdbook-theme-` as required by that mdBook version.
- `../book.toml` selects Frappé as the default for both light and dark mode.
- `../mermaid-init.js` keeps diagrams in sync with light/dark theme changes.

The generated CSS is checked in, so normal documentation builds and deployment
only need mdBook and mdbook-mermaid, not Node.js or Sass.

To refresh the CSS from an updated upstream checkout, run from the project root:

```sh
pnpm --dir ~/Projects/mdBook install --frozen-lockfile
pnpm --dir ~/Projects/mdBook run build
cp ~/Projects/mdBook/dist/catppuccin.css website/theme/catppuccin.css
```

Update the source revision and license when refreshing. When upgrading mdBook,
compare `index.hbs` with the new default template and retain the Catppuccin theme
buttons (including Auto).

Validate with `just docs-check`, then use `just docs` to check the theme picker
and Mermaid diagrams in a browser.
