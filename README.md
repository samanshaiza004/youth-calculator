# Youth Calculator

The first Youth Utility Suite application. It is maintained as a standalone
repository. Its first commit deliberately targeted Developer Preview 0 to
capture missing platform capabilities; the current app targets protocol
`0.0.3` and the exact SDK revision in `Youth.lock`.

```text
youth check
youth test
youth dev
youth build --release
```

The calculator uses bounded decimal arithmetic with twelve significant digits.
Its durable state is the canonical calculator model; display text is always
derived. The view uses a host-laid-out command row and equal-track keypad.
Digits, decimal point, operators, Enter/equals, Escape/clear, and Backspace use
logical shortcuts; Tab and arrows use host-owned focus policy. See
`FINDINGS.md` for application-driven platform evidence.

The vendored WIT directory is an inspectable contract snapshot. Rust bindings
and export plumbing come only from the exact `youth-sdk` revision in
`Youth.lock`.
