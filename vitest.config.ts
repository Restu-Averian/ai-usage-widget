import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    environment: "node", // Will be changed to jsdom/happy-dom for React in M2
    include: ["tests/**/*.test.ts", "src/**/*.test.ts"],
    globals: true,
  },
});
