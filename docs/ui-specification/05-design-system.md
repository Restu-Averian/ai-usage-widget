# Design System

## 1. Visual Character

AI Usage Dock uses a compact, calm, native-adjacent visual language.

The interface should not imitate macOS or Windows pixel-for-pixel. It should feel appropriate on both platforms while maintaining one product identity.

Keywords:

```text
compact
precise
quiet
trustworthy
technical
softly elevated
low distraction
```

---

## 2. Theme Strategy

Supported themes:

```text
System
Light
Dark
```

Default:

```text
System
```

Dark mode is optimized for low-glare use but must not use pure black as the primary background.

---

## 3. Color Tokens

Use semantic CSS variables. Components must not hardcode provider-specific colors.

### 3.1 Light theme

```css
:root {
  --color-bg-canvas: #f4f5f7;
  --color-bg-surface: #ffffff;
  --color-bg-elevated: #ffffff;
  --color-bg-subtle: #f7f8fa;
  --color-bg-hover: #f0f2f5;
  --color-bg-pressed: #e8ebef;

  --color-text-primary: #16181d;
  --color-text-secondary: #5d6470;
  --color-text-tertiary: #858c97;
  --color-text-inverse: #ffffff;

  --color-border-subtle: #e5e8ec;
  --color-border-strong: #cdd2d9;
  --color-focus-ring: #4d73ff;

  --color-accent: #4d73ff;
  --color-accent-hover: #3f63e6;
  --color-accent-subtle: #edf1ff;

  --color-success: #1d8a5b;
  --color-success-subtle: #e8f7f0;

  --color-warning: #a96612;
  --color-warning-subtle: #fff4de;

  --color-critical: #c44545;
  --color-critical-subtle: #fdecec;

  --color-offline: #667085;
  --color-offline-subtle: #eef0f3;

  --color-chart-track: #e7eaf0;
  --color-chart-line: #4d73ff;
  --color-chart-area: rgba(77, 115, 255, 0.12);
}
```

### 3.2 Dark theme

```css
[data-theme="dark"] {
  --color-bg-canvas: #111318;
  --color-bg-surface: #181b21;
  --color-bg-elevated: #1e222a;
  --color-bg-subtle: #20242c;
  --color-bg-hover: #282d36;
  --color-bg-pressed: #303640;

  --color-text-primary: #f3f5f7;
  --color-text-secondary: #aeb5c0;
  --color-text-tertiary: #7e8794;
  --color-text-inverse: #111318;

  --color-border-subtle: #2a3039;
  --color-border-strong: #3a424e;
  --color-focus-ring: #7d99ff;

  --color-accent: #7d99ff;
  --color-accent-hover: #94aaff;
  --color-accent-subtle: #222c4b;

  --color-success: #58c995;
  --color-success-subtle: #17372a;

  --color-warning: #e2a64a;
  --color-warning-subtle: #3a2b15;

  --color-critical: #f07a7a;
  --color-critical-subtle: #402121;

  --color-offline: #98a2b3;
  --color-offline-subtle: #292e36;

  --color-chart-track: #2b313a;
  --color-chart-line: #7d99ff;
  --color-chart-area: rgba(125, 153, 255, 0.14);
}
```

### 3.3 Usage status mapping

```text
0–79.99%       normal/accent
80–94.99%      warning
95–100%        critical
unknown        neutral
stale          neutral + stale label
offline        offline tone
```

Do not use green for low usage by default. Low usage is a neutral/brand state, not a success message.

---

## 4. Provider Branding

Provider logos may be used for identification.

Rules:

- Use official or legally permitted marks.
- Do not recolor full-color logos unless provider guidelines allow it.
- Provider colors appear only in small identity surfaces.
- Usage charts use semantic app colors, not provider colors.
- The product should remain coherent when all provider logos appear together.
- Provide monochrome fallback icons where needed.

---

## 5. Typography

Use a system-first font stack:

```css
font-family:
  ui-sans-serif,
  -apple-system,
  BlinkMacSystemFont,
  "Segoe UI",
  Inter,
  sans-serif;
```

Monospace:

```css
font-family:
  ui-monospace, SFMono-Regular, "SF Mono", "Cascadia Code", "Roboto Mono",
  monospace;
```

### Type scale

| Token        |  Size | Line height | Weight | Usage                  |
| ------------ | ----: | ----------: | -----: | ---------------------- |
| `display-sm` | 28 px |       34 px |    650 | Hero percentage        |
| `heading-lg` | 20 px |       26 px |    650 | Welcome heading        |
| `heading-md` | 16 px |       22 px |    650 | Screen heading         |
| `body-md`    | 14 px |       20 px |    450 | Primary body           |
| `body-sm`    | 13 px |       18 px |    450 | Compact descriptions   |
| `label-md`   | 13 px |       18 px |    600 | Controls and cards     |
| `label-sm`   | 12 px |       16 px |    600 | Metadata               |
| `caption`    | 11 px |       15 px |    500 | Footer and helper text |

Use tabular numbers for:

- percentages;
- cost;
- token counts;
- reset countdowns.

---

## 6. Spacing

Base unit:

```text
4 px
```

Scale:

```text
space-0   0
space-1   4 px
space-2   8 px
space-3   12 px
space-4   16 px
space-5   20 px
space-6   24 px
space-8   32 px
space-10  40 px
```

Recommended usage:

- Popup horizontal padding: 16 px.
- Header horizontal padding: 12–16 px.
- Card internal padding: 12–16 px.
- Section gap: 20–24 px.
- Related control gap: 8 px.
- Icon-text gap: 8 px.

---

## 7. Radius

```text
radius-sm     8 px
radius-md    12 px
radius-lg    16 px
radius-pill  999 px
```

Usage:

- Buttons: 10–12 px.
- Cards: 12 px.
- Popup visual container: platform-managed or 14–16 px where supported.
- Badges: pill.
- Input: 10 px.

Avoid applying a different radius to every component.

---

## 8. Borders

Default:

```text
1 px solid var(--color-border-subtle)
```

Strong border is reserved for:

- focus;
- selected segmented control;
- high-emphasis input;
- confirmation dialogs.

Do not use borders around every text section.

---

## 9. Elevation

### Popup

Use native window shadow where possible.

### Internal elevation

```text
Level 0: background only
Level 1: subtle card border
Level 2: elevated dialog/popover
```

Avoid heavy shadows inside the compact popup.

Suggested dark/light dialog shadow:

```css
box-shadow:
  0 18px 50px rgba(0, 0, 0, 0.24),
  0 4px 12px rgba(0, 0, 0, 0.12);
```

---

## 10. Iconography

Preferred:

- one consistent outline icon set;
- 1.75–2 px optical stroke;
- rounded line caps;
- 16, 18, and 20 px sizes.

Common icons:

```text
refresh
settings
arrow-left
external-link
shield
key
terminal
cloud
wifi-off
warning
error
check
chevron-right
trash
info
```

Requirements:

- Icon-only buttons have accessible labels and tooltips.
- Do not mix several icon libraries.
- Icons never replace provider names in critical connection flows.

---

## 11. Buttons

### Primary

- Filled accent.
- One primary action per state.
- Minimum height: 36 px.
- Full width in connection empty states.

### Secondary

- Subtle background or outline.
- Used for API connection and retry actions.

### Tertiary

- Text button.
- Used for dashboard links and low-priority actions.

### Destructive

- Critical text/background.
- Only for confirmed destructive actions.

Button states:

```text
default
hover
pressed
focus-visible
disabled
loading
```

Loading buttons preserve their width.

---

## 12. Inputs

Height:

```text
36–40 px
```

Requirements:

- Visible label.
- Placeholder is not the label.
- Error text appears below.
- Focus ring uses semantic token.
- API key field uses password masking.
- Select controls must be keyboard accessible.

---

## 13. Progress Bars

Height:

```text
8 px default
6 px compact
```

Rules:

- Rounded track.
- Semantic status color.
- Text value appears outside the bar.
- Unknown values use no filled bar.
- Animated changes respect reduced motion.
- Do not use gradients for normal usage.

---

## 14. Charts

- One line/area series per chart.
- No 3D effects.
- No provider-color rainbow.
- Tooltip uses elevated surface.
- Axis labels are minimal.
- History uses consistent time intervals.
- Graph remains understandable without color.

---

## 15. Density

Default density:

```text
Comfortable compact
```

Compact mode changes:

- slightly smaller hero;
- 12 px horizontal padding;
- 6 px progress bars;
- provider icons removed from tabs;
- history chart may collapse behind `Show history`;
- no reduction below accessible target sizes.

Compact mode must not simply scale the whole UI down.
