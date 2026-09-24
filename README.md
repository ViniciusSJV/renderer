# Ray Tracer

The [complete PowerShell walkthrough](TESTME.md) covers source
generation through Librarian, lexical questions, graph verification, explicit
prompt preparation, Ollama server/model setup, API submission and response evaluation.
The [first real manual attempt](ai/consultas/manual-001/EVALUATION-OLLAMA-01.md)
has verified transport artifacts and a partially correct explanation requiring clarification.

For the renderer + Librarian evidence workflow, see [TESTME.md](TESTME.md).

Lexical questions and a source-backed structural graph are now available through
`catalog_sources search`, `graph`, and `verify-graph`. See the
[search walkthrough](ai/acervo/renderer-search-v1/README.md) and [WORKFLOW](WORKFLOW.md).
The algorithm lives in Librarian; renderer supplies vocabulary and reference questions.

A design of a Ray Tracer Render in Rust based on the book:
The ray tracer challenge. By Jamis Buck.

[The Ray Tracer Challenge](http://www.raytracerchallenge.com/)

- [x] Chapter 1 - Tuples, Points, and Vectors
- [x] Chapter 2 - Drawing on a Canvas
- [x] Chapter 3 - Matrices
- [x] Chapter 4 - Matrix Transformations
- [x] Chapter 5 - Ray-Sphere Intersections
- [x] Chapter 6 - Light and Shading
- [x] Chapter 7 - Making a Scene
- [x] Chapter 8 - Shadows
- [x] Chapter 9 - Planes
- [x] Chapter 10 - Patterns
- [x] Chapter 11 - Reflection and Refraction
- [x] Chapter 12 - Cubes
- [x] Chapter 13 - Cylinders
- [x] Chapter 14 - Groups
- [x] Chapter 15 - Triangles
- [ ] Chapter 16 - Constructive Solid Geometry (CSG)
- [ ] Chapter 17 - Next Steps

## Build and Librarian integration

The evidence tools use [Librarian](https://github.com/ViniciusSJV/librarian),
pinned to Git revision `a009af8276c4bb58c67905a10fea32f1cbbf8a38` in both
`Cargo.toml` and `Cargo.lock`. Cargo downloads the dependency; a separate local
checkout is required for the new local `librarian-ingest` development dependency:
keep the updated Librarian checkout next to this directory at `../librarian`.
The published v0.1.0 does not contain this new crate yet. The existing core and
Graph Engine dependencies remain pinned to Git. See the
[automatic source catalog](ai/acervo/renderer-auto-v1/README.md).

```sh
cargo fetch --locked
cargo test --locked
cargo test --locked --test librarian_integration
```

The integration test checks the external-scene dossier against its reference
query and rejects an altered source. It does not contact an LLM. The full suite
includes Unix-specific capture tools and local HTTP test servers; on native
Windows, select supported targets explicitly.

See the [evidence laboratory](ai/README.md) and the
[integration report](ai/experimentos/22-integracao-librarian/README.md).

# samples

![Chapter 10](https://github.com/ViniciusSJV/renderer/blob/master/cap10.png?raw=true)

![Chapter 11-1](https://github.com/ViniciusSJV/renderer/blob/master/cap11-reflection-4K.png?raw=true)

![Chapter 11-2](https://github.com/ViniciusSJV/renderer/blob/master/cap11-final-fresnel-effect.png?raw=true)

![Chapter 11-3](https://github.com/ViniciusSJV/renderer/blob/master/cap11-with-more-than-one-light-source.png?raw=true)

![Chapter 12](https://github.com/ViniciusSJV/renderer/blob/master/cap12.png?raw=true)

![Chapter 13](https://github.com/ViniciusSJV/renderer/blob/master/cap13.png?raw=true)

![Chapter 14](https://github.com/ViniciusSJV/renderer/blob/master/cap14.png?raw=true)

![Chapter 15](https://github.com/ViniciusSJV/renderer/blob/master/cap15-groups.png?raw=true)

![Chapter 15](https://github.com/ViniciusSJV/renderer/blob/master/cap15.png?raw=true)
