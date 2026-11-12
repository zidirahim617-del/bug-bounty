# ISSUE_069 — Filecoin (Lotus) Graphsync & CBOR Fast-Path Decoder Audit (Work 8)

**Date executed:** 2026-10-22
**Platform / target:** Filecoin Protocol / Lotus Go Client (`filecoin-project/lotus`)
**Subsystem:** Graphsync transfer subsystem & CBOR binary fast-path decoders
**Track:** Security Audit / External Bounty ($200–$1,000 range)

## Scope & Audit Findings
1. **Graphsync Protocol Audit:**
   - Inspected block exchange requests (`graphsync/request.go`) and response matching.
   - Verified bounds checking on incoming message chunks to prevent integer overflow during response allocation.
2. **CBOR Fast-Path Decoder Audit:**
   - Inspected binary CBOR unmarshaling (`cborutil` / `go-cbor-gen`) for nested depth limit checks and recursive allocation safety.
   - Confirmed that excessively nested CBOR arrays/maps without recursion depth guards can trigger stack exhaustion (DoS).

## Status & Deliverables
- Audit notes compiled and formatted for submission or internal tracking.
- Contributes to the external bounty pipeline alongside Cosmos SDK recon.
