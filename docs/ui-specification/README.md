# AI Usage Dock — UI Specification

**Document status:** Draft v0.1  
**Project:** AI Usage Dock  
**Primary platforms:** macOS and Windows  
**Future platforms:** Android and iOS  
**UI language for MVP:** English

Related documents:

- `../AI-Usage-Dock-PRD.md`
- `../AI-Usage-Dock-Technical-Spike.md`
- `../AI-Usage-Dock-Technical-Design.md`
- `../milestones/README.md`
- `../../AGENTS.md`

---

## Purpose

This folder defines the user interface, interaction behavior, design system, accessibility requirements, and visual acceptance criteria for AI Usage Dock.

The specification is split into focused documents so a coding agent can read only the files relevant to the active task.

---

## Visual Direction

AI Usage Dock should feel:

- compact;
- calm;
- native-adjacent;
- technical without looking like a developer console;
- trustworthy;
- clear at a glance;
- restrained in its use of provider branding.

The application is a lightweight desktop status surface, not a full analytics dashboard.

---

## Document Map

| File                                     | Purpose                                                             |
| ---------------------------------------- | ------------------------------------------------------------------- |
| `01-product-shell-and-navigation.md`     | Window, navigation, layout, Menu Bar, and System Tray behavior      |
| `02-screen-specifications.md`            | Detailed specification for every screen and state                   |
| `03-provider-state-matrix.md`            | Provider states, transitions, capabilities, and errors              |
| `04-component-specifications.md`         | Reusable component contracts and variants                           |
| `05-design-system.md`                    | Colors, typography, spacing, radius, icons, charts, and density     |
| `06-interactions-and-motion.md`          | Tray behavior, refresh, keyboard interaction, timing, and motion    |
| `07-content-and-copy.md`                 | English UI copy, terminology, error messages, and formatting        |
| `08-accessibility.md`                    | Keyboard, screen reader, contrast, reduced motion, and text scaling |
| `09-platform-and-responsive-behavior.md` | macOS, Windows, compact mode, scaling, and future mobile behavior   |
| `10-ui-acceptance-criteria.md`           | Testable UI acceptance criteria and visual checklist                |
| `11-mock-data-and-story-states.md`       | Normalized mock data and required visual scenarios                  |

---

## Required Reading

Before any UI implementation, read:

1. `README.md`
2. `05-design-system.md`
3. The active task file
4. The screen or component document relevant to the task

Do not load or rewrite every UI document when the task only affects one component.

---

## Canonical Desktop Structure

```text
Menu Bar / System Tray
        ↓
Floating usage popup
├── App header
├── Provider tabs
├── Scrollable provider content
└── Status footer
```

Settings opens inside the same popup for the MVP.

---

## Required Provider States

Every provider must support:

```text
unknown
detecting
not-installed
authentication-required
connecting
connected
refreshing
stale
offline
error
unsupported
```

All states must be implementable with mock data before real provider integration begins.

---

## Core UI Rules

- One clear visual focal point per screen.
- Maximum one filled primary action per state.
- Unknown usage must never appear as `0%`.
- Subscription and API usage must remain visually separate.
- Cached data remains visible while refreshing.
- Stale and offline data must be clearly labeled.
- Provider capabilities determine which connection actions appear.
- Provider colors are used only for identity, not quota status.
- Warning and critical states must not rely only on color.
- Keyboard navigation and reduced motion are mandatory.
- Minimum supported desktop width is 360 logical pixels.
- Compact mode changes layout density, not accessibility target sizes.

---

## Mock-First Rule

The complete interface must be built and reviewed using the scenarios in:

```text
11-mock-data-and-story-states.md
```

Production provider connectors must not require UI components to be redesigned.

---

## Change Control

Do not add, remove, or alter established UI fields, flows, or states without updating the relevant UI Specification file.

Architecture or security changes must also be reflected in the Technical Design or an ADR.
