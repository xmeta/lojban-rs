// 参照パーサー(zantufa-0.9999 / zantufa-1.9999 / maftufa-1.9999)の一括判定ドライバ。
//
// 使い方: node tests/data/refparse.js <z0|z1|maftufa> <probes.txt>
//   出力: 1行=1プローブの 1(ok) / 0(err)
//
// 注意: 各文法モジュール末尾には process.argv[2] を消費する CLI デモブロックが
// あるため、require 時に argv を退避して発火を防ぐ(argv を渡さないこと)。
// parse 前に camxes_preproc.js の前処理を適用する。
const savedArgv = process.argv;
process.argv = [savedArgv[0]];
const path = require('path');
const fs = require('fs');

const which = savedArgv[2];
const file = savedArgv[3];
const dir = process.env.GERNA_CIPRA_JS || '/tmp/opencode/gerna_cipra/js';
const parserPath = {
  z0: path.join(dir, 'zantufa-0.9999.js'),
  z1: path.join(dir, 'zantufa-1.9999.js'),
  maftufa: path.join(dir, 'maftufa-1.9999.js'),
}[which];
if (!parserPath || !file) {
  console.error('usage: node refparse.js <z0|z1|maftufa> <probes.txt>');
  process.exit(2);
}
if (!fs.existsSync(parserPath)) {
  console.error('parser not found: ' + parserPath);
  console.error('git clone --depth 1 https://github.com/guskant/gerna_cipra first');
  process.exit(2);
}
const camxes = require(parserPath);
const preprocessing = require(path.join(dir, 'camxes_preproc.js')).preprocessing;

const results = [];
for (const line of fs.readFileSync(file, 'utf8').split('\n')) {
  if (line === '') continue; // ファイル終端の改行
  let ok = true;
  try {
    camxes.parse(preprocessing(line));
  } catch (e) {
    ok = false;
  }
  results.push(ok ? 1 : 0);
}
console.log(results.join('\n'));
process.argv = savedArgv;
