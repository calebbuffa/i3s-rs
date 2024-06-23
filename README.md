# I3S Rust

This project is a minimal API for *reading* Esri's I3S file format.

## To Do

- [x] Integrated Mesh API
- [x] Point Cloud API
- [x] Building API
- [x] Point API
- [x] 3D Object API
- [x] Read from `.slpk` file
- [x] Stream from REST
- [ ] Do not return `Box<dyn Error>`
- [ ] `open(&str) -> (Formats, SceneLayers)`
- [ ] Create common structs, traits, etc. to minimize repeated code between secne layer types
- [ ] Get node geometries
- [ ] Get node textures/materials/colors
- [ ] Get node attributes
- [ ] Get node features
- [ ] Add dependency features
- [ ] Read Hash Table

## Examples

```bash
cargo run --example <example-name>
```

## [I3S Specification](https://github.com/Esri/i3s-spec)

This library is an implementation of the offical i3s-spec, which is licensed under the Creative Commons Attribution-NoDerivatives 4.0 International Public License. No modifications have been made to the specification. For questions about I3S, visit the official GitHub repository.
