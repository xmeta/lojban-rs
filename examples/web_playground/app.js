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

const HISTORY_KEY = 'lojban-playground-history-v1';
const DRAFT_KEY = 'lojban-playground-draft-v1';
const encoder = new TextEncoder();
let debounceTimer;
let toastTimer;
let activeRequest = 0;
let activeTab = 'tree';
let lastData = null;
let activeRangeElement = null;
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

function bindRange(element, node) {
  if (!Number.isInteger(node.start) || !Number.isInteger(node.end)) return;
  element.classList.add('range-target');
  element.tabIndex = 0;
  element.setAttribute('role', 'button');
  element.title = `Select source bytes ${node.start}–${node.end}`;
  const activate = (event) => {
    event.stopPropagation();
    event.preventDefault();
    selectSourceRange(node.start, node.end, element);
  };
  element.addEventListener('click', activate);
  element.addEventListener('keydown', (event) => {
    if (event.key === 'Enter' || event.key === ' ') activate(event);
  });
}
function renderTree(node, depth = 0) {
  if (!node.children?.length) {
    const leaf = document.createElement('div');
    leaf.className = 'tree-leaf';
    const text = textLabel(node.text);
    bindRange(text, node);
    leaf.append(ruleLabel(node.rule), text);
    return leaf;
  }

  const details = document.createElement('details');
  details.className = 'tree-node';
  details.open = depth < 2;
  const summary = document.createElement('summary');
  const text = textLabel(compactText(node.text));
  bindRange(text, node);
  summary.append(ruleLabel(node.rule), text);
  details.append(summary);
  for (const child of node.children) details.append(renderTree(child, depth + 1));
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
    bindRange(card, leaf);

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
async function parseNow() {
  const requestId = ++activeRequest;
  const started = performance.now();
  setStatus('pending', 'Parsing…');
  latency.textContent = '';
  errorView.hidden = true;

  try {
    const response = await fetch('/api/parse', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain; charset=utf-8' },
      body: source.value,
    });
    const data = await response.json();
    if (requestId !== activeRequest) return;
    const roundTrip = performance.now() - started;
    renderStats(data.stats);
    latency.textContent = Number.isFinite(data.elapsed_ms)
      ? `${data.elapsed_ms.toFixed(2)} ms parser · ${roundTrip.toFixed(1)} ms round trip`
      : `${roundTrip.toFixed(1)} ms round trip`;

    if (!data.ok) {
      lastData = null;
      setStatus('error', 'Parse error');
      clearOutputs();
      errorView.textContent = data.error;
      errorView.hidden = false;
      return;
    }
    lastData = data;
    setStatus('ok', 'Valid Lojban');
    treeView.replaceChildren(renderTree(data.ast));
    renderWords(data.leaves);
    jsonView.textContent = data.pretty;
    sexprView.textContent = data.sexpr;
    saveHistory(source.value);
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

clearHistoryButton.addEventListener('click', () => {
  localStorage.removeItem(HISTORY_KEY);
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
  if (event.altKey && ['1', '2', '3', '4'].includes(event.key)) {
    event.preventDefault();
    switchTab(['tree', 'words', 'json', 'sexpr'][Number(event.key) - 1]);
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
