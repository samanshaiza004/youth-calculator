# Calculator Findings

This is the canonical evidence record for the Calculator entry in the Youth
Utility Suite. Findings describe observed application friction; they do not
automatically authorize platform features.

## Open findings

| ID | Summary | Next decision |
| --- | --- | --- |
| CALC-F001 | DP0 presentation cannot express a calculator layout | Add only the semantic layout/alignment data proved necessary here |
| CALC-F002 | Command dispatch and canonical state persistence are repetitive | Reassess after the protocol-backed command API exists |

## Findings index

| ID | Summary | Status | Primary implication |
| --- | --- | --- | --- |
| CALC-F001 | DP0 presentation cannot express a calculator layout | Open | Protocol needs row/grid intent and end text alignment |
| CALC-F002 | Command dispatch and canonical state persistence are repetitive | Open | Commands may be SDK-owned; structured state remains unproven |
| CALC-F003 | The external app needs no raw WIT concepts | Addressed | Preserve the DP0 SDK boundary while adding capabilities |

## CALC-F001 — DP0 presentation cannot express a calculator layout

- **Status:** Open
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
- **Resolution:** Pending protocol `0.0.3`, renderer evidence, and conversion of
  this exact app without raw protocol code.

## CALC-F002 — Command dispatch and canonical state persistence are repetitive

- **Status:** Open
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
- **Resolution:** Reassess after the DP1 SDK command API. Do not add structured
  state until another app proves the same pattern.

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
