# Spike Workspace

This directory contains experimental spike code and tests to determine the technical feasibility of provider integrations.

## Safety Rules

1. **No Real Secrets**: Do not store any real API keys, passwords, or personal info in this directory.
2. **Sanitize Fixtures**: All fixtures placed in `spike/fixtures/` must be manually sanitized to remove any sensitive info (e.g. email addresses, names, API keys).
3. **Use `.private/` for Raw Captures**: Raw captures and screenshots must be placed in `.private/raw-captures/` and `.private/screenshots/`, respectively, which are ignored by Git.
4. **No `.private/` in Source Control**: Never commit `.private/` to Git.
