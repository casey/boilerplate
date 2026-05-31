Changelog
=========

[1.1.1](https://github.com/casey/boilerplate/releases/tag/1.1.1) - 2025-12-10
-----------------------------------------------------------------------------

[1.1.0](https://github.com/casey/boilerplate/releases/tag/1.1.0) - 2025-12-02
-----------------------------------------------------------------------------

- Ensure template is recompiled when template file changes (#65)
- Allow suppressing or forcing generation of `IntoResponse` implementation (#64)
- Use `no_std` if the `reload` feature is not enabled (#63)
- Don't require `use boilerplate::Escape` when escaping is enabled (#62)
- Merge `html-escaper` into `boilerplate` (#61)
- Only insert empty text tokens when reloading is enabled (#60)
- Allow reloading with text block insertion and deletion (#59)
- Allow reloading templates from their original path (#57)
- Refine compatibility (#56)
- Add parser tests (#55)
- Document derive macro first in readme (#53)
- Style fixes (#52)
- Add hot reloading (#51)
- Add description to boilerplate-macros (#50)
- De-duplicate workspace metadata (#49)
- Move proc macros into `boilerplate-macros` (#48)
- Separate parsing and implementation (#47)
- Remove duplicate CONTRIBUTING file (#46)

[1.0.1](https://github.com/casey/boilerplate/releases/tag/1.0.1) - 2024-08-17
-----------------------------------------------------------------------------

- Make axum integration compatible with both axum 0.6 and 0.7 (#44)
- Add template nesting doctest example (#43)
- Fix readme typo (#41)
- Update dependencies (#40)

[1.0.0](https://github.com/casey/boilerplate/releases/tag/1.0.0) - 2023-07-04
-----------------------------------------------------------------------------

- Placate clippy (#38)

[0.2.5](https://github.com/casey/boilerplate/releases/tag/0.2.5) - 2023-01-01
-----------------------------------------------------------------------------

- Add function-like macro (#36)
- Improve readme (#34)
- Expand readme (#33)

[0.2.4](https://github.com/casey/boilerplate/releases/tag/0.2.4) - 2022-12-28
-----------------------------------------------------------------------------

- Remove escaping warning (#31)
- Terminate line blocks with EOF (#30)
- Add filename attribute (#29)

[0.2.3](https://github.com/casey/boilerplate/releases/tag/0.2.3) - 2022-12-12
-----------------------------------------------------------------------------

- Support context types with lifetimes and generics (#27)

[0.2.2](https://github.com/casey/boilerplate/releases/tag/0.2.2) - 2022-12-06
-----------------------------------------------------------------------------

- Split CI into multiple jobs (#25)
- Use `boilerplate_` prefix to avoid conflicts (#24)

[0.2.1](https://github.com/casey/boilerplate/releases/tag/0.2.1) - 2022-10-04
-----------------------------------------------------------------------------

- Add charset=utf-8 to text mime types (#21)

[0.2.0](https://github.com/casey/boilerplate/releases/tag/0.2.0) - 2022-09-18
-----------------------------------------------------------------------------

- Use alternative escaping approach (#18)

[0.1.0](https://github.com/casey/boilerplate/releases/tag/0.1.0) - 2022-09-17
-----------------------------------------------------------------------------

- Improve doc formatting (#15)
- Add HTML escaping (#11)
- Use Rust codeblock in readme (#14)
- Rename `Display` to `Boilerplate` (#13)
- Parse macro input with syn::parse_macro_input (#12)
- Add doctests (#9)
- Inline templates (#8)
- tests/lib.rs -> tests/integration.rs (#7)

[0.0.2](https://github.com/casey/boilerplate/releases/tag/0.0.2) - 2022-08-10
-----------------------------------------------------------------------------

- Avoid moving interpolations (#5)
- Add /tmp to .gitignore (#4)

[0.0.1](https://github.com/casey/boilerplate/releases/tag/0.0.1) - 2022-08-10
-----------------------------------------------------------------------------

- Initial implementation (#1)
- Initial commit
