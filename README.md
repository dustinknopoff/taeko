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

### Taeko-Core

Named `taeko-core`

- [ ] Create a central trait `Context` which describes the process 
  - [ ] `Context` impls API for adding and running below plugins 
  - [ ] Add a lifecycle? (Pre-process, process, render, write)? 
  - [ ] To support dynamically rendered frontends (i.e. Yew), write is a default on `feature` 
  - [ ] A way of holding on to Salsa Databases 
  - [ ] A `build()` method which has a default implementation that gets all plugins, iterates, and calls in order of life-cycle
- [ ] Include `FS` trait and `WebRequest` trait? Or in plugins? `async` not going to work... 
  - [ ] Go broader?...salsa inputs for file, blob, json, yaml, toml...Forcing the struct implementation to handle async/deserializing

### Taeko-Plugins

In a sub-directory `taeko-plugins`, all prefixed with `taeko`

- [ ] Make FS access in to a proper API
  - [ ] Expose a Glob/WalkDir-wrapper
  - [ ] Make Markdown layer in to proper API
  - [ ] (expose pulldown-cmark options)
  - [ ] Add optional feature parallel
  - [ ] Add JSON layer
  - [ ] Add YAML layer
  - [ ] Add TOML layer
- [ ] Make Web Request API?
  - [ ] Interface with a CMS api?
- [ ] Image manipulation
- [ ] CSS/SCSS integration
- [ ] Adding Search
- [ ] Internationalization?
- [ ] cached Google Fonts
- [ ] Offline support
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