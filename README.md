# Youth Calculator

The first Youth Utility Suite application. It is maintained as a standalone
repository and initially targets the Developer Preview 0 SDK so the missing
platform capabilities are observed before Youth grows to support them.

```text
youth check
youth test
youth dev
youth build --release
```

The calculator uses bounded decimal arithmetic with twelve significant digits.
Its durable state is the canonical calculator model; display text is always
derived. See `FINDINGS.md` for application-driven platform evidence.

The vendored WIT directory is an inspectable contract snapshot. Rust bindings
and export plumbing come only from the exact `youth-sdk` revision in
`Youth.lock`.
