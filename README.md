# Taeko

## WARNING: Not even close to Alpha

API is unstable, probably undocumented, liable to change at any time, and leak implementation details.

## Core Tenants

1. Be fast.
   - Designed for quick development cycle, i.e. cache-first (using [**salsa**](https://crates.io/crates/salsa))
   - watching for file changes should re-run code iff input content changes
   - fetched CMS content should re-run code iff it differs from cached content
2. Get out of the way as much as possible.
   - The majority of a codebase using taeko should be made up of a user's code, not interactions with taeko APIs
   - Similar requirements should be applied to plugins where possible
3. Front-end agnostic: (use for native development?, dynamic frontend in addition to static)
4. Plugin forward, core should be as minimal as possible, facilitating lifecycle
   - Core modified when existing APIs are incapable of handling a use-case
5. Unopinionated.

## Development Brainstorming and Potentional Plans

- [ ] figure out where to include parallel processing
- [ ] Remove all unwraps
- [ ] Reduce allocations

## Structure

### Taeko Core

- [x] Core Database which keeps track of text/blobs
  - [ ] crate Error type

### Taeko Plugins

3 Categories:
1. Transformer: a trait/DB describing a translation from text/blob to another content type (example: `taeko-markdown-transformer`)
2. Source: a struct which executes some code retrieving data to be fed in to DBs (example: `taeko-fs-walker`)
3. Helper: A struct which is a 'recipe' for common actions, i.e. merging `taeko-fs-walker` and `taeko-markdown-transformer` in to a single object to use.

- [x] Make FS access in to a proper API
  - [x] Globbing/Walker-wrapper
  - [x] Markdown layer
    - [ ] Expose pulldown-cmark options
  - [x] Add JSON layer
  - [x] Add YAML layer
  - [x] Add TOML layer
- [ ] Remote access APIs
- [ ] UI-facing APIs
  - [ ] CSS/SCSS integration
  - [ ] cached Google Fonts
  - [ ] Internationalization?
  - [ ] Image manipulation
  - [ ] Adding Search
- [ ] Offline support?
- [ ] GUI options (not that you'd want to use the majority of these)
- [ ] Example using [Maud](https://crates.io/crates/maud)
- [ ] Example using [Iced](https://crates.io/crates/iced)
- [ ] Example using [Tera](https://crates.io/crates/tera)
- [ ] Example using [Yew](https://crates.io/crates/yew)
- [ ] Expand upon theory of separating templating in to separate creates that _can_ be made in to lamda Fns, and use for dynamically updating pages at DNS lookup

### Development Requirements

- [ ] hash-names of files
- [ ] watching for changes and updating web-preview (as additional plugin)?

## Inspirations

- [GatsbyJS](https://github.com/gatsbyjs/gatsby)
- [Zola](https://github.com/getzola/zola)
- [Publish](https://github.com/JohnSundell/Publish)
- [Arcs](https://github.com/Michael-F-Bryan/arcs)