# MPA - Marko Plain-Text Archive

The CLI utility "mpa" contained in this repository allows you to pack and unpack .mpa files.

MPA is an archive format, devised by Marko Vujnovic, like .zip or .rar, but it uses a plain-text file format which has the benefit of making it storable in a VCS and easily diff-able and readable. It's useful for storing an arbitrarily complex file hierarchy, consisting of textual files, in a single text file, such as, for instance, a whole Rust project in a single .mpa file. The project [rustscript](https://github.com/Marko-Vujnovic/marko_rustscript) (written by Marko Vujnovic), which makes it possible to use Rust as a scripting language, uses it in this manner.

#### If using the mpa executable:
```sh
mpa pack tests/ExampleProject produced
mpa unpack tests/ExampleProject.mpa produced
```

#### If using as a Rust library:
```rust
pack2("tests/ExampleProject", "produced");
unpack2("tests/ExampleProject.mpa", "produced");
```
