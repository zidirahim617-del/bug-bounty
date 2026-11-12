# ISSUE_059 — Kitewell Stellar Wave 10 batch (3 modules)

1. **Issue ID:** `ISSUE_059` (covers GitHub issues #289, #269, #278)
2. **Repository & File:** `Kitewell-lab/Kitewell` | `backend/src/addressValidate.js`, `frontend/src/amountFormat.js`, `frontend/src/addressShorten.js`, plus `docs/features/*.md`
3. **Classification:** Feature / utility module additions (Wave backlog)
4. **Severity Level:** Medium
5. **Proof / Evidence:**
   - Open wave backlog of 40 issues on `Kitewell-lab/Kitewell`; repo is small (~1.5 MB) and clones cleanly.
   - Each issue requires a self-contained module, at least one test file per behavior, and a `docs/features/*.md` doc, with no edits to existing files.
6. **Proposed Fix:**
   - `#289 addressValidate.js` — validate Stellar `G...` public keys (length 56, base32 charset, `G` prefix). 1 module + 8 behavior tests + doc.
   - `#269 amountFormat.js` — convert decimal XLM amounts to/from stroops without float rounding. 1 module + 9 behavior tests + doc.
   - `#278 addressShorten.js` — shorten a `G...` address to `GBBD...FLA5`; return unchanged when the requested sides do not fit. 1 module + 8 behavior tests + doc.
7. **Verification:**
   - **Before:** No such modules exist; wave backlog unserved.
   - **After:** 33 files written under `C:\Users\L13\AppData\Local\Temp\opencode\kw_kit\`. `node --test` green for all three modules: `addressValidate` 8/8, `amountFormat` 9/9, `addressShorten` 8/8. 0 failures.
   - Note: issues mention Vitest, but the repo runs Node's built-in `node:test`; tests match the repo's actual runner.
8. **Review Status:** PRs opened, awaiting maintainer review.
   - `Kitewell-lab/Kitewell#308` — `fix/289-address-validate` (closes #289) — OPEN, MERGEABLE, `closingIssuesReferences` → #289 confirmed.
   - `Kitewell-lab/Kitewell#309` — `feat/269-amount-format` (closes #269) — OPEN, MERGEABLE, `closingIssuesReferences` → #269 confirmed.
   - `Kitewell-lab/Kitewell#310` — `feat/278-address-shorten` (closes #278) — OPEN, MERGEABLE, `closingIssuesReferences` → #278 confirmed.
   - No CI checks are configured on these branches; `mergeStateStatus: UNSTABLE` reflects absent status, not a failure. Local `node --test` suites are green (25/25).
9. **Payment Status:** Submitted — Drips Wave applications confirmed on all three issues (`wave:application-id` present). Awaiting assignment and merge.
