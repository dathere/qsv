#!/usr/bin/env python3
"""Dump real CSV field values for the perf harnesses.

  column-major (H1c, H5), non-empty only:  dump_fields.py <csv> cm  <out> [rows]
  row-major   (layout),   empties kept:    dump_fields.py <csv> rm  <out> [rows]

Empty fields are excluded from the column-major dump because
TypedMinMax::add_with_parsed early-returns on len==0 before reaching add_bytes.
"""
import csv, struct, sys

path, mode, out = sys.argv[1], sys.argv[2], sys.argv[3]
limit = int(sys.argv[4]) if len(sys.argv) > 4 else 200_000

with open(path, 'rb') as f:
    r = csv.reader((l.decode('utf-8', 'replace') for l in f))
    hdr = next(r)
    ncol = len(hdr)
    cols = [[] for _ in range(ncol)]
    rows = 0
    for row in r:
        if len(row) != ncol:
            continue
        for i in range(ncol):
            v = row[i].encode('utf-8', 'replace')[:255]
            if mode == 'rm' or v:
                cols[i].append(v)
        rows += 1
        if rows >= limit:
            break

with open(out, 'wb') as o:
    if mode == 'rm':
        o.write(struct.pack('<II', ncol, rows))
        for c in cols:
            for v in c:
                o.write(struct.pack('<B', len(v))); o.write(v)
    else:
        o.write(struct.pack('<I', ncol))
        for c in cols:
            o.write(struct.pack('<I', len(c)))
            for v in c:
                o.write(struct.pack('<B', len(v))); o.write(v)
print(f"{out}: {ncol} cols x {rows} rows ({sum(len(c) for c in cols)} values)")
