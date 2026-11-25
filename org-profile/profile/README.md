<div align="center">

<img src="./assets/hero-banner.svg" width="100%" alt="Quack Robotics — Micro Duck hero banner"/>

### Open-source bipedal stack for a ~42&nbsp;cm duck-inspired robot.

**Micro Duck** builds on [Open Duck Mini v2](https://github.com/apirrone/Open_Duck_Mini) by [apirrone](https://github.com/apirrone) (Apache-2.0).

</div>

---

## What we're building

[Quack Robotics](https://github.com/QuackRobotics) is a multi-repo effort to design, simulate, train, and operate **Micro Duck** — an open-source biped with PCBA and mechanical CAD, Rust firmware on Radxa with servo drivers, a fleet-oriented **axum** + **tonic** backend, **MuJoCo** simulation assets, **Isaac Lab** training with **ONNX** export, and a **Three.js** dashboard. Repository layout, governance, and shared CI live in [**quack-core**](https://github.com/QuackRobotics/quack-core).

## Repositories

| | Repository | Focus |
|:---:|:---|:---|
| <img src="./assets/icons/quack-core.svg" width="40" height="40" alt=""/> | [**quack-core**](https://github.com/QuackRobotics/quack-core) | Monorepo root, governance, docs, CI/CD |
| <img src="./assets/icons/quack-hardware.svg" width="40" height="40" alt=""/> | [**quack-hardware**](https://github.com/QuackRobotics/quack-hardware) | PCBA, mechanical CAD, BOM, assembly |
| <img src="./assets/icons/quack-firmware.svg" width="40" height="40" alt=""/> | [**quack-firmware**](https://github.com/QuackRobotics/quack-firmware) | Radxa + servo driver firmware (Rust) |
| <img src="./assets/icons/quack-api.svg" width="40" height="40" alt=""/> | [**quack-api**](https://github.com/QuackRobotics/quack-api) | Backend API, fleet management, auth |
| <img src="./assets/icons/quack-ml.svg" width="40" height="40" alt=""/> | [**quack-ml**](https://github.com/QuackRobotics/quack-ml) | ML training, policies, domain randomization |
| <img src="./assets/icons/quack-sim.svg" width="40" height="40" alt=""/> | [**quack-sim**](https://github.com/QuackRobotics/quack-sim) | MuJoCo MJCF simulation, sim2real |
| <img src="./assets/icons/quack-dashboard.svg" width="40" height="40" alt=""/> | [**quack-dashboard**](https://github.com/QuackRobotics/quack-dashboard) | Web dashboard and visualization |

<details>
<summary><strong>Clone all public repos</strong></summary>

```bash
gh repo list QuackRobotics --limit 50 --json name -q '.[].name' \
  | xargs -I {} gh repo clone QuackRobotics/{}
```

</details>

## Architecture

How the seven repositories connect in the Micro Duck stack:

```mermaid
flowchart TB
  subgraph physical["Physical platform"]
    HW["quack-hardware<br/>PCBA · CAD · BOM"]
    FW["quack-firmware<br/>Radxa · servos · Rust"]
  end

  subgraph cloud["Backend & UI"]
    API["quack-api<br/>axum · tonic"]
    DASH["quack-dashboard<br/>Three.js"]
  end

  subgraph research["Training & simulation"]
    SIM["quack-sim<br/>MuJoCo MJCF"]
    ML["quack-ml<br/>Isaac Lab · ONNX"]
  end

  CORE["quack-core<br/>governance · docs · CI"]

  HW --> FW
  FW <-->|control · telemetry| API
  API --> DASH
  SIM --> ML
  ML -->|policies| FW
  SIM -.->|sim2real validation| FW
  CORE -.-> FW
  CORE -.-> API
```

<p align="center">
  <img src="./assets/architecture.svg" width="88%" alt="Architecture diagram (SVG fallback)"/>
</p>

## Tech stack

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/axum-000000?style=for-the-badge&logo=rust&logoColor=white" alt="axum"/>
  <img src="https://img.shields.io/badge/tonic-000000?style=for-the-badge&logo=rust&logoColor=white" alt="tonic"/>
  <img src="https://img.shields.io/badge/Isaac_Lab-76B900?style=for-the-badge&logo=nvidia&logoColor=white" alt="Isaac Lab"/>
  <img src="https://img.shields.io/badge/ONNX-005CED?style=for-the-badge&logo=onnx&logoColor=white" alt="ONNX"/>
  <img src="https://img.shields.io/badge/MuJoCo-1E88E5?style=for-the-badge" alt="MuJoCo"/>
  <img src="https://img.shields.io/badge/Three.js-000000?style=for-the-badge&logo=threedotjs&logoColor=white" alt="Three.js"/>
</p>

## Community & docs

- **Governance & contributing:** see [quack-core/GOVERNANCE.md](https://github.com/QuackRobotics/quack-core/blob/main/GOVERNANCE.md) and [CONTRIBUTING.md](https://github.com/QuackRobotics/quack-core/blob/main/CONTRIBUTING.md)
- **GitHub Discussions:** RFCs, design questions, announcements (per quack-core README)
- **License mix:** Apache-2.0 software, CERN-OHL-S 2.0 hardware, CC-BY-SA 4.0 docs — details in [quack-core](https://github.com/QuackRobotics/quack-core/blob/main/README.md)

---

<p align="center">
  <sub>Derived from Open Duck Mini v2 · <a href="https://github.com/QuackRobotics/quack-core/blob/main/NOTICE">NOTICE</a></sub>
</p>