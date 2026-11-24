// SPDX-FileCopyrightText: 2026 Quack Robotics contributors
// SPDX-License-Identifier: Apache-2.0

//! Joint identities and the policy index order.
//!
//! The policy order comes from Open_Duck_Playground f698836: left leg 0-4, head 5-8,
//! right leg 9-13, antennas 14-15. The upstream MJCF (Open_Duck_Mini b23317a) lists the
//! antennas between the head and the right leg, so any other source order is remapped
//! by joint name, never by position.

pub const JOINT_COUNT: usize = 16;
/// Joints driven by the walking policy (serial bus servos); the antennas are not.
pub const POLICY_JOINT_COUNT: usize = 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Joint {
    LeftHipYaw,
    LeftHipRoll,
    LeftHipPitch,
    LeftKnee,
    LeftAnkle,
    NeckPitch,
    HeadPitch,
    HeadYaw,
    HeadRoll,
    RightHipYaw,
    RightHipRoll,
    RightHipPitch,
    RightKnee,
    RightAnkle,
    LeftAntenna,
    RightAntenna,
}

/// Policy index order (Playground f698836). `POLICY_ORDER[i]` is the joint at index `i`.
pub const POLICY_ORDER: [Joint; JOINT_COUNT] = [
    Joint::LeftHipYaw,
    Joint::LeftHipRoll,
    Joint::LeftHipPitch,
    Joint::LeftKnee,
    Joint::LeftAnkle,
    Joint::NeckPitch,
    Joint::HeadPitch,
    Joint::HeadYaw,
    Joint::HeadRoll,
    Joint::RightHipYaw,
    Joint::RightHipRoll,
    Joint::RightHipPitch,
    Joint::RightKnee,
    Joint::RightAnkle,
    Joint::LeftAntenna,
    Joint::RightAntenna,
];

impl Joint {
    pub fn name(self) -> &'static str {
        match self {
            Joint::LeftHipYaw => "left_hip_yaw",
            Joint::LeftHipRoll => "left_hip_roll",
            Joint::LeftHipPitch => "left_hip_pitch",
            Joint::LeftKnee => "left_knee",
            Joint::LeftAnkle => "left_ankle",
            Joint::NeckPitch => "neck_pitch",
            Joint::HeadPitch => "head_pitch",
            Joint::HeadYaw => "head_yaw",
            Joint::HeadRoll => "head_roll",
            Joint::RightHipYaw => "right_hip_yaw",
            Joint::RightHipRoll => "right_hip_roll",
            Joint::RightHipPitch => "right_hip_pitch",
            Joint::RightKnee => "right_knee",
            Joint::RightAnkle => "right_ankle",
            Joint::LeftAntenna => "left_antenna",
            Joint::RightAntenna => "right_antenna",
        }
    }

    pub fn from_name(name: &str) -> Option<Joint> {
        POLICY_ORDER.into_iter().find(|joint| joint.name() == name)
    }

    /// Index of this joint in the policy order.
    pub fn policy_index(self) -> usize {
        POLICY_ORDER
            .iter()
            .position(|&joint| joint == self)
            .expect("POLICY_ORDER lists every Joint")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NameMapError {
    WrongCount(usize),
    Unknown(String),
    Duplicate(Joint),
}

/// For a source that names its joints in its own order (MJCF actuators, servo ID table),
/// returns `policy_index_of[source_index]`.
pub fn policy_indices_by_name(source_names: &[&str]) -> Result<[usize; JOINT_COUNT], NameMapError> {
    if source_names.len() != JOINT_COUNT {
        return Err(NameMapError::WrongCount(source_names.len()));
    }
    let mut policy_index_of = [0; JOINT_COUNT];
    let mut seen = [false; JOINT_COUNT];
    for (source_index, name) in source_names.iter().enumerate() {
        let joint =
            Joint::from_name(name).ok_or_else(|| NameMapError::Unknown(name.to_string()))?;
        let policy_index = joint.policy_index();
        if seen[policy_index] {
            return Err(NameMapError::Duplicate(joint));
        }
        seen[policy_index] = true;
        policy_index_of[source_index] = policy_index;
    }
    Ok(policy_index_of)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAYGROUND_F698836: [&str; JOINT_COUNT] = [
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

    /// Actuator order of the upstream MJCF (Open_Duck_Mini b23317a robot.xml).
    const MJCF_B23317A: [&str; JOINT_COUNT] = [
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

    fn policy_order_names() -> Vec<&'static str> {
        POLICY_ORDER.iter().map(|joint| joint.name()).collect()
    }

    #[test]
    fn policy_order_is_playground_f698836() {
        assert_eq!(policy_order_names(), PLAYGROUND_F698836);
    }

    #[test]
    fn policy_order_is_not_mjcf_b23317a() {
        assert_ne!(policy_order_names(), MJCF_B23317A);
        assert_eq!(POLICY_ORDER[9], Joint::RightHipYaw);
        assert_eq!(POLICY_ORDER[14], Joint::LeftAntenna);
    }

    #[test]
    fn walking_policy_joints_exclude_antennas() {
        assert!(!POLICY_ORDER[..POLICY_JOINT_COUNT].contains(&Joint::LeftAntenna));
        assert!(!POLICY_ORDER[..POLICY_JOINT_COUNT].contains(&Joint::RightAntenna));
    }

    #[test]
    fn mjcf_order_is_remapped_by_name() {
        let policy_index_of = policy_indices_by_name(&MJCF_B23317A).unwrap();
        for (source_index, name) in MJCF_B23317A.iter().enumerate() {
            assert_eq!(PLAYGROUND_F698836[policy_index_of[source_index]], *name);
        }
        assert_eq!(policy_index_of[9], 14, "left_antenna");
        assert_eq!(policy_index_of[11], 9, "right_hip_yaw");
    }

    #[test]
    fn name_round_trips() {
        for joint in POLICY_ORDER {
            assert_eq!(Joint::from_name(joint.name()), Some(joint));
        }
        assert_eq!(Joint::from_name("tail"), None);
    }

    #[test]
    fn bad_name_tables_are_rejected() {
        assert_eq!(
            policy_indices_by_name(&MJCF_B23317A[..15]),
            Err(NameMapError::WrongCount(15))
        );
        let mut duplicated = MJCF_B23317A;
        duplicated[15] = "left_knee";
        assert_eq!(
            policy_indices_by_name(&duplicated),
            Err(NameMapError::Duplicate(Joint::LeftKnee))
        );
        let mut unknown = MJCF_B23317A;
        unknown[0] = "tail";
        assert_eq!(
            policy_indices_by_name(&unknown),
            Err(NameMapError::Unknown("tail".to_string()))
        );
    }
}
