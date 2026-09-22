"""Recalcula o resultado histórico; não executa benchmark nem autentica sua origem."""
import json
import statistics
from pathlib import Path

path = Path(__file__).resolve().parent.parent / "comparison.json"
data = json.loads(path.read_text())
assert (data["threads"], data["profile"], data["width"], data["height"]) == (4, "release", 64, 64)
assert data["warmups_per_run"] == 3 and data["samples_per_run"] == 5
assert len(data["runs"]) == 14
for round_id in range(7):
    runs = data["runs"][round_id * 2:round_id * 2 + 2]
    expected = ["before", "after"] if round_id % 2 == 0 else ["after", "before"]
    assert [r["version"] for r in runs] == expected
    assert all(r["round"] == round_id for r in runs)
    assert all(len(r["samples_ns"]) == 5 for r in runs)
    assert all(isinstance(n, int) and n > 0 for r in runs for n in r["samples_ns"])
medians = {}
for version in ("before", "after"):
    samples = [n for r in data["runs"] if r["version"] == version for n in r["samples_ns"]]
    assert len(samples) == 35
    medians[version] = statistics.median(samples)
    assert medians[version] == data["median_ns"][version]
    print(f"{version}: n={len(samples)} median_ns={medians[version]} min_ns={min(samples)} max_ns={max(samples)}")
reduction = 100 * (1 - medians["after"] / medians["before"])
assert abs(reduction - data["reduction_percent"]) < 1e-10
print(f"reduction_percent={reduction:.8f}")
print("Escopo: consistência aritmética do arquivo histórico; sem nova medição ou autenticação da coleta.")
