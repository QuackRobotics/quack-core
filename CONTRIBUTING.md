# Contributing to Quack Robotics

> Welcome! This guide covers the contribution workflow for all 7 repositories in the `QuackRobotics` organization.

---

## 1. Quick Start

```bash
# Pick a repo (example: quack-firmware)
git clone https://github.com/QuackRobotics/quack-firmware
cd quack-firmware

# Create a feature branch
git checkout -b feat/your-change

# Make changes, commit with DCO sign-off
git commit -s -m "feat: add servo temperature monitoring

- Read TMP1075 via I2C on each servo bus
- Publish /servo/temps topic
- Closes #42"

# Push and open PR
git push origin feat/your-change
gh pr create --fill
```

---

## 2. Developer Certificate of Origin (DCO)

**All commits must be signed off.** This certifies you have the right to submit the code under the project license.

```bash
git commit -s -m "your message"
```

The `-s` flag appends:
```
Signed-off-by: Your Name <email@example.com>
```

**No DCO = no merge.** GitHub enforces this via the DCO app (to be configured).

---

## 3. Code Style & Linting

| Language | Formatter | Linter | Config |
|----------|-----------|--------|--------|
| Rust | `rustfmt` | `clippy` | `rustfmt.toml`, `clippy.toml` |
| Python | `ruff format` | `ruff check` | `pyproject.toml` |
| TypeScript/JS | `prettier` | `eslint` | `.prettierrc`, `eslint.config.js` |
| Markdown | `prettier` | `markdownlint` | `.markdownlint.json` |
| Docker | — | `hadolint` | `.hadolint.yaml` |
| Protobuf | `buf format` | `buf lint` | `buf.yaml`, `buf.lock` |

**Run locally before pushing:**
```bash
# Rust
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings

# Python
ruff format .
ruff check .

# TypeScript (dashboard, API SDKs)
prettier --check .
eslint .

# Markdown (all repos)
markdownlint **/*.md

# Docker
hadolint Dockerfile
```

---

## 4. Testing Requirements

| Repo | Test Command | Coverage Target |
|------|--------------|-----------------|
| quack-firmware | `cargo test --all` | 80% |
| quack-api | `cargo test --all` + `pytest` | 80% |
| quack-ml | `pytest tests/` | 70% |
| quack-sim | `pytest tests/` (MuJoCo headless) | 60% |
| quack-dashboard | `npm test` / `vitest` | 70% |
| quack-hardware | `pytest tests/` (BOM validation, DRC) | 80% |

**CI will fail if coverage drops below target.**

---

## 5. Pull Request Checklist

Every PR must:

- [ ] Target `main` branch
- [ ] Have a descriptive title (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`)
- [ ] Reference an issue (`Closes #123` or `Relates to #123`)
- [ ] Include DCO sign-off on all commits
- [ ] Pass all CI checks (green ✅)
- [ ] Have **2 approvals** (one from CODEOWNER, one from maintainer committee)
- [ ] Update documentation if behavior changes
- [ ] Add tests for new functionality
- [ ] No binary blobs — firmware builds from source

---

## 6. License Headers (REUSE Compliant)

**Every source file must start with a license header.**

### Rust (`.rs`)
```rust
// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0
```

### Python (`.py`)
```python
# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0
```

### TypeScript/JavaScript (`.ts`, `.js`, `.tsx`, `.jsx`)
```typescript
// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0
```

### Markdown (`.md`)
```markdown
<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->
```

### Hardware (KiCad, STEP, Gerber — use `.license` sidecar or header comment where format allows)

See [REUSE spec](https://reuse.software/spec/) for details.

---

## 7. Repository-Specific Notes

### quack-hardware
- KiCad 8+ for PCBA
- FreeCAD 1.0+ for mechanical
- BOM exported as CSV (`bom.csv`) with LCSC/DigiKey part numbers
- Assembly PDF generated from KiCad + FreeCAD

### quack-firmware
- Rust 2021 edition, MSRV = latest stable - 2
- `no_std` + `embassy` for Radxa Zero 3W
- Servo driver: PCA9685 (I2C) → custom STM32 (SPI) in v0.2
- Build: `cargo build --release --target thumbv7em-none-eabihf`

### quack-api
- Axum 0.7 + Tonic 0.10 (gRPC)
- OpenAPI spec at `docs/openapi.yaml`
- SQLite (dev) / PostgreSQL (prod) via SQLx
- Auth: JWT + device code flow

### quack-ml
- Isaac Lab 2.0 + PyTorch 2.3
- ONNX export via `torch.onnx.export`
- Training configs in `configs/` (Hydra)
- Reproducibility: seed, git SHA, Docker image digest logged

### quack-sim
- MuJoCo 3.2+ (MJCF)
- Assets in `assets/` (STL, OBJ, textures)
- Headless rendering via `mujoco-python` + `egl`

### quack-dashboard
- Three.js r160+ + React 18 + Vite
- WebGPU preferred, WebGL fallback
- Real-time via WebSocket to quack-api

### quack-core (this repo)
- Governance, RFCs, docs (mkdocs-material)
- Auto-deploy to GitHub Pages on merge to `main`
- Versioned docs: `v0.x`, `main`

---

## 8. Issue Labels

| Label | Meaning |
|-------|---------|
| `bug` | Something is broken |
| `feature` | New functionality |
| `hardware` | PCBA/mechanical/BOM |
| `sim2real` | Simulation vs. reality gap |
| `safety` | Safety incident or risk |
| `good first issue` | Beginner-friendly |
| `help wanted` | Community contribution welcome |
| `needs-rfc` | Requires RFC before implementation |
| `blocked` | Waiting on external dependency |
| `priority: P0-P3` | Urgency |

---

## 9. Getting Help

- **GitHub Discussions** — design questions, RFCs
- **Discord** `#quack-robotics` — real-time help
- **Matrix** `#quack-robotics:matrix.org` — encrypted chat
- **Email** `maintainers@quack-robotics.org` (to be set up) — security issues

---

## 10. Recognition

Contributors are recognized in:
- `AUTHORS.md` (auto-generated from git)
- Release changelogs
- Quarterly Quackathon hall of fame

---

*Happy hacking! 🦆*
