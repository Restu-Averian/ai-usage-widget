# M1 — Repository Foundation

**Project:** AI Usage Dock  
**Document status:** Draft v0.1  
**Last updated:** 2026-07-16

Related documents:

- `../AI-Usage-Dock-PRD.md`
- `../AI-Usage-Dock-Technical-Spike.md`
- `../AI-Usage-Dock-Technical-Design.md`
- `../ui-specification/README.md`
- `../../AGENTS.md`

---

## 1. Objective

Create a stable repository structure, development workflow, and set of engineering rules before feature implementation grows.

## 2. Intended Outcome

The repository can be opened by a coding agent or developer and immediately communicates:

- what the product is;
- where each kind of code belongs;
- which commands verify changes;
- which security boundaries cannot be crossed;
- which milestone is active.

## 3. Dependencies

- PRD.
- Technical Design.
- UI Specification.
- `AGENTS.md`.
- Milestone roadmap.

M1 may run in parallel with M0 where no provider conclusion is required.

## 4. In Scope

- Initialize Git.
- Place documentation in canonical locations.
- Place `AGENTS.md` at repository root.
- Select and configure package manager.
- Create initial frontend and Rust folder boundaries.
- Add `.gitignore`.
- Ignore `.private/`, environment files, raw captures, and build artifacts.
- Add formatting and lint configuration.
- Define repository scripts.
- Add initial ADR directory.
- Add README development instructions.
- Establish task directory convention.
- Prepare CI commands without requiring full release packaging.

## 5. Out of Scope

- Desktop tray behavior.
- Final frontend UI.
- Real provider integration.
- Production database implementation.
- Signing and release artifacts.

## 6. Canonical Structure

```text
ai-usage-widget/
├── AGENTS.md
├── MILESTONES.md              # optional compatibility pointer
├── README.md
├── docs/
│   ├── AI-Usage-Dock-PRD.md
│   ├── AI-Usage-Dock-Technical-Spike.md
│   ├── AI-Usage-Dock-Technical-Design.md
│   ├── ui-specification/
│   ├── milestones/
│   ├── tasks/
│   └── adr/
├── spike/
├── src/
├── src-tauri/
└── .private/                  # ignored
```

## 7. Suggested Task Groups

### M1-A — Documentation placement

- Move `AGENTS.md` to root.
- Move roadmap into `docs/milestones/README.md`.
- Add milestone files.
- Validate relative links.

### M1-B — Tooling

- Initialize package manager.
- Add lint, format, typecheck, and test scripts.
- Add Rust formatting and lint expectations.
- Pin toolchain versions where appropriate.

### M1-C — Repository protection

- Add `.gitignore`.
- Protect `.private/`.
- Add secret-file patterns.
- Add pre-commit or secret-scan policy if selected.

### M1-D — Architecture records

Create initial ADRs:

```text
ADR-001-use-tauri-2.md
ADR-002-provider-logic-in-rust.md
ADR-003-separate-subscription-and-api.md
ADR-004-local-first-mvp.md
ADR-005-gate-pty-on-spike.md
```

### M1-E — Development onboarding

- Add README setup section.
- Add verification commands.
- Add task workflow.
- Add platform prerequisites.

## 8. Acceptance Criteria

- `AGENTS.md` is at repository root.
- All core documents are under `docs/`.
- `docs/milestones/README.md` links to M0–M12.
- `.private/` is Git-ignored.
- Environment and secret files are Git-ignored.
- Package dependencies use a lockfile.
- Frontend verification commands are defined.
- Rust verification commands are documented.
- No placeholder secret exists in committed source.
- Initial ADRs exist.
- A new developer can identify the active milestone.

## 9. Verification

```text
git status
git check-ignore .private/example.raw.log
validate Markdown links
run formatting configuration checks
run available lint/typecheck commands
inspect repository tree
```

## 10. Risks

- Documentation may be placed where agents do not automatically read it.
- Relative document links may break after moves.
- Too much scaffolding may prematurely lock architecture.
- Tooling versions may diverge between machines.

## 11. Review Questions

- Does the structure match the Technical Design?
- Is root-level guidance easy for agents to discover?
- Are secrets and raw captures protected?
- Are commands reproducible?
- Has unnecessary implementation been avoided?

## 12. Exit and Handoff

M2 and M3 may begin when:

- root instructions are stable;
- repository scripts are known;
- directory boundaries are agreed;
- documentation links resolve.
