// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Final joint targets and the head safety box.
//!
//! Pipeline per 50 Hz tick, in policy order:
//! 1. `target = default + ACTION_SCALE * action` (policy joints 0-13).
//! 2. Add the expressive head overlay (ADD, never replace), after [`limit_overlay`].
//! 3. Clamp the head joints to [`HEAD_BOX`] on these FINAL targets.
//! 4. Only then add `joints_offset` calibration ([`servo_commands`]).
//!
//! Clamping before calibration keeps the box in joint space: whatever the calibration
//! offset, the joint the servo is told to reach stays inside the box. The box is the
//! Chief ruling of 2026-11-24 16:56 (printed head 15.03 mm vs 5.73 mm bar, quack-sim 05cd64d).
//!
//! Angles are radians. Sign convention (upstream b23317a FK): `+head_pitch` is nose-up.

use std::fmt;
use std::ops::Range;

use crate::joints::{HEAD_PITCH, HEAD_ROLL, HEAD_YAW, JOINT_COUNT, NECK_PITCH, POLICY_JOINT_COUNT};

pub const ACTION_SCALE: f32 = 0.25;

pub const OBS_LEN: usize = 101;
/// Observation slots of the velocity command (vx, vy, wz).
pub const OBS_VELOCITY_COMMAND: Range<usize> = 6..9;
/// Observation slots of the head command. Always 0: the overlay never feeds the policy.
pub const OBS_HEAD_COMMAND: Range<usize> = 9..13;

const fn degrees(value: f32) -> f32 {
    value * (std::f32::consts::PI / 180.0)
}

/// `(joint index, min, max)` in radians, applied to final targets.
pub const HEAD_BOX: [(usize, f32, f32); 4] = [
    (NECK_PITCH, degrees(-15.0), degrees(15.0)),
    (HEAD_PITCH, degrees(-15.0), degrees(15.0)),
    (HEAD_YAW, degrees(-50.0), degrees(50.0)),
    (HEAD_ROLL, degrees(-8.0), degrees(5.0)),
];

/// Standing still: overlay yaw limit (±).
pub const HOLD_OVERLAY_YAW_MAX: f32 = degrees(10.0);
/// Standing still: overlay nose-up pitch limit. The final head box (±15°) keeps the result
/// strictly below it.
pub const HOLD_OVERLAY_PITCH_NOSE_UP_MAX: f32 = degrees(20.0);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VelocityCommand {
    pub vx: f32,
    pub vy: f32,
    pub wz: f32,
}

impl VelocityCommand {
    pub fn is_walking(&self) -> bool {
        self.vx != 0.0 || self.vy != 0.0 || self.wz != 0.0
    }
}

/// Additive expressive head offsets (Luxo), radians.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HeadOverlay {
    pub neck_pitch: f32,
    pub head_pitch: f32,
    pub head_yaw: f32,
    pub head_roll: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetError {
    /// A NaN or infinite input would reach a servo; the tick must be rejected.
    NonFinite { joint: usize },
}

impl fmt::Display for TargetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { joint } => write!(formatter, "non-finite target for joint {joint}"),
        }
    }
}

impl std::error::Error for TargetError {}

/// Walking: no overlay yaw or roll. Holding: yaw within ±10°, nose-up pitch capped at 20°.
pub fn limit_overlay(overlay: HeadOverlay, command: VelocityCommand) -> HeadOverlay {
    if command.is_walking() {
        return HeadOverlay {
            head_yaw: 0.0,
            head_roll: 0.0,
            ..overlay
        };
    }
    HeadOverlay {
        head_yaw: overlay
            .head_yaw
            .clamp(-HOLD_OVERLAY_YAW_MAX, HOLD_OVERLAY_YAW_MAX),
        head_pitch: overlay.head_pitch.min(HOLD_OVERLAY_PITCH_NOSE_UP_MAX),
        ..overlay
    }
}

pub fn clamp_head(targets: &mut [f32; JOINT_COUNT]) {
    for (joint, min, max) in HEAD_BOX {
        targets[joint] = targets[joint].clamp(min, max);
    }
}

/// Final joint-space targets: default + scaled action + limited overlay, head clamped.
pub fn final_targets(
    default_pose: &[f32; JOINT_COUNT],
    action: &[f32; POLICY_JOINT_COUNT],
    overlay: HeadOverlay,
    command: VelocityCommand,
) -> Result<[f32; JOINT_COUNT], TargetError> {
    let mut targets = *default_pose;
    for (joint, value) in action.iter().enumerate() {
        targets[joint] += ACTION_SCALE * value;
    }
    let overlay = limit_overlay(overlay, command);
    targets[NECK_PITCH] += overlay.neck_pitch;
    targets[HEAD_PITCH] += overlay.head_pitch;
    targets[HEAD_YAW] += overlay.head_yaw;
    targets[HEAD_ROLL] += overlay.head_roll;
    if let Some(joint) = targets.iter().position(|value| !value.is_finite()) {
        return Err(TargetError::NonFinite { joint });
    }
    clamp_head(&mut targets);
    Ok(targets)
}

/// Servo-frame commands: clamped final targets plus per-joint calibration offset.
pub fn servo_commands(
    final_targets: &[f32; JOINT_COUNT],
    joints_offset: &[f32; JOINT_COUNT],
) -> [f32; JOINT_COUNT] {
    std::array::from_fn(|joint| final_targets[joint] + joints_offset[joint])
}

pub fn write_command_obs(observation: &mut [f32; OBS_LEN], command: VelocityCommand) {
    observation[OBS_VELOCITY_COMMAND].copy_from_slice(&[command.vx, command.vy, command.wz]);
    observation[OBS_HEAD_COMMAND].fill(0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const STILL: VelocityCommand = VelocityCommand {
        vx: 0.0,
        vy: 0.0,
        wz: 0.0,
    };
    const WALK: VelocityCommand = VelocityCommand {
        vx: 0.1,
        vy: 0.0,
        wz: 0.0,
    };
    const ZERO_ACTION: [f32; POLICY_JOINT_COUNT] = [0.0; POLICY_JOINT_COUNT];
    const TOLERANCE: f32 = 1e-6;

    fn in_box(joint_targets: &[f32; JOINT_COUNT]) -> bool {
        HEAD_BOX.iter().all(|(joint, min, max)| {
            joint_targets[*joint] >= *min - TOLERANCE && joint_targets[*joint] <= *max + TOLERANCE
        })
    }

    #[test]
    fn target_is_default_plus_quarter_action() {
        let default_pose: [f32; JOINT_COUNT] = std::array::from_fn(|joint| 0.001 * joint as f32);
        let action = [0.2; POLICY_JOINT_COUNT];
        let targets = final_targets(&default_pose, &action, HeadOverlay::default(), STILL).unwrap();
        for joint in 0..POLICY_JOINT_COUNT {
            assert!((targets[joint] - (default_pose[joint] + 0.05)).abs() < TOLERANCE);
        }
        assert_eq!(targets[14..], default_pose[14..]);
    }

    #[test]
    fn overlay_adds_to_policy_target() {
        let action = [
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let overlay = HeadOverlay {
            head_pitch: degrees(3.0),
            ..HeadOverlay::default()
        };
        let targets = final_targets(&[0.0; JOINT_COUNT], &action, overlay, STILL).unwrap();
        assert!((targets[HEAD_PITCH] - (0.05 + degrees(3.0))).abs() < TOLERANCE);
    }

    #[test]
    fn head_box_clamps_final_targets_both_sides() {
        let mut action = ZERO_ACTION;
        action[NECK_PITCH] = 10.0;
        action[HEAD_PITCH] = -10.0;
        action[HEAD_YAW] = 10.0;
        action[HEAD_ROLL] = 10.0;
        let high =
            final_targets(&[0.0; JOINT_COUNT], &action, HeadOverlay::default(), STILL).unwrap();
        assert_eq!(high[NECK_PITCH], degrees(15.0));
        assert_eq!(high[HEAD_PITCH], degrees(-15.0));
        assert_eq!(high[HEAD_YAW], degrees(50.0));
        assert_eq!(high[HEAD_ROLL], degrees(5.0));
        action[HEAD_ROLL] = -10.0;
        let low =
            final_targets(&[0.0; JOINT_COUNT], &action, HeadOverlay::default(), STILL).unwrap();
        assert_eq!(low[HEAD_ROLL], degrees(-8.0));
    }

    #[test]
    fn default_pose_cannot_escape_box() {
        let mut default_pose = [0.0; JOINT_COUNT];
        default_pose[HEAD_ROLL] = degrees(30.0);
        let targets =
            final_targets(&default_pose, &ZERO_ACTION, HeadOverlay::default(), STILL).unwrap();
        assert_eq!(targets[HEAD_ROLL], degrees(5.0));
    }

    #[test]
    fn joints_offset_cannot_move_commanded_head_out_of_box() {
        let mut action = ZERO_ACTION;
        action[NECK_PITCH] = 4.0;
        action[HEAD_PITCH] = -4.0;
        action[HEAD_YAW] = 8.0;
        action[HEAD_ROLL] = 4.0;
        for offset_degrees in [-40.0, -10.0, -3.0, 0.0, 3.0, 10.0, 40.0] {
            let joints_offset = [degrees(offset_degrees); JOINT_COUNT];
            let targets =
                final_targets(&[0.0; JOINT_COUNT], &action, HeadOverlay::default(), STILL).unwrap();
            let servo = servo_commands(&targets, &joints_offset);
            let commanded_joint: [f32; JOINT_COUNT] =
                std::array::from_fn(|joint| servo[joint] - joints_offset[joint]);
            assert!(in_box(&commanded_joint), "offset {offset_degrees} deg");
            assert!((commanded_joint[HEAD_ROLL] - degrees(5.0)).abs() < TOLERANCE);
        }
    }

    #[test]
    fn walking_zeroes_overlay_yaw_and_roll() {
        let overlay = HeadOverlay {
            neck_pitch: degrees(2.0),
            head_pitch: degrees(4.0),
            head_yaw: degrees(8.0),
            head_roll: degrees(-3.0),
        };
        let limited = limit_overlay(overlay, WALK);
        assert_eq!(limited.head_yaw, 0.0);
        assert_eq!(limited.head_roll, 0.0);
        assert_eq!(limited.neck_pitch, overlay.neck_pitch);
        assert_eq!(limited.head_pitch, overlay.head_pitch);
        let targets = final_targets(&[0.0; JOINT_COUNT], &ZERO_ACTION, overlay, WALK).unwrap();
        assert_eq!(targets[HEAD_YAW], 0.0);
        assert_eq!(targets[HEAD_ROLL], 0.0);
        let turning = VelocityCommand { wz: -0.3, ..STILL };
        assert_eq!(limit_overlay(overlay, turning).head_yaw, 0.0);
    }

    #[test]
    fn hold_limits_overlay_yaw_and_nose_up() {
        let overlay = HeadOverlay {
            neck_pitch: 0.0,
            head_pitch: degrees(35.0),
            head_yaw: degrees(-25.0),
            head_roll: degrees(2.0),
        };
        let limited = limit_overlay(overlay, STILL);
        assert_eq!(limited.head_yaw, -HOLD_OVERLAY_YAW_MAX);
        assert_eq!(limited.head_pitch, HOLD_OVERLAY_PITCH_NOSE_UP_MAX);
        assert_eq!(limited.head_roll, overlay.head_roll);
        let nose_down = HeadOverlay {
            head_pitch: degrees(-12.0),
            ..overlay
        };
        assert_eq!(limit_overlay(nose_down, STILL).head_pitch, degrees(-12.0));
        let mut default_pose = [0.0; JOINT_COUNT];
        default_pose[HEAD_PITCH] = degrees(10.0);
        let targets = final_targets(&default_pose, &ZERO_ACTION, overlay, STILL).unwrap();
        assert!(targets[HEAD_PITCH] < HOLD_OVERLAY_PITCH_NOSE_UP_MAX);
    }

    #[test]
    fn non_finite_input_is_rejected() {
        let mut action = ZERO_ACTION;
        action[3] = f32::NAN;
        assert_eq!(
            final_targets(&[0.0; JOINT_COUNT], &action, HeadOverlay::default(), STILL),
            Err(TargetError::NonFinite { joint: 3 })
        );
        let overlay = HeadOverlay {
            head_roll: f32::INFINITY,
            ..HeadOverlay::default()
        };
        assert_eq!(
            final_targets(&[0.0; JOINT_COUNT], &ZERO_ACTION, overlay, STILL),
            Err(TargetError::NonFinite { joint: HEAD_ROLL })
        );
    }

    #[test]
    fn head_command_obs_is_always_zero() {
        let mut observation = [7.0; OBS_LEN];
        write_command_obs(&mut observation, WALK);
        assert_eq!(observation[9..13], [0.0; 4]);
        assert_eq!(observation[6..9], [0.1, 0.0, 0.0]);
        assert_eq!(observation[5], 7.0);
        assert_eq!(observation[13], 7.0);
    }
}
