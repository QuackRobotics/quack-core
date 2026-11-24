// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Quack-core HAL: the pure, hardware-free part of the 50 Hz control path.
//!
//! - [`joints`]: policy joint order (Open Duck Playground f698836), mapped by name.
//! - [`head`]: policy action + expressive overlay -> final joint targets, head box clamp,
//!   then `joints_offset` calibration.
//! - [`power`]: PFO_N power-fail input debounce.

pub mod head;
pub mod joints;
pub mod power;
