[![npm](https://img.shields.io/npm/v/dag-rs.svg)](https://npmjs.com/package/dag-rs) 
[![](https://img.shields.io/npm/dt/dag-rs?style=flat&label=downloads&color=3b9648&labelColor=484848&logo=npm)](https://www.npmjs.com/package/dag-rs)

# Demo

[![Watch the video](https://vumbnail.com/1169357006.jpg)](https://vimeo.com/1169357006?share=copy)

# dag-rs

`dag-rs` (Directed Acyclic Graph) parses workspace and constructs a Directed Acyclic Graph (DAG) of local package dependencies within a monorepo or similar setups. Each edge in the graph represents a `dependent → dependency` relationship.


## Installation

```
npm install dag-rs
# or
yarn add dag-rs
```

> [!NOTE]
> Right now  `getAffectedPkg` doesnt support circular dependency it might log `[1]    867885 segmentation fault (core dumped)  node usage.cjs`

## Usage

### ESM
Get the full dependency graph:

```js
// dag-rs.mjs
import { dag } from 'dag-rs';

for (const edge of dag()) {
  console.log(edge);
}
```

Example structure:
```
packages/
  core/
  ui/        (depends on core)
  app/       (depends on ui)
  web/       (depends on ui)
```

Output:

```js
// dependent : dependency

{ 'ui': 'core' }
{ 'app': 'ui' }
{ 'web': 'ui' }
```


This means:

- `ui` depends on `core`
- `app` depends on `ui`
- `web` depends on `ui`

Find all packages affected by a specific package:

```js
import { getAffectedPkg } from 'dag-rs';

const pkg = "core";

for (const affected of getAffectedPkg(pkg)) {
  console.log(affected);
}
```

Output:

```
ui
app
web
```

This means:

- `ui` directly depends on `core`
- `app` depends on `ui` → indirectly affected by `core`
- `web` depends on `ui` → indirectly affected by `core`




## Real-world Example: Rocket.Chat Fuselage

Repository: https://github.com/RocketChat/fuselage

### getAffectedPkg("@rocket.chat/fuselage") → returns all the direct , transitive dependencies
```
// dependents of @rocket.chat/fuselage package

@rocket.chat/layout
@rocket.chat/fuselage-toastbar
@rocket.chat/fuselage-forms
@rocket.chat/onboarding-ui

```

## Example Output:
### dag() → returns all packages in the project

```
// dependent : dependency

{ '@rocket.chat/fuselage-monorepo': '@rocket.chat/prettier-config' }
{ '@rocket.chat/fuselage-monorepo': 'update-readme' }
{ '@rocket.chat/css-in-js': '@rocket.chat/css-supports' }
{ '@rocket.chat/css-in-js': '@rocket.chat/memo' }
{
  '@rocket.chat/css-in-js': '@rocket.chat/stylis-logical-props-middleware'
}
{ '@rocket.chat/css-in-js': 'lint-all' }
{ '@rocket.chat/css-supports': '@rocket.chat/memo' }
{ '@rocket.chat/css-supports': 'lint-all' }
{ '@rocket.chat/emitter': 'lint-all' }
{ '@rocket.chat/fuselage': '@rocket.chat/css-in-js' }
{ '@rocket.chat/fuselage': '@rocket.chat/css-supports' }
{ '@rocket.chat/fuselage': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/fuselage': '@rocket.chat/memo' }
{ '@rocket.chat/fuselage': '@rocket.chat/styled' }
{ '@rocket.chat/fuselage': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/fuselage': '@rocket.chat/icons' }
{ '@rocket.chat/fuselage': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/fuselage': '@rocket.chat/icons' }
{ '@rocket.chat/fuselage': '@rocket.chat/storybook-dark-mode' }
{ '@rocket.chat/fuselage': 'lint-all' }
{ '@rocket.chat/fuselage': 'testing-utils' }
{ '@rocket.chat/fuselage-forms': '@rocket.chat/emitter' }
{ '@rocket.chat/fuselage-forms': '@rocket.chat/fuselage' }
{ '@rocket.chat/fuselage-forms': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/fuselage-forms': '@rocket.chat/fuselage' }
{ '@rocket.chat/fuselage-forms': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/fuselage-forms': '@rocket.chat/storybook-dark-mode' }
{ '@rocket.chat/fuselage-forms': 'lint-all' }
{ '@rocket.chat/fuselage-hooks': '@rocket.chat/emitter' }
{ '@rocket.chat/fuselage-hooks': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/fuselage-hooks': '@rocket.chat/emitter' }
{ '@rocket.chat/fuselage-hooks': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/fuselage-hooks': 'lint-all' }
{ '@rocket.chat/fuselage-hooks': 'testing-utils' }
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/fuselage' }
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/styled' }
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/fuselage' }
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/layout' }
{
  '@rocket.chat/fuselage-toastbar': '@rocket.chat/storybook-dark-mode'
}
{ '@rocket.chat/fuselage-toastbar': '@rocket.chat/styled' }
{ '@rocket.chat/fuselage-toastbar': 'lint-all' }
{ '@rocket.chat/fuselage-tokens': 'build-design-tokens' }
{ '@rocket.chat/fuselage-tokens': 'lint-all' }
{ '@rocket.chat/icons': 'build-icons' }
{ '@rocket.chat/icons': 'lint-all' }
{ '@rocket.chat/layout': '@rocket.chat/fuselage' }
{ '@rocket.chat/layout': '@rocket.chat/fuselage' }
{ '@rocket.chat/layout': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/layout': '@rocket.chat/storybook-dark-mode' }
{ '@rocket.chat/layout': 'lint-all' }
{ '@rocket.chat/logo': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/logo': '@rocket.chat/styled' }
{ '@rocket.chat/logo': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/logo': 'build-logo' }
{ '@rocket.chat/logo': 'lint-all' }
{ '@rocket.chat/memo': 'lint-all' }
{ '@rocket.chat/mp3-encoder': 'lint-all' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/fuselage' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/icons' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/layout' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/logo' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/styled' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/fuselage' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/fuselage-hooks' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/fuselage-tokens' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/icons' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/layout' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/logo' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/storybook-dark-mode' }
{ '@rocket.chat/onboarding-ui': '@rocket.chat/styled' }
{ '@rocket.chat/onboarding-ui': 'lint-all' }
{ '@rocket.chat/prettier-config': 'lint-all' }
{ '@rocket.chat/storybook-dark-mode': 'lint-all' }
{ '@rocket.chat/string-helpers': 'lint-all' }
{ '@rocket.chat/styled': '@rocket.chat/css-in-js' }
{ '@rocket.chat/styled': 'lint-all' }
{
  '@rocket.chat/stylis-logical-props-middleware': '@rocket.chat/css-supports'
}
{ '@rocket.chat/stylis-logical-props-middleware': 'lint-all' }
{ 'build-design-tokens': 'tools-utils' }
{ 'build-design-tokens': 'lint-all' }
{ 'build-icons': 'tools-utils' }
{ 'build-icons': 'lint-all' }
{ 'build-logo': 'lint-all' }
{ 'build-logo': 'tools-utils' }
{ 'testing-utils': 'lint-all' }
{ 'tools-utils': 'lint-all' }
{ 'update-readme': 'lint-all' }
```

## Building dag-rs

Building dag requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

To run the build, run:

```sh
$ yarn install
$ yarn build
```

## Project Layout

| Entry          | Purpose                                                                                                                                  |
|----------------|------------------------------------------------------------------------------------------------------------------------------------------|
| `Cargo.toml`   | The Cargo [manifest file](https://doc.rust-lang.org/cargo/reference/manifest.html), which informs the `cargo` command.                   |
| `README.md`    | This file.                                                                                                                               |
| `src/`         | The directory tree containing the Rust source code for the project.                                                                      |
| `lib.rs`       | Entry point for the Rust source code.                                                                                                          |
| `index.node`   | The main module, a [Node addon](https://nodejs.org/api/addons.html) generated by the build and pointed to by `"main"` in `package.json`. |
| `package.json` | The npm [manifest file](https://docs.npmjs.com/cli/v7/configuring-npm/package-json), which informs the `npm` command.                    |
| `target/`      | Binary artifacts generated by the Rust build.                                                                                            |

## Contribution
All forms of contribution are welcome!
