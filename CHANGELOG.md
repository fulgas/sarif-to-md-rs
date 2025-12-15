## [1.0.2](https://github.com/fulgas/sarif-to-md-rs/compare/v1.0.1...v1.0.2) (2025-12-15)

## [1.0.1](https://github.com/fulgas/sarif-to-md-rs/compare/v1.0.0...v1.0.1) (2025-12-10)

### Bug Fixes

* remove invalid rustdoc flags causing docs.rs build failures ([f6fbc0c](https://github.com/fulgas/sarif-to-md-rs/commit/f6fbc0cca161271af4b9a743170530a8abe530ff))

## 1.0.0 (2025-12-09)

### ⚠ BREAKING CHANGES

* Parser trait now uses generics instead of ParsedReport
enum. Factory methods require explicit type parameters.

Signed-off-by: Nelson Silva <2473927+fulgas@users.noreply.github.com>
* **cli:** Command structure has changed
- Old: snyk-to-md <container|code> [OPTIONS]
- New: snyk-to-md <json|sarif> [SUBCOMMAND] [OPTIONS]

- Organize by input format (JSON/SARIF) as primary subcommands
- Add report type (container/code) as argument for JSON subcommand
- Add helper methods to Cli struct for cleaner parameter extraction
- Simplify main.rs by using CLI helper methods
- Improve command discoverability and logical grouping

Signed-off-by: Nelson Silva <2473927+fulgas@users.noreply.github.com>
* Complete architectural overhaul of the parsing and generation system

- Add serde-sarif dependency for SARIF format support
- Introduce ParserFormat enum (Json, Sarif) to distinguish input formats
- Implement factory pattern for parser format selection
  - Create ParserFormatFactory to manage format-specific parsers
  - Create ParserTypeFactory interface for type-specific parser creation
- Introduce ParsedReport enum to unify different report types
  - Container variant wraps Box<SnykContainer>
  - Code variant holds String placeholder
- Restructure markdown generator architecture
  - Create MarkdownParserFormatFactory for format-aware generation
  - Implement MarkdownFormatFactory trait for format-specific generators
  - Separate JSON and SARIF generator implementations
- Move vulnerability parsing logic from parser to generator layer
  - Consolidate deduplication logic in CommonMarkGenerator
  - Remove intermediate SecurityReport model
- Update CLI to support input format selection
  - Add --input-format/-t flag for format specification
  - Rename --format to --output-format/-f for clarity
  - Update short flags for better usability (-i, -o, -e)
- Fix vulnerability template to render nested dependency paths
  - Update macros.md with format_path helper
  - Modify vulnerability.md to handle Vec<Vec<String>> paths
- Make severity field required in VulnerabilitiesItem schema
- Remove obsolete model module from core library
- Add SARIF format scaffolding (generators and parsers)

This refactoring enables multi-format input support and cleaner separation
of concerns between parsing, internal representation, and output generation.

Signed-off-by: Nelson Silva <2473927+fulgas@users.noreply.github.com>

### Features

* a new beginning ([f060233](https://github.com/fulgas/sarif-to-md-rs/commit/f060233ec219d318ca794a2813faf77b42ecf4fa))
* add comprehensive CI improvements with coverage reporting ([f73e060](https://github.com/fulgas/sarif-to-md-rs/commit/f73e060e1b7c42b0255a1d69a12a974a7d710acd))
* add comprehensive cross-platform build workflow ([ff8f51a](https://github.com/fulgas/sarif-to-md-rs/commit/ff8f51a6710127587e59a63c3d4a1d99db32b168))
* add comprehensive testing infrastructure, examples, and markdown linting ([4103b2b](https://github.com/fulgas/sarif-to-md-rs/commit/4103b2b42afaf02738cea7c19be79e209d96e1ae))
* add SARIF format support ([af3d534](https://github.com/fulgas/sarif-to-md-rs/commit/af3d534ee2d84f9c28f2a7dbf7ab02a7494b0640))
* configure semantic-release with draft and prerelease support ([15db554](https://github.com/fulgas/sarif-to-md-rs/commit/15db554200fa934b22d54dda570fd92a61685b84))
* **core:** implement builder pattern for markdown report generation ([37551e7](https://github.com/fulgas/sarif-to-md-rs/commit/37551e7d671c3cda299638b069393b25640238d4))
* **core:** refactor with modular templates and configurable emoji support ([fb7ecb3](https://github.com/fulgas/sarif-to-md-rs/commit/fb7ecb37a7530420cbe271cc471e51310a965a83))
* enable automated publishing for beta and stable releases ([d6e2ebd](https://github.com/fulgas/sarif-to-md-rs/commit/d6e2ebd762b71127db25cbab0a9432031afd72b9))
* enhance release workflow with GitHub App authentication ([9478721](https://github.com/fulgas/sarif-to-md-rs/commit/9478721c69aae98a9f2750b06c7c594feb4e0692))
* extract serde_snyk_container module for better code organization ([9d5e1f8](https://github.com/fulgas/sarif-to-md-rs/commit/9d5e1f8bb31a783b8a12d9edd633795b457c6ab0))
* implement cross-platform build workflow ([2fb61cc](https://github.com/fulgas/sarif-to-md-rs/commit/2fb61cc57d72fcf31e72bb4d6802ea44d7c2b7c0))
* implement matrix-based competitive benchmarking workflow ([a715e70](https://github.com/fulgas/sarif-to-md-rs/commit/a715e70448844ee75ac383164cb4aaaf15e3235a))
* implement Pandoc-based HTML generation and fix benchmark links ([05c4b67](https://github.com/fulgas/sarif-to-md-rs/commit/05c4b677063adc29be9a8e7f49d3dc38c04f7664))
* merge all workflows into release ([bfa1e16](https://github.com/fulgas/sarif-to-md-rs/commit/bfa1e16a6e780ac9fa0067fc4b40d1cc0bac65fb))
* modernize CI testing with cargo-nextest and cargo-llvm-cov ([ef23264](https://github.com/fulgas/sarif-to-md-rs/commit/ef23264c1fa8b1d3a06b2b1bdb1b239c2964c8de))
* rename snyk to sarif ([f689d8f](https://github.com/fulgas/sarif-to-md-rs/commit/f689d8fcd326d9f422a9f848cd9fb82a62ac749b))
* **sarif:** add comprehensive rule metadata display ([0a93bf7](https://github.com/fulgas/sarif-to-md-rs/commit/0a93bf7e2a09ff126f17a15de4bb1bfe435bd477))
* simplify benchmarks to focus on internal performance tracking ([8926486](https://github.com/fulgas/sarif-to-md-rs/commit/8926486b31c1c6a31b6acae33824fe45dafbe51c))
* simplify build workflow with direct GITHUB_REF_NAME usage ([4608d89](https://github.com/fulgas/sarif-to-md-rs/commit/4608d89723a382807759d57a3689df7111f06c58))
* streamline issue management and enhance security infrastructure ([e20a0aa](https://github.com/fulgas/sarif-to-md-rs/commit/e20a0aa69ab50db5edeceb19885522fb0397147d))

### Bug Fixes

* add support for beta channel ([6dd928b](https://github.com/fulgas/sarif-to-md-rs/commit/6dd928b82cb77690ead56c9af871531bfd57d5e5))
* correct shell script syntax error in benchmarks workflow ([ddfc1f5](https://github.com/fulgas/sarif-to-md-rs/commit/ddfc1f513e192f18d91578b446fb586e4811928a))
* improve competitive benchmarking reliability and GitHub Pages deployment ([02c5d67](https://github.com/fulgas/sarif-to-md-rs/commit/02c5d6771f0e6dd403ee2067f10e91e224c7e197))
* improve crates.io publishing workflow and dependency management ([7def4fd](https://github.com/fulgas/sarif-to-md-rs/commit/7def4fdf12e687c1e7c228daa5d38bee1e3ddff2))
* quote description strings in issue template checklists ([002fe3f](https://github.com/fulgas/sarif-to-md-rs/commit/002fe3f83f55a581f75193f9d20ab54167e55c20))
* remove invalid --update flag from cargo audit command ([d63b0e2](https://github.com/fulgas/sarif-to-md-rs/commit/d63b0e2a53f0707b716fe88a6c17c39e68e55f6e))
* rename benchmarks workflow ([ab498d2](https://github.com/fulgas/sarif-to-md-rs/commit/ab498d21a6c947b992ea3372229e8b60b11627ca))
* set correct branches ([37a388a](https://github.com/fulgas/sarif-to-md-rs/commit/37a388aacee4a5e4a2217979703e74beef3a79e1))
* set correct dependency for sarif-to-md-core ([28d49a4](https://github.com/fulgas/sarif-to-md-rs/commit/28d49a4a6a3de40bc9de6b603767a2a7cd066044))
* set pull requests permission ([9596b8a](https://github.com/fulgas/sarif-to-md-rs/commit/9596b8ac13111bf0a25c49d1dae49df4ad955379))
* use proper GitHub Actions conditionals for cross-compilation ([814f6b5](https://github.com/fulgas/sarif-to-md-rs/commit/814f6b5ba380a86b09ac9433e27585a1a60941e1))

### Documentation

* add comprehensive API documentation and configure docs.rs ([b0ea012](https://github.com/fulgas/sarif-to-md-rs/commit/b0ea01236287c4fd1c66d71d7391f59ee34310df))
* add comprehensive documentation and PR template ([517d629](https://github.com/fulgas/sarif-to-md-rs/commit/517d62905638b91af512792d8b589198e589fee1))

### Code Refactoring

* **build:** replace prettyplease with rustfmt-wrapper ([8021951](https://github.com/fulgas/sarif-to-md-rs/commit/8021951336c297059ae90ecb8d5b441f9f5a3449))
* **cli:** reorganize commands by input format ([dc7c2a5](https://github.com/fulgas/sarif-to-md-rs/commit/dc7c2a5679eb832d416c11cb955cbf3098db3619))
* **core:** improve clarity and performance ([d924a04](https://github.com/fulgas/sarif-to-md-rs/commit/d924a0426b2eee9ee961a78deefaab768f8b5733))
* **core:** replace string severity with typed enums ([bcd905a](https://github.com/fulgas/sarif-to-md-rs/commit/bcd905a6a0b4aebce61b7ffbeae04227910143aa))
* enhance test infrastructure and separate CI workflows ([ec2c9c6](https://github.com/fulgas/sarif-to-md-rs/commit/ec2c9c6d05c0ca00f7a121d73f7be8f627be5700))
* improve template macros and build process ([7551e9a](https://github.com/fulgas/sarif-to-md-rs/commit/7551e9ab61deb13541016441b818b9e00e5bbbf0))
* migrate from enums to generics for type safety ([7712f81](https://github.com/fulgas/sarif-to-md-rs/commit/7712f81c850a4738b6ea122272f6a03283cb60a4))
* remove redundant performance tracking and focus on Criterion reports ([47be068](https://github.com/fulgas/sarif-to-md-rs/commit/47be0688ef1ea3c1db2c164836b23e958656ce5f))
* remove unused CLI subcommands and simplify command structure ([529d947](https://github.com/fulgas/sarif-to-md-rs/commit/529d947516c7dbec3a7e6701c691b4bcf07a38cb))
* restructure parser and generator architecture with format abstraction ([69198a9](https://github.com/fulgas/sarif-to-md-rs/commit/69198a931ee151d8b353305b0d7134831bb186fe))
* **sarif:** unify generators using MarkdownFormat enum ([892a6ec](https://github.com/fulgas/sarif-to-md-rs/commit/892a6ec39755601fcef5addf67d20c013f633ec8))
* **sarif:** unify templates using format flag ([5f4dc04](https://github.com/fulgas/sarif-to-md-rs/commit/5f4dc04f1edfdb24eb3197dc76cb26600fb5c99c))
