# Component Specifications

## 1. Component Principles

Components should be:

- small;
- typed;
- accessible;
- provider-neutral where practical;
- driven by normalized data;
- usable with mock fixtures;
- visually stable across loading and success states.

Do not create one giant `ProviderDashboard` containing all states and connection types.

---

## 2. `AppHeader`

### Purpose

Display product identity and top-level actions.

### Props

```ts
type AppHeaderProps = {
  title: string;
  showAppIcon?: boolean;
  refreshing: boolean;
  refreshDisabled?: boolean;
  onRefresh: () => void;
  onOpenSettings: () => void;
};
```

### States

- default;
- refreshing;
- refresh unavailable;
- compact.

### Requirements

- Buttons have accessible labels.
- Spinner does not change button size.
- Settings button remains available during refresh.

---

## 3. `ProviderTabs`

### Props

```ts
type ProviderTabItem = {
  id: ProviderId;
  label: string;
  status: "none" | "warning" | "critical" | "attention" | "error";
};

type ProviderTabsProps = {
  items: ProviderTabItem[];
  selected: ProviderId;
  onChange: (provider: ProviderId) => void;
};
```

### Requirements

- Correct ARIA tab roles.
- Arrow-key navigation.
- Equal-width layout.
- Status indicator is not the only accessible signal.
- Provider icon is optional based on available width.

---

## 4. `ProviderIdentity`

### Purpose

Show provider icon, plan, account label, and connection status.

```text
[Claude icon] Claude Max
               r••••@example.com
                                      Connected
```

### Props

```ts
type ProviderIdentityProps = {
  provider: ProviderId;
  planName?: string;
  accountLabel?: string;
  statusLabel: string;
  statusTone: StatusTone;
};
```

### Requirements

- Account labels are already masked by backend or domain layer.
- Do not display raw credentials.
- Collapse secondary line when both plan and account are unavailable.

---

## 5. `UsageHero`

### Purpose

Present the most critical metric.

### Props

```ts
type UsageHeroProps = {
  value?: number;
  valueLabel: string;
  remainingPercent?: number;
  resetAt?: string;
  status: "normal" | "warning" | "critical" | "unknown";
  loading?: boolean;
};
```

### Variants

#### Percentage

```text
68%
Used this week
```

#### Cost

```text
$12.00
This month
```

#### Unknown

```text
Usage available
Percentage not reported
```

### Requirements

- Do not render an empty ring for unknown usage.
- Ring has a textual label.
- Ring animation runs only on first fresh render.
- Reduced motion disables animated progress.

---

## 6. `UsageRing`

### Props

```ts
type UsageRingProps = {
  value: number;
  size?: "sm" | "md" | "lg";
  tone: "normal" | "warning" | "critical";
  label: string;
};
```

### Sizes

| Size | Diameter |
| ---- | -------: |
| `sm` |    72 px |
| `md` |   112 px |
| `lg` |   148 px |

MVP hero uses `md`.

### Requirements

- Clamp visual value to `0..100`.
- Preserve precise value in accessible label.
- Track remains visible at 0%.
- 100% does not overflow.
- Use SVG, not canvas, for accessibility and scaling.

---

## 7. `UsageWindowCard`

### Layout

```text
Weekly limit                         68%
████████████████░░░░░░░░
32% remaining · Resets in 2d 8h
```

### Props

```ts
type UsageWindowCardProps = {
  label: string;
  model?: string;
  usedPercent?: number;
  remainingPercent?: number;
  resetAt?: string;
  isPrimary?: boolean;
  reliability?: Reliability;
};
```

### Requirements

- Supports unknown percentage.
- Model appears as a small secondary label.
- Exact reset timestamp is available.
- Progress bars include screen-reader values.
- Cards use low elevation and do not compete with the hero.

---

## 8. `HistoryChart`

### Props

```ts
type HistoryPoint = {
  timestamp: string;
  value: number;
};

type HistoryChartProps = {
  points: HistoryPoint[];
  metricLabel: string;
  periodLabel: string;
  emptyMessage: string;
};
```

### Requirements

- Use Recharts or equivalent SVG chart.
- No legend for one series.
- Tooltip shows exact value and timestamp.
- Provide a textual summary for screen readers.
- Empty history does not display fake points.
- Chart uses semantic theme tokens, not provider brand colors.

---

## 9. `ConnectionStatePanel`

### Variants

```text
disconnected
not-installed
connecting
authentication-required
unsupported
error
```

### Props

```ts
type ConnectionStatePanelProps = {
  provider: ProviderId;
  state: ConnectionStateVariant;
  title: string;
  description: string;
  primaryAction?: ActionSpec;
  secondaryAction?: ActionSpec;
  tertiaryAction?: ActionSpec;
  securityNote?: string;
};
```

### Requirements

- Maximum one primary filled button.
- Description width remains readable.
- Security note uses a shield icon or neutral inline note.
- Do not use red for normal disconnected state.

---

## 10. `ConnectionMethodButton`

### Variants

```text
local-cli
api-key
official-dashboard
```

### Layout

```text
[icon] Connect Claude Code
       View subscription quota
```

### Requirements

- Title and subtitle.
- Entire row is interactive.
- Dashboard fallback uses secondary styling.
- Disabled state explains why in tooltip or supporting copy.

---

## 11. `InlineBanner`

### Tones

```text
info
warning
critical
offline
success
```

### Props

```ts
type InlineBannerProps = {
  tone: BannerTone;
  title?: string;
  message: string;
  action?: ActionSpec;
  dismissible?: boolean;
};
```

### Requirements

- Compact.
- Avoid large alert blocks.
- Critical provider exhaustion is not automatically treated as an application error.
- Offline banner remains until connectivity or refresh state changes.

---

## 12. `StatusFooter`

### Props

```ts
type StatusFooterProps = {
  freshnessLabel: string;
  statusLabel?: string;
  statusTone?: StatusTone;
  details?: string;
};
```

### Requirements

- Sticky to bottom of popup.
- One line at default width.
- May wrap to two lines at minimum width.
- `details` is accessible through tooltip or popover.

---

## 13. Settings Components

### `SettingsSection`

```ts
type SettingsSectionProps = {
  title: string;
  description?: string;
  children: React.ReactNode;
};
```

### `ToggleRow`

```ts
type ToggleRowProps = {
  label: string;
  description?: string;
  checked: boolean;
  disabled?: boolean;
  onChange: (checked: boolean) => void;
};
```

### `SelectRow`

```ts
type SelectRowProps<T extends string> = {
  label: string;
  description?: string;
  value: T;
  options: Array<{ value: T; label: string }>;
  onChange: (value: T) => void;
};
```

### `NavigationRow`

```ts
type NavigationRowProps = {
  label: string;
  description?: string;
  status?: string;
  onActivate: () => void;
};
```

### Requirements

- Entire row may be clickable when it contains one control.
- Toggle label remains clickable.
- Avoid placing a chevron beside a toggle.
- Destructive rows are grouped separately.

---

## 14. `ApiKeyDialog`

### Props

```ts
type ApiKeyDialogProps = {
  provider: ProviderId;
  open: boolean;
  requirementNote?: string;
  submitting: boolean;
  error?: string;
  onSubmit: (secret: string) => Promise<void>;
  onClose: () => void;
};
```

### Requirements

- Secret is not placed in global state.
- Dialog has a descriptive title.
- Escape closes only when not submitting.
- Validation errors do not expose provider response bodies.
- Submit label: `Validate and Save`.

---

## 15. `ReliabilityBadge`

### Values

```text
Official API
Official CLI
Experimental
Dashboard only
```

### Requirements

- Compact text badge.
- Neutral by default.
- Experimental includes an explanatory tooltip.
- It must not visually compete with quota status.

---

## 16. `ResetCountdown`

### Props

```ts
type ResetCountdownProps = {
  resetAt?: string;
  now?: string;
  exactDateLabel?: string;
};
```

### Display examples

```text
Resets in 46m
Resets in 2d 8h
Resets Jul 20, 08:00
Reset time passed
Reset time unavailable
```

### Update frequency

- Less than 1 hour: update each minute.
- One hour or more: update each minute.
- Seconds are not shown in MVP.
- Recompute immediately after wake.
- Do not declare a confirmed reset without refreshed provider data.

---

## 17. Skeleton Components

Required skeletons:

- provider identity;
- usage hero;
- two usage cards;
- footer.

Rules:

- Use skeletons only when no cached content exists.
- Skeleton geometry closely matches final content.
- No pulsing animation when reduced motion is enabled.
- Do not skeleton the fixed app header.
