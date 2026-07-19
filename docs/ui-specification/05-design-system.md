# 05 — Design System

The application uses one fixed dark theme.

## Visual direction

- compact but not cramped;
- high contrast;
- calm neutral surfaces;
- one clear quota focal point;
- minimal borders and decoration.

## Tokens

Use semantic tokens rather than a theme switch:

```text
background
surface
surface-elevated
text-primary
text-secondary
border
accent
success
warning
critical
```

Do not implement light-mode tokens, system-theme listeners, compact-mode density tokens, or appearance persistence.

## Spacing

Use a small consistent scale such as 4, 8, 12, 16, 20, and 24 pixels. The popup should remain readable at its platform-constrained size.
