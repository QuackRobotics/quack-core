# Quack Robotics Governance

> **Status:** Draft — open for comment until 2026-12-01

## 1. Purpose

This document defines the governance model for the **Quack Robotics** (formerly `quack-core`) organization. It covers the maintainer committee, the RFC process, release cadence, and decision-making authority.

---

## 2. Maintainer Committee

The project is governed by a **3-person maintainer committee**:

| Role | GitHub | Responsibility |
|------|--------|----------------|
| **Release Lead (Torvalds)** | @naouflex | Governance, CI/CD, releases, licensing, community health |
| **Hardware Lead (Woz)** | TBD | PCBA, mechanical, BOM, assembly, supply chain |
| **Software Lead (Page)** | TBD | Backend API, SDKs, dashboard, fleet management |

> **Week 1 action:** Quack Chief to nominate 2 co-maintainers. See [#1](https://github.com/QuackRobotics/quack-core/issues/1).

### 2.1 Committee Duties

- Approve/reject RFCs (see §4)
- Merge rights to `main` across all 7 repos
- Cut monthly patch and quarterly minor releases
- Enforce DCO, license headers, and REUSE compliance
- Track community health metrics (time-to-first-response, contributor retention)

### 2.2 Adding/Removing Maintainers

- Requires unanimous committee vote + Quack Chief approval
- Documented in `MAINTAINERS.md` (to be created)

---

## 3. Licensing

| Asset Type | License | SPDX Identifier |
|------------|---------|-----------------|
| Hardware (PCBA, CAD, mechanical) | CERN-OHL-S 2.0 | `CERN-OHL-S-2.0` |
| Software (firmware, backend, ML, sim, dashboard) | Apache-2.0 | `Apache-2.0` |
| Documentation | CC-BY-SA 4.0 | `CC-BY-SA-4.0` |

**All source files must carry a license header** (REUSE compliant). See `CONTRIBUTING.md` for the exact header format.

No binary blobs in any repository — firmware builds from source.

---

## 4. RFC Process (Request for Comments)

Any **architectural change**, **hardware revision**, **new dependency**, or **policy change** requires an RFC.

### 4.1 RFC Lifecycle

```
DRAFT → REVIEW → APPROVED → IMPLEMENTED → CLOSED
  │         │          │           │
  │         │          │           └─ Merged to main, docs updated
  │         │          └─ Committee unanimous + 7-day comment period
  │         └─ Open for comment (min 7 days), discussed in Discord/GitHub Discussions
  └─ Opened as PR against quack-core/docs/rfcs/NNNN-title.md
```

### 4.2 RFC Template

See `docs/rfcs/TEMPLATE.md` (to be created). Minimum sections:

- Summary
- Motivation
- Detailed Design
- Alternatives Considered
- Impact (hardware, firmware, sim, ML, dashboard, API)
- Migration Plan
- Security Considerations
- Testing Plan

### 4.3 Fast-Track

Bug fixes, documentation updates, and dependency updates **do not** require an RFC.

---

## 5. Release Cadence

| Release Type | Frequency | Version Scheme | Contents |
|--------------|-----------|----------------|----------|
| **Patch** | Monthly (1st Saturday) | `v0.Y.Z` | Bug fixes, docs, dependency updates |
| **Minor** | Quarterly (Mar/Jun/Sep/Dec) | `v0.Y.0` | New features, hardware revs, policy updates |
| **Major** | As needed | `vX.0.0` | Breaking changes, architecture overhauls |

### 5.1 Release Artifacts

Every release **must** include:

- Signed Git tag (`git tag -s v0.Y.Z -m "Release v0.Y.Z"`)
- SBOM (Software Bill of Materials) in SPDX JSON format
- cosign attestation (`cosign sign --yes v0.Y.Z`)
- GitHub Release with changelog
- Hardware artifacts (Gerbers, BOM CSV, assembly PDF) versioned with firmware

### 5.2 Release Branches

- `main` = development branch (next minor)
- `release/v0.Y` = stabilization branch for current minor
- Only committee members may create release branches

---

## 6. Contribution Flow

1. **Good First Issues** labeled `good first issue` + `help wanted`
2. PR template enforces DCO sign-off (`git commit -s`)
3. 2 approvals required (one from code owner, one from committee)
4. CI must pass (`cargo test`, `pytest`, `hadolint`, `markdownlint`)
5. Linear history enforced — squash merge only

See `CONTRIBUTING.md` for details.

---

## 7. Security

- **Dependabot** enabled on all repos (weekly)
- **cargo-audit** + **pip-audit** in CI
- **CodeQL** analysis on PRs (GitHub Advanced Security — free for public)
- **SLSA Level 2** for releases (provenance, signed artifacts)
- Security issues reported via `SECURITY.md` (private disclosure)

---

## 8. Community

| Channel | Purpose |
|---------|---------|
| GitHub Discussions | RFCs, design questions, announcements |
| Discord (`#quack-robotics`) | Real-time chat, pair programming, support |
| Matrix (`#quack-robotics:matrix.org`) | Bridged to Discord, encrypted option |
| Quarterly **Quackathon** | 48-hour hackathon, demo day, swag |

### 8.1 Code of Conduct

Contributor Covenant v2.1 — see `CODE_OF_CONDUCT.md` (to be added).

---

## 9. Decision Log

| Date | Decision | RFC |
|------|----------|-----|
| 2026-11-21 | Org created as `QuackRobotics`, 7 repos initialized | — |
| 2026-11-21 | Branch protection: 2 reviews, CI required, linear history | — |
| 2026-11-21 | Licensing: HW=CERN-OHL-S-2.0, SW=Apache-2.0, Docs=CC-BY-SA-4.0 | — |

---

## 10. Amendments

This governance document can be amended by:

1. RFC opened against `quack-core/GOVERNANCE.md`
2. 7-day comment period
3. Unanimous committee vote
4. Quack Chief ratification

---

*Questions? Open a discussion or ping @naouflex.*
