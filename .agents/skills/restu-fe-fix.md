# Skill: Restu FE Fix

Use this skill when the user provides:

focus on src/path/file.jsx
case: current issue
expected: target behavior

## Rules

- Inspect `focus on` file first.
- Inspect related imports/helpers only when needed.
- Do not scan whole repo.
- Make minimal safe edit.
- Preserve unrelated behavior.
- Reuse existing helper/component/pattern before creating new one.
- Handle only relevant risks:
  - null
  - undefined
  - empty array
  - invalid input
  - missing nested data
  - failed response
  - loading state
  - unsafe optional data
  - fallback display
- Keep code simple and maintainable.
- Avoid duplicate logic and deep nesting.
- Use `useMemo` only for meaningful derived values.
- Use `useCallback` only when useful or matching repo pattern.
- Keep complex JSX logic above return.
- Remove unused imports.
- Add JSDoc only for new helper, complex function, custom hook, or non-obvious edge-case logic.

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

## Commands

If shell is needed and RTK exists, prefer:

rtk git status
rtk git diff
rtk rg "keyword"
rtk sed -n '1,220p' path/to/file
rtk npm run lint
rtk npm run build
rtk pnpm lint
rtk pnpm build

Avoid huge output:

cat large-file
find .
grep -R "keyword" .
git diff
npm test
npm run build

## Final answer

Use Indonesian casual style.

Format:

Done bre.

Changed:
- ...

Files:
- ...

Verify:
- ...

Note:
- ...

Keep it short. Include Note only if needed.
