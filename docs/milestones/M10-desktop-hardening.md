# M10 — Desktop Hardening

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

Prepare AI Usage Dock for reliable installation and continued use outside the development environment.

## 2. Intended Outcome

Signed, packaged desktop builds operate safely on supported macOS and Windows targets.

## 3. Dependencies

- Stable core functionality.
- Stable database migrations.
- Stable provider connectors/fallbacks.
- Release identity and versioning.
- Signing credentials and release infrastructure.

## 4. In Scope

- macOS signing.
- macOS notarization.
- Apple Silicon release.
- Intel/universal decision.
- Windows installer.
- Windows signing.
- Signed updater.
- Migration recovery.
- Rolling redacted logs.
- Diagnostics export.
- Wake and network recovery.
- Multiple-monitor testing.
- Display scaling.
- CLI path-change handling.
- Crash-safe startup.
- Release build optimization.
- Uninstall behavior documentation.

## 5. Out of Scope

- Public beta support process.
- Mobile sync.
- Unsigned automatic updater.
- Silent destructive recovery.

## 6. Suggested Task Groups

### M10-A — Build identity

- bundle identifier.
- application metadata.
- icons.
- versioning.
- release channels.

### M10-B — macOS release

- sign.
- notarize.
- package.
- test Gatekeeper.
- test clean-machine install.

### M10-C — Windows release

- installer.
- sign.
- clean-machine test.
- display scaling.
- uninstall.

### M10-D — Updater

- signed manifests.
- signature verification.
- update UI.
- failure recovery.

### M10-E — Reliability

- migration backup/recovery.
- wake/network refresh.
- rolling logs.
- diagnostics.
- path-change approval.

## 7. Acceptance Criteria

- Signed macOS build installs and opens on a clean test machine.
- Notarization passes.
- Signed Windows installer installs and opens.
- Popup works across required monitor/scaling scenarios.
- Updater verifies signatures.
- Unsigned updates cannot install automatically.
- Database failure does not silently delete user data.
- Migration recovery behavior is documented.
- Logs pass secret-scan review.
- App recovers after sleep and network restoration.
- CLI path change triggers review.
- Uninstall behavior is documented.
- Release checklist exists.

## 8. Verification

```text
clean macOS install
Gatekeeper/notarization verification
clean Windows install
signature verification
upgrade from previous database version
failed update test
sleep/wake test
offline/online test
multiple monitors
100–200% Windows scaling
log secret scan
```

## 9. Risks

- Signing credentials may be unavailable.
- Updater can become a supply-chain risk.
- Database migration failure may threaten user history.
- Platform behavior may vary outside developer machines.
- Anti-virus reputation may affect early Windows builds.

## 10. Review Questions

- Can users install without development tools?
- Are updates cryptographically trusted?
- Is recovery non-destructive?
- Are logs safe enough to export?
- Are supported OS/architecture combinations explicit?

## 11. Exit and Handoff

M11 receives release artifacts, known limitations, diagnostics, and a repeatable release process.
