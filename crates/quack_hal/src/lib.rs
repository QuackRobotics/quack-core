// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Hardware-independent core of the quack-core HAL.
//!
//! - [`joints`]: policy joint order (Playground f698836), mapped by name.
//! - [`head_clamp`]: head safety box applied to final joint targets, before calibration offsets.
//! - [`power_fail`]: PFO_N debounce.

pub mod head_clamp;
pub mod joints;
pub mod power_fail;
