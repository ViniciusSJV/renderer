# Ray Tracer

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

## Build

```sh
cargo test --locked
cargo run --locked --release --bin bench_render
```

## Librarian

This project is only a source for [Librarian](https://github.com/ViniciusSJV/librarian)
and has no Librarian dependency. It keeps the consumer profile, nothing else:

| File | Role |
| --- | --- |
| [playme.json](playme.json) | Profile passed to Librarian's PLAYME |
| [ai/acervo/renderer-sources.json](ai/acervo/renderer-sources.json) | Files that form the edition |
| [ai/acervo/renderer-lexicon.json](ai/acervo/renderer-lexicon.json) | Portuguese → code vocabulary |
| [ai/ollama/Modelfile](ai/ollama/Modelfile) | Local model configuration (`renderer-analyst`) |
| [ai/playme/](ai/playme) | Explanation instructions and prior criteria |

With the Librarian checkout next to this directory (`../librarian`):

```powershell
.\PLAYME.cmd -Question 'Onde a câmera calcula a direção dos raios?'
```

Runs are written to `ai/consultas/` (ignored by Git). Flow and artifacts are in
the [Librarian WORKFLOW](../librarian/WORKFLOW.md). The lessons, experiments and
legacy tools from when Librarian was built here are linked from its `HISTORICO.md`.

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
