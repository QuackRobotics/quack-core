# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0
"""Sim gate: run the upstream Open Duck Mini v2 walking policy in MuJoCo.

The gate passes only if the duck walks for --duration seconds without
falling and covers at least --min-progress metres forward. Any missing
file, body, sensor or shape mismatch fails it loudly (exit 2); a fall or
too little progress fails it as a gate result (exit 1).

This is our own reimplementation of the upstream inference loop, written
from reading these files (cited by line, our code copies none of them):

  PG  = apirrone/Open_Duck_Playground @ b9be205ac64488c23504ca42e5ec790337adeec3
        playground/open_duck_mini_v2/mujoco_infer_base.py (BASE)
        playground/open_duck_mini_v2/mujoco_infer.py       (INFER)
  The policy file comes from apirrone/Open_Duck_Mini @ b23317a
  (BEST_WALK_ONNX_2.onnx, Apache-2.0). The MJCF, meshes and
  reference-motion pickle come from PG at the commit above.
"""
import argparse
import json
import pathlib
import pickle
import sys

import mujoco
import numpy as np
import onnxruntime as ort

SIM_DT = 0.002          # BASE L15
DECIMATION = 10         # BASE L16 -> policy at 1 / (0.002 * 10) = 50 Hz
DOF_VEL_SCALE = 0.05    # INFER L30
ACTION_SCALE = 0.25     # INFER L31
MAX_MOTOR_VEL = 5.24    # INFER L58, rad/s; clip applied at INFER L216-225
ACCEL_X_BIAS = 1.3      # INFER L74
OBS_DIM = 101           # 3+3+7+14+14+14*3+14+2+2, the order is INFER L84-101
ACT_DIM = 14
BASE_BODY = "base"      # PG xmls/open_duck_mini_v2.xml L58, carries the freejoint
FEET = ("foot_assembly", "foot_assembly_2")  # BASE L280-283
FALL_FRACTION = 0.5     # fall = base height below 50% of standing height


class GateError(Exception):
    """A setup problem: the gate cannot give a meaningful result."""


def require(condition, message):
    if not condition:
        raise GateError(message)


def body_id(model, name):
    index = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_BODY, name)
    require(index >= 0, f"body '{name}' not found in model")
    return index


def sensor_slice(model, name):
    index = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_SENSOR, name)
    require(index >= 0, f"sensor '{name}' not found in model")
    start = model.sensor_adr[index]
    return slice(start, start + model.sensor_dim[index])


def foot_contacts(model, data, foot_ids, floor_id):
    # BASE L259-283: a foot is in contact if any contact pairs its body with the floor.
    found = []
    for foot in foot_ids:
        touching = False
        for contact in data.contact[: data.ncon]:
            pair = {model.geom_bodyid[contact.geom1], model.geom_bodyid[contact.geom2]}
            if pair == {foot, floor_id}:
                touching = True
                break
        found.append(float(touching))
    return np.array(found)


def steps_in_period(pkl_path):
    # PG playground/common/poly_reference_motion_numpy.py: the period length comes
    # from the first entry, as int(period * fps). The file is sha256-pinned before use.
    with open(pkl_path, "rb") as handle:
        table = pickle.load(handle)
    first = next(iter(table.values()))
    return int(first["period"] * first["fps"])


def run(args):
    upstream = pathlib.Path(args.upstream_dir)
    scene = upstream / "playground" / "xmls" / "scene_flat_terrain.xml"
    policy_path = upstream / "BEST_WALK_ONNX_2.onnx"
    pkl_path = upstream / "playground" / "data" / "polynomial_coefficients.pkl"
    for path in (scene, policy_path, pkl_path):
        require(path.is_file(), f"missing upstream file: {path}")

    model = mujoco.MjModel.from_xml_path(str(scene))
    model.opt.timestep = SIM_DT
    data = mujoco.MjData(model)
    require(model.nu == ACT_DIM, f"model has {model.nu} actuators, policy expects {ACT_DIM}")

    session = ort.InferenceSession(str(policy_path))
    model_input = session.get_inputs()[0]
    model_output = session.get_outputs()[0]
    require(list(model_input.shape) == [1, OBS_DIM], f"policy input shape {model_input.shape} != [1, {OBS_DIM}]")
    require(list(model_output.shape) == [1, ACT_DIM], f"policy output shape {model_output.shape} != [1, {ACT_DIM}]")

    base = body_id(model, BASE_BODY)
    floor = body_id(model, "floor")
    feet = [body_id(model, name) for name in FEET]
    gyro = sensor_slice(model, "gyro")
    accel = sensor_slice(model, "accelerometer")

    # BASE L118-128: start from keyframe "home"; its ctrl is the default pose.
    home = model.keyframe("home")
    data.qpos[:] = home.qpos
    data.ctrl[:] = home.ctrl
    default_actuator = home.ctrl.copy()
    joint_qpos_adr = np.array([model.jnt_qposadr[model.actuator_trnid[i, 0]] for i in range(model.nu)])
    joint_qvel_adr = np.array([model.jnt_dofadr[model.actuator_trnid[i, 0]] for i in range(model.nu)])
    mujoco.mj_forward(model, data)

    standing_z = float(data.xpos[base, 2])
    start_x = float(data.xpos[base, 0])
    fall_z = FALL_FRACTION * standing_z
    period = steps_in_period(pkl_path)
    command = np.array([args.cmd_x, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])  # INFER L52, L152-154
    print(f"standing base_z={standing_z:.4f} m  fall_threshold={fall_z:.4f} m  "
          f"policy_rate={1.0 / (SIM_DT * DECIMATION):.0f} Hz  steps_in_period={period}  "
          f"cmd_x={args.cmd_x} m/s  sabotage={args.sabotage}")

    actions = [np.zeros(ACT_DIM) for _ in range(3)]  # last, last_last, last_last_last
    motor_targets = default_actuator.copy()
    prev_targets = default_actuator.copy()
    phase_i = 0.0
    phase = np.zeros(2)
    max_step = MAX_MOTOR_VEL * SIM_DT * DECIMATION
    total_steps = round(args.duration / SIM_DT)

    for step in range(1, total_steps + 1):
        mujoco.mj_step(model, data)  # INFER L170
        if step % DECIMATION == 0:   # INFER L174
            phase_i = (phase_i + 1.0) % period  # INFER L176-179
            angle = phase_i / period * 2 * np.pi
            phase = np.array([np.cos(angle), np.sin(angle)])  # INFER L182-197
            accel_obs = data.sensordata[accel].copy()
            accel_obs[0] += ACCEL_X_BIAS
            obs = np.concatenate([
                data.sensordata[gyro],
                accel_obs,
                command,
                data.qpos[joint_qpos_adr] - default_actuator,
                data.qvel[joint_qvel_adr] * DOF_VEL_SCALE,
                actions[0], actions[1], actions[2],
                motor_targets,
                foot_contacts(model, data, feet, floor),
                phase,
            ]).astype(np.float32)
            require(obs.shape == (OBS_DIM,), f"observation shape {obs.shape} != ({OBS_DIM},)")
            action = session.run(None, {model_input.name: obs.reshape(1, -1)})[0][0]
            if args.sabotage == "zero-action":
                action = np.zeros(ACT_DIM, dtype=np.float32)
            actions = [action.copy(), actions[0], actions[1]]  # INFER L208-210
            motor_targets = default_actuator + action * ACTION_SCALE  # INFER L212-214
            motor_targets = np.clip(motor_targets, prev_targets - max_step, prev_targets + max_step)
            prev_targets = motor_targets.copy()
            data.ctrl[:] = motor_targets  # INFER L231

        base_z = float(data.xpos[base, 2])
        if base_z < fall_z:
            print(f"FELL at t={step * SIM_DT:.2f}s base_z={base_z:.4f} < {fall_z:.4f}")
            return 1, summary(standing_z, base_z, data.xpos[base, 0] - start_x, step * SIM_DT, "fell")

    progress = float(data.xpos[base, 0] - start_x)
    base_z = float(data.xpos[base, 2])
    if progress < args.min_progress:
        print(f"TOO LITTLE PROGRESS: {progress:.3f} m < {args.min_progress} m in {args.duration}s")
        return 1, summary(standing_z, base_z, progress, args.duration, "no_progress")
    return 0, summary(standing_z, base_z, progress, args.duration, "walked")


def summary(standing_z, base_z, progress, seconds, outcome):
    return {"outcome": outcome, "seconds": round(seconds, 2), "standing_z": round(standing_z, 4),
            "final_base_z": round(base_z, 4), "forward_progress_m": round(float(progress), 3)}


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--upstream-dir", default="upstream")
    parser.add_argument("--duration", type=float, default=10.0)
    parser.add_argument("--cmd-x", type=float, default=0.1, help="forward command, m/s (upstream range +-0.15)")
    parser.add_argument("--min-progress", type=float, default=0.3, help="metres forward required")
    parser.add_argument("--sabotage", choices=["none", "zero-action"], default="none",
                        help="deliberately break the policy output to prove the gate can fail")
    args = parser.parse_args()
    try:
        code, result = run(args)
    except GateError as error:
        print(f"SIM GATE SETUP ERROR: {error}")
        return 2
    print("RESULT " + json.dumps(result))
    print("SIM GATE PASSED" if code == 0 else "SIM GATE FAILED")
    return code


if __name__ == "__main__":
    sys.exit(main())