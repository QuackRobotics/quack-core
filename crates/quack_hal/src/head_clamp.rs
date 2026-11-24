// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Head safety box.
//!
//! Applied to the final joint targets (policy output plus any expression overlay), in
//! joint space, before the per-servo calibration offset is added. Applying it after the
//! offset would let a calibration offset shift the box off the head's true zero.

use crate::joints::{Joint, JOINT_COUNT};

/// PENDING: head_roll limit is not ruled yet (Quack Chief, projector clearance review).
/// ±5° is a placeholder until the ruling lands; do not treat it as validated.
pub const HEAD_ROLL_LIMIT_DEG_PENDING: f32 = 5.0;

/// Symmetric limits in degrees, as (joint, limit).
pub const HEAD_LIMITS_DEG: [(Joint, f32); 4] = [
    (Joint::NeckPitch, 15.0),
    (Joint::HeadPitch, 15.0),
    (Joint::HeadYaw, 50.0),
    (Joint::HeadRoll, HEAD_ROLL_LIMIT_DEG_PENDING),
];

/// Clamps the head joints of `targets_rad` (policy order) into the box; other joints untouched.
pub fn clamp_head(targets_rad: &mut [f32; JOINT_COUNT]) {
    for (joint, limit_deg) in HEAD_LIMITS_DEG {
        let limit_rad = limit_deg.to_radians();
        let target = &mut targets_rad[joint.policy_index()];
        *target = target.clamp(-limit_rad, limit_rad);
    }
}

/// Final command to the servos: clamp in joint space, then add calibration offsets.
pub fn servo_targets(
    targets_rad: &[f32; JOINT_COUNT],
    joints_offset_rad: &[f32; JOINT_COUNT],
) -> [f32; JOINT_COUNT] {
    let mut commanded = *targets_rad;
    clamp_head(&mut commanded);
    let mut servo = [0.0; JOINT_COUNT];
    for index in 0..JOINT_COUNT {
        servo[index] = commanded[index] + joints_offset_rad[index];
    }
    servo
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-6;

    fn head_index(joint: Joint) -> usize {
        joint.policy_index()
    }

    #[test]
    fn head_targets_are_clamped_to_box() {
        let mut targets = [1.5; JOINT_COUNT];
        clamp_head(&mut targets);
        for (joint, limit_deg) in HEAD_LIMITS_DEG {
            assert!((targets[head_index(joint)] - limit_deg.to_radians()).abs() < EPS);
        }
        assert_eq!(targets[Joint::LeftKnee.policy_index()], 1.5);
        assert_eq!(targets[Joint::LeftAntenna.policy_index()], 1.5);

        let mut negative = [-1.5; JOINT_COUNT];
        clamp_head(&mut negative);
        let roll = negative[head_index(Joint::HeadRoll)];
        assert!((roll + HEAD_ROLL_LIMIT_DEG_PENDING.to_radians()).abs() < EPS);
    }

    #[test]
    fn targets_inside_box_pass_through() {
        let mut targets = [0.0; JOINT_COUNT];
        targets[head_index(Joint::HeadYaw)] = 40f32.to_radians();
        let before = targets;
        clamp_head(&mut targets);
        assert_eq!(targets, before);
    }

    #[test]
    fn joints_offset_cannot_move_commanded_head_off_zero() {
        let targets = [0.0; JOINT_COUNT];
        let mut offsets = [0.0; JOINT_COUNT];
        for (joint, _) in HEAD_LIMITS_DEG {
            offsets[head_index(joint)] = 20f32.to_radians();
        }
        let servo = servo_targets(&targets, &offsets);
        for (joint, _) in HEAD_LIMITS_DEG {
            let index = head_index(joint);
            assert!(
                (servo[index] - offsets[index]).abs() < EPS,
                "{joint:?} moved off zero"
            );
        }
    }

    #[test]
    fn joints_offset_cannot_push_commanded_head_out_of_box() {
        let mut targets = [0.0; JOINT_COUNT];
        let mut offsets = [0.0; JOINT_COUNT];
        for (joint, _) in HEAD_LIMITS_DEG {
            targets[head_index(joint)] = 90f32.to_radians();
            offsets[head_index(joint)] = -12f32.to_radians();
        }
        let servo = servo_targets(&targets, &offsets);
        for (joint, limit_deg) in HEAD_LIMITS_DEG {
            let index = head_index(joint);
            let commanded = servo[index] - offsets[index];
            assert!(
                (commanded - limit_deg.to_radians()).abs() < EPS,
                "{joint:?}"
            );
        }
    }

    #[test]
    fn offsets_apply_to_every_joint() {
        let targets = [0.1; JOINT_COUNT];
        let offsets = [0.05; JOINT_COUNT];
        let servo = servo_targets(&targets, &offsets);
        assert!((servo[Joint::RightKnee.policy_index()] - 0.15).abs() < EPS);
    }
}
