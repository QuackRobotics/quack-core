// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Joint index order used by the policy and every HAL array.
//!
//! The order is the Open Duck Playground f698836 actuator order (Chief ruling, pin A,
//! 2026-11-24): left leg 0-4, head 5-8, right leg 9-13, antennas 14-15. The upstream
//! Open_Duck_Mini b23317a `robot.xml` lists the antennas at 9-10 instead; that order is
//! for mechanics/BOM only. Anything arriving in another order (a MuJoCo model, the servo
//! bus) is mapped by name with [`JointMap::from_names`], never by position.

use std::fmt;

pub const JOINT_COUNT: usize = 16;
/// The policy drives indices `0..POLICY_JOINT_COUNT`; the two SG90 antennas are not policy outputs.
pub const POLICY_JOINT_COUNT: usize = 14;

pub const NECK_PITCH: usize = 5;
pub const HEAD_PITCH: usize = 6;
pub const HEAD_YAW: usize = 7;
pub const HEAD_ROLL: usize = 8;

pub const POLICY_ORDER: [&str; JOINT_COUNT] = [
    "left_hip_yaw",
    "left_hip_roll",
    "left_hip_pitch",
    "left_knee",
    "left_ankle",
    "neck_pitch",
    "head_pitch",
    "head_yaw",
    "head_roll",
    "right_hip_yaw",
    "right_hip_roll",
    "right_hip_pitch",
    "right_knee",
    "right_ankle",
    "left_antenna",
    "right_antenna",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JointMapError {
    WrongCount { expected: usize, got: usize },
    UnknownName(String),
    DuplicateName(String),
}

impl fmt::Display for JointMapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongCount { expected, got } => {
                write!(formatter, "expected {expected} joint names, got {got}")
            }
            Self::UnknownName(name) => write!(formatter, "unknown joint name {name:?}"),
            Self::DuplicateName(name) => write!(formatter, "joint name {name:?} appears twice"),
        }
    }
}

impl std::error::Error for JointMapError {}

/// Policy index of a joint name, or `None` if the name is not a quack-core joint.
pub fn policy_index(name: &str) -> Option<usize> {
    POLICY_ORDER.iter().position(|candidate| *candidate == name)
}

/// Permutation between an external joint order (by name) and [`POLICY_ORDER`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JointMap {
    /// `policy_of_external[i]` is the policy index of external slot `i`.
    policy_of_external: [usize; JOINT_COUNT],
}

impl JointMap {
    pub fn from_names(external_names: &[&str]) -> Result<Self, JointMapError> {
        if external_names.len() != JOINT_COUNT {
            return Err(JointMapError::WrongCount {
                expected: JOINT_COUNT,
                got: external_names.len(),
            });
        }
        let mut policy_of_external = [0; JOINT_COUNT];
        let mut seen = [false; JOINT_COUNT];
        for (external_slot, name) in external_names.iter().enumerate() {
            let index =
                policy_index(name).ok_or_else(|| JointMapError::UnknownName(name.to_string()))?;
            if seen[index] {
                return Err(JointMapError::DuplicateName(name.to_string()));
            }
            seen[index] = true;
            policy_of_external[external_slot] = index;
        }
        Ok(Self { policy_of_external })
    }

    pub fn is_identity(&self) -> bool {
        self.policy_of_external
            .iter()
            .enumerate()
            .all(|(slot, index)| slot == *index)
    }

    pub fn to_policy<T: Copy>(&self, external: &[T; JOINT_COUNT]) -> [T; JOINT_COUNT] {
        let mut policy = *external;
        for (slot, value) in external.iter().enumerate() {
            policy[self.policy_of_external[slot]] = *value;
        }
        policy
    }

    pub fn to_external<T: Copy>(&self, policy: &[T; JOINT_COUNT]) -> [T; JOINT_COUNT] {
        let mut external = *policy;
        for (slot, value) in external.iter_mut().enumerate() {
            *value = policy[self.policy_of_external[slot]];
        }
        external
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Actuator order of upstream Open_Duck_Mini b23317a robot.xml (mechanics/BOM only).
    const B23317A_ORDER: [&str; JOINT_COUNT] = [
        "left_hip_yaw",
        "left_hip_roll",
        "left_hip_pitch",
        "left_knee",
        "left_ankle",
        "neck_pitch",
        "head_pitch",
        "head_yaw",
        "head_roll",
        "left_antenna",
        "right_antenna",
        "right_hip_yaw",
        "right_hip_roll",
        "right_hip_pitch",
        "right_knee",
        "right_ankle",
    ];

    #[test]
    fn policy_order_groups_match_ruling() {
        assert!(POLICY_ORDER[0..5].iter().all(|n| n.starts_with("left_")));
        assert_eq!(
            POLICY_ORDER[NECK_PITCH..=HEAD_ROLL],
            ["neck_pitch", "head_pitch", "head_yaw", "head_roll"]
        );
        assert!(POLICY_ORDER[9..14].iter().all(|n| n.starts_with("right_")));
        assert_eq!(POLICY_ORDER[14..16], ["left_antenna", "right_antenna"]);
    }

    #[test]
    fn b23317a_order_is_not_policy_order() {
        let map = JointMap::from_names(&B23317A_ORDER).unwrap();
        assert!(
            !map.is_identity(),
            "b23317a order must be remapped by name, never used as policy order"
        );
        assert_ne!(B23317A_ORDER, POLICY_ORDER);
    }

    #[test]
    fn b23317a_vector_lands_by_name() {
        let map = JointMap::from_names(&B23317A_ORDER).unwrap();
        let external: [usize; JOINT_COUNT] = std::array::from_fn(|slot| slot);
        let policy = map.to_policy(&external);
        for (index, name) in POLICY_ORDER.iter().enumerate() {
            let slot = B23317A_ORDER.iter().position(|n| n == name).unwrap();
            assert_eq!(policy[index], slot, "{name}");
        }
        assert_eq!(map.to_external(&policy), external);
    }

    #[test]
    fn policy_order_maps_to_identity() {
        assert!(JointMap::from_names(&POLICY_ORDER).unwrap().is_identity());
    }

    #[test]
    fn rejects_bad_name_lists() {
        assert_eq!(
            JointMap::from_names(&POLICY_ORDER[..14]),
            Err(JointMapError::WrongCount {
                expected: 16,
                got: 14
            })
        );
        let mut unknown = POLICY_ORDER;
        unknown[3] = "left_shin";
        assert_eq!(
            JointMap::from_names(&unknown),
            Err(JointMapError::UnknownName("left_shin".into()))
        );
        let mut duplicate = POLICY_ORDER;
        duplicate[15] = "left_antenna";
        assert_eq!(
            JointMap::from_names(&duplicate),
            Err(JointMapError::DuplicateName("left_antenna".into()))
        );
    }
}
