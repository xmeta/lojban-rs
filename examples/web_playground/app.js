const source = document.querySelector('#source');
const parseButton = document.querySelector('#parse-button');
const clearButton = document.querySelector('#clear-button');
const sampleSelect = document.querySelector('#sample-select');
const shareButton = document.querySelector('#share-button');
const copyButton = document.querySelector('#copy-button');
const downloadButton = document.querySelector('#download-button');
const status = document.querySelector('#status');
const statusText = document.querySelector('#status-text');
const latency = document.querySelector('#latency');
const stats = document.querySelector('#stats');
const treeView = document.querySelector('#tree-view');
const wordView = document.querySelector('#word-view');
const jsonView = document.querySelector('#json-view');
const sexprView = document.querySelector('#sexpr-view');
const errorView = document.querySelector('#error-view');
const charCount = document.querySelector('#char-count');
const selectionInfo = document.querySelector('#selection-info');
const historyList = document.querySelector('#history-list');
const clearHistoryButton = document.querySelector('#clear-history');
const expandTreeButton = document.querySelector('#expand-tree');
const collapseTreeButton = document.querySelector('#collapse-tree');
const toast = document.querySelector('#toast');
const inspector = document.querySelector('#inspector');
const inspectorRule = document.querySelector('#inspector-rule');
const inspectorDescription = document.querySelector('#inspector-description');
const inspectorText = document.querySelector('#inspector-text');
const inspectorRange = document.querySelector('#inspector-range');
const inspectorDepth = document.querySelector('#inspector-depth');
const inspectorChildren = document.querySelector('#inspector-children');
const inspectorPath = document.querySelector('#inspector-path');
const closeInspectorButton = document.querySelector('#close-inspector');
const regressionSource = document.querySelector('#regression-source');
const regressionRunButton = document.querySelector('#regression-run');
const regressionUseCurrentButton = document.querySelector('#regression-use-current');
const regressionSummary = document.querySelector('#regression-summary');
const regressionResults = document.querySelector('#regression-results');

const HISTORY_KEY = 'lojban-playground-history-v1';
const DRAFT_KEY = 'lojban-playground-draft-v1';
const encoder = new TextEncoder();
let debounceTimer;
let toastTimer;
let activeRequest = 0;
let activeTab = 'tree';
let lastData = null;
let activeRangeElement = null;

const RULE_HELP = {
  text: '入力全体を表す解析木のルートです。',
  content: '先頭の .i などを除いた、実際の発話内容です。',
  item: '文・フラグメント・自由修飾語など、発話を構成する単位です。',
  sentence: '通常のLojban文です。項と述語から構成されます。',
  prenex_sentence: 'zo\'u を使う前置スコープ文です。',
  gek_sentence: 'ganai … gi … などの先接続文です。',
  fragment: '項だけなど、完全な文ではない発話断片です。',
  free: '感情標識・呼格・注釈などの自由修飾要素です。',
  terms_full: '主語等の項リスト、任意の cu、述語をまとめた構造です。',
  terms: '複数の項の並びです。自由修飾語や接続も含められます。',
  term: '文中の1つの項またはタグ付き要素です。',
  tagged: 'FA・BAI・FIhO・時制などのタグが付いた項です。',
  na_ku: '項位置で用いる否定 naku / na ku です。',
  termset: 'nu\'i … nu\'u による項setです。',
  bridi_tail: '述語と、それに続く項のまとまりです。gi\'e 等の連鎖も含みます。',
  tail_terms: '述語に続く項の列と、任意の vau です。',
  sumti: '人物・物・命題などを指す「項」です。',
  KOhA_clause: 'mi・do・ri・ke\'a・di\'u などの代名詞です。',
  desc: 'le / lo / la などで作る記述句です。',
  quant_desc: '数量詞を伴う記述句です。',
  quant_selbri: 'pa prenu のような数量詞+述語の項です。',
  bare_number: '単独で項として使われる数詞です。',
  abstraction: 'nu / ka / du\'u などで文を抽象化した項です。',
  lahe_sumti: 'la\'e / lu\'e などで参照先を変換した項です。',
  lu_quote: 'lu … li\'u による文引用です。',
  zo_quote: 'zo の直後の1語を引用します。',
  zoi_quote: 'zoi DELIM … DELIM による任意テキスト引用です。',
  lohu_quote: 'lo\'u … le\'u による誤文引用です。',
  li_mex: 'li … lo\'o による数理表現です。',
  tanru: '複数の述語を組み合わせた複合述語です。',
  tanru_unit: 'tanruを構成する1つの述語単位です。',
  tense_marks: '時制・相・方位・モダルなどのマーク列です。',
  s_marks: 'na / ja\'a / se / na\'e などの述語マークです。',
  co_tail: 'co によるtanruの逆順構造です。',
  guhek_selbri: 'gu\'e … gi による先接続述語です。',
};

function setStatus(kind, text) {
  status.className = `status ${kind}`;
  statusText.textContent = text;
}

function updateCharCount() {
  const count = Array.from(source.value).length;
  charCount.textContent = `${count} char${count === 1 ? '' : 's'}`;
}

function renderStats(value) {
  stats.replaceChildren();
  const order = ['tokens', 'cmavo', 'gismu', 'lujvo', 'fuivla', 'cmevla', 'unknown'];
  for (const key of order) {
    if (key !== 'tokens' && !value[key]) continue;
    const pill = document.createElement('span');
    pill.className = 'stat-pill';
    pill.textContent = `${key} ${value[key]}`;
    stats.append(pill);
  }
}

function byteOffsetToIndex(text, target) {
  if (target <= 0) return 0;
  let bytes = 0;
  let units = 0;
  for (const char of text) {
    const nextBytes = bytes + encoder.encode(char).length;
    if (nextBytes > target) return units;
    bytes = nextBytes;
    units += char.length;
    if (bytes === target) return units;
  }
  return text.length;
}
function ruleDescription(rule) {
  if (RULE_HELP[rule]) return RULE_HELP[rule];
  if (rule.endsWith('_clause')) return `${rule.replace(/_clause$/, '')} selma'o の語境界付きcmavo規則です。`;
  if (rule.endsWith('_core')) return `${rule.replace(/_core$/, '')} selma'o の語彙コア規則です。`;
  if (/gismu/i.test(rule)) return '5文字の基本内容語(gismu)を認識する形態論規則です。';
  if (/lujvo/i.test(rule)) return 'rafsiを組み合わせた複合内容語(lujvo)を認識する規則です。';
  if (/fuhivla/i.test(rule)) return "借用語・自由形式内容語(fu'ivla)を認識する規則です。";
  if (/cmevla/i.test(rule)) return '固有名詞(cmevla)を認識する形態論規則です。';
  return 'pest文法内の内部規則です。詳細は docs/parsing-guide.md と src/grammar/lojban.pest を参照してください。';
}

function clearInspector() {
  inspector.hidden = true;
}

function showInspector(node, depth, path) {
  inspector.hidden = false;
  inspectorRule.textContent = node.rule;
  inspectorDescription.textContent = ruleDescription(node.rule);
  inspectorText.textContent = node.text || '(empty)';
  inspectorRange.textContent = Number.isInteger(node.start) ? `${node.start}–${node.end}` : '—';
  inspectorDepth.textContent = String(depth);
  inspectorChildren.textContent = String(node.children?.length || 0);
  inspectorPath.textContent = path.join(' › ');
}

function clearRangeHighlight() {
  activeRangeElement?.classList.remove('range-active');
  activeRangeElement = null;
  selectionInfo.textContent = 'Ctrl/⌘ + Enter: Parse · Ctrl/⌘ + K: Focus';
}

function selectSourceRange(startByte, endByte, element) {
  clearRangeHighlight();
  const start = byteOffsetToIndex(source.value, startByte);
  const end = byteOffsetToIndex(source.value, endByte);
  source.focus();
  source.setSelectionRange(start, end);
  element?.classList.add('range-active');
  activeRangeElement = element || null;
  selectionInfo.textContent = `selected bytes ${startByte}–${endByte}`;
}

function bindRange(element, node, onActivate) {
  if (!Number.isInteger(node.start) || !Number.isInteger(node.end)) return;
  element.classList.add('range-target');
  element.tabIndex = 0;
  element.setAttribute('role', 'button');
  element.title = `Select source bytes ${node.start}–${node.end}`;
  const activate = (event) => {
    event.stopPropagation();
    event.preventDefault();
    selectSourceRange(node.start, node.end, element);
    onActivate?.();
  };
  element.addEventListener('click', activate);
  element.addEventListener('keydown', (event) => {
    if (event.key === 'Enter' || event.key === ' ') activate(event);
  });
}
function renderTree(node, depth = 0, parentPath = []) {
  const path = [...parentPath, node.rule];
  const inspect = () => showInspector(node, depth, path);
  if (!node.children?.length) {
    const leaf = document.createElement('div');
    leaf.className = 'tree-leaf';
    const rule = ruleLabel(node.rule);
    const text = textLabel(node.text);
    bindRange(rule, node, inspect);
    bindRange(text, node, inspect);
    leaf.append(rule, text);
    return leaf;
  }

  const details = document.createElement('details');
  details.className = 'tree-node';
  details.open = depth < 2;
  const summary = document.createElement('summary');
  const rule = ruleLabel(node.rule);
  const text = textLabel(compactText(node.text));
  bindRange(rule, node, inspect);
  bindRange(text, node, inspect);
  summary.append(rule, text);
  details.append(summary);
  for (const child of node.children) details.append(renderTree(child, depth + 1, path));
  return details;
}

function ruleLabel(rule) {
  const span = document.createElement('span');
  span.className = 'rule';
  span.textContent = rule;
  return span;
}

function textLabel(text) {
  const span = document.createElement('span');
  span.className = 'node-text';
  span.textContent = JSON.stringify(text);
  return span;
}

function compactText(text) {
  const oneLine = text.replace(/\s+/g, ' ').trim();
  return oneLine.length > 48 ? `${oneLine.slice(0, 45)}…` : oneLine;
}
function renderWords(leaves) {
  wordView.replaceChildren();
  for (const leaf of leaves) {
    const card = document.createElement('button');
    card.className = 'word-card';
    card.type = 'button';
    bindRange(card, leaf, () => showInspector(leaf, 0, ['word', leaf.rule]));

    const word = document.createElement('div');
    word.className = 'word-text';
    word.textContent = leaf.text;

    const meta = document.createElement('div');
    meta.className = 'word-meta';
    const wordClass = document.createElement('span');
    wordClass.className = 'word-class';
    wordClass.textContent = leaf.class;
    const wordRule = document.createElement('span');
    wordRule.className = 'word-rule';
    wordRule.textContent = leaf.rule;
    const range = document.createElement('span');
    range.textContent = `${leaf.start}–${leaf.end}`;
    meta.append(wordClass, document.createElement('br'), wordRule, document.createElement('br'), range);
    card.append(word, meta);
    wordView.append(card);
  }
}

function wordsAsTsv(leaves) {
  const rows = leaves.map((leaf) =>
    [leaf.text, leaf.class, leaf.rule, leaf.start, leaf.end].join('\t'));
  return ['text\tclass\trule\tstart\tend', ...rows].join('\n');
}

function switchTab(name) {
  const tab = document.querySelector(`.tab[data-tab="${name}"]`);
  if (!tab) return;
  activeTab = name;
  document.querySelectorAll('.tab').forEach((item) => item.classList.toggle('active', item === tab));
  document.querySelectorAll('.tab-content').forEach((item) =>
    item.classList.toggle('active', item.id === `tab-${name}`));
}
function currentOutput() {
  if (!lastData?.ok) return null;
  switch (activeTab) {
    case 'tree': return { text: lastData.tree, type: 'text/plain', ext: 'tree.txt' };
    case 'words': return { text: wordsAsTsv(lastData.leaves), type: 'text/tab-separated-values', ext: 'words.tsv' };
    case 'json': return { text: lastData.pretty, type: 'application/json', ext: 'ast.json' };
    case 'sexpr': return { text: lastData.sexpr, type: 'text/plain', ext: 'sexpr.txt' };
    default: return null;
  }
}

function showToast(message) {
  clearTimeout(toastTimer);
  toast.textContent = message;
  toast.hidden = false;
  toastTimer = setTimeout(() => { toast.hidden = true; }, 1800);
}

async function copyText(text) {
  if (navigator.clipboard?.writeText) {
    await navigator.clipboard.writeText(text);
    return;
  }
  const helper = document.createElement('textarea');
  helper.value = text;
  helper.style.position = 'fixed';
  helper.style.opacity = '0';
  document.body.append(helper);
  helper.select();
  document.execCommand('copy');
  helper.remove();
}
async function copyCurrent() {
  const output = currentOutput();
  if (!output) return showToast('コピーできる解析結果がありません');
  try {
    await copyText(output.text);
    showToast(`${activeTab} をコピーしました`);
  } catch (error) {
    showToast(`コピーに失敗しました: ${error}`);
  }
}

function downloadCurrent() {
  const output = currentOutput();
  if (!output) return showToast('保存できる解析結果がありません');
  const blob = new Blob([output.text], { type: `${output.type};charset=utf-8` });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `lojban-${output.ext}`;
  document.body.append(link);
  link.click();
  link.remove();
  URL.revokeObjectURL(url);
  showToast(`${activeTab} を保存しました`);
}

async function shareCurrent() {
  const url = new URL(window.location.href);
  url.search = '';
  if (source.value) url.searchParams.set('q', source.value);
  try {
    await copyText(url.toString());
    history.replaceState(null, '', url);
    showToast('共有URLをコピーしました');
  } catch (error) {
    showToast(`共有URLのコピーに失敗しました: ${error}`);
  }
}
function loadHistory() {
  try {
    const parsed = JSON.parse(localStorage.getItem(HISTORY_KEY) || '[]');
    return Array.isArray(parsed) ? parsed.filter((item) => typeof item === 'string') : [];
  } catch {
    return [];
  }
}

function saveHistory(text) {
  const value = text.trim();
  if (!value) return;
  const next = [value, ...loadHistory().filter((item) => item !== value)].slice(0, 8);
  try { localStorage.setItem(HISTORY_KEY, JSON.stringify(next)); } catch { return; }
  renderHistory();
}

function renderHistory() {
  historyList.replaceChildren();
  const items = loadHistory();
  if (!items.length) {
    const empty = document.createElement('span');
    empty.className = 'history-empty';
    empty.textContent = 'まだ履歴はありません';
    historyList.append(empty);
    return;
  }
  for (const text of items) {
    const button = document.createElement('button');
    button.className = 'history-chip';
    button.type = 'button';
    button.textContent = text.replace(/\s+/g, ' ');
    button.title = text;
    button.addEventListener('click', () => setSource(text, true));
    historyList.append(button);
  }
}
function syncSample() {
  const match = [...sampleSelect.options].find((option) => option.value === source.value);
  if (match) sampleSelect.value = match.value;
  else sampleSelect.selectedIndex = -1;
}

function storeDraft() {
  try { localStorage.setItem(DRAFT_KEY, source.value); } catch { /* storage unavailable */ }
}

function setSource(text, parse = false) {
  source.value = text;
  updateCharCount();
  clearRangeHighlight();
  syncSample();
  storeDraft();
  if (parse) {
    clearTimeout(debounceTimer);
    parseNow();
  }
  source.focus();
}

function clearOutputs() {
  treeView.replaceChildren();
  wordView.replaceChildren();
  jsonView.textContent = '';
  sexprView.textContent = '';
}

function selectErrorLocation(details) {
  if (!details || !Number.isInteger(details.start)) return;
  const start = byteOffsetToIndex(source.value, details.start);
  let end = byteOffsetToIndex(source.value, details.end ?? details.start);
  if (end <= start && start < source.value.length) {
    const point = source.value.codePointAt(start);
    end = Math.min(source.value.length, start + (point > 0xffff ? 2 : 1));
  }
  clearRangeHighlight();
  source.focus();
  source.setSelectionRange(start, end);
  selectionInfo.textContent = `error at line ${details.line}, column ${details.column} · byte ${details.start}`;
}

function renderParseError(data) {
  errorView.replaceChildren();
  const message = document.createElement('div');
  message.className = 'error-message';
  message.textContent = data.error;
  errorView.append(message);

  const details = data.details;
  if (!details) return;
  const meta = document.createElement('div');
  meta.className = 'error-meta';
  meta.textContent = `line ${details.line}, column ${details.column} · bytes ${details.start}–${details.end}`;
  errorView.append(meta);

  if (details.expected?.length) {
    const heading = document.createElement('div');
    heading.className = 'expected-heading';
    heading.textContent = 'Expected grammar rules';
    const chips = document.createElement('div');
    chips.className = 'expected-rules';
    for (const rule of details.expected) {
      const chip = document.createElement('button');
      chip.type = 'button';
      chip.className = 'expected-rule';
      chip.textContent = rule;
      chip.title = ruleDescription(rule);
      chip.addEventListener('click', () => showInspector({ rule, text: '', start: details.start, end: details.end }, 0, ['parse error', rule]));
      chips.append(chip);
    }
    errorView.append(heading, chips);
  }
  selectErrorLocation(details);
}

async function parseNow() {
  const requestId = ++activeRequest;
  const inputText = source.value;
  const started = performance.now();
  clearInspector();
  setStatus('pending', 'Parsing…');
  latency.textContent = '';
  errorView.hidden = true;

  try {
    const data = await window.lojbanTransport.parse(inputText);
    if (requestId !== activeRequest) return;
    const roundTrip = performance.now() - started;
    renderStats(data.stats);
    latency.textContent = Number.isFinite(data.elapsed_ms)
      ? `${data.elapsed_ms.toFixed(2)} ms parser · ${window.lojbanTransport.timingLabel(roundTrip)}`
      : window.lojbanTransport.timingLabel(roundTrip);

    if (!data.ok) {
      lastData = null;
      setStatus('error', 'Parse error');
      clearOutputs();
      renderParseError(data);
      errorView.hidden = false;
      return;
    }
    lastData = data;
    setStatus('ok', 'Valid Lojban');
    treeView.replaceChildren(renderTree(data.ast));
    renderWords(data.leaves);
    jsonView.textContent = data.pretty;
    sexprView.textContent = data.sexpr;
    saveHistory(inputText);
  } catch (error) {
    if (requestId !== activeRequest) return;
    lastData = null;
    setStatus('error', 'Request failed');
    latency.textContent = `${(performance.now() - started).toFixed(1)} ms`;
    clearOutputs();
    errorView.textContent = String(error);
    errorView.hidden = false;
  }
}

function scheduleParse() {
  const url = new URL(window.location.href);
  if (url.searchParams.has('q')) {
    url.searchParams.delete('q');
    history.replaceState(null, '', url);
  }
  updateCharCount();
  clearRangeHighlight();
  syncSample();
  storeDraft();
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(parseNow, 240);
}

function regressionMetric(label, value, kind = '') {
  const item = document.createElement('div');
  item.className = `regression-metric ${kind}`.trim();
  const strong = document.createElement('strong');
  strong.textContent = value;
  const span = document.createElement('span');
  span.textContent = label;
  item.append(strong, span);
  return item;
}

function renderRegression(data, roundTrip) {
  regressionSummary.replaceChildren();
  const rate = data.total ? (data.passed / data.total) * 100 : 0;
  regressionSummary.append(
    regressionMetric('cases', String(data.total)),
    regressionMetric('passed', String(data.passed), 'pass'),
    regressionMetric('failed', String(data.failed), data.failed ? 'fail' : 'pass'),
    regressionMetric('pass rate', `${rate.toFixed(1)}%`, data.failed ? '' : 'pass'),
    regressionMetric('parser total', `${data.elapsed_ms.toFixed(2)} ms`),
    regressionMetric('round trip', `${roundTrip.toFixed(1)} ms`),
  );
  if (data.truncated) {
    const warning = document.createElement('div');
    warning.className = 'regression-warning';
    warning.textContent = '上限200ケースまでを実行しました。残りは省略されています。';
    regressionSummary.append(warning);
  }

  regressionResults.replaceChildren();
  for (const testCase of data.cases) {
    const row = document.createElement('tr');
    row.className = testCase.ok ? 'case-pass' : 'case-fail';
    const line = document.createElement('td');
    line.textContent = String(testCase.line);
    const state = document.createElement('td');
    const badge = document.createElement('span');
    badge.className = `case-badge ${testCase.ok ? 'pass' : 'fail'}`;
    badge.textContent = testCase.ok ? 'PASS' : 'FAIL';
    state.append(badge);
    const input = document.createElement('td');
    const open = document.createElement('button');
    open.type = 'button';
    open.className = 'case-input';
    open.textContent = testCase.text;
    open.title = 'Playgroundでこのケースを開く';
    open.addEventListener('click', () => setSource(testCase.text, true));
    input.append(open);
    const time = document.createElement('td');
    time.textContent = `${testCase.elapsed_ms.toFixed(2)} ms`;
    const diagnostic = document.createElement('td');
    if (testCase.ok) {
      diagnostic.textContent = '—';
    } else {
      const message = document.createElement('div');
      message.className = 'case-error';
      message.textContent = testCase.error;
      diagnostic.append(message);
      if (testCase.details?.expected?.length) {
        const expected = document.createElement('div');
        expected.className = 'case-expected';
        expected.textContent = `expected: ${testCase.details.expected.slice(0, 5).join(', ')}`;
        diagnostic.append(expected);
      }
    }
    row.append(line, state, input, time, diagnostic);
    regressionResults.append(row);
  }
}

async function runRegression() {
  regressionRunButton.disabled = true;
  regressionRunButton.textContent = 'Running…';
  regressionSummary.textContent = 'Batch parsing…';
  regressionResults.replaceChildren();
  const started = performance.now();
  try {
    const data = await window.lojbanTransport.regression(regressionSource.value);
    renderRegression(data, performance.now() - started);
  } catch (error) {
    regressionSummary.textContent = `Regression request failed: ${error}`;
  } finally {
    regressionRunButton.disabled = false;
    regressionRunButton.textContent = 'Run batch';
  }
}

for (const tab of document.querySelectorAll('.tab')) {
  tab.addEventListener('click', () => switchTab(tab.dataset.tab));
}
sampleSelect.addEventListener('change', () => setSource(sampleSelect.value, true));
source.addEventListener('input', scheduleParse);
parseButton.addEventListener('click', () => {
  clearTimeout(debounceTimer);
  parseNow();
});
clearButton.addEventListener('click', () => setSource('', true));
shareButton.addEventListener('click', shareCurrent);
copyButton.addEventListener('click', copyCurrent);
downloadButton.addEventListener('click', downloadCurrent);
closeInspectorButton.addEventListener('click', clearInspector);
regressionRunButton.addEventListener('click', runRegression);
regressionUseCurrentButton.addEventListener('click', () => {
  const current = source.value.trim();
  if (!current) return;
  const existing = regressionSource.value.trimEnd();
  regressionSource.value = existing ? `${existing}\n${current}` : current;
  regressionSource.focus();
  regressionSource.setSelectionRange(regressionSource.value.length, regressionSource.value.length);
});

clearHistoryButton.addEventListener('click', () => {
  try { localStorage.removeItem(HISTORY_KEY); } catch { /* ignore */ }
  renderHistory();
  showToast('履歴を削除しました');
});

expandTreeButton.addEventListener('click', () => {
  treeView.querySelectorAll('details').forEach((node) => { node.open = true; });
});
collapseTreeButton.addEventListener('click', () => {
  treeView.querySelectorAll('details').forEach((node) => { node.open = false; });
});

document.addEventListener('keydown', (event) => {
  if (event.key === 'Enter' && (event.metaKey || event.ctrlKey)) {
    event.preventDefault();
    clearTimeout(debounceTimer);
    parseNow();
    return;
  }
  if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k') {
    event.preventDefault();
    source.focus();
    return;
  }
  if (event.altKey && ['Digit1', 'Digit2', 'Digit3', 'Digit4'].includes(event.code)) {
    event.preventDefault();
    switchTab(['tree', 'words', 'json', 'sexpr'][Number(event.code.slice(-1)) - 1]);
    return;
  }
  if (event.key === 'Escape') clearRangeHighlight();
});

function initialInput() {
  const query = new URL(window.location.href).searchParams.get('q');
  if (query !== null) return query;
  try {
    return localStorage.getItem(DRAFT_KEY) || source.value;
  } catch {
    return source.value;
  }
}

source.value = initialInput();
updateCharCount();
syncSample();
renderHistory();
parseNow();
