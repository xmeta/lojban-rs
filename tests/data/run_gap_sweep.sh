#!/usr/bin/env bash
# GAP 掃引の全自動実行スクリプト。
#
# 1. tests/data/generate_gap_probes.py でプローブ行列を生成
# 2. 本パーサーを cargo build --release でビルドし --lines バッチで判定
# 3. 参照パーサー z0/z1/maftufa(gerna_cipra)を tests/data/refparse.js で判定
# 4. 比較表を tests/data/gap_sweep_results.csv に書き出す
#
# 前提: git clone --depth 1 https://github.com/guskant/gerna_cipra 済みで
#       $GERNA_CIPRA_JS(既定 /tmp/opencode/gerna_cipra/js)に *.js があること。
# node / python3 が必要。
set -euo pipefail
cd "$(dirname "$0")/../.."

DATA=tests/data
PROBES=$DATA/gap_probes.txt
CSV=$DATA/gap_sweep_results.csv
JS_DIR=${GERNA_CIPRA_JS:-/tmp/opencode/gerna_cipra/js}

python3 $DATA/generate_gap_probes.py
cargo build --release --quiet
BIN=target/release/lojban

# 本パーサー: 失敗行は stderr に「N: 解析エラー…」形式で出る
"$BIN" --lines -q -f "$PROBES" 2> "$DATA/.ours_err.txt" || true
grep -o '^[0-9]*' "$DATA/.ours_err.txt" > "$DATA/.ours_failed.txt"

for p in z0 z1 maftufa; do
  node $DATA/refparse.js "$p" "$PROBES" > "$DATA/.ref_$p.txt"
done

python3 - "$PROBES" "$CSV" "$DATA/.ours_failed.txt" \
    "$DATA/.ref_z0.txt" "$DATA/.ref_z1.txt" "$DATA/.ref_maftufa.txt" <<'PYEOF'
import csv, sys

probes_path, csv_path, ours_path, z0_path, z1_path, maftufa_path = sys.argv[1:7]
probes = [l for l in open(probes_path, encoding="utf-8").read().split("\n") if l]
ours_failed = {int(l) for l in open(ours_path) if l.strip()}
def col(p):
    return [l.strip() for l in open(p) if l.strip()]
z0, z1, maftufa = col(z0_path), col(z1_path), col(maftufa_path)
assert len(probes) == len(z0) == len(z1) == len(maftufa), (
    len(probes), len(z0), len(z1), len(maftufa))

with open(csv_path, "w", newline="", encoding="utf-8") as f:
    w = csv.writer(f)
    w.writerow(["line_no", "input", "ours", "z0", "z1", "maftufa"])
    for i, probe in enumerate(probes, 1):
        w.writerow([
            i, probe,
            0 if i in ours_failed else 1,
            z0[i - 1], z1[i - 1], maftufa[i - 1],
        ])
print(f"wrote {len(probes)} rows -> {csv_path}")
PYEOF

rm -f "$DATA/.ours_err.txt" "$DATA/.ours_failed.txt" "$DATA/.ref_z0.txt" \
      "$DATA/.ref_z1.txt" "$DATA/.ref_maftufa.txt"

# GAP(参照 ok / ours err)と OVER(ours ok / 参照全 err)のサマリ
python3 - "$CSV" <<'PYEOF'
import csv, sys

rows = list(csv.DictReader(open(sys.argv[1], encoding="utf-8")))
gaps = [r for r in rows if r["ours"] == "0"
        and (r["z0"] == "1" or r["z1"] == "1" or r["maftufa"] == "1")]
overs = [r for r in rows if r["ours"] == "1"
         and r["z0"] == "0" and r["z1"] == "0" and r["maftufa"] == "0"]
print(f"total={len(rows)} ours_ok={sum(1 for r in rows if r['ours']=='1')}"
      f" z0_ok={sum(1 for r in rows if r['z0']=='1')}"
      f" z1_ok={sum(1 for r in rows if r['z1']=='1')}"
      f" maftufa_ok={sum(1 for r in rows if r['maftufa']=='1')}")
print(f"--- GAP (ref ok / ours err): {len(gaps)}")
for r in gaps:
    print(f"  L{r['line_no']:>5} {r['input']}  [z0={r['z0']} z1={r['z1']} maf={r['maftufa']}]")
print(f"--- OVER (ours ok / all ref err): {len(overs)}")
for r in overs:
    print(f"  L{r['line_no']:>5} {r['input']}")
PYEOF
