# Calculator Findings

This is the canonical evidence record for the Calculator entry in the Youth
Utility Suite. Findings describe observed application friction; they do not
automatically authorize platform features.

## Open findings

| ID | Summary | Next decision |
| --- | --- | --- |
| CALC-F002 | Canonical state persistence remains repetitive | Keep explicit typed state until another app proves a common structured-state shape |
| CALC-F009 | Explicit updates can diverge from reconstructed view output | Gather evidence from more dynamic applications before changing the explicit-patch model |

## Findings index

| ID | Summary | Status | Primary implication |
| --- | --- | --- | --- |
| CALC-F001 | DP0 presentation cannot express a calculator layout | Addressed | Protocol `0.0.3` carries bounded layout/alignment/shortcut intent |
| CALC-F002 | Command dispatch and canonical state persistence are repetitive | Deferred | SDK commands remove ID pairing; structured state remains unproven |
| CALC-F003 | The external app needs no raw WIT concepts | Addressed | Preserve the DP0 SDK boundary while adding capabilities |
| CALC-F004 | Compatibility normalization belongs at the runtime boundary | Addressed | Presentation, interaction, and tools consume normalized semantics rather than version-specific variants |
| CALC-F005 | Semantic tests initially treated aligned text as a different role | Addressed | Test normalized semantic roles, not compact host representation variants |
| CALC-F006 | Git SDK resolution inspected an invalid template placeholder manifest | Addressed | Embedded source templates must remain syntactically valid before rendering |
| CALC-F007 | Activation-only tests could not prove keyboard policy | Addressed | Test logical keys and semantic focus through the host interaction layer |
| CALC-F008 | The provisional font lacked common calculator punctuation | Addressed | Printable ASCII coverage belongs to the renderer; broader Unicode remains future text-stack work |
| CALC-F009 | Explicit updates can diverge from reconstructed view output | Deferred | Retain explicit patches while testing convergence and gathering evidence for later update models |

## CALC-F001 — DP0 presentation cannot express a calculator layout

- **Status:** Addressed
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** Initial `Application::view`
- **Platform:** Platform-independent source inspection
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Commit:** Initial app proof
- **Evidence:** Protocol `0.0.2` and its SDK expose only a vertical column box.
  The calculator's display and nineteen buttons can mount, activate, persist,
  and test, but every control is one full vertical sequence. The display also
  has no semantic end alignment. Node labels or whitespace cannot safely encode
  either layout rule.
- **Developer impact:** A correct calculator model produces an unusable tall
  window and cannot resemble a conventional calculator.
- **What could not be expressed:** Horizontal command strip, equal-track
  keypad, end-aligned numeric display, keyboard focus, or logical shortcuts.
- **What felt repetitive:** Nothing in the model proof changes the missing
  presentation intent.
- **What leaked WIT details:** None.
- **What required host policy:** Exact track sizing, spacing, clipping, focus,
  and key interpretation.
- **Unavoidable protocol addition:** Semantic row/grid layout, text alignment,
  and bounded logical shortcut declarations.
- **What remains SDK/application behavior:** Layout builders and calculator
  number formatting.
- **Impact:** Functional headless behavior is available, but native usability
  and reasonable window size are blocked.
- **Resolution:** Protocol `0.0.3` now expresses a four-control row, equal-track
  four-column keypad, end-aligned display, and logical shortcuts. The converted
  app contains no geometry or raw protocol code.

## CALC-F002 — Command dispatch and canonical state persistence are repetitive

- **Status:** Deferred
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** Initial event and persistence adapter
- **Platform:** Platform-independent source inspection
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Commit:** Initial app proof
- **Evidence:** Nineteen symbolic button IDs are repeated between the view and
  a linear activation-to-command dispatch table. Saving the canonical model
  also expands into typed calls for mode, entry, accumulator, pending operator,
  and repeated-equals state.
- **Developer impact:** Domain logic remains clear, but adapter code is longer
  than the visual tree and makes omissions easy.
- **What could not be expressed:** A typed view-backed command whose identity,
  button, shortcuts, and event query are declared together.
- **What felt repetitive:** Node/command pairing, activation matching, optional
  value deletion, and typed state field naming.
- **What leaked WIT details:** No WIT types leaked; repetition occurs within the
  SDK's current semantic API.
- **What required host policy:** Shortcut resolution must remain host-owned.
- **Unavoidable protocol addition:** Only shortcut metadata; command identity
  and dispatch do not need a new guest event.
- **What remains SDK/application behavior:** Distinct `CommandId`, command
  builders, `Events::commanded`, and the calculator's persistence adapter.
- **Impact:** Source verbosity and risk of mismatched symbolic names.
- **Resolution:** `Button::command` and `Events::commanded` remove node/command
  pairing while retaining distinct ID types and domains. Canonical state calls
  remain explicit and are deferred, not promoted into a protocol feature,
  until another application proves the same structured-state shape.

## CALC-F003 — The external app needs no raw WIT concepts

- **Status:** Addressed
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** Initial build and semantic tests
- **Platform:** Rust/WASIp2
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Commit:** Initial app proof
- **Evidence:** Application code uses SDK trees, symbolic node names, typed
  state, semantic activations, and updates. It contains no generated bindings,
  numeric node IDs, revisions, acknowledgements, raw patches, state imports,
  or component export implementation.
- **Developer impact:** Calculator work starts from domain behavior instead of
  Component Model mechanics.
- **What could not be expressed:** Presentation and keyboard semantics, not
  protocol plumbing.
- **What felt repetitive:** SDK-level command and persistence adapters only.
- **What leaked WIT details:** None; the vendored WIT remains inspectable data.
- **What required host policy:** None for the headless model proof.
- **Unavoidable protocol addition:** None for arithmetic, state, or mouse
  activation.
- **What remains SDK/application behavior:** All calculator domain logic.
- **Impact:** Raw-WIT concept count is zero.
- **Resolution:** Locked by source audit, `youth check`, semantic tests, and the
  exact SDK revision in `Youth.lock`.

## CALC-F004 — Compatibility normalization belongs at the runtime boundary

- **Status:** Addressed
- **Observed:** 2026-07-21
- **Application:** Youth Calculator and unchanged DP0 Tally
- **Workflow stage:** Protocol `0.0.3` host integration
- **Platform:** Platform-independent runtime boundary
- **Evidence:** Branching on `0.0.2` versus `0.0.3` inside layout, interaction,
  rendering, or semantic testing would duplicate policy and make equivalent
  roles behave differently across component versions.
- **Resolution:** Both protocol worlds convert into the same normalized tree
  and patch model at the runtime boundary. Downstream tools query normalized
  semantics and remain independent of wire-version storage variants.

## CALC-F005 — Aligned text exposed a test-runner representation assumption

- **Status:** Addressed
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** First protocol `0.0.3` `youth test`
- **Platform:** macOS, headless runtime
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Evidence:** The display mounted as normalized aligned text and diagnostics
  printed `text("0")`, yet `expect text display "0"` rejected it because the
  runner matched only the compact default-alignment enum variant.
- **Developer impact:** Adding presentation intent incorrectly changed the
  meaning of an existing semantic assertion.
- **What leaked WIT details:** No app code leaked WIT, but a host normalization
  detail leaked into tooling behavior.
- **Resolution:** Text assertions now use the normalized semantic text
  accessor. Alignment remains irrelevant unless a future assertion explicitly
  tests it.

## CALC-F006 — Template placeholders affected Git SDK discovery

- **Status:** Addressed
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** Fresh exact-revision SDK resolution
- **Platform:** macOS/Cargo
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Evidence:** Cargo found the CLI's embedded template while scanning the Git
  repository and diagnosed `{{package}}` as an invalid package name before
  continuing to compile `youth-sdk`.
- **Developer impact:** Successful SDK builds emitted an alarming unrelated
  error, weakening fresh-install diagnostics.
- **What felt repetitive or leaked:** Monorepo template implementation leaked
  into an external dependency fetch.
- **Resolution:** The template uses a valid package-name sentinel that the
  generator replaces, and its source directory is explicitly excluded from
  workspace discovery. Templates must be valid before rendering.

## CALC-F007 — Activation-only tests could not prove keyboard policy

- **Status:** Addressed
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** Gate C acceptance test
- **Platform:** Headless, real Youth runtime
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Evidence:** `activate <node>` proved guest behavior but bypassed host-owned
  Tab/arrow focus, default/cancel keys, and logical shortcut resolution.
- **Developer impact:** Native keyboard behavior could regress while semantic
  application tests stayed green.
- **What could not be expressed:** A host logical-key input and semantic focus
  assertion without native scan codes.
- **What leaked WIT details:** None; keys terminate in the host interaction
  state machine and the guest still receives ordinary node activation.
- **What required host policy:** Focus traversal, logical key matching, and
  Enter/Escape/Backspace precedence.
- **Resolution:** The deliberately narrow DSL adds `key` and `expect focus`.
  The calculator test now covers Tab, row arrows, Shift+Tab, character
  shortcuts, Enter/default, Escape/cancel, restart focus clearing, persistence,
  and a direct semantic activation in one real-runtime scenario.

## CALC-F008 — The provisional font lacked common calculator punctuation

- **Status:** Addressed
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** Native `youth run` presentation
- **Platform:** macOS
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Commit:** `8ef8e40`
- **Resolution commit:** Youth `2761e35`
- **Evidence:** The app deliberately labels controls with plain ASCII `+/-`,
  `/`, `*`, `-`, `+`, `.`, and `=`. Youth's deterministic 5x7 debug font only
  defines letters, digits, colon, hyphen, underscore, and space; its fallback
  glyph is a question mark. The native window therefore replaces every
  unsupported punctuation character with `?`, while `-` renders correctly.
- **Developer impact:** Common, semantically clear labels become ambiguous even
  though the component tree, hit targets, shortcuts, and calculator behavior
  are correct.
- **What could not be expressed:** Nothing. The application already supplies
  the intended UTF-8 labels.
- **What felt repetitive:** Nothing in application code; changing labels to
  words would merely work around a host renderer limitation.
- **What leaked WIT details:** None.
- **What required host policy:** Font selection, supported glyph repertoire,
  missing-glyph fallback, text measurement, and deterministic rasterization.
- **Unavoidable protocol addition:** None. Text already crosses the component
  boundary correctly.
- **What remains SDK/application behavior:** The label strings and command
  semantics remain application-owned; neither the SDK nor the app should know
  which glyphs a host renderer can draw.
- **Resolution:** Youth's provisional renderer now covers all 95 printable
  ASCII characters (`U+0020..=U+007E`). Tests lock every calculator operator
  glyph and a representative pixel fixture rather than one golden fixture per
  character. This is the final bounded extension of the debug font; a real
  Unicode text stack remains required before text-editing applications.

## CALC-F009 — Explicit updates can diverge from reconstructed view output

- **Status:** Deferred
- **Observed:** 2026-07-21
- **Application:** Youth Calculator
- **Workflow stage:** Gate C source and restart-test audit
- **Platform:** Platform-independent source and headless runtime
- **Local path:** `/Users/keina/dev/youth-calculator`
- **Commit:** `8ef8e40`
- **Evidence:** Both `Application::view` and `Application::handle` call the same
  `Model::display` formatter, so the formatting algorithm is not duplicated.
  The handler must still know that the `display` node is affected and return a
  `set_text` update. The restart acceptance test proves that the incrementally
  patched `"3.5"` display is reconstructed as `"3.5"` by a fresh `view` from
  durable state, but Youth does not generally compare a post-handle tree with
  a newly constructed view.
- **Intended invariant:** After an accepted turn, applying its update to the
  previous normalized tree should produce the same guest-owned semantic tree
  as a fresh view from committed durable state. Host-owned interaction and
  presentation state are excluded. DP1 does not generally enforce this.
- **Developer impact:** This calculator has only one changing node, so explicit
  patching is easy. More dynamic apps could omit an affected node and produce
  a live tree that differs from restart or read-only resync output.
- **What could not be expressed:** An automatic assertion that an accepted
  patch and a fresh view of committed state are semantically identical.
- **What felt repetitive:** The display node name and `model.display()` call
  appear in both initial view construction and update construction; the actual
  formatting logic remains shared.
- **What leaked WIT details:** None. `Update::set_text` is an SDK semantic API,
  and the app does not see revisions or raw patches.
- **What required host policy:** Transaction ordering and authoritative-tree
  installation remain host-owned. Choosing when to execute or compare a fresh
  view would also be runtime/tooling policy.
- **Unavoidable protocol addition:** None demonstrated by this application.
- **What remains SDK/application behavior:** Explicit update construction and
  the shared display formatter remain appropriate for DP1.
- **Next decision:** Keep explicit patches while Scratchpad, Timer, and Todo
  reveal whether manual affected-node knowledge becomes repetitive or unsafe.
  Compare three later directions only with that evidence: explicit patches,
  SDK tree diffing, or declared reactive dependencies.
