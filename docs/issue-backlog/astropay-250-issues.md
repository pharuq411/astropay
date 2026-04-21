# ASTROpay 250-Issue Backlog

This backlog is scoped against the code that actually exists in this repo.

That matters because the dangerous lie here would be pretending the Rust migration is done. It is not. The Next.js app still owns buyer checkout XDR build and settlement execution, while Rust owns only part of the backend surface.

Each issue below is written to be contributor-friendly:

- every item has a unique ID
- every item has labels
- every item has a clear done condition
- every item respects the current Next.js to Rust transition instead of hand-waving it away

## Label taxonomy

- `area:frontend`
- `area:backend-rust`
- `area:stellar`
- `area:database`
- `area:testing`
- `area:observability`
- `area:infrastructure`
- `area:docs`
- `area:security`
- `type:feature`
- `type:bug`
- `type:refactor`
- `type:test`
- `type:docs`
- `type:ops`
- `type:performance`
- `difficulty:starter`
- `difficulty:intermediate`
- `difficulty:advanced`

## Frontend Checkout And Invoice UX

Relevant code:
- `usdc-payment-link-tool/app/pay/[publicId]/page.tsx`
- `usdc-payment-link-tool/components/PayWithFreighter.tsx`
- `usdc-payment-link-tool/components/InvoiceCreateForm.tsx`
- `usdc-payment-link-tool/app/(dashboard)/dashboard/invoices/[id]/page.tsx`

### AP-001 Clarify checkout amount breakdown on hosted payment page
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The public checkout page explains gross amount, expiry, and fee handling without implying the payer is sending funds directly to the merchant.

### AP-002 Add countdown timer for invoice expiry on public checkout
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The pay page shows a live expiry countdown and flips cleanly to an expired state when the deadline passes.

### AP-003 Show memo and destination details in a collapsible metadata panel
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Advanced payment metadata can be revealed for debugging without cluttering the default checkout experience.

### AP-004 Surface transaction submission progress states in the Freighter flow
- Labels: `area:frontend`, `type:feature`, `difficulty:intermediate`
- Done when: Checkout exposes distinct states for connect, XDR requested, signed, submitted, and confirmed or failed.

### AP-005 Prevent duplicate pay attempts from repeated button clicks
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Repeated clicks cannot trigger duplicate checkout requests while one attempt is already in flight.

### AP-006 Add retry action for recoverable wallet and network failures
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Recoverable checkout failures show a retry path that resets only the failed step instead of forcing a full reload.

### AP-007 Render a dedicated failed payment state instead of generic error text
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Payment failures have a clear state view with actionable copy instead of a raw exception-like message.

### AP-008 Add pending-settlement explainer after payment detection
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The UI distinguishes invoice payment detection from merchant settlement so contributors stop conflating those states.

### AP-009 Improve mobile layout for checkout QR and wallet CTA
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: The pay screen is readable and tappable on small screens with no clipped QR, overlapping copy, or hidden CTA.

### AP-010 Add empty-state copy for the expired invoice view
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Expired invoices render purpose-built guidance instead of falling through to a generic failure path.

### AP-011 Add skeleton loading state while public invoice data is fetched
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The public pay route shows an intentional loading state instead of layout jump or blank content.

### AP-012 Add merchant branding block to hosted checkout without breaking trust cues
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The pay page can display merchant business identity while still emphasizing that payment lands in the platform treasury.

### AP-013 Add copy-to-clipboard action for invoice reference data
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Users can copy invoice ID, public ID, and memo values from invoice detail views without manual text selection.

### AP-014 Add copy payment URI action on hosted checkout
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The checkout view exposes a copyable Stellar payment URI for wallet and support workflows.

### AP-015 Run an accessibility pass on the pay page interactions
- Labels: `area:frontend`, `type:bug`, `difficulty:intermediate`
- Done when: Keyboard navigation, focus order, button labels, and status announcements pass a basic accessibility review.

### AP-016 Normalize currency formatting across dashboard and public invoice views
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: All invoice amounts render with the same USD formatting rules and do not drift between pages.

### AP-017 Truncate Stellar public keys in UI with full-value reveal affordance
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Long keys are readable in tight layouts while full values remain accessible through copy or expand interactions.

### AP-018 Add explicit visual badge mapping for invoice states
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: `pending`, `paid`, `expired`, `settled`, and `failed` states use consistent visual treatment across the app.

### AP-019 Add keyboard-first navigation for primary checkout actions
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: A user can complete all non-wallet checkout interactions without relying on a mouse or touch.

### AP-020 Improve the not-found state for missing public invoices
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Missing invoice routes explain whether the invoice was never valid, expired, or removed, without exposing internals.

### AP-021 Add dashboard filters for invoice state
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can filter invoices by lifecycle state from the dashboard rather than scanning a flat list.

### AP-022 Add dashboard sorting controls for invoice date and amount
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can sort invoice lists predictably by creation time and value.

### AP-023 Add an activity timeline to the invoice detail page
- Labels: `area:frontend`, `type:feature`, `difficulty:intermediate`
- Done when: Invoice detail surfaces a human-readable timeline of creation, payment detection, expiry, and settlement milestones.

### AP-024 Add pagination or incremental loading to the dashboard invoice list
- Labels: `area:frontend`, `type:feature`, `difficulty:intermediate`
- Done when: The dashboard can handle more than the first page of invoices without loading an unbounded list.

### AP-025 Add a copy hosted-link action in merchant dashboard views
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can copy the public payment link directly from the dashboard and invoice detail screens.

### AP-026 Regenerate QR image on demand without reloading the page
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: QR refresh is possible from the invoice detail surface without forcing a navigation cycle.

### AP-027 Add invoice duplicate action for repeat billing
- Labels: `area:frontend`, `type:feature`, `difficulty:intermediate`
- Done when: Merchants can prefill a new invoice from an existing one without copying fields manually.

### AP-028 Allow safe invoice edits before any payment attempt exists
- Labels: `area:frontend`, `type:feature`, `difficulty:advanced`
- Done when: Draft invoice fields can be edited before payment activity starts and the resulting state transitions remain coherent.

### AP-029 Improve redirect behavior after login to preserve intended destination
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Auth-required flows return merchants to the page they actually wanted after successful login.

### AP-030 Preserve intended destination across register and login screens
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Both auth screens preserve target navigation instead of always dumping the merchant on a default route.

### AP-031 Normalize unauthorized versus not-found routing behavior
- Labels: `area:frontend`, `type:bug`, `difficulty:intermediate`
- Done when: Route guards do not leak resource existence and the user sees the right screen for auth versus existence failures.

### AP-032 Stop stale invoice status polling after page navigation
- Labels: `area:frontend`, `type:bug`, `difficulty:intermediate`
- Done when: Leaving an invoice or checkout view tears down polling cleanly and prevents orphaned client requests.

### AP-033 Show last-updated timestamp on invoice detail
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can see when invoice data was last refreshed or changed.

### AP-034 Add manual refresh control on invoice detail
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can trigger an explicit status refresh without reloading the entire dashboard.

### AP-035 Show success confirmation after invoice creation redirect
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The post-create flow confirms invoice creation clearly instead of relying on silent navigation.

### AP-036 Add inline validation feedback in the invoice create form
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Description and amount errors are shown next to the relevant fields before the request is sent.

### AP-037 Prevent zero and negative amounts in the UI before submit
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: The invoice form blocks invalid amount values client-side without relying only on API rejection.

### AP-038 Trim and normalize invoice descriptions before submit
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Leading and trailing whitespace is removed and obviously malformed descriptions are normalized consistently.

### AP-039 Add a character counter for invoice description length
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can see how close the description is to backend limits while typing.

### AP-040 Display settlement-status badge in the dashboard list
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can distinguish `paid` from `settled` at list level without opening each invoice.

## Frontend Reliability And Contributor Docs

Relevant code:
- `usdc-payment-link-tool/lib/http.ts`
- `usdc-payment-link-tool/lib/data.ts`
- `usdc-payment-link-tool/README.md`
- `usdc-payment-link-tool/CODEX_PROMPTS.md`

### AP-041 Centralize client-side API error mapping
- Labels: `area:frontend`, `type:refactor`, `difficulty:intermediate`
- Done when: The UI uses one shared error-mapping layer instead of scattering bespoke string handling across components.

### AP-042 Standardize route-handler JSON error shape in the Next.js app
- Labels: `area:frontend`, `type:refactor`, `difficulty:intermediate`
- Done when: Next.js route handlers return a predictable error contract that frontend components can consume safely.

### AP-043 Extract invoice-status polling into a reusable hook
- Labels: `area:frontend`, `type:refactor`, `difficulty:intermediate`
- Done when: Polling logic is centralized, typed, and reusable instead of being embedded in page-specific code.

### AP-044 Stop polling once an invoice reaches a terminal state
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Client polling halts automatically after `expired`, `failed`, or `settled`.

### AP-045 Add backoff to invoice status polling
- Labels: `area:frontend`, `type:performance`, `difficulty:intermediate`
- Done when: The client reduces polling intensity after repeated unchanged responses or transient network failures.

### AP-046 Redirect merchants to login when API responses return 401
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Merchant-only UI surfaces do not leave the user in a broken state after an expired session.

### AP-047 Distinguish network timeouts from validation errors in UI copy
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Error messaging reflects whether the failure was client input, server rejection, or upstream connectivity.

### AP-048 Add a frontend architecture map to the app README
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: Contributors can see which pages, components, and route handlers own each major flow.

### AP-049 Document the invoice lifecycle model in frontend docs
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: Frontend contributors stop inventing state names and understand the existing invoice and payout lifecycles.

### AP-050 Add a route ownership matrix for legacy backend behavior in Next.js
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: The README explains which backend responsibilities still live in Next.js and why.

### AP-051 Add a screenshot-driven UI walkthrough for merchant flows
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: New contributors can see the intended auth, invoice creation, and hosted checkout path before editing UI code.

### AP-052 Add an architecture diagram that shows Next.js and Rust boundaries
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: The repo includes a visual of the current split instead of leaving contributors to infer it from file names.

### AP-053 Document the public checkout sequence step by step
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: Docs explain the current buyer path from invoice open to signed XDR submission and later reconciliation.

### AP-054 Document the merchant dashboard invoice lifecycle
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: Merchants and contributors can map dashboard states to backend state transitions without guesswork.

### AP-055 Document local Freighter test setup
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: Contributors have explicit setup steps for running wallet-dependent flows locally.

### AP-056 Add a troubleshooting guide for common checkout failures
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: The repo documents how to diagnose wallet, Horizon, env, and memo-matching failures.

### AP-057 Document frontend-only environment variables and why they exist
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: Contributors can distinguish server-only secrets from browser-safe variables.

### AP-058 Add a copy style guide for payment state messaging
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: UI wording for `pending`, `paid`, `expired`, `settled`, and `failed` becomes deliberate and consistent.

### AP-059 Add a contributor checklist for editing Next.js route handlers
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: Contributors have a short guardrail list covering auth, validation, DB access, and error handling.

### AP-060 Add a label-and-difficulty guide for frontend issues
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: The repo explains how `starter`, `intermediate`, and `advanced` should be applied to frontend work items.

### AP-061 Audit UI copy that still implies direct-to-merchant payment
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: No user-facing copy contradicts the treasury-first custody model.

### AP-062 Improve `not-found.tsx` so missing invoices and missing routes are not conflated
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: The app renders sharper not-found messaging for invoice-related misses without exposing internals.

### AP-063 Improve loading states on the dashboard invoice list
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Dashboard loading feels intentional instead of flashing between blank and full states.

### AP-064 Add an empty state for merchants with zero invoices
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: The dashboard teaches a first-time merchant what to do next instead of rendering dead space.

### AP-065 Prevent stale cached invoice detail after creation redirect
- Labels: `area:frontend`, `type:bug`, `difficulty:intermediate`
- Done when: Opening a newly created invoice after redirect always shows fresh persisted data.

### AP-066 Add manual refetch action after delayed webhook or reconciliation
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Merchants can explicitly request a refresh when they know a status change may be imminent.

### AP-067 Make login and register error messages field-specific
- Labels: `area:frontend`, `type:bug`, `difficulty:starter`
- Done when: Auth forms highlight the failing field instead of collapsing everything into one generic error banner.

### AP-068 Add password visibility toggles with accessible labels
- Labels: `area:frontend`, `type:feature`, `difficulty:starter`
- Done when: Auth forms support password reveal without breaking keyboard and screen-reader behavior.

### AP-069 Add a frontend test-gap section to the README
- Labels: `area:docs`, `type:docs`, `difficulty:starter`
- Done when: The frontend docs list the highest-risk untested flows instead of pretending the app is already covered.

### AP-070 Document the migration policy for moving route handlers into Rust
- Labels: `area:docs`, `type:docs`, `difficulty:intermediate`
- Done when: Contributors know the decision criteria for keeping logic in Next.js versus porting it to Axum.

## Rust Auth API And Security

Relevant code:
- `rust-backend/src/main.rs`
- `rust-backend/src/handlers/auth.rs`
- `rust-backend/src/auth.rs`
- `rust-backend/src/error.rs`
- `rust-backend/src/config.rs`

### AP-071 Replace ad hoc unauthorized strings with a consistent Rust API error contract
- Labels: `area:backend-rust`, `area:security`, `type:refactor`, `difficulty:intermediate`
- Done when: Auth failures return one machine-readable shape and callers no longer parse arbitrary message text.

### AP-072 Add contract tests for `GET /api/auth/me`
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:starter`
- Done when: The service has explicit tests for authenticated, unauthenticated, and malformed-cookie `me` requests.

### AP-073 Add session refresh logic before long-lived cookies silently expire
- Labels: `area:backend-rust`, `area:security`, `type:feature`, `difficulty:advanced`
- Done when: Valid active sessions can be refreshed deliberately without widening attack surface or breaking logout semantics.

### AP-074 Add a scheduled cleanup path for expired sessions
- Labels: `area:backend-rust`, `area:database`, `type:ops`, `difficulty:starter`
- Done when: Expired session rows stop accumulating forever and cleanup logic is documented and testable.

### AP-075 Add login rate limiting in the Rust auth flow
- Labels: `area:security`, `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Repeated login attempts are throttled without locking out legitimate merchants behind routine mistakes.

### AP-076 Normalize emails on registration and login
- Labels: `area:backend-rust`, `type:bug`, `difficulty:starter`
- Done when: Email comparisons are case-safe and the same merchant cannot register equivalent mixed-case addresses.

### AP-077 Reject duplicate merchant and settlement key combinations on registration
- Labels: `area:security`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Registration fails cleanly when incoming wallet assignments would create ambiguous merchant ownership.

### AP-078 Add password policy enforcement in Rust
- Labels: `area:security`, `area:backend-rust`, `type:feature`, `difficulty:starter`
- Done when: The service enforces a minimum viable password policy and documents the rule.

### AP-079 Add tests for secure-cookie flag behavior
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:starter`
- Done when: Session cookie tests cover secure and non-secure environments without guesswork.

### AP-080 Review cookie-backed auth flow for CSRF exposure
- Labels: `area:security`, `area:backend-rust`, `type:docs`, `difficulty:advanced`
- Done when: The repo contains a concrete CSRF assessment and any required mitigations are implemented or explicitly deferred.

### AP-081 Add request ID middleware to the Axum stack
- Labels: `area:backend-rust`, `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Every Rust request has a correlation ID that is logged and returned when appropriate.

### AP-082 Add structured auth audit logging in Rust
- Labels: `area:backend-rust`, `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Login, logout, and registration success or failure paths emit useful structured audit events without leaking secrets.

### AP-083 Separate public invoice endpoints from merchant-only endpoints explicitly
- Labels: `area:backend-rust`, `type:refactor`, `difficulty:intermediate`
- Done when: Route organization makes it obvious which handlers require auth and which are safe for public checkout use.

### AP-084 Add idempotency-key support for invoice creation in Rust
- Labels: `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Retried merchant creates do not silently produce duplicate invoices.

### AP-085 Replace float-based invoice amount parsing with decimal-safe handling
- Labels: `area:backend-rust`, `type:bug`, `difficulty:advanced`
- Done when: Invoice amounts are parsed and stored without float rounding risk in request handling.

### AP-086 Return machine-readable error codes from the Rust API
- Labels: `area:backend-rust`, `type:refactor`, `difficulty:intermediate`
- Done when: Clients can branch on stable error codes instead of brittle message strings.

### AP-087 Add API contract documentation for Rust routes
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:intermediate`
- Done when: The Rust service documents request and response shapes for every implemented endpoint.

### AP-088 Add a dedicated Axum extractor for the authenticated merchant
- Labels: `area:backend-rust`, `type:refactor`, `difficulty:intermediate`
- Done when: Authenticated handlers stop repeating merchant lookup and unauthorized handling boilerplate.

### AP-089 Add a shared error-to-response mapper for the Rust service
- Labels: `area:backend-rust`, `type:refactor`, `difficulty:intermediate`
- Done when: Error translation is centralized and new handlers cannot accidentally invent their own response shapes.

### AP-090 Add graceful handling for malformed UUID path parameters
- Labels: `area:backend-rust`, `type:bug`, `difficulty:starter`
- Done when: Bad UUID inputs return predictable client errors instead of noisy framework failures.

### AP-091 Add explicit authorization tests for foreign invoice access
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:starter`
- Done when: Every merchant-only invoice handler proves it rejects access to another merchant's records.

### AP-092 Add groundwork for session revocation on password change
- Labels: `area:security`, `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: The model and auth flow can invalidate outstanding sessions after sensitive account changes.

### AP-093 Validate required secrets at Rust boot time with actionable errors
- Labels: `area:backend-rust`, `area:security`, `type:bug`, `difficulty:starter`
- Done when: Missing or invalid secrets fail fast with useful startup diagnostics.

### AP-094 Review cron authorization parsing for edge cases and header spoofing
- Labels: `area:security`, `area:backend-rust`, `type:bug`, `difficulty:intermediate`
- Done when: Cron auth rejects malformed headers predictably and the logic is covered by tests.

### AP-095 Support secure cookie behavior behind reverse proxies
- Labels: `area:backend-rust`, `area:security`, `type:feature`, `difficulty:advanced`
- Done when: Cookie security decisions can account for trusted proxy headers without weakening local development.

### AP-096 Add database timeout handling for slow Rust queries
- Labels: `area:backend-rust`, `area:database`, `type:performance`, `difficulty:intermediate`
- Done when: Slow queries fail in a controlled way and logs make the timeout cause obvious.

### AP-097 Add per-route tracing spans to Rust handlers
- Labels: `area:observability`, `area:backend-rust`, `type:feature`, `difficulty:starter`
- Done when: Each Rust handler emits a named tracing span with enough metadata to diagnose slow or failing requests.

### AP-098 Upgrade `/healthz` to include dependency readiness checks
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Health probes can optionally confirm DB and config readiness instead of only process liveness.

### AP-099 Split the main Axum router into feature-specific subrouters
- Labels: `area:backend-rust`, `type:refactor`, `difficulty:intermediate`
- Done when: `main.rs` stops owning all route wiring and the service layout is easier to evolve.

### AP-100 Add an API versioning strategy note before route growth becomes chaotic
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:starter`
- Done when: The repo states how route versioning will be handled before external clients depend on unstable shapes.

### AP-101 Emit audit events for login success and failure
- Labels: `area:backend-rust`, `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Login outcomes are written to an auditable trail without logging raw secrets.

### AP-102 Emit audit events for invoice creation in Rust
- Labels: `area:backend-rust`, `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Invoice creation can be traced to a merchant and timestamp for later diagnostics.

### AP-103 Track unauthorized invoice-access attempts
- Labels: `area:security`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: The service records rejected cross-merchant access attempts for monitoring and investigation.

### AP-104 Document Rust service ownership boundaries
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:starter`
- Done when: Contributors can see exactly which responsibilities Rust owns today and which still belong to Next.js.

### AP-105 Add a security assumptions document for the Rust backend
- Labels: `area:docs`, `area:security`, `type:docs`, `difficulty:intermediate`
- Done when: The repo spells out trust boundaries, secret handling assumptions, and areas that still need review.

## Rust Checkout And Invoice Parity

Relevant code:
- `rust-backend/src/handlers/invoices.rs`
- `rust-backend/src/stellar.rs`
- `usdc-payment-link-tool/app/api/invoices/[id]/checkout/route.ts`
- `usdc-payment-link-tool/lib/stellar.ts`

### AP-106 Implement Rust checkout `build-xdr` mode
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: `POST /api/invoices/{id}/checkout` can build a buyer payment XDR that matches current frontend expectations.

### AP-107 Implement Rust checkout `submit-xdr` mode
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: The Rust checkout route can accept a signed XDR, submit it to Horizon, and return a safe response contract.

### AP-108 Validate payer public-key format in the Rust checkout route
- Labels: `area:backend-rust`, `area:stellar`, `type:bug`, `difficulty:starter`
- Done when: Bad payer keys are rejected with a client-safe error before any network call is made.

### AP-109 Reject checkout for non-pending invoices
- Labels: `area:backend-rust`, `type:bug`, `difficulty:starter`
- Done when: Rust checkout refuses invoices that are already paid, expired, settled, or failed.

### AP-110 Reject checkout for expired invoices
- Labels: `area:backend-rust`, `type:bug`, `difficulty:starter`
- Done when: Expired invoices cannot generate or submit new payment XDRs from the Rust route.

### AP-111 Block checkout when invoice payment fields changed since the client loaded the page
- Labels: `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: The checkout path detects stale client context for memo, amount, asset, or destination changes.

### AP-112 Add idempotency guard for signed XDR submission
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: Replayed signed submissions cannot create ambiguous duplicate processing in Rust.

### AP-113 Persist buyer public key for submitted checkout attempts
- Labels: `area:backend-rust`, `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: The system can tell which buyer account attempted which checkout submission.

### AP-114 Record checkout attempts in `payment_events`
- Labels: `area:backend-rust`, `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: Checkout build and submit milestones create audit events that can be traced later.

### AP-115 Add tests for memo collision handling
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:intermediate`
- Done when: The Rust checkout path proves it cannot silently reuse a memo across active invoices.

### AP-116 Port checkout URL and QR generation helpers into Rust
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Rust can own invoice checkout URL and QR generation without relying on frontend-only helpers.

### AP-117 Extract shared Stellar amount-formatting helpers in Rust
- Labels: `area:backend-rust`, `area:stellar`, `type:refactor`, `difficulty:starter`
- Done when: Amount conversion rules are centralized and covered by unit tests.

### AP-118 Return the network passphrase from Rust checkout responses
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:starter`
- Done when: Rust checkout responses provide the passphrase the wallet needs, matching the existing contract.

### AP-119 Normalize Stellar SDK errors into safe API responses
- Labels: `area:backend-rust`, `area:stellar`, `type:bug`, `difficulty:intermediate`
- Done when: Upstream Stellar errors are translated into actionable but non-sensitive client responses.

### AP-120 Add unit tests for invoice amount-to-asset formatting in Rust
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:starter`
- Done when: Rust proves that invoice cents map to exact asset strings for checkout and reconciliation.

### AP-121 Add contract tests proving Rust checkout matches the Next.js response shape
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:advanced`
- Done when: A parity suite fails if Rust checkout response fields diverge from the current frontend integration contract.

### AP-122 Add a feature flag to route checkout traffic between Next.js and Rust
- Labels: `area:infrastructure`, `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Checkout ownership can be shifted gradually instead of requiring an unsafe full cutover.

### AP-123 Document fallback strategy for a partial Rust checkout rollout
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:starter`
- Done when: The repo defines what happens if Rust checkout fails and traffic must move back to Next.js temporarily.

### AP-124 Support public invoice lookup by `public_id` in Rust
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Rust can serve the public checkout flow using the invoice identifier the frontend actually exposes.

### AP-125 Add a dedicated Rust handler for public invoice details
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Rust exposes the public invoice data needed by the pay page without leaking merchant-only fields.

### AP-126 Expose QR data URL from the Rust invoice creation path
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Merchants creating invoices through Rust receive the same QR support the Next.js path provides today.

### AP-127 Add typed request and response models for checkout modes
- Labels: `area:backend-rust`, `type:refactor`, `difficulty:intermediate`
- Done when: Rust checkout no longer depends on loosely shaped JSON bodies.

### AP-128 Reject oversized memos before transaction build
- Labels: `area:backend-rust`, `area:stellar`, `type:bug`, `difficulty:starter`
- Done when: Rust validates memo size upfront and fails clearly before invoking Stellar transaction builders.

### AP-129 Validate invoice asset config against service config before checkout
- Labels: `area:backend-rust`, `area:stellar`, `type:bug`, `difficulty:intermediate`
- Done when: Rust refuses to build checkout transactions if invoice asset fields are incompatible with service configuration.

### AP-130 Add a Rust integration test against Stellar testnet for XDR build
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:advanced`
- Done when: The service proves it can produce valid payment XDRs against the configured network assumptions.

### AP-131 Add a Rust integration test for signed XDR submission
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:advanced`
- Done when: Rust can submit a signed transaction in a controlled test flow and record the result correctly.

### AP-132 Add replay protection for duplicate transaction-hash callbacks
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: The service handles duplicate post-submit notifications without corrupting invoice state.

### AP-133 Store the last checkout-attempt timestamp on invoices
- Labels: `area:database`, `area:backend-rust`, `type:feature`, `difficulty:starter`
- Done when: The system can show when a buyer last attempted checkout for support and fraud analysis.

### AP-134 Expose backend-owner metadata for checkout during migration
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Operators can tell whether a given checkout request was served by Next.js or Rust.

### AP-135 Replace the Rust checkout placeholder response with a migration-warning header until parity lands
- Labels: `area:backend-rust`, `type:feature`, `difficulty:starter`
- Done when: The current `501` path becomes more actionable for integrators without pretending the feature exists.

### AP-136 Add Rust-side API docs for checkout caveats and temporary gaps
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:starter`
- Done when: The backend README explains the current checkout limitations and planned parity milestones.

### AP-137 Validate that invoice destination matches the platform treasury before building checkout XDRs
- Labels: `area:security`, `area:backend-rust`, `type:bug`, `difficulty:intermediate`
- Done when: Rust refuses to create XDRs for invoices whose destination violates the custody architecture.

### AP-138 Add compatibility notes for the frontend team before wiring Rust public pay endpoints
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:starter`
- Done when: Frontend contributors know what fields and behaviors must remain stable during the migration.

### AP-139 Add structured tracing around the checkout lifecycle in Rust
- Labels: `area:observability`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Rust checkout logs enough milestones to debug build, sign, submit, and response failures.

### AP-140 Record the Horizon transaction URL after successful submission
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:starter`
- Done when: Successful submissions retain a linkable Horizon reference for operator debugging.

### AP-141 Support optional client correlation IDs in checkout requests
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: A frontend-generated correlation ID can flow through the Rust checkout path into logs and events.

### AP-142 Emit dead-letter events when checkout submission fails after signing
- Labels: `area:backend-rust`, `area:observability`, `type:feature`, `difficulty:advanced`
- Done when: Post-signing failures are preserved for later investigation instead of disappearing into transient logs.

### AP-143 Document parity checkpoints required before removing the Next.js checkout route
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:starter`
- Done when: The repo states the non-negotiable tests and behaviors Rust must match before cutover.

### AP-144 Add a smoke script for exercising Rust checkout locally
- Labels: `area:infrastructure`, `area:backend-rust`, `type:ops`, `difficulty:intermediate`
- Done when: Contributors can run a local smoke flow against Rust checkout without manually stitching every request.

### AP-145 Link checkout-attempt records to later reconciliation results
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: Operators can correlate an attempted checkout with the eventual detected on-chain payment or failure.

## Settlement Reconciliation And Webhooks

Relevant code:
- `rust-backend/src/handlers/cron.rs`
- `rust-backend/src/stellar.rs`
- `usdc-payment-link-tool/app/api/cron/reconcile/route.ts`
- `usdc-payment-link-tool/app/api/cron/settle/route.ts`
- `usdc-payment-link-tool/app/api/webhooks/stellar/route.ts`

### AP-146 Implement Rust settlement cron transaction builder
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: Rust can build merchant payout transactions from queued payout records using the treasury account.

### AP-147 Implement Rust settlement submission with treasury signing
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: The Rust settle route can sign and submit payout transactions safely when the treasury secret is present.

### AP-148 Split payout states into submitted and settled based on chain evidence
- Labels: `area:backend-rust`, `area:database`, `type:feature`, `difficulty:advanced`
- Done when: Rust does not mark a payout as settled before the chain evidence actually supports that conclusion.

### AP-149 Mark payouts submitted before later reconciliation confirms settlement
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: The payout lifecycle preserves the distinction between network submission and confirmed settlement.

### AP-150 Add retry backoff policy for failed settlements
- Labels: `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Repeated payout failures back off deliberately instead of hammering the same broken path endlessly.

### AP-151 Add dead-letter handling for repeatedly failed payouts
- Labels: `area:backend-rust`, `area:observability`, `type:feature`, `difficulty:advanced`
- Done when: Chronic settlement failures move into an operator-visible dead-letter path instead of remaining in an ambiguous loop.

### AP-152 Validate settlement destination public keys when payouts are queued
- Labels: `area:security`, `area:backend-rust`, `type:bug`, `difficulty:intermediate`
- Done when: Payout records cannot be queued with malformed or obviously invalid destination keys.

### AP-153 Reject settlement execution when the treasury signing key is missing
- Labels: `area:security`, `area:backend-rust`, `type:bug`, `difficulty:starter`
- Done when: The settle route fails fast with an actionable message and never pretends payout execution succeeded.

### AP-154 Add idempotent guard around payout processing loops
- Labels: `area:backend-rust`, `area:database`, `type:feature`, `difficulty:advanced`
- Done when: Concurrent or repeated settle runs cannot process the same payout twice.

### AP-155 Record payout attempt count and last failure reason
- Labels: `area:database`, `area:backend-rust`, `type:feature`, `difficulty:starter`
- Done when: Operators can see how many times a payout was attempted and why it last failed.

### AP-156 Define settlement memo strategy and test it
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:intermediate`
- Done when: Settlement memos are deterministic, documented, and validated against Stellar length constraints.

### AP-157 Reconcile submitted payouts from Horizon by transaction hash
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: Rust can confirm or fail submitted payouts by querying chain state rather than trusting submission alone.

### AP-158 Add webhook secret rotation support
- Labels: `area:security`, `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Webhook auth can rotate secrets safely without forcing downtime or blind trust in one static token.

### AP-159 Make the webhook handler idempotent by transaction hash
- Labels: `area:backend-rust`, `area:stellar`, `type:bug`, `difficulty:intermediate`
- Done when: Duplicate webhook deliveries cannot mutate invoice state twice.

### AP-160 Store raw webhook payloads with source metadata
- Labels: `area:database`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Operators can inspect webhook payloads, source, and arrival time for audit and debugging.

### AP-161 Add replay detection window for webhook deliveries
- Labels: `area:security`, `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: The service can identify suspicious duplicate webhook deliveries within a defined replay window.

### AP-162 Add webhook-to-invoice correlation metrics
- Labels: `area:observability`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Operators can measure how often webhook deliveries resolve invoices versus producing misses or mismatches.

### AP-163 Expand reconciliation scanning beyond a fixed 100 invoices
- Labels: `area:backend-rust`, `area:performance`, `type:feature`, `difficulty:intermediate`
- Done when: Reconciliation can process backlogs larger than the current arbitrary cap without manual babysitting.

### AP-164 Add checkpoint cursor support for Horizon reconciliation
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: Reconciliation can resume from a known checkpoint instead of resweeping the same history blindly.

### AP-165 Handle Horizon rate limiting with backoff and observability
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: Rate-limited scans back off safely and logs make throttling obvious.

### AP-166 Handle Horizon partial outages without flipping invoices to failed
- Labels: `area:backend-rust`, `area:stellar`, `type:bug`, `difficulty:advanced`
- Done when: Upstream outages do not corrupt invoice state or create fake terminal failures.

### AP-167 Distinguish expired invoices from unpaid invoices that may still settle late
- Labels: `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: The system preserves the difference between true expiry and late-arriving chain evidence.

### AP-168 Add a late-payment exception flow for payments detected after expiry
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:advanced`
- Done when: Late payments create an explicit exception path instead of disappearing or silently mutating invoice history.

### AP-169 Emit mismatch event when memo matches but amount differs
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:intermediate`
- Done when: Reconciliation records amount mismatches instead of treating them as invisible misses.

### AP-170 Emit mismatch event when amount matches but asset differs
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:intermediate`
- Done when: Asset mismatches are captured as explicit operator-visible events.

### AP-171 Emit mismatch event when destination matches but memo is missing or wrong
- Labels: `area:backend-rust`, `area:stellar`, `type:feature`, `difficulty:intermediate`
- Done when: Reconciliation captures memo-related misses for support and fraud review.

### AP-172 Add an operator query for orphan payments
- Labels: `area:database`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Operators can find on-chain payments that hit treasury but did not cleanly map to an invoice.

### AP-173 Add payout-queue health endpoint or query surface
- Labels: `area:backend-rust`, `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Operators can inspect whether queued payouts are piling up abnormally.

### AP-174 Add dry-run mode for reconcile and settle routes
- Labels: `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Cron jobs can simulate what they would do without mutating invoice or payout state.

### AP-175 Add a cron-run audit table
- Labels: `area:database`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Every reconcile and settle run records timing, outcome, and key counters.

### AP-176 Add manual replay endpoint for a specific invoice reconciliation
- Labels: `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Operators can rescan one invoice safely without rerunning global reconciliation.

### AP-177 Add manual replay endpoint for a specific payout settlement
- Labels: `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Operators can re-attempt one payout in a controlled and auditable way.

### AP-178 Add settlement batch-size configuration
- Labels: `area:backend-rust`, `type:feature`, `difficulty:starter`
- Done when: Operators can tune how many queued payouts one settle run processes.

### AP-179 Add reconciliation scan-window configuration
- Labels: `area:backend-rust`, `type:feature`, `difficulty:starter`
- Done when: Reconciliation can be tuned for time window or invoice volume without code edits.

### AP-180 Add a Horizon client abstraction for Rust testability
- Labels: `area:backend-rust`, `area:stellar`, `type:refactor`, `difficulty:advanced`
- Done when: Rust can test reconciliation and settlement flows without binding every case to live Horizon calls.

### AP-181 Add unit tests for payment payload matching edge cases
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:intermediate`
- Done when: Matching logic covers destination, asset, amount, and memo edge cases explicitly.

### AP-182 Add integration tests for reconcile-to-payout transition
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:advanced`
- Done when: The service proves a pending invoice becomes paid and enqueues exactly one payout.

### AP-183 Add integration tests for settle-to-invoice-settled transition
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:advanced`
- Done when: The service proves a payout settlement updates both payout and invoice records coherently.

### AP-184 Document treasury key custody and rotation requirements
- Labels: `area:docs`, `area:security`, `type:docs`, `difficulty:intermediate`
- Done when: The repo spells out how treasury secrets should be stored, rotated, and never exposed to frontend runtimes.

### AP-185 Document webhook-provider assumptions and failure modes
- Labels: `area:docs`, `area:backend-rust`, `type:docs`, `difficulty:starter`
- Done when: Contributors know what guarantees the webhook integration does and does not provide.

## Database Schema Migrations And Performance

Relevant code:
- `usdc-payment-link-tool/migrations/001_init.sql`
- `rust-backend/src/db.rs`
- `rust-backend/src/models.rs`
- `rust-backend/src/bin/migrate.rs`

### AP-186 Add or align a `schema_migrations` table contract for Rust migration ownership
- Labels: `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: Migration state tracking is explicit and both runtimes agree on how schema versions are recorded.

### AP-187 Add a check constraint enforcing `gross_amount_cents = platform_fee_cents + net_amount_cents`
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: The database rejects invoices whose money math does not add up.

### AP-188 Add a check constraint preventing `paid_at` from preceding invoice creation
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: Time-travel invoice data is blocked by the schema rather than tolerated quietly.

### AP-189 Add a check constraint preventing `settled_at` from preceding `paid_at`
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: The schema enforces the obvious payment-before-settlement ordering.

### AP-190 Add a partial index for pending invoices by expiry
- Labels: `area:database`, `type:performance`, `difficulty:starter`
- Done when: Reconciliation queries can locate expiring or pending invoices efficiently at scale.

### AP-191 Add a partial index for queued payouts by creation time
- Labels: `area:database`, `type:performance`, `difficulty:starter`
- Done when: Settlement scans can process queued payouts without a full-table scan.

### AP-192 Add unique-index strategy for `transaction_hash` when present
- Labels: `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: The schema prevents one on-chain payment hash from being attached ambiguously to multiple invoices.

### AP-193 Add unique-index strategy for `settlement_hash` when present
- Labels: `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: One settlement transaction cannot be recorded against multiple payout records accidentally.

### AP-194 Add a format constraint for invoice `public_id`
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: Invoice public IDs follow one predictable shape instead of accepting arbitrary text.

### AP-195 Review session-cleanup indexes for expiry-heavy workloads
- Labels: `area:database`, `type:performance`, `difficulty:intermediate`
- Done when: Session cleanup and lookup queries are backed by the right indexes and documented assumptions.

### AP-196 Add an index on `payment_events.event_type`
- Labels: `area:database`, `type:performance`, `difficulty:starter`
- Done when: Event-type filtering no longer requires scanning the full event table.

### AP-197 Add a JSONB indexing plan for invoice metadata queries
- Labels: `area:database`, `type:performance`, `difficulty:intermediate`
- Done when: The repo documents and optionally implements indexing only for metadata access patterns that are actually needed.

### AP-198 Normalize merchant email uniqueness using `citext` or equivalent
- Labels: `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: Merchant email uniqueness is case-safe at the database level as well as in application logic.

### AP-199 Add migration for payout attempt counters and last error fields
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: The payout table can record retry count and most recent failure reason.

### AP-200 Add migration for the cron-run audit table
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: The schema supports persisted metadata for reconcile and settle runs.

### AP-201 Add migration for `last_checkout_attempt_at` on invoices
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: Invoice records can store the last observed checkout-attempt timestamp.

### AP-202 Add migration for a dedicated `checkout_attempts` table
- Labels: `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: Checkout build and submit actions can be tracked separately from generic payment events.

### AP-203 Add migration for a `webhook_deliveries` audit table
- Labels: `area:database`, `type:feature`, `difficulty:intermediate`
- Done when: Webhook requests can be stored with source, status, and replay metadata.

### AP-204 Add a forward-only migration discipline document
- Labels: `area:docs`, `area:database`, `type:docs`, `difficulty:starter`
- Done when: Contributors know migrations should be additive and explicit rather than casually rewritten.

### AP-205 Add rollback notes for each migration file
- Labels: `area:docs`, `area:database`, `type:docs`, `difficulty:starter`
- Done when: Operators have a rollback or mitigation note for every schema change even if migrations remain forward-only.

### AP-206 Add a seed-data script for demo merchants and invoices
- Labels: `area:database`, `type:ops`, `difficulty:starter`
- Done when: Contributors can stand up realistic local data without inventing rows by hand.

### AP-207 Analyze invoice dashboard list queries for wasted columns and scans
- Labels: `area:database`, `type:performance`, `difficulty:intermediate`
- Done when: The main merchant list query is measured and optimized against realistic row counts.

### AP-208 Analyze reconciliation queries for large pending-invoice backlogs
- Labels: `area:database`, `type:performance`, `difficulty:intermediate`
- Done when: The repo captures query plans and concrete improvements for the reconciliation path.

### AP-209 Add retention policy for sessions and payment events
- Labels: `area:database`, `type:docs`, `difficulty:starter`
- Done when: The repo defines how long these records should live and why.

### AP-210 Add archival strategy for settled invoices
- Labels: `area:database`, `type:docs`, `difficulty:intermediate`
- Done when: The repo explains how historical invoice data can be retained without making hot queries degrade forever.

### AP-211 Audit Rust queries that rely on `SELECT *`
- Labels: `area:database`, `area:backend-rust`, `type:refactor`, `difficulty:starter`
- Done when: Contributors can see where broad selects still exist and why they are risky for schema evolution.

### AP-212 Replace `SELECT *` with explicit columns in Rust query paths
- Labels: `area:database`, `area:backend-rust`, `type:refactor`, `difficulty:intermediate`
- Done when: Rust model loading does not silently depend on every column remaining present and ordered.

### AP-213 Add a row-locking strategy for concurrent payout workers
- Labels: `area:database`, `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: Concurrent settlement workers can cooperate without double-processing payout rows.

### AP-214 Review transaction isolation for invoice-paid and payout-queued writes
- Labels: `area:database`, `area:backend-rust`, `type:feature`, `difficulty:advanced`
- Done when: The repo documents and tests the isolation guarantees required for coherent money-state transitions.

### AP-215 Add a database connection-pool tuning guide
- Labels: `area:docs`, `area:database`, `type:docs`, `difficulty:starter`
- Done when: Operators know how to size pools for local dev, Railway, and production-like environments.

### AP-216 Validate `PGSSL` mode at startup
- Labels: `area:database`, `type:bug`, `difficulty:starter`
- Done when: Bad SSL configuration is rejected early with a message that explains the valid modes.

### AP-217 Add migration tests against a clean Postgres instance
- Labels: `area:testing`, `area:database`, `type:test`, `difficulty:intermediate`
- Done when: The migration chain can be applied from scratch in automation and not just by hope.

### AP-218 Add a large-volume fixture dataset for performance tests
- Labels: `area:testing`, `area:database`, `type:test`, `difficulty:intermediate`
- Done when: The repo includes repeatable high-volume fixtures for invoice, payout, and event workloads.

### AP-219 Add a database constraint preventing empty `business_name`
- Labels: `area:database`, `type:feature`, `difficulty:starter`
- Done when: Merchants cannot persist blank business names because the schema enforces the rule.

### AP-220 Document schema ownership across Next.js and Rust code paths
- Labels: `area:docs`, `area:database`, `type:docs`, `difficulty:starter`
- Done when: Contributors know which runtime currently writes which tables and where hidden coupling still exists.

## Testing QA Observability And Infrastructure

Relevant code:
- `usdc-payment-link-tool/package.json`
- `rust-backend/Cargo.toml`
- `usdc-payment-link-tool/vercel.json`
- `usdc-payment-link-tool/Dockerfile`
- `usdc-payment-link-tool/railway.json`

### AP-221 Add a unit-test harness for shared frontend utilities
- Labels: `area:testing`, `type:test`, `difficulty:starter`
- Done when: Shared helpers such as formatting, validation, and HTTP utilities have a repeatable local and CI test runner.

### AP-222 Add a Playwright smoke test for merchant auth and invoice creation
- Labels: `area:testing`, `type:test`, `difficulty:intermediate`
- Done when: CI can prove a merchant can register or log in and create an invoice through the UI.

### AP-223 Add a Playwright checkout smoke test with a mocked wallet adapter
- Labels: `area:testing`, `type:test`, `difficulty:advanced`
- Done when: The public pay flow is exercised end to end without requiring a real browser wallet in CI.

### AP-224 Add an end-to-end test for invoice expiry transition
- Labels: `area:testing`, `type:test`, `difficulty:intermediate`
- Done when: Automation proves invoices move from pending to expired under the expected conditions.

### AP-225 Add an end-to-end test for paid invoice status after reconciliation
- Labels: `area:testing`, `type:test`, `difficulty:advanced`
- Done when: The system proves a detected on-chain payment moves invoice state correctly in a full-stack test.

### AP-226 Add contract tests between Next.js and Rust route responses
- Labels: `area:testing`, `type:test`, `difficulty:advanced`
- Done when: A shared suite fails if equivalent routes diverge in status codes or response shapes.

### AP-227 Add Rust unit tests for auth helpers and cookie handling
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:starter`
- Done when: Session token generation, parsing, and cookie semantics are covered directly in Rust tests.

### AP-228 Add Rust integration tests for invoice CRUD against ephemeral Postgres
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:intermediate`
- Done when: Rust CRUD paths are exercised against a real database instead of only unit-level assumptions.

### AP-229 Add Rust integration tests for cron authorization rejection paths
- Labels: `area:testing`, `area:backend-rust`, `type:test`, `difficulty:starter`
- Done when: Unauthorized cron invocations are covered for missing, malformed, and wrong bearer tokens.

### AP-230 Add a fixture library for Horizon payment-payload variants
- Labels: `area:testing`, `area:stellar`, `type:test`, `difficulty:intermediate`
- Done when: Reconciliation tests can reuse realistic Horizon payloads for good, bad, and ambiguous matches.

### AP-231 Add a load test for reconciliation with ten thousand pending invoices
- Labels: `area:testing`, `type:test`, `difficulty:advanced`
- Done when: The team can measure throughput and bottlenecks for a realistic pending-invoice backlog.

### AP-232 Add failure-injection tests for Horizon timeouts and partial outages
- Labels: `area:testing`, `area:stellar`, `type:test`, `difficulty:advanced`
- Done when: The system proves upstream failures do not create incorrect terminal invoice states.

### AP-233 Add a chaos-test plan for duplicate webhooks and duplicate cron triggers
- Labels: `area:testing`, `type:test`, `difficulty:advanced`
- Done when: The repo documents and, where practical, automates duplicate-delivery scenarios that can corrupt state.

### AP-234 Add a manual QA release checklist
- Labels: `area:docs`, `area:testing`, `type:docs`, `difficulty:starter`
- Done when: Releases have a short but ruthless manual test checklist instead of vibes.

### AP-235 Add CI jobs that run Next.js and Rust checks separately
- Labels: `area:infrastructure`, `area:testing`, `type:ops`, `difficulty:intermediate`
- Done when: CI isolates failures by runtime and stops one stack from hiding the other stack's breakage.

### AP-236 Add JSON structured logging in Rust
- Labels: `area:observability`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Rust logs emit structured fields suitable for aggregation instead of only human-formatted lines.

### AP-237 Propagate correlation IDs from frontend requests into Rust logs
- Labels: `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: A merchant action can be traced across frontend and backend logs with one shared identifier.

### AP-238 Add error classification tags for user, system, and upstream failures
- Labels: `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Failures can be grouped by cause instead of being one undifferentiated error stream.

### AP-239 Add Sentry or equivalent error tracing for the Next.js app
- Labels: `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Frontend runtime and route-handler exceptions are captured with enough metadata to debug production failures.

### AP-240 Add Sentry or equivalent error tracing for the Rust service
- Labels: `area:observability`, `area:backend-rust`, `type:feature`, `difficulty:intermediate`
- Done when: Rust exceptions, panics, and high-value error contexts are shipped to an alertable sink.

### AP-241 Add log-redaction rules for wallet keys, session tokens, and cookies
- Labels: `area:observability`, `area:security`, `type:feature`, `difficulty:advanced`
- Done when: Logs are useful for debugging but cannot leak secrets or high-risk identifiers.

### AP-242 Add a metrics spec for invoice and payout lifecycle events
- Labels: `area:observability`, `type:docs`, `difficulty:starter`
- Done when: The repo defines the counters, gauges, and timings that actually matter to ASTROpay operations.

### AP-243 Add alert thresholds for stuck pending invoices and queued payouts
- Labels: `area:observability`, `type:feature`, `difficulty:intermediate`
- Done when: Operators are alerted when money-state queues stop moving for suspiciously long periods.

### AP-244 Add a dashboard spec for payment success rate and payout latency
- Labels: `area:observability`, `type:docs`, `difficulty:starter`
- Done when: The team has a concrete monitoring dashboard design instead of vague “we should monitor this” language.

### AP-245 Add an incident runbook for failed reconciliation or settlement jobs
- Labels: `area:docs`, `area:observability`, `type:docs`, `difficulty:intermediate`
- Done when: The repo contains a step-by-step operational response for the failure modes most likely to hurt money movement.

### AP-246 Add Railway deployment guidance for running Next.js and Rust as separate services
- Labels: `area:infrastructure`, `type:docs`, `difficulty:starter`
- Done when: The repo explains how to deploy the web app and Rust worker independently instead of pretending one runtime should do everything.

### AP-247 Add Docker Compose for local Next.js plus Rust plus Postgres
- Labels: `area:infrastructure`, `type:feature`, `difficulty:intermediate`
- Done when: Contributors can boot the full stack locally with one compose flow and minimal manual wiring.

### AP-248 Add a production-readiness checklist for cutting checkout over to Rust
- Labels: `area:docs`, `area:infrastructure`, `type:docs`, `difficulty:intermediate`
- Done when: The repo names the tests, rollout gates, and rollback triggers required before Rust owns checkout.

### AP-249 Add GitHub Actions workflow for lint, typecheck, and `cargo test`
- Labels: `area:infrastructure`, `area:testing`, `type:ops`, `difficulty:intermediate`
- Done when: The repo enforces baseline automation for both stacks on every pull request.

### AP-250 Add a staged release and rollback playbook for architecture changes
- Labels: `area:docs`, `area:infrastructure`, `type:docs`, `difficulty:advanced`
- Done when: The repo documents how to ship risky backend cutovers gradually and how to back out without corrupting invoice or payout state.
