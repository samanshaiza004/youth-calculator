# Calculator Findings

This is the canonical evidence record for the Calculator entry in the Youth
Utility Suite. Findings describe observed application friction; they do not
automatically authorize platform features.

## Open findings

| ID | Summary | Next decision |
| --- | --- | --- |
| CALC-F002 | Canonical state persistence remains repetitive | Keep explicit typed state until another app proves a common structured-state shape |

## Findings index

| ID | Summary | Status | Primary implication |
| --- | --- | --- | --- |
| CALC-F001 | DP0 presentation cannot express a calculator layout | Addressed | Protocol `0.0.3` carries bounded layout/alignment/shortcut intent |
| CALC-F002 | Command dispatch and canonical state persistence are repetitive | Deferred | SDK commands remove ID pairing; structured state remains unproven |
| CALC-F003 | The external app needs no raw WIT concepts | Addressed | Preserve the DP0 SDK boundary while adding capabilities |
| CALC-F005 | Semantic tests initially treated aligned text as a different role | Addressed | Test normalized semantic roles, not compact host representation variants |
| CALC-F006 | Git SDK resolution inspected an invalid template placeholder manifest | Addressed | Embedded source templates must remain syntactically valid before rendering |

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
