---
name: screenshot-ui-slicer
description: Slice UI from screenshots/mockups into the current frontend project with close visual fidelity. Use when the user asks to convert, slice, implement, clone, or match a screenshot/design. Inspect the project styling first; do not assume Tailwind or any UI library.
argument-hint: "[target file/page] [screenshot/context]"
---

# Screenshot UI Slicer

## Mission

Implement the attached screenshot/mockup into the existing frontend project with close visual parity and project-native styling.

Visual source of truth: screenshot.  
Technical source of truth: current project.

## Mode

Be direct. Do not over-explain. Do not redesign. Do not introduce a new styling system.

## Workflow

1. Inspect the screenshot.
   - Identify layout, spacing, typography, colors, radius, shadows, images, icons, hierarchy, and responsive intent.

2. Inspect the project before coding.
   - Check package.json, existing components, pages, styles, theme, UI library, assets, aliases, and naming conventions.

3. Detect the styling approach.
   - Tailwind, Ant Design, Chakra, MUI, shadcn/ui, CSS Modules, SCSS, vanilla CSS, styled-components, emotion, or custom design system.
   - Use the dominant existing approach only.

4. Implement.
   - Reuse existing components/assets first.
   - Match screenshot spacing, alignment, size, font weight, color, radius, shadow, crop, and hierarchy.
   - Keep code clean, maintainable, accessible, and responsive.
   - Avoid unrelated refactors.

5. Verify.
   - Run available lint/build/typecheck when reasonable.
   - If preview/browser is available, render and compare against the screenshot.
   - Fix obvious visual mismatches.

## Styling Rules

- If Tailwind exists: use existing Tailwind patterns/config.
- If Ant Design exists: use AntD components and project override pattern.
- If Chakra/MUI exists: use theme-aware components/props.
- If CSS Modules/SCSS/vanilla CSS exists: use scoped semantic classes.
- If styled-components/emotion exists: follow existing styled patterns.
- If mixed: follow the target area’s existing convention.

## Hard Rules

- Do not assume Tailwind.
- Do not add a UI library.
- Do not redesign the screenshot.
- Do not create generic AI-looking UI.
- Do not modify unrelated files.
- Do not use fake assets when existing assets fit.
- Do not leave unused imports or console logs.

## Output

Return only:

- styling system detected
- files changed
- what was matched
- checks run
- remaining mismatch, if any
