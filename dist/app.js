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
const collectionCards = document.getElementById("collection-cards");
const collectionTitle = document.getElementById("collection-title");
const collectionDescription = document.getElementById("collection-description");
const collectionMeta = document.getElementById("collection-meta");
const collectionRows = document.getElementById("collection-rows");
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
const detailEpisodeInfo = document.getElementById("detail-episode-info");
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
let selectedCollectionKey = "all";
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
  if (item.series_title) {
    lines.push(`series_title: ${yamlScalar(item.series_title)}`);
  }
  if (typeof item.season_number === "number") {
    lines.push(`season_number: ${item.season_number}`);
  }
  if (typeof item.episode_start === "number") {
    lines.push(`episode_start: ${item.episode_start}`);
  }
  if (typeof item.episode_end === "number") {
    lines.push(`episode_end: ${item.episode_end}`);
  }
  if (item.episode_title) {
    lines.push(`episode_title: ${yamlScalar(item.episode_title)}`);
  }
  if (typeof item.episode_count === "number") {
    lines.push(`episode_count: ${item.episode_count}`);
  }
  if (typeof item.runtime_minutes === "number") {
    lines.push(`runtime_minutes: ${item.runtime_minutes}`);
  }
  if (typeof item.average_score === "number") {
    lines.push(`average_score: ${item.average_score}`);
  }
  if (item.format) {
    lines.push(`format: ${yamlScalar(item.format)}`);
  }
  if (item.airing_season) {
    lines.push(`airing_season: ${yamlScalar(item.airing_season)}`);
  }
  if (typeof item.anilist_id === "number") {
    lines.push(`anilist_id: ${item.anilist_id}`);
  }
  if (item.anilist_url) {
    lines.push(`anilist_url: ${yamlScalar(item.anilist_url)}`);
  }
  if (item.collection_path) {
    lines.push(`collection_path: ${yamlScalar(item.collection_path)}`);
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
    if (detailEpisodeInfo) detailEpisodeInfo.textContent = "";
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
  if (detailEpisodeInfo) {
    const series = item.series_title || item.title || "Unbekannt";
    const season = typeof item.season_number === "number" ? `Staffel ${item.season_number}` : "Staffel -";
    const episodeStart = typeof item.episode_start === "number" ? `Folge ${item.episode_start}` : "Folge -";
    const episodeEnd =
      typeof item.episode_end === "number" && item.episode_end !== item.episode_start
        ? ` bis ${item.episode_end}`
        : "";
    const runtime = typeof item.runtime_minutes === "number" ? `${item.runtime_minutes} Min.` : "-";
    const score = typeof item.average_score === "number" ? `${Math.round(item.average_score)} / 100` : "-";
    const url = item.anilist_url ? `AniList: ${item.anilist_url}` : "AniList: -";
    detailEpisodeInfo.textContent = `${series} | ${season} | ${episodeStart}${episodeEnd} | Laufzeit ${runtime} | Score ${score} | ${url}`;
  }

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
  renderCollections(plan.items);

  const selected = getSelectedItem();
  renderDetail(selected);

  if (statusStrip) {
    const rootText = plan.vault_root ? ` Vault: ${plan.vault_root}.` : "";
    const noteText = plan.note ? ` ${plan.note}` : "";
    statusStrip.textContent = `${plan.title} (${plan.source}) geladen: ${plan.summary.total_files} Dateien, ${plan.summary.items_needing_review} zur Prüfung, ${plan.summary.duplicates} Duplikat(e).${rootText}${noteText}`;
  }
}

function buildCollectionCatalog(items) {
  const staticCollections = [
    {
      key: "all",
      label: "Alle Dateien",
      description: "Der gesamte aktuelle Scan.",
      rule: "Alle gefundenen Einträge",
      items,
    },
    {
      key: "review",
      label: "Zur Prüfung",
      description: "Alles, was noch nicht klar ist.",
      rule: "needs_review oder Duplikat",
      items: items.filter((item) => item.needs_review || item.duplicate_of),
    },
    {
      key: "duplicates",
      label: "Duplikate",
      description: "Markierte Mehrfachfunde.",
      rule: "duplicate_of gesetzt",
      items: items.filter((item) => item.duplicate_of),
    },
    {
      key: "corrected",
      label: "Korrigiert",
      description: "Bereits lokal angepasst.",
      rule: "lokale Korrektur vorhanden",
      items: items.filter((item) => item.has_correction),
    },
    {
      key: "photos",
      label: "Fotos",
      description: "Fotos und Kameraaufnahmen.",
      rule: "media_type photo",
      items: items.filter((item) => item.media_type === "photo"),
    },
    {
      key: "images",
      label: "Bilder",
      description: "Grafiken, Screenshots, Icons.",
      rule: "media_type image",
      items: items.filter((item) => item.media_type === "image"),
    },
    {
      key: "anime",
      label: "Anime",
      description: "Anime und verwandte Inhalte.",
      rule: "media_type anime",
      items: items.filter((item) => item.media_type === "anime"),
    },
    {
      key: "video",
      label: "Video",
      description: "Filme, Serien und sonstige Videos.",
      rule: "film, series, video-misc",
      items: items.filter((item) =>
        ["film", "series", "video-misc"].includes(item.media_type)
      ),
    },
    {
      key: "audio",
      label: "Audio",
      description: "Musik, Hörbücher und Podcasts.",
      rule: "music, audiobook, podcast",
      items: items.filter((item) =>
        ["music-track", "music-album", "audiobook", "podcast"].includes(item.media_type)
      ),
    },
    {
      key: "books",
      label: "Bücher",
      description: "Bücher, E-Books, Comics und Manga.",
      rule: "book, ebook, comic, manga",
      items: items.filter((item) =>
        ["book", "ebook", "comic", "manga"].includes(item.media_type)
      ),
    },
    {
      key: "docs",
      label: "Dokumente",
      description: "PDFs und andere Dokumente.",
      rule: "media_type document",
      items: items.filter((item) => item.media_type === "document"),
    },
  ];

  const pathCollections = Array.from(
    new Map(
      items
        .map((item) => item.collection_path)
        .filter((value) => typeof value === "string" && value.length > 0)
        .map((value) => [value, value])
    ).values()
  )
    .sort((left, right) => left.localeCompare(right))
    .map((path) => ({
      key: `path:${path}`,
      label: path.split("/").join(" > "),
      description: "Pfadbasierte Sammlung aus AniList- und Vault-Metadaten.",
      rule: path,
      items: items.filter((item) => item.collection_path === path),
    }));

  return [...staticCollections, ...pathCollections];
}

function renderCollectionMeta(collection) {
  if (!collectionMeta) {
    return;
  }

  clearNode(collectionMeta);

  const totalBytes = collection.items.reduce((sum, item) => sum + (item.size_bytes || 0), 0);
  const reviewCount = collection.items.filter((item) => item.needs_review).length;
  const duplicateCount = collection.items.filter((item) => item.duplicate_of).length;

  [
    ["Treffer", `${collection.items.length}`],
    ["Größe", formatBytes(totalBytes)],
    ["Review", `${reviewCount} / ${duplicateCount}`],
  ].forEach(([label, value]) => {
    const block = document.createElement("div");
    const span = document.createElement("span");
    const strong = document.createElement("strong");
    span.textContent = label;
    strong.textContent = value;
    block.appendChild(span);
    block.appendChild(strong);
    collectionMeta.appendChild(block);
  });
}

function renderCollectionCards(collections) {
  if (!collectionCards) {
    return;
  }

  clearNode(collectionCards);

  collections.forEach((collection) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "collection-chip";
    button.classList.toggle("is-active", collection.key === selectedCollectionKey);
    button.addEventListener("click", () => {
      selectedCollectionKey = collection.key;
      renderCollections(currentPlan?.items ?? []);
    });

    const label = document.createElement("strong");
    label.textContent = collection.label;
    const count = document.createElement("span");
    count.textContent = `${collection.items.length}`;

    button.appendChild(label);
    button.appendChild(count);
    collectionCards.appendChild(button);
  });
}

function renderCollectionRows(collection) {
  if (!collectionRows) {
    return;
  }

  clearNode(collectionRows);

  collection.items.forEach((item) => {
    const target = item.target_path || statusLabel(item.status || "inbox");
    collectionRows.appendChild(
      createRow(item, [item.source_path, mediaTypeLabel(item.media_type), target], false)
    );
  });

  if (!collection.items.length) {
    const empty = document.createElement("div");
    empty.className = "collection-empty";
    empty.textContent = "Keine Einträge in dieser Sammlung.";
    collectionRows.appendChild(empty);
  }
}

function renderCollections(items) {
  const collections = buildCollectionCatalog(items);
  const selected =
    collections.find((collection) => collection.key === selectedCollectionKey) ?? collections[0];

  if (!selected) {
    if (collectionTitle) {
      collectionTitle.textContent = "Keine Sammlungen";
    }
    if (collectionDescription) {
      collectionDescription.textContent = "Für den aktuellen Scan wurden keine Einträge gefunden.";
    }
    if (collectionMeta) {
      clearNode(collectionMeta);
    }
    if (collectionRows) {
      clearNode(collectionRows);
    }
    return;
  }

  if (selected.key !== selectedCollectionKey) {
    selectedCollectionKey = selected.key;
  }

  if (collectionTitle) {
    collectionTitle.textContent = selected.label;
  }
  if (collectionDescription) {
    collectionDescription.textContent = `${selected.description} Regel: ${selected.rule}.`;
  }
  if (metricCollections) {
    metricCollections.textContent = `${collections.length} smart`;
  }

  renderCollectionCards(collections);
  renderCollectionMeta(selected);
  renderCollectionRows(selected);
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
