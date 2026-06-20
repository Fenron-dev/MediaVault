const tabs = Array.from(document.querySelectorAll("[data-tab]"));
const views = Array.from(document.querySelectorAll("[data-view]"));
const title = document.getElementById("view-title");
const statusStrip = document.getElementById("status-strip");
const demoButton = document.getElementById("run-demo");
const inboxRows = document.getElementById("inbox-rows");
const reviewRows = document.getElementById("review-rows");
const metricInbox = document.getElementById("metric-inbox");
const metricReview = document.getElementById("metric-review");
const metricDuplicates = document.getElementById("metric-duplicates");
const metricCollections = document.getElementById("metric-collections");
const vaultRootInput = document.getElementById("vault-root-input");
const vaultRootSave = document.getElementById("vault-root-save");
const vaultRootClear = document.getElementById("vault-root-clear");
const vaultRootHint = document.getElementById("vault-root-hint");
const detailHeading = document.getElementById("detail-heading");
const detailSubtitle = document.getElementById("detail-subtitle");
const detailSourcePath = document.getElementById("detail-source-path");
const detailTargetPath = document.getElementById("detail-target-path");
const detailMediaType = document.getElementById("detail-media-type");
const detailClassificationSource = document.getElementById("detail-classification-source");
const detailConfidence = document.getElementById("detail-confidence");
const detailSize = document.getElementById("detail-size");
const detailStatusLabel = document.getElementById("detail-status-label");
const detailSidecarPath = document.getElementById("detail-sidecar-path");
const detailTitle = document.getElementById("detail-title");
const detailMediaTypeInput = document.getElementById("detail-media-type-input");
const detailYear = document.getElementById("detail-year");
const detailStatus = document.getElementById("detail-status");
const detailNotes = document.getElementById("detail-notes");
const detailApply = document.getElementById("detail-apply");
const detailReset = document.getElementById("detail-reset");
const detailSidecarPreview = document.getElementById("detail-sidecar-preview");
const auditTrailNode = document.getElementById("audit-trail");

const storageKey = "mediavault.vaultRoot";
const correctionsKey = "mediavault.reviewCorrections";
const auditTrailKey = "mediavault.auditTrail";

const labels = {
  overview: "Überblick",
  inbox: "Inbox",
  review: "Prüfung",
  collections: "Sammlungen",
  settings: "Einstellungen",
};

const mediaTypeOptions = [
  ["unclassified", "Unklassifiziert"],
  ["film", "Film"],
  ["series", "Serie"],
  ["anime", "Anime"],
  ["photo", "Foto"],
  ["image", "Bild"],
  ["music-track", "Musikstück"],
  ["music-album", "Album"],
  ["book", "Buch"],
  ["ebook", "E-Book"],
  ["manga", "Manga"],
  ["comic", "Comic"],
  ["audiobook", "Hörbuch"],
  ["video-game", "Spiel"],
  ["document", "Dokument"],
  ["archive", "Archiv"],
  ["software", "Software"],
  ["3d-model", "3D-Modell"],
  ["video-misc", "Video"],
];

const statusOptions = [
  ["inbox", "Inbox"],
  ["needs-review", "Prüfung nötig"],
  ["in-library", "Bibliothek"],
  ["wishlist", "Wunschliste"],
  ["completed", "Abgeschlossen"],
  ["on-hold", "Pausiert"],
  ["archived", "Archiviert"],
  ["ignored", "Ignoriert"],
];

let sourcePlan = null;
let currentPlan = null;
let selectedItemKey = "";
let corrections = loadStoredJson(correctionsKey, {});
let auditTrail = loadStoredJson(auditTrailKey, []);

function setActiveTab(tab) {
  tabs.forEach((button) => {
    button.classList.toggle("is-active", button.dataset.tab === tab);
  });

  views.forEach((view) => {
    view.classList.toggle("is-active", view.dataset.view === tab);
  });

  if (title) {
    title.textContent = labels[tab] ?? "MediaVault";
  }

  if (statusStrip) {
    statusStrip.textContent = `Ansicht gewechselt: ${labels[tab] ?? tab}.`;
  }
}

function setMetrics(summary) {
  if (metricInbox) {
    metricInbox.textContent = `${summary.total_files} Dateien`;
  }
  if (metricReview) {
    metricReview.textContent = `${summary.items_needing_review} unklar`;
  }
  if (metricDuplicates) {
    metricDuplicates.textContent = `${summary.duplicates} gefunden`;
  }
  if (metricCollections) {
    metricCollections.textContent = `${summary.smart_collections} smart`;
  }
}

function clearNode(node) {
  while (node && node.firstChild) {
    node.removeChild(node.firstChild);
  }
}

function loadStoredJson(key, fallback) {
  try {
    const raw = localStorage.getItem(key);
    return raw ? JSON.parse(raw) : fallback;
  } catch {
    return fallback;
  }
}

function saveStoredJson(key, value) {
  localStorage.setItem(key, JSON.stringify(value));
}

function sanitizeSegment(value) {
  return String(value)
    .replace(/[\\/:"*?<>|]/g, " ")
    .replace(/[\u0000-\u001f]/g, " ")
    .split(/\s+/)
    .filter(Boolean)
    .join(" ");
}

function basename(value) {
  const normalized = String(value);
  const index = normalized.lastIndexOf("/");
  return index >= 0 ? normalized.slice(index + 1) : normalized;
}

function fileStem(value) {
  const name = basename(value);
  const index = name.lastIndexOf(".");
  return index > 0 ? name.slice(0, index) : name;
}

function extensionOf(value) {
  const name = basename(value);
  const index = name.lastIndexOf(".");
  return index > 0 ? name.slice(index + 1) : "";
}

function formatBytes(bytes) {
  if (!Number.isFinite(bytes)) {
    return "-";
  }

  const units = ["B", "KB", "MB", "GB", "TB"];
  let value = bytes;
  let unitIndex = 0;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }

  return `${value.toFixed(value >= 10 || unitIndex === 0 ? 0 : 1)} ${units[unitIndex]}`;
}

function formatConfidence(value) {
  if (typeof value !== "number") {
    return "-";
  }

  return `${Math.round(value * 100)} %`;
}

function mediaTypeLabel(value) {
  const found = mediaTypeOptions.find(([candidate]) => candidate === value);
  return found ? found[1] : value ?? "-";
}

function statusLabel(value) {
  const found = statusOptions.find(([candidate]) => candidate === value);
  return found ? found[1] : value ?? "-";
}

function classificationSourceLabel(value) {
  switch (value) {
    case "api":
      return "API";
    case "ai":
      return "KI";
    case "user":
      return "Benutzer";
    case "folder":
      return "Ordner";
    case "filename":
      return "Dateiname";
    case "extension":
      return "Erweiterung";
    case "unknown":
      return "Unbekannt";
    default:
      return value ?? "-";
  }
}

function folderSegmentFor(mediaType) {
  switch (mediaType) {
    case "film":
      return "Movies";
    case "series":
      return "Series";
    case "anime":
      return "Anime";
    case "photo":
      return "Photos";
    case "image":
      return "Images";
    case "music-album":
      return "Music/Albums";
    case "music-track":
      return "Music/Tracks";
    case "book":
      return "Books";
    case "ebook":
      return "Ebooks";
    case "manga":
      return "Manga";
    case "comic":
      return "Comics";
    case "audiobook":
      return "Audiobooks";
    case "video-game":
      return "Video Games";
    case "document":
      return "Documents";
    case "archive":
      return "Archives";
    case "software":
      return "Software";
    case "3d-model":
      return "3D Models";
    case "video-misc":
      return "Videos";
    default:
      return "Unclassified";
  }
}

function normalizeYear(value) {
  if (value === null || value === undefined || value === "") {
    return null;
  }

  const parsed = Number.parseInt(String(value), 10);
  return Number.isNaN(parsed) ? null : parsed;
}

function projectItem(item) {
  const correction = corrections[item.source_path] ?? {};
  const effective = { ...item };

  if (typeof correction.title === "string") {
    effective.title = correction.title;
  }
  if (typeof correction.media_type === "string") {
    effective.media_type = correction.media_type;
  }
  if (typeof correction.year !== "undefined") {
    effective.year = correction.year;
  }
  if (typeof correction.status === "string") {
    effective.status = correction.status;
  }
  if (typeof correction.notes === "string") {
    effective.notes = correction.notes;
  }

  effective.has_correction = Boolean(correction.updated_at);
  effective.folder_segment = correction.media_type
    ? folderSegmentFor(correction.media_type)
    : effective.folder_segment ?? folderSegmentFor(effective.media_type);
  effective.target_path = buildTargetPath(effective);
  effective.sidecar_path = buildSidecarPath(effective.target_path);
  effective.sidecar_preview = buildSidecarPreview(effective);
  return effective;
}

function projectPlan(plan) {
  const cloned = JSON.parse(JSON.stringify(plan));
  cloned.items = cloned.items.map((item) => projectItem(item));
  return cloned;
}

function buildTargetPath(item) {
  const mediaType = item.media_type ?? "unclassified";
  const folderSegment = item.folder_segment ?? folderSegmentFor(mediaType);
  const title = sanitizeSegment(item.title || fileStem(item.source_path) || "untitled");
  const yearSuffix = item.year ? ` (${item.year})` : "";
  const extension = extensionOf(item.source_path);
  const filename = extension ? `${title}${yearSuffix}.${extension}` : `${title}${yearSuffix}`;
  return `${folderSegment}/${title}${yearSuffix}/${filename}`;
}

function buildSidecarPath(targetPath) {
  const normalized = String(targetPath);
  const slashIndex = normalized.lastIndexOf("/");
  const fileName = slashIndex >= 0 ? normalized.slice(slashIndex + 1) : normalized;
  const stemIndex = fileName.lastIndexOf(".");
  const stem = stemIndex > 0 ? fileName.slice(0, stemIndex) : fileName;
  const prefix = slashIndex >= 0 ? normalized.slice(0, slashIndex + 1) : "";
  return `${prefix}${stem}.mediashelf.yaml`;
}

function yamlScalar(value) {
  return JSON.stringify(String(value));
}

function buildSidecarPreview(item) {
  const targetPath = item.target_path || item.source_path;
  const status = item.status || (item.needs_review ? "needs-review" : "inbox");
  const lines = [
    "---",
    `id: ${yamlScalar(`preview-${item.source_path}`)}`,
    `media_type: ${yamlScalar(item.media_type ?? "unclassified")}`,
    `source_file_path: ${yamlScalar(targetPath)}`,
    `original_filename: ${yamlScalar(basename(item.source_path))}`,
  ];

  if (item.title) {
    lines.push(`title: ${yamlScalar(item.title)}`);
  }
  if (item.year) {
    lines.push(`year: ${item.year}`);
  }

  lines.push(`status: ${yamlScalar(status)}`);

  if (item.notes) {
    lines.push(`notes: ${yamlScalar(item.notes)}`);
  }

  lines.push("---");
  return lines.join("\n");
}

function updateAuditTrail(message) {
  const entry = {
    at: new Date().toISOString(),
    message,
  };
  auditTrail = [entry, ...auditTrail].slice(0, 8);
  saveStoredJson(auditTrailKey, auditTrail);
  renderAuditTrail();
}

function renderAuditTrail() {
  if (!auditTrailNode) {
    return;
  }

  clearNode(auditTrailNode);

  if (!auditTrail.length) {
    const empty = document.createElement("div");
    empty.className = "audit-entry";
    empty.textContent = "Noch keine lokalen Korrekturen.";
    auditTrailNode.appendChild(empty);
    return;
  }

  auditTrail.forEach((entry) => {
    const block = document.createElement("div");
    block.className = "audit-entry";

    const heading = document.createElement("strong");
    heading.textContent = new Date(entry.at).toLocaleString("de-DE");

    const body = document.createElement("span");
    body.textContent = entry.message;

    block.appendChild(heading);
    block.appendChild(body);
    auditTrailNode.appendChild(block);
  });
}

function createRow(item, cells, selected) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "table-row table-row-button";
  row.classList.toggle("is-selected", selected);
  row.dataset.sourcePath = item.source_path;

  cells.forEach((cell) => {
    const span = document.createElement("span");
    span.textContent = cell;
    row.appendChild(span);
  });

  row.addEventListener("click", () => {
    selectedItemKey = item.source_path;
    setActiveTab("review");
    renderPlan(currentPlan);
    renderDetail(getSelectedItem());
  });

  return row;
}

function getSelectedItem() {
  if (!currentPlan || !currentPlan.items.length) {
    return null;
  }

  return (
    currentPlan.items.find((item) => item.source_path === selectedItemKey) ??
    currentPlan.items[0]
  );
}

function renderInboxRows(items) {
  if (!inboxRows) {
    return;
  }

  clearNode(inboxRows);

  items.forEach((item) => {
    const status = item.duplicate_of
      ? "Duplikat"
      : item.needs_review
      ? "Review"
      : "klar";
    const target = item.duplicate_of || item.needs_review
      ? "Inbox/_review_queue"
      : item.target_path ?? "Inbox/_review_queue";
    inboxRows.appendChild(createRow(item, [item.source_path, status, target], false));
  });
}

function renderReviewRows(items) {
  if (!reviewRows) {
    return;
  }

  clearNode(reviewRows);

  const reviewItems = items.filter((item) => item.needs_review || item.duplicate_of);

  reviewItems.forEach((item) => {
    const status = item.duplicate_of
      ? "Duplikat"
      : item.has_correction
      ? "Korrigiert"
      : item.manual_review
      ? "Review"
      : "Prüfung";
    const action = item.steps[item.steps.length - 1] ?? "Prüfen";
    reviewRows.appendChild(
      createRow(item, [item.source_path, status, action], item.source_path === selectedItemKey)
    );
  });

  if (!selectedItemKey && reviewItems.length) {
    selectedItemKey = reviewItems[0].source_path;
  }
}

function renderDetail(item) {
  if (!detailHeading || !detailSubtitle) {
    return;
  }

  if (!item) {
    detailHeading.textContent = "Eintrag auswählen";
    detailSubtitle.textContent = "Die gewählte Datei wird hier mit Zielpfad, Sidecar-Vorschau und Korrekturfeldern angezeigt.";

    if (detailSourcePath) detailSourcePath.textContent = "-";
    if (detailTargetPath) detailTargetPath.textContent = "-";
    if (detailMediaType) detailMediaType.textContent = "-";
    if (detailClassificationSource) detailClassificationSource.textContent = "-";
    if (detailConfidence) detailConfidence.textContent = "-";
    if (detailSize) detailSize.textContent = "-";
    if (detailStatusLabel) detailStatusLabel.textContent = "-";
    if (detailSidecarPath) detailSidecarPath.textContent = "-";
    if (detailTitle) detailTitle.value = "";
    if (detailMediaTypeInput) detailMediaTypeInput.value = "unclassified";
    if (detailYear) detailYear.value = "";
    if (detailStatus) detailStatus.value = "inbox";
    if (detailNotes) detailNotes.value = "";
    if (detailSidecarPreview) detailSidecarPreview.textContent = "Wähle einen Eintrag aus.";
    return;
  }

  detailHeading.textContent = item.title || fileStem(item.source_path) || "Unbenannt";
  detailSubtitle.textContent = item.has_correction
    ? "Für diesen Eintrag existiert bereits eine lokale Korrektur."
    : "Die gewählte Datei wird hier mit Zielpfad, Sidecar-Vorschau und Korrekturfeldern angezeigt.";

  if (detailSourcePath) detailSourcePath.textContent = item.source_path;
  if (detailTargetPath) detailTargetPath.textContent = item.target_path || "-";
  if (detailMediaType) detailMediaType.textContent = mediaTypeLabel(item.media_type);
  if (detailClassificationSource) {
    detailClassificationSource.textContent = classificationSourceLabel(item.classification_source);
  }
  if (detailConfidence) detailConfidence.textContent = formatConfidence(item.confidence);
  if (detailSize) detailSize.textContent = formatBytes(item.size_bytes);
  if (detailStatusLabel) detailStatusLabel.textContent = statusLabel(item.status || (item.needs_review ? "needs-review" : "inbox"));
  if (detailSidecarPath) detailSidecarPath.textContent = item.sidecar_path ?? "-";

  if (detailTitle) detailTitle.value = item.title ?? "";
  if (detailMediaTypeInput) detailMediaTypeInput.value = item.media_type ?? "unclassified";
  if (detailYear) detailYear.value = item.year ?? "";
  if (detailStatus) detailStatus.value = item.status ?? (item.needs_review ? "needs-review" : "inbox");
  if (detailNotes) detailNotes.value = item.notes ?? "";
  if (detailSidecarPreview) detailSidecarPreview.textContent = item.sidecar_preview;
}

function renderPlan(plan) {
  if (!plan) {
    return;
  }

  setMetrics(plan.summary);
  renderInboxRows(plan.items);
  renderReviewRows(plan.items);

  const selected = getSelectedItem();
  renderDetail(selected);

  if (statusStrip) {
    const rootText = plan.vault_root ? ` Vault: ${plan.vault_root}.` : "";
    const noteText = plan.note ? ` ${plan.note}` : "";
    statusStrip.textContent = `${plan.title} (${plan.source}) geladen: ${plan.summary.total_files} Dateien, ${plan.summary.items_needing_review} zur Prüfung, ${plan.summary.duplicates} Duplikat(e).${rootText}${noteText}`;
  }
}

function getVaultRoot() {
  if (vaultRootInput && vaultRootInput.value.trim()) {
    return vaultRootInput.value.trim();
  }

  return localStorage.getItem(storageKey) ?? "";
}

function syncVaultHint() {
  if (!vaultRootHint) {
    return;
  }

  const value = getVaultRoot();
  vaultRootHint.textContent = value
    ? `Aktiver Vault-Pfad: ${value}`
    : "Leer lassen, dann versucht MediaVault den Vault automatisch zu finden.";
}

function populateSelectors() {
  if (detailMediaTypeInput) {
    clearNode(detailMediaTypeInput);
    mediaTypeOptions.forEach(([value, label]) => {
      const option = document.createElement("option");
      option.value = value;
      option.textContent = label;
      detailMediaTypeInput.appendChild(option);
    });
  }

  if (detailStatus) {
    clearNode(detailStatus);
    statusOptions.forEach(([value, label]) => {
      const option = document.createElement("option");
      option.value = value;
      option.textContent = label;
      detailStatus.appendChild(option);
    });
  }
}

function upsertCorrection(item) {
  corrections[item.source_path] = {
    title: detailTitle?.value.trim() || null,
    media_type: detailMediaTypeInput?.value || item.media_type,
    year: normalizeYear(detailYear?.value),
    status: detailStatus?.value || null,
    notes: detailNotes?.value.trim() || null,
    updated_at: new Date().toISOString(),
  };

  if (!corrections[item.source_path].title) {
    delete corrections[item.source_path].title;
  }
  if (corrections[item.source_path].year === null) {
    delete corrections[item.source_path].year;
  }
  if (!corrections[item.source_path].status) {
    delete corrections[item.source_path].status;
  }
  if (!corrections[item.source_path].notes) {
    delete corrections[item.source_path].notes;
  }

  saveStoredJson(correctionsKey, corrections);
}

function clearCorrection(item) {
  delete corrections[item.source_path];
  saveStoredJson(correctionsKey, corrections);
}

async function loadPlan() {
  if (statusStrip) {
    statusStrip.textContent = "Vault-Scan wird geladen...";
  }

  try {
    const root = getVaultRoot();
    const path = root
      ? `/api/vault-plan?root=${encodeURIComponent(root)}`
      : "/api/vault-plan";
    const response = await fetch(path);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    sourcePlan = await response.json();
    currentPlan = projectPlan(sourcePlan);

    if (!selectedItemKey || !currentPlan.items.some((item) => item.source_path === selectedItemKey)) {
      const preferred = currentPlan.items.find((item) => item.needs_review || item.duplicate_of);
      selectedItemKey = preferred ? preferred.source_path : currentPlan.items[0]?.source_path ?? "";
    } else {
      currentPlan.items = currentPlan.items.map((item) => projectItem(item));
    }

    renderPlan(currentPlan);
  } catch (error) {
    if (statusStrip) {
      statusStrip.textContent = `Vault-Scan konnte nicht geladen werden: ${error.message}`;
    }
  }
}

tabs.forEach((button) => {
  button.addEventListener("click", () => setActiveTab(button.dataset.tab));
});

if (demoButton) {
  demoButton.addEventListener("click", () => {
    setActiveTab("review");
    loadPlan();
  });
}

if (vaultRootInput) {
  const savedRoot = localStorage.getItem(storageKey);
  if (savedRoot) {
    vaultRootInput.value = savedRoot;
  }

  vaultRootInput.addEventListener("input", () => {
    syncVaultHint();
  });
}

if (vaultRootSave) {
  vaultRootSave.addEventListener("click", () => {
    if (!vaultRootInput) {
      return;
    }

    const value = vaultRootInput.value.trim();
    if (value) {
      localStorage.setItem(storageKey, value);
    } else {
      localStorage.removeItem(storageKey);
    }

    syncVaultHint();
    loadPlan();
  });
}

if (vaultRootClear) {
  vaultRootClear.addEventListener("click", () => {
    if (vaultRootInput) {
      vaultRootInput.value = "";
    }

    localStorage.removeItem(storageKey);
    syncVaultHint();
    loadPlan();
  });
}

if (detailApply) {
  detailApply.addEventListener("click", () => {
    const item = getSelectedItem();
    if (!item) {
      return;
    }

    upsertCorrection(item);
    currentPlan = projectPlan(sourcePlan);
    selectedItemKey = item.source_path;
    renderPlan(currentPlan);
    updateAuditTrail(`Korrektur übernommen für ${item.source_path}.`);
  });
}

if (detailReset) {
  detailReset.addEventListener("click", () => {
    const item = getSelectedItem();
    if (!item) {
      return;
    }

    clearCorrection(item);
    currentPlan = projectPlan(sourcePlan);
    selectedItemKey = item.source_path;
    renderPlan(currentPlan);
    updateAuditTrail(`Lokale Korrektur zurückgesetzt für ${item.source_path}.`);
  });
}

populateSelectors();
renderAuditTrail();
syncVaultHint();
loadPlan();
