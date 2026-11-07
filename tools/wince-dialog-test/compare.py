"""Compare probe observations; report remaining differences without hiding them."""
import argparse
import json
import re
from pathlib import Path


def observations(path):
    lines = Path(path).read_text().splitlines()
    if not lines or lines[0] != 'dialog-test 2 (numbers hexadecimal)' or lines[-1] != 'end':
        raise ValueError(f'{path}: incomplete or unsupported probe log')
    result = {}
    case = 'global'
    for line in lines[1:-1]:
        if line.startswith('case '):
            case = line[5:]
        elif line.startswith('metrics '):
            # Metric lines follow each snapshot; retain the snapshot label.
            result[f'{case}/{snapshot}/metrics'] = line
        elif line.startswith(('init window=', 'after create ', 'after show ', 'after normal pump ')):
            snapshot, data = line.split(' window=', 1)
            for name, value in re.findall(r'(window|client|style|edit|focus_is_edit)=([^ ]+)', 'window=' + data):
                result[f'{case}/{snapshot}/{name}'] = value
        else:
            name, _, value = line.partition('=')
            result[f'{case}/{name}'] = value
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('reference')
    parser.add_argument('actual')
    parser.add_argument('--output')
    args = parser.parse_args()
    reference, actual = observations(args.reference), observations(args.actual)
    keys = sorted(reference.keys() | actual.keys())
    differences = [dict(observation=k, reference=reference.get(k), actual=actual.get(k))
                   for k in keys if reference.get(k) != actual.get(k)]
    report = dict(reference=args.reference, actual=args.actual,
                  observations=len(keys), matches=len(keys) - len(differences),
                  differences=differences)
    encoded = json.dumps(report, indent=2) + '\n'
    if args.output:
        Path(args.output).write_text(encoded)
    print(encoded, end='')
    return bool(differences)


if __name__ == '__main__':
    raise SystemExit(main())
