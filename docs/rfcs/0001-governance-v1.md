# RFC 0001: Governance v1 — Maintainer Committee, RFC Process, Release Cadence, Licensing

**Status:** DRAFT → REVIEW (open for comment until 2026-12-01)
**Author:** Torvalds (Release Lead)
**Reviewers:** Woz (Hardware Lead, nominee), Page (Software Lead, nominee), Quack Chief

---

## Summary

Establish the foundational governance model for Quack Robotics: a 3-person maintainer committee, RFC-based decision process, monthly patch / quarterly minor release cadence, and a three-tier licensing scheme (Hardware: CERN-OHL-S 2.0, Software: Apache-2.0, Docs: CC-BY-SA 4.0).

---

## Motivation

- **Clear authority**: Avoid design-by-committee; single Hardware Lead and Software Lead for cross-cutting decisions
- **Predictable releases**: Monthly patches for stability, quarterly minors for features, signed artifacts + SBOM for supply chain
- **License clarity**: Upstream Open Duck Mini v2 is Apache-2.0; hardware needs strong copyleft (CERN-OHL-S); docs need share-alike (CC-BY-SA)
- **Community onboarding**: RFC template + 7-day comment period gives contributors a known path

---

## Detailed Design

### Maintainer Committee (3 persons)

| Role | Nominee | Scope |
|------|---------|-------|
| Release Lead (Chair) | Torvalds (@naouflex) | Governance, CI/CD, releases, licensing, community health |
| Hardware Lead | Woz (TBD) | PCBA, mechanical, BOM, assembly, thermal, battery safety, DFM, supply chain |
| Software Lead | Page (TBD) | Backend API, SDKs, dashboard, auth, fleet mgmt, API governance |

- **Quorum**: 3/3 for RFC approval, 2/3 for release cuts
- **Term**: Indefinite; removal requires unanimous vote + Quack Chief approval
- **Documented in**: `MAINTAINERS.md` (to be created on approval)

### RFC Process

```
DRAFT → REVIEW → APPROVED → IMPLEMENTED → CLOSED
  │         │          │           │
  │         │          │           └─ Merged to main, docs updated
  │         │          └─ Committee unanimous + 7-day comment period
  │         └─ Open for comment (min 7 days), discussed in Discord/GitHub Discussions
  └─ Opened as PR against quack-core/docs/rfcs/NNNN-title.md
```

**RFC Template** (`docs/rfcs/TEMPLATE.md`): Summary, Motivation, Detailed Design, Alternatives, Impact, Migration Plan, Security, Testing Plan.

**Fast-track** (no RFC needed): bug fixes, docs updates, dependency updates.

### Release Cadence

| Type | Frequency | Version | Contents |
|------|-----------|---------|----------|
| Patch | Monthly (1st Saturday) | `v0.Y.Z` | Bug fixes, docs, deps |
| Minor | Quarterly (Mar/Jun/Sep/Dec) | `v0.Y.0` | Features, hardware revs, policy updates |
| Major | As needed | `vX.0.0` | Breaking changes, architecture overhauls |

**Every release must include**: signed tag, SBOM (SPDX JSON), cosign attestation, GitHub Release with changelog, hardware artifacts versioned with firmware.

### Licensing

| Asset Type | License | SPDX | Rationale |
|------------|---------|------|-----------|
| Hardware (PCBA, CAD, mechanical) | CERN-OHL-S 2.0 | `CERN-OHL-S-2.0` | Strong copyleft for hardware designs; aligns with open hardware norms |
| Software (firmware, backend, ML, sim, dashboard) | Apache-2.0 | `Apache-2.0` | Permissive, patent grant, compatible with upstream Open Duck Mini v2 |
| Documentation | CC-BY-SA 4.0 | `CC-BY-SA-4.0` | Share-alike for docs; encourages derivative tutorials/translations |

**REUSE-compliant headers required** on every source file (see `CONTRIBUTING.md`).

### Security Posture

- Dependabot weekly on all repos
- `cargo-audit` + `pip-audit` in CI
- CodeQL on PRs (GitHub Advanced Security)
- SLSA Level 2 target for releases (provenance, signed artifacts)
- Private disclosure via `SECURITY.md`

---

## Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| Single BDFL | Bus factor; no hardware/software separation |
| 5-person committee | Decision latency; quorum harder |
| MIT for software | No patent grant; upstream is Apache-2.0 |
| GPL for hardware | CERN-OHL-S purpose-built for hardware |

---

## Impact

| Area | Impact |
|------|--------|
| Hardware | Woz owns BOM/CAD changes; RFC required for rev >$50 or >1 day |
| Firmware | Page owns API contracts; RFC for breaking changes |
| Sim/ML | Hinton/Atlas reference committee for sim2real scope trades |
| Dashboard | Page owns UX/API; RFC for user-facing changes |
| CI/CD | Torvalds owns pipeline; RFC for new required gates |

---

## Migration Plan

1. **Week 1** (this RFC): Committee nominated, governance docs open for comment
2. **Week 2**: Committee acceptance confirmed; `MAINTAINERS.md` created; `CODEOWNERS` per repo
3. **Week 3**: First RFCs for hardware spec freeze, API v1 freeze, sim gate definition
4. **Ongoing**: All architectural changes via RFC

---

## Security Considerations

- Committee members have merge rights → 2FA mandatory, GPG-signed commits
- Release artifacts signed + cosign attestation → supply chain integrity
- SBOM generated per release → vulnerability scanning

---

## Testing Plan

- CI gate (`ci` job) enforces: lint, tests, sim-gate, security, SBOM
- Branch protection: 2 reviews + CI pass + linear history
- Release dry-run on `release/v0.Y` branch before tagging

---

## References

- [GOVERNANCE.md](../GOVERNANCE.md) — Full governance document (this RFC proposes its adoption)
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Contribution flow, DCO, style, license headers
- [Open Duck Mini v2](https://github.com/apirrone/Open_Duck_Mini) — Upstream (Apache-2.0)
- [CERN-OHL-S 2.0](https://ohwr.org/project/cernohl/wikis/Documents/CERN-OHL-S-2.0)
- [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [CC-BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)

---

## Discussion

**Open for comment until 2026-12-01.** Comment on this PR, in GitHub Discussions, or in Discord `#quack-robotics`.

**Committee vote**: After comment period, unanimous vote by Torvalds + Woz + Page required for APPROVED.