# Accessibility

## 1. Target

AI Usage Dock should meet WCAG 2.2 AA principles for its WebView interface where applicable.

Desktop-native shell behavior must remain keyboard and assistive-technology friendly.

---

## 2. Keyboard

All functionality must be available without a mouse.

Requirements:

- logical tab order;
- visible focus indicator;
- arrow navigation for provider tabs;
- Escape closes the topmost layer;
- Enter and Space activate controls;
- no keyboard traps;
- focus returns to the triggering control after dialogs;
- disabled controls are skipped or correctly announced.

---

## 3. Focus Styling

Use:

```text
2 px focus ring
2 px offset where layout allows
```

Focus appears only for keyboard/focus-visible interaction.

Do not remove native focus without replacement.

---

## 4. Screen Reader Semantics

### Provider tabs

- `role="tablist"`
- each tab uses `role="tab"`
- selected tab sets `aria-selected="true"`
- panel uses `role="tabpanel"`

### Usage ring

Accessible label example:

```text
Weekly usage: 68 percent used, 32 percent remaining.
Resets in 2 days and 8 hours.
```

### Progress bar

Use semantic progress attributes:

```text
aria-valuemin="0"
aria-valuemax="100"
aria-valuenow="68"
aria-label="Weekly limit"
```

When unknown, do not set a fake value.

### Charts

Charts require a textual alternative:

```text
Usage increased from 24 percent on Monday
to 68 percent on Sunday.
```

A hidden data table may be used for richer history.

---

## 5. Color and Contrast

- Body text target: at least 4.5:1.
- Large text target: at least 3:1.
- UI boundaries and focus indicators: at least 3:1 where required.
- Warning and critical states include icon/text, not color alone.
- Stale and offline states include explicit labels.

Provider logos must remain recognizable in both themes.

---

## 6. Motion

Respect:

```css
@media (prefers-reduced-motion: reduce);
```

Disable:

- progress interpolation;
- skeleton pulse;
- entry fade;
- decorative motion.

No information depends on animation.

---

## 7. Text Scaling

The popup must remain usable at increased text scaling.

Requirements:

- support at least 125% text scaling without clipped controls;
- allow vertical growth and scrolling;
- avoid fixed text containers;
- tab labels must remain visible;
- truncate only account labels and technical paths;
- critical action text must not truncate.

At extreme scaling, the popup may use the maximum supported width.

---

## 8. Interactive Targets

Preferred minimum:

```text
40 × 40 logical px
```

Absolute minimum for compact controls:

```text
32 × 32 logical px
```

Controls with smaller visual icons still receive a larger interactive area.

---

## 9. Error Identification

Validation errors:

- identify the affected field;
- include text, not just border color;
- remain associated using `aria-describedby`;
- do not clear until corrected or resubmitted.

Example:

```text
The API key could not be validated.
```

---

## 10. Live Regions

Use polite live announcements for:

- refresh completion;
- connection established;
- connection lost;
- validation result.

Avoid announcing:

- every countdown update;
- every chart redraw;
- each background refresh start.

Critical destructive errors may use assertive announcements.

---

## 11. Tooltips and Truncation

Any truncated label must be available through:

- tooltip;
- accessible name;
- details view.

Tooltips must be reachable by keyboard and not hover-only.

---

## 12. High Contrast

Test with:

- macOS Increase Contrast;
- Windows Contrast Themes;
- browser forced-colors behavior where applicable.

In forced-color environments:

- preserve progress boundaries;
- use system colors;
- avoid background-image-only status indicators;
- ensure selected tabs remain apparent.

---

## 13. Native Notifications

Notifications must contain:

- provider name;
- concise condition;
- actionable meaning.

Example:

```text
Claude weekly usage reached 95%.
```

Avoid vague copy:

```text
Usage alert
```

Notification click opens the relevant provider view.
