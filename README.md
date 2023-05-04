# A PDF tool & library written in Rust

[![github]](https://github.com/Tanja-4732/pdfdbg)&ensp;[![crates-io]](https://crates.io/crates/pdfdbg)&ensp;[![docs-rs]](https://docs.rs/pdfdbg/latest/pdfdbg)

This project is a work in progress. It is not yet ready for production, or even for testing.

This tool is meant to help with debugging PDF files.

Some goals are:

- Find the coordinates of text on a page
- Enumerate all rectangles on a page, and their coordinates
  - Filter by color, fill, stroke, etc.

Advanced goals:

- Create PDF forms
- Insert JavaScript into a PDF file

## License

[![GNU Lesser General Public License v3.0](https://www.gnu.org/graphics/lgplv3-with-text-154x68.png)](https://www.gnu.org/licenses/lgpl-3.0.html)

Copyright (C) 2023 [@Tanja-4732](https://github.com/Tanja-4732)

The pdfdbg project consists of (either now or in the future (planned)) the following components:

- pdfdbg (the CLI binary, licensed as AGPLv3+)
- pdfdbg (the library, licensed as LGPLv3+)
- A GUI application (planned, licensed as AGPLv3+)

The following text applies to all components, but only the library is licensed under the LGPLv3+;
the other components are licensed under the AGPLv3+ "GNU Affero General Public License", as seen [here](https://www.gnu.org/licenses/agpl-3.0.en.html). For the other components, the text remains similar, but the LGPLv3+ is replaced with the AGPLv3+.

pdfdbg is free software: you can redistribute it and/or modify it under the terms of the [GNU Lesser General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

pdfdbg is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Lesser General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Lesser General Public License](/LICENSE.md) along with pdfdbg. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/lgpl-3.0.html>.

Notice: This project is not affiliated with the any authoritative agencies regarding project in any way, shape, or form, and must not be considered "official" in any capacity.

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
