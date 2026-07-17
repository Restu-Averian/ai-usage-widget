# M11 — Public Beta

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

Release AI Usage Dock to a limited group and collect evidence about product value, reliability, and provider compatibility.

## 2. Intended Outcome

External users can install, connect supported providers, understand limitations, and submit useful feedback without developer assistance.

## 3. Dependencies

- M10 signed builds.
- Privacy documentation.
- Supported provider version matrix.
- Known limitations.
- Feedback channel.
- Release notes.

## 4. In Scope

- Onboarding polish.
- Installation documentation.
- Supported CLI version matrix.
- Troubleshooting guide.
- Privacy explanation.
- Security limitations.
- Diagnostics export.
- Feedback process.
- Release notes.
- Changelog.
- License review.
- Beta issue triage.
- Upgrade path between beta versions.

## 5. Out of Scope

- Guaranteed support for every provider version.
- Enterprise administration.
- Mobile application.
- Hosted credential storage.
- Large-scale telemetry without explicit privacy design.

## 6. Suggested Task Groups

- Beta onboarding.
- Documentation.
- compatibility matrix.
- diagnostics.
- feedback templates.
- issue triage labels.
- release notes.
- license audit.
- privacy review.

## 7. Beta Success Signals

Track manually or through privacy-reviewed mechanisms:

- installation success;
- provider connection success;
- percentage of users with at least one working provider;
- frequency of unsupported CLI versions;
- refresh reliability;
- notification usefulness;
- memory and startup complaints;
- user understanding of subscription vs API;
- demand for mobile access.

## 8. Acceptance Criteria

- Beta users can install without development tools.
- Onboarding accurately explains connection methods.
- Provider password/cookie policy is explicit.
- Supported provider versions are documented.
- Unsupported states have troubleshooting guidance.
- Diagnostics export is sanitized.
- Feedback can include app/CLI versions without secrets.
- Release notes and changelog exist.
- License obligations are satisfied.
- Upgrade between beta builds is tested.
- Critical issues have a triage process.

## 9. Verification

```text
fresh-user onboarding test
installation documentation test
diagnostics secret scan
supported-version test
upgrade test
uninstall/reinstall test
feedback submission dry run
```

## 10. Risks

- Provider updates may break beta connectors.
- Diagnostics may accidentally reveal personal data.
- Early Windows builds may trigger reputation warnings.
- Users may expect unsupported mobile access.
- Public messaging may overstate accuracy.

## 11. Review Questions

- Do users understand the product in one minute?
- Which provider drives most value?
- Are fallbacks acceptable?
- Is maintenance cost sustainable?
- Is mobile sync justified?

## 12. Exit and Handoff

Possible decisions after beta:

```text
Proceed to stable desktop release
Continue beta and improve providers
Reframe around a subset of providers
Prioritize mobile sync
Pause product
```

M12 begins only if real demand and a stable snapshot schema exist.
