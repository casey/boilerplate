Changelog
=========

[1.1.1](https://github.com/casey/boilerplate/releases/tag/1.1.1) - 2025-12-10
-----------------------------------------------------------------------------

[1.1.0](https://github.com/casey/boilerplate/releases/tag/1.1.0) - 2025-12-02
-----------------------------------------------------------------------------

- Ensure template is recompiled when template file changes ([#65](https://github.com/casey/filepack/pull/65) by [casey](https://github.com/casey))
- Allow suppressing or forcing generation of `IntoResponse` implementation ([#64](https://github.com/casey/filepack/pull/64) by [casey](https://github.com/casey))
- Use `no_std` if the `reload` feature is not enabled ([#63](https://github.com/casey/filepack/pull/63) by [casey](https://github.com/casey))
- Don't require `use boilerplate::Escape` when escaping is enabled ([#62](https://github.com/casey/filepack/pull/62) by [casey](https://github.com/casey))
- Merge `html-escaper` into `boilerplate` ([#61](https://github.com/casey/filepack/pull/61) by [casey](https://github.com/casey))
- Only insert empty text tokens when reloading is enabled ([#60](https://github.com/casey/filepack/pull/60) by [casey](https://github.com/casey))
- Allow reloading with text block insertion and deletion ([#59](https://github.com/casey/filepack/pull/59) by [casey](https://github.com/casey))
- Allow reloading templates from their original path ([#57](https://github.com/casey/filepack/pull/57) by [casey](https://github.com/casey))
- Refine compatibility ([#56](https://github.com/casey/filepack/pull/56) by [casey](https://github.com/casey))
- Add parser tests ([#55](https://github.com/casey/filepack/pull/55) by [casey](https://github.com/casey))
- Document derive macro first in readme ([#53](https://github.com/casey/filepack/pull/53) by [casey](https://github.com/casey))
- Style fixes ([#52](https://github.com/casey/filepack/pull/52) by [casey](https://github.com/casey))
- Add hot reloading ([#51](https://github.com/casey/filepack/pull/51) by [casey](https://github.com/casey))
- Add description to boilerplate-macros ([#50](https://github.com/casey/filepack/pull/50) by [casey](https://github.com/casey))
- De-duplicate workspace metadata ([#49](https://github.com/casey/filepack/pull/49) by [casey](https://github.com/casey))
- Move proc macros into `boilerplate-macros` ([#48](https://github.com/casey/filepack/pull/48) by [casey](https://github.com/casey))
- Separate parsing and implementation ([#47](https://github.com/casey/filepack/pull/47) by [casey](https://github.com/casey))
- Remove duplicate CONTRIBUTING file ([#46](https://github.com/casey/filepack/pull/46) by [casey](https://github.com/casey))

[1.0.1](https://github.com/casey/boilerplate/releases/tag/1.0.1) - 2024-08-17
-----------------------------------------------------------------------------

- Make axum integration compatible with both axum 0.6 and 0.7 ([#44](https://github.com/casey/filepack/pull/44) by [casey](https://github.com/casey))
- Add template nesting doctest example ([#43](https://github.com/casey/filepack/pull/43) by [casey](https://github.com/casey))
- Fix readme typo ([#41](https://github.com/casey/filepack/pull/41) by [casey](https://github.com/casey))
- Update dependencies ([#40](https://github.com/casey/filepack/pull/40) by [casey](https://github.com/casey))

[1.0.0](https://github.com/casey/boilerplate/releases/tag/1.0.0) - 2023-07-04
-----------------------------------------------------------------------------

- Placate clippy ([#38](https://github.com/casey/filepack/pull/38) by [casey](https://github.com/casey))

[0.2.5](https://github.com/casey/boilerplate/releases/tag/0.2.5) - 2023-01-01
-----------------------------------------------------------------------------

- Add function-like macro ([#36](https://github.com/casey/filepack/pull/36) by [casey](https://github.com/casey))
- Improve readme ([#34](https://github.com/casey/filepack/pull/34) by [casey](https://github.com/casey))
- Expand readme ([#33](https://github.com/casey/filepack/pull/33) by [casey](https://github.com/casey))

[0.2.4](https://github.com/casey/boilerplate/releases/tag/0.2.4) - 2022-12-28
-----------------------------------------------------------------------------

- Remove escaping warning ([#31](https://github.com/casey/filepack/pull/31) by [casey](https://github.com/casey))
- Terminate line blocks with EOF ([#30](https://github.com/casey/filepack/pull/30) by [casey](https://github.com/casey))
- Add filename attribute ([#29](https://github.com/casey/filepack/pull/29) by [casey](https://github.com/casey))

[0.2.3](https://github.com/casey/boilerplate/releases/tag/0.2.3) - 2022-12-12
-----------------------------------------------------------------------------

- Support context types with lifetimes and generics ([#27](https://github.com/casey/filepack/pull/27) by [casey](https://github.com/casey))

[0.2.2](https://github.com/casey/boilerplate/releases/tag/0.2.2) - 2022-12-06
-----------------------------------------------------------------------------

- Split CI into multiple jobs ([#25](https://github.com/casey/filepack/pull/25) by [casey](https://github.com/casey))
- Use `boilerplate_` prefix to avoid conflicts ([#24](https://github.com/casey/filepack/pull/24) by [casey](https://github.com/casey))

[0.2.1](https://github.com/casey/boilerplate/releases/tag/0.2.1) - 2022-10-04
-----------------------------------------------------------------------------

- Add charset=utf-8 to text mime types ([#21](https://github.com/casey/filepack/pull/21) by [casey](https://github.com/casey))

[0.2.0](https://github.com/casey/boilerplate/releases/tag/0.2.0) - 2022-09-18
-----------------------------------------------------------------------------

- Use alternative escaping approach ([#18](https://github.com/casey/filepack/pull/18) by [casey](https://github.com/casey))

[0.1.0](https://github.com/casey/boilerplate/releases/tag/0.1.0) - 2022-09-17
-----------------------------------------------------------------------------

- Improve doc formatting ([#15](https://github.com/casey/filepack/pull/15) by [casey](https://github.com/casey))
- Add HTML escaping ([#11](https://github.com/casey/filepack/pull/11) by [casey](https://github.com/casey))
- Use Rust codeblock in readme ([#14](https://github.com/casey/filepack/pull/14) by [terror](https://github.com/terror))
- Rename `Display` to `Boilerplate` ([#13](https://github.com/casey/filepack/pull/13) by [casey](https://github.com/casey))
- Parse macro input with syn::parse_macro_input ([#12](https://github.com/casey/filepack/pull/12) by [casey](https://github.com/casey))
- Add doctests ([#9](https://github.com/casey/filepack/pull/9) by [casey](https://github.com/casey))
- Inline templates ([#8](https://github.com/casey/filepack/pull/8) by [casey](https://github.com/casey))
- tests/lib.rs -> tests/integration.rs ([#7](https://github.com/casey/filepack/pull/7) by [casey](https://github.com/casey))

[0.0.2](https://github.com/casey/boilerplate/releases/tag/0.0.2) - 2022-08-10
-----------------------------------------------------------------------------

- Avoid moving interpolations ([#5](https://github.com/casey/filepack/pull/5) by [casey](https://github.com/casey))
- Add /tmp to .gitignore ([#4](https://github.com/casey/filepack/pull/4) by [casey](https://github.com/casey))

[0.0.1](https://github.com/casey/boilerplate/releases/tag/0.0.1) - 2022-08-10
-----------------------------------------------------------------------------

- Initial implementation ([#1](https://github.com/casey/filepack/pull/1) by [casey](https://github.com/casey))
- Initial commit
