<!--
SPDX-FileCopyrightText: 2026 Quack Robotics contributors
SPDX-License-Identifier: CC-BY-SA-4.0
-->

# Quack Robotics (quack-core)

**Micro Duck** — A ~42 cm open-source bipedal robot, building on
[Open Duck Mini v2](https://github.com/apirrone/Open_Duck_Mini) by apirrone (Apache-2.0).

## Repository Structure

| Repo | Purpose | License |
|------|---------|---------|
| [`quack-core`](https://github.com/QuackRobotics/quack-core) | Governance, docs, CI/CD, release automation | Apache-2.0 |
| [`quack-hardware`](https://github.com/QuackRobotics/quack-hardware) | PCBA, mechanical CAD, BOM, assembly | CERN-OHL-S 2.0 |
| [`quack-firmware`](https://github.com/QuackRobotics/quack-firmware) | Embedded firmware (Rust) | Apache-2.0 |
| [`quack-api`](https://github.com/QuackRobotics/quack-api) | Backend API, fleet management, auth | Apache-2.0 |
| [`quack-ml`](https://github.com/QuackRobotics/quack-ml) | ML training, policies, domain randomization | Apache-2.0 |
| [`quack-sim`](https://github.com/QuackRobotics/quack-sim) | MuJoCo/Isaac Lab simulation, sim2real | Apache-2.0 |
| [`quack-dashboard`](https://github.com/QuackRobotics/quack-dashboard) | Web dashboard, visualization | Apache-2.0 |

## Upstream Attribution

This project is derived from **Open Duck Mini v2** by [apirrone](https://github.com/apirrone) (Apache-2.0).  
See [`NOTICE`](NOTICE) for full attribution and license details.

## Quick Start

```bash
# Clone the org (all repos)
gh repo list QuackRobotics --json name -q '.[].name' | xargs -I {} gh repo clone QuackRobotics/{}
```

## Documentation

- [Governance](GOVERNANCE.md) — Maintainer committee, RFC process, release cadence
- [Contributing](CONTRIBUTING.md) — DCO, code style, test requirements, PR checklist
- [Security](SECURITY.md) — Vulnerability disclosure, SLSA targets

## Community

- GitHub Discussions: RFCs, design questions, announcements
- Discord: `#quack-robotics` (real-time chat, pair programming)
- Matrix: `#quack-robotics:matrix.org` (bridged, encrypted)
- Quarterly **Quackathon**: 48-hour hackathon + demo day

## License

- **Software**: Apache-2.0 (see `LICENSE`)
- **Hardware**: CERN-OHL-S 2.0
- **Documentation**: CC-BY-SA 4.0

All source files carry REUSE-compliant license headers. No binary blobs — firmware builds from source.
