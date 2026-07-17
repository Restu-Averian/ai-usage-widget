# 3. Subscription vs. API Usage Separation

Date: 2026-07-16

## Status

Accepted

## Context

Many AI providers offer two distinct types of access quotas:

1. **Subscription Usage**: A flat-rate monthly tier (e.g., Claude Pro, ChatGPT Plus) where usage limits are dynamic, undocumented, or based on time windows.
2. **API Usage**: A pay-as-you-go model where usage is strictly tracked in tokens or credits against a pre-funded budget.

Attempting to merge these into a single "usage percentage" creates significant confusion for the end user, as a 100% API consumption does not mean the Subscription is exhausted.

## Decision

We will **strictly separate Subscription usage and API usage** throughout the architecture.

- They will be represented by different domain types and database records.
- They will be presented independently in the UI.
- We will not merge them into a single aggregate percentage or status.
- By default, API budgets will not replace Subscription quota in the primary Menu Bar visual state unless explicitly configured by the user.

## Consequences

- **Pros**:
  - Unambiguous communication of limits to the user.
  - Prevents data corruption and logic bugs by maintaining separate state machines for each usage type.
- **Cons**:
  - UI complexity is slightly higher to display both sources.
  - Backend must handle polling and error boundaries for each source independently.
