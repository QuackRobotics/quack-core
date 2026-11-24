# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0
"""Unit tests for sim/sim_gate.py that need no upstream assets."""
import importlib.util
import pathlib
import pickle
import sys

SPEC = importlib.util.spec_from_file_location(
    "sim_gate", pathlib.Path(__file__).resolve().parent.parent / "sim_gate.py"
)
sim_gate = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(sim_gate)


def test_missing_upstream_is_a_setup_error_not_a_pass(tmp_path, monkeypatch, capsys):
    monkeypatch.setattr(sys, "argv", ["sim_gate.py", "--upstream-dir", str(tmp_path)])
    assert sim_gate.main() == 2
    assert "SIM GATE SETUP ERROR: missing upstream file" in capsys.readouterr().out


def test_steps_in_period_uses_first_entry(tmp_path):
    table = {"a": {"period": 0.54, "fps": 50}, "b": {"period": 9.0, "fps": 1}}
    path = tmp_path / "coeffs.pkl"
    path.write_bytes(pickle.dumps(table))
    assert sim_gate.steps_in_period(path) == 27


def test_observation_layout_adds_up_to_policy_input():
    # gyro, accel, command, qpos, qvel, 3 past actions, targets, contacts, phase
    parts = [3, 3, 7, sim_gate.ACT_DIM, sim_gate.ACT_DIM, 3 * sim_gate.ACT_DIM, sim_gate.ACT_DIM, 2, 2]
    assert sum(parts) == sim_gate.OBS_DIM == 101


def test_policy_rate_is_50_hz():
    assert round(1.0 / (sim_gate.SIM_DT * sim_gate.DECIMATION)) == 50