---
name: restu-fe-fix
description: Restu's compact frontend fix workflow for React/Vite/JS/JSX. User provides focus/case/expected only. Apply focused inspection, minimal safe edit, existing helper reuse, relevant error handling, maintainable hook order, useful JSDoc, RTK-first commands, and concise Indonesian final answer.
---

# Restu FE Fix

Default: fast, focused, low-token.

## Input

User provides:

focus on src/path/file.jsx
case: current issue
expected: target behavior

Rules are built in. Do not ask user to repeat rules.

## Workflow

1. Inspect `focus on` file first.
2. Inspect related imports/helpers only when needed.
3. Identify likely root cause briefly.
4. Fix only what is needed for `expected`.
5. Preserve unrelated behavior.
6. Verify with smallest useful command.
7. Answer short.

Do not scan whole repo, refactor unrelated code, install deps, paste long logs, or touch `src/js-toolkit` unless asked.

## Coding rules

- Reuse existing helper/component/pattern before creating new one.
- Handle only relevant risks: null, undefined, empty array, invalid input, missing nested data, failed response, loading state, unsafe optional data, fallback display.
- Keep code simple, readable, and maintainable.
- Avoid duplicate logic and deep nesting.
- Use `useMemo` only for meaningful derived values.
- Use `useCallback` only when useful or matching existing pattern.
- Keep complex JSX logic above return.
- Remove unused imports.
- Keep JS/JSX as JS/JSX unless file is already TS/TSX.

## React order

When editing component logic, prefer:

1. useGetDictionary
2. useState
3. useMemo
4. useCallback
5. useEffect
6. local helpers / handlers
7. return JSX

Do not reorder unrelated code just for order.

## JSDoc

Add JSDoc only for new helper, complex function, custom hook, or non-obvious edge-case logic.  
Do not add noisy JSDoc for simple state, simple callback, trivial condition, or obvious JSX.

## RTK / commands

When shell is needed, verify RTK once:

command -v rtk && rtk --version

Prefer RTK:

rtk git status
rtk git diff
rtk git diff --staged
rtk rg "keyword"
rtk sed -n '1,220p' path/to/file
rtk npm run lint
rtk npm run build
rtk npm test
rtk pnpm lint
rtk pnpm build
rtk pnpm test

Avoid huge output:

cat large-file
find .
grep -R "keyword" .
git diff
npm test
npm run build

Use focused alternatives:

sed -n '1,220p' path/to/file
rg "keyword" path/
git diff -- path/to/file

## Final answer

Use Indonesian casual style.

Format:

Done bre.

Changed:

- ...

Files:

- [file-name.jsx](src/path/file-name.jsx)

Verify:

- ...

Note:

- ...

Keep it short. Include Note only if needed.
