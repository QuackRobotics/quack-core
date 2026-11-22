#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Quack Robotics contributors
# SPDX-License-Identifier: Apache-2.0
#
# Fetch the sim-gate assets at pinned upstream commits and verify every file
# against a pinned sha256. Any download error or mismatch exits nonzero.
#   - Open_Duck_Playground @ PG_COMMIT: MJCF scene, meshes, reference-motion pickle
#     (sums in sim/playground.SHA256SUMS; no upstream licence yet, fetched, not redistributed)
#   - Open_Duck_Mini @ ODM_COMMIT: BEST_WALK_ONNX_2.onnx (Apache-2.0)
set -euo pipefail

DEST="${1:-upstream}"
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PG_COMMIT=b9be205ac64488c23504ca42e5ec790337adeec3
ODM_COMMIT=b23317a485b3cec7d8417f352478778b3475173c
ONNX_SHA256=3c606f9381a1710cc8fecdb7442787dcbfce3ee9bc02a6f1224774ab2b3a1067

PG_RAW="https://raw.githubusercontent.com/apirrone/Open_Duck_Playground/${PG_COMMIT}/playground/open_duck_mini_v2"
ODM_RAW="https://raw.githubusercontent.com/apirrone/Open_Duck_Mini/${ODM_COMMIT}"

mkdir -p "${DEST}/playground"
count=0
while read -r digest path; do
  [ -n "${digest}" ] || continue
  mkdir -p "${DEST}/playground/$(dirname "${path}")"
  curl -fsSL --retry 3 -o "${DEST}/playground/${path}" "${PG_RAW}/${path}"
  count=$((count + 1))
done < "${HERE}/playground.SHA256SUMS"
(cd "${DEST}/playground" && sha256sum --check --strict --quiet "${HERE}/playground.SHA256SUMS")
echo "playground: ${count} files verified @ ${PG_COMMIT:0:7}"

curl -fsSL --retry 3 -o "${DEST}/BEST_WALK_ONNX_2.onnx" "${ODM_RAW}/BEST_WALK_ONNX_2.onnx"
echo "${ONNX_SHA256}  ${DEST}/BEST_WALK_ONNX_2.onnx" | sha256sum --check --strict