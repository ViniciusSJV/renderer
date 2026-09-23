"""Validate this edition and recalculate historical samples; never runs a benchmark/LLM.

Run from renderer root: python3 ai/acervo/renderer-v1/validate.py NEW_BUNDLE
Uses the pinned renderer CLI. The optional --librarian PATH exercises a local library.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import statistics
import subprocess


def check(condition, message):
    if not condition:
        raise ValueError(message)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('destination', type=Path)
    parser.add_argument('--librarian', type=Path)
    args = parser.parse_args()
    base = Path(__file__).resolve().parent
    root = base.parents[2]
    dossier = json.loads((base / 'dossier.json').read_text())
    catalog = json.loads((base / 'catalog.json').read_text())
    check(not args.destination.exists(), 'Destination must be new')
    check(catalog['id'] == dossier['id'], 'Catalog identity mismatch')
    check(len(catalog['sources']) == len(dossier['sources']), 'Catalog coverage mismatch')
    for source, entry in zip(dossier['sources'], catalog['sources']):
        check(entry['source_id'] == source['id'], 'Source identity mismatch')
        for field in ['path', 'sha256', 'git_commit']:
            check(entry[field] == source[field], f'Edition mismatch: {field}')
        content = (root / source['path']).read_bytes()
        check(hashlib.sha256(content).hexdigest() == source['sha256'], 'Source hash mismatch')
        check(content.decode().splitlines() == source['lines'], 'Source lines mismatch')
        committed = subprocess.check_output(['git', 'show', source['git_commit'] + ':' + source['path']], cwd=root)
        check(committed == content, 'Git edition mismatch')
    raw = (root / 'ai/experimentos/20-render-cpu/external-scene-baseline.txt').read_text()
    rounds = re.findall(r'round=(\d+)\n([^r]+)', raw)
    check([int(i) for i, _ in rounds] == list(range(7)), 'Expected seven rounds')
    samples = []
    for _, body in rounds:
        pairs = re.findall(r'^sample_(\d+)=(\d+)ns$', body, re.M)
        check([int(i) for i, _ in pairs] == list(range(5)), 'Expected five samples per round')
        values = [int(v) for _, v in pairs]
        summary = re.search(r'width=96 height=64 samples=5 min_ns=(\d+) median_ns=(\d+) max_ns=(\d+)', body)
        check(summary is not None, 'Invalid round summary')
        check(list(map(int, summary.groups())) == [min(values), statistics.median(values), max(values)], 'Round statistics mismatch')
        samples.extend(values)
    measured = dict(samples=len(samples), median_ns=statistics.median(samples), min_ns=min(samples), max_ns=max(samples))
    check(measured == dict(samples=35, median_ns=13232986, min_ns=12330831, max_ns=21777221), 'Global statistics mismatch')
    stats = (root / 'ai/experimentos/20-render-cpu/external-scene-statistics.txt').read_text().splitlines()
    check(stats[1:] == ['Amostras: 35 (7 execucoes de 5 amostras)', 'Mediana global: 13232986 ns', 'Minimo global: 12330831 ns', 'Maximo global: 21777221 ns'], 'Derived report mismatch')
    ids = dossier['selection']['fact_ids']
    destination = args.destination.resolve()
    if args.librarian:
        command = ['cargo', 'run', '--locked', '--manifest-path', str(args.librarian.resolve() / 'Cargo.toml'), '-p', 'librarian-graph-engine', '--example', 'export', '--', str(base / 'dossier.json'), str(base / 'question.txt'), str(destination), '2', *ids]
    else:
        command = ['cargo', 'run', '--locked', '--bin', 'validate_evidence', '--', str(base / 'dossier.json')]
        for fact in ids:
            command += ['--fact', fact]
        command += ['--context', '2', '--question', str(base / 'question.txt'), '--bundle', str(destination)]
    subprocess.run(command, cwd=root, check=True)
    query = json.loads((destination / 'query.json').read_text())
    check([s['fact_id'] for s in query['evidence']['selections']] == ids, 'Selection order mismatch')
    for selection in query['evidence']['selections']:
        check(selection['evidence_unknowns'] == dossier['unknowns'], 'Limits lost in export')
    origin = json.loads((destination / 'origin.json').read_text())
    for name in ['dossier', 'question', 'query']:
        check(hashlib.sha256((destination / origin[name]['file']).read_bytes()).hexdigest() == origin[name]['sha256'], 'Bundle hash mismatch')
    print(json.dumps(dict(status='passed', sources=len(catalog['sources']), selected_facts=ids, recalculated=measured, llm_evaluation='not_performed'), ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
