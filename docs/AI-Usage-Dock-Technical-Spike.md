# Technical Spike — AI Usage Widget

**Document status:** Completed foundation evidence  
**Last updated:** 2026-07-19

## 1. Purpose

Validate safe local integration paths before production provider implementation.

Active product providers:

- Codex;
- Antigravity.

## 2. Conclusions

### Codex

A real local path has been proven:

```text
Codex CLI
→ Codex app-server
→ Rust owner
→ typed Tauri IPC
→ React UI
```

The current risk is no longer basic feasibility. The active risks are:

- deterministic app-server ownership;
- preventing duplicate child processes;
- handling initialization failure without affecting the tray;
- version-aware parsing;
- accurately mapping provider `usedPercent` to user-facing remaining quota.

Codex is the only provider implemented during M5.

### Antigravity

Antigravity remains the second active provider. Its connector work is deferred to M6 and must use the safest supported local interface confirmed at implementation time.

Do not begin Antigravity integration before M5 is marked PASS.

## 3. Safety Gates

Allowed integration priority:

```text
official structured interface
→ official local service or stable output
→ version-aware text parser
→ official dashboard fallback
```

Prohibited:

- browser-cookie extraction;
- credential-file reading;
- private traffic reverse engineering;
- generic frontend shell access;
- fabricated quota or reset values.

## 4. Normalization Decision

Provider adapters preserve the source semantic when practical:

```ts
type ProviderWindow = {
  period: "weekly" | "daily" | "monthly" | "other";
  usedPercent: number | null;
  resetsAt: string | null;
};
```

The presentation layer derives remaining quota. Unknown values remain unknown.

## 5. Updated Spike Exit

M0 remains PASS because:

- Codex has a viable real integration path;
- Antigravity has a dedicated later validation/integration milestone;
- unsafe credential and browser approaches are rejected;
- provider work is isolated behind Rust-owned connectors;
- the product no longer promises unsupported extra providers.
