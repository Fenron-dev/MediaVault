const tabs = Array.from(document.querySelectorAll("[data-tab]"));
const views = Array.from(document.querySelectorAll("[data-view]"));
const vaultGate = document.getElementById("vault-gate");
const vaultOpenPath = document.getElementById("vault-open-path");
const vaultOpenButton = document.getElementById("vault-open-button");
const vaultCreateName = document.getElementById("vault-create-name");
const vaultCreatePath = document.getElementById("vault-create-path");
const vaultCreateButton = document.getElementById("vault-create-button");
const recentVaultsList = document.getElementById("recent-vaults-list");
const title = document.getElementById("view-title");
const statusStrip = document.getElementById("status-strip");
const demoButton = document.getElementById("run-demo");
const inboxRows = document.getElementById("inbox-rows");
const reviewRows = document.getElementById("review-rows");
const metricInbox = document.getElementById("metric-inbox");
const metricReview = document.getElementById("metric-review");
const metricDuplicates = document.getElementById("metric-duplicates");
const metricCollections = document.getElementById("metric-collections");
const collectionSidebar = document.getElementById("collection-sidebar");
const collectionSidebarList = document.getElementById("collection-sidebar-list");
const collectionBackDashboard = document.getElementById("collection-back-dashboard");
const collectionBreadcrumb = document.getElementById("collection-breadcrumb");
const collectionTitle = document.getElementById("collection-title");
const collectionDescription = document.getElementById("collection-description");
const collectionMeta = document.getElementById("collection-meta");
const collectionProfile = document.getElementById("collection-profile");
const collectionRows = document.getElementById("collection-rows");
const collectionEditor = document.getElementById("collection-editor");
const collectionEditorTitle = document.getElementById("collection-editor-title");
const collectionEditorSubtitle = document.getElementById("collection-editor-subtitle");
const collectionEditorSource = document.getElementById("collection-editor-source");
const collectionEditorTarget = document.getElementById("collection-editor-target");
const collectionEditorType = document.getElementById("collection-editor-type");
const collectionEditorSidecar = document.getElementById("collection-editor-sidecar");
const collectionEditorName = document.getElementById("collection-editor-name");
const collectionEditorMediaType = document.getElementById("collection-editor-media-type");
const collectionEditorYear = document.getElementById("collection-editor-year");
const collectionEditorStatus = document.getElementById("collection-editor-status");
const collectionEditorNotes = document.getElementById("collection-editor-notes");
const collectionEditorFetch = document.getElementById("collection-editor-fetch");
const collectionEditorApply = document.getElementById("collection-editor-apply");
const collectionEditorReset = document.getElementById("collection-editor-reset");
const collectionEditorSidecarPreview = document.getElementById("collection-editor-sidecar-preview");
const collectionEditorApiFeedback = document.getElementById("collection-editor-api-feedback");
const vaultRootInput = document.getElementById("vault-root-input");
const vaultRootSave = document.getElementById("vault-root-save");
const vaultRootClear = document.getElementById("vault-root-clear");
const vaultRootHint = document.getElementById("vault-root-hint");
const templateAnimeTv = document.getElementById("template-anime-tv");
const templateAnimeMovie = document.getElementById("template-anime-movie");
const templateSeries = document.getElementById("template-series");
const templateFilm = document.getElementById("template-film");
const templatesSave = document.getElementById("templates-save");
const templatesReset = document.getElementById("templates-reset");
const templatesHint = document.getElementById("templates-hint");
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
const detailFetchMetadata = document.getElementById("detail-fetch-metadata");
const detailApply = document.getElementById("detail-apply");
const detailReset = document.getElementById("detail-reset");
const detailSidecarPreview = document.getElementById("detail-sidecar-preview");
const detailApiFeedback = document.getElementById("detail-api-feedback");
const inspectorToggle = document.getElementById("inspector-toggle");
const inspectorTitle = document.getElementById("inspector-title");
const inspectorProperties = document.getElementById("inspector-properties");
const inspectorEditToggle = document.getElementById("inspector-edit-toggle");
const inspectorFetchMetadata = document.getElementById("inspector-fetch-metadata");
const inspectorEdit = document.getElementById("inspector-edit");
const inspectorName = document.getElementById("inspector-name");
const inspectorMediaType = document.getElementById("inspector-media-type");
const inspectorYear = document.getElementById("inspector-year");
const inspectorStatus = document.getElementById("inspector-status");
const inspectorApply = document.getElementById("inspector-apply");
const inspectorApiFeedback = document.getElementById("inspector-api-feedback");
const inspectorPropertyAddToggle = document.getElementById("inspector-property-add-toggle");
const inspectorPropertyAddForm = document.getElementById("inspector-property-add-form");
const inspectorPropertyKey = document.getElementById("inspector-property-key");
const inspectorPropertyType = document.getElementById("inspector-property-type");
const inspectorPropertyValue = document.getElementById("inspector-property-value");
const inspectorPropertyAdd = document.getElementById("inspector-property-add");
const inspectorYaml = document.getElementById("inspector-yaml");
const inspectorYamlSave = document.getElementById("inspector-yaml-save");
const inspectorYamlReset = document.getElementById("inspector-yaml-reset");
const inspectorYamlHint = document.getElementById("inspector-yaml-hint");
const inspectorYamlPanel = document.getElementById("inspector-yaml-panel");
const auditTrailNode = document.getElementById("audit-trail");

const storageKey = "mediavault.vaultRoot";
const recentVaultsKey = "mediavault.recentVaults";
const pathTemplatesKey = "mediavault.pathTemplates";
const correctionsKey = "mediavault.reviewCorrections";
const metadataKey = "mediavault.apiMetadata";
const yamlOverridesKey = "mediavault.yamlOverrides";
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
  ["anime-tv", "Anime (TV)"],
  ["anime-movie", "Anime (Movie)"],
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
  ["in-library", "Sammlung"],
  ["wishlist", "Wunschliste"],
  ["completed", "Abgeschlossen"],
  ["on-hold", "Pausiert"],
  ["archived", "Archiviert"],
  ["ignored", "Ignoriert"],
];

let sourcePlan = null;
let currentPlan = null;
let selectedItemKey = "";
let selectedCollectionKey = "";
let selectedCollectionItemKey = "";
let corrections = loadStoredJson(correctionsKey, {});
let apiMetadata = loadStoredJson(metadataKey, {});
let yamlOverrides = loadStoredJson(yamlOverridesKey, {});
let pathTemplates = normalizeTemplateConfig(loadStoredJson(pathTemplatesKey, defaultPathTemplates()));
let auditTrail = loadStoredJson(auditTrailKey, []);
let inspectorItemKey = "";
let inspectorSelection = null;

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

  if (collectionSidebar) {
    collectionSidebar.classList.toggle("is-active", tab === "collections");
  }
  document.body.classList.toggle("is-collections", tab === "collections");

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

function defaultPathTemplates() {
  return {
    animeTv: "Anime/Serien/{series}/Staffel {season}/{series} - {episode_label}.{ext}",
    animeMovie: "Anime/Filme/{title} ({year})/{title} ({year}).{ext}",
    series: "Serien/{series}/Staffel {season}/{series} - {episode_label}.{ext}",
    film: "Filme/{title} ({year})/{title} ({year}).{ext}",
  };
}

function normalizeTemplateConfig(value) {
  return {
    ...defaultPathTemplates(),
    ...(value && typeof value === "object" ? value : {}),
  };
}

function currentActiveTab() {
  return tabs.find((button) => button.classList.contains("is-active"))?.dataset.tab ?? "overview";
}

function recentVaults() {
  return loadStoredJson(recentVaultsKey, []);
}

function rememberVault(path, name = "") {
  const value = String(path || "").trim();
  if (!value) {
    return;
  }
  const next = [
    { path: value, name: name || basename(value) || "MediaVault" },
    ...recentVaults().filter((vault) => vault.path !== value),
  ].slice(0, 6);
  saveStoredJson(recentVaultsKey, next);
  renderRecentVaults();
}

function openVault(path, name = "") {
  const value = String(path || "").trim();
  if (!value) {
    if (statusStrip) {
      statusStrip.textContent = "Bitte einen Vault-Pfad eintragen.";
    }
    return;
  }

  localStorage.setItem(storageKey, value);
  if (vaultRootInput) {
    vaultRootInput.value = value;
  }
  rememberVault(value, name);
  document.body.classList.add("is-vault-open");
  syncVaultHint();
  loadPlan();
}

function renderRecentVaults() {
  if (!recentVaultsList) {
    return;
  }
  clearNode(recentVaultsList);
  const vaults = recentVaults();
  if (!vaults.length) {
    const empty = document.createElement("p");
    empty.className = "field-hint";
    empty.textContent = "Noch keine zuletzt geöffneten Vaults.";
    recentVaultsList.appendChild(empty);
    return;
  }

  vaults.forEach((vault) => {
    const row = document.createElement("button");
    row.type = "button";
    row.className = "recent-vault-row";
    row.addEventListener("click", () => openVault(vault.path, vault.name));

    const titleNode = document.createElement("strong");
    titleNode.textContent = vault.name || basename(vault.path);
    const pathNode = document.createElement("span");
    pathNode.textContent = vault.path;
    row.appendChild(titleNode);
    row.appendChild(pathNode);
    recentVaultsList.appendChild(row);
  });
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
  if (value === "anime-tv") return "Anime (TV)";
  if (value === "anime-movie") return "Anime (Movie)";
  const found = mediaTypeOptions.find(([candidate]) => candidate === value);
  return found ? found[1] : value ?? "-";
}

function canonicalMediaType(value) {
  switch (value) {
    case "anime-tv":
    case "anime-movie":
      return "anime";
    default:
      return value ?? "unclassified";
  }
}

function formatForMediaSelection(value) {
  switch (value) {
    case "anime-tv":
      return "TV";
    case "anime-movie":
      return "MOVIE";
    default:
      return null;
  }
}

function mediaSelectionValue(item) {
  if (item.media_type === "anime") {
    return String(item.format ?? "").toUpperCase() === "MOVIE" ? "anime-movie" : "anime-tv";
  }

  return item.media_type ?? "unclassified";
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

function cleanTitleText(value) {
  return String(value ?? "")
    .replace(/\[[^\]]*]/g, " ")
    .replace(/\([^)]*\)/g, " ")
    .replace(/[._]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();
}

function episodeMarker(value) {
  const match = String(value ?? "").match(/\bS(\d{1,2})E(\d{1,3})(?:\s*(?:[-+]|E)\s*(\d{1,3}))?\b/i);
  if (!match) {
    return null;
  }

  return {
    season: Number.parseInt(match[1], 10),
    episodeStart: Number.parseInt(match[2], 10),
    episodeEnd: match[3] ? Number.parseInt(match[3], 10) : Number.parseInt(match[2], 10),
    raw: match[0],
  };
}

function hasEpisodeMarker(item) {
  return Boolean(
    episodeMarker(item.source_path) ||
      episodeMarker(item.title) ||
      episodeMarker(item.series_title)
  );
}

function candidateText(item) {
  return cleanTitleText(
    [item.series_title, item.title, fileStem(item.source_path)].filter(Boolean).join(" ")
  );
}

function stripEpisodeMarkerText(value) {
  return cleanTitleText(String(value ?? "").replace(/\bS\d{1,2}E\d{1,3}(?:\s*(?:[-+]|E)\s*\d{1,3})?\b/gi, " "));
}

function deriveSeriesTitle(item) {
  if (item.series_title && !episodeMarker(item.series_title)) {
    return cleanTitleText(item.series_title);
  }

  if (item.title && !episodeMarker(item.title)) {
    return cleanTitleText(item.title);
  }

  const pathParts = String(item.source_path ?? "").split("/").map(cleanTitleText).filter(Boolean);
  const animeIndex = pathParts.findIndex((part) => part.toLowerCase() === "anime");
  if (animeIndex >= 0 && pathParts[animeIndex + 1]) {
    const candidate = pathParts[animeIndex + 1];
    if (!/^staffel|^season|^s\d+/i.test(candidate)) {
      return candidate;
    }
  }

  const stripped = stripEpisodeMarkerText(fileStem(item.source_path));
  return stripped || "Unbekannte Serie";
}

function deriveEpisodeTitle(item) {
  const stem = cleanTitleText(fileStem(item.source_path));
  const marker = episodeMarker(stem);
  if (!marker) {
    return item.episode_title || "";
  }

  const afterMarker = cleanTitleText(stem.slice(stem.toLowerCase().indexOf(marker.raw.toLowerCase()) + marker.raw.length));
  return item.episode_title || afterMarker;
}

function isAnimeMovie(item) {
  return item.media_type === "anime" && String(item.format ?? "").toUpperCase() === "MOVIE";
}

function isAnimeSeries(item) {
  return item.media_type === "anime" && !isAnimeMovie(item);
}

function pathWithoutFilename(path) {
  const value = String(path || "");
  const index = value.lastIndexOf("/");
  return index >= 0 ? value.slice(0, index) : value;
}

function compactPath(path) {
  return String(path)
    .replace(/\/+/g, "/")
    .replace(/\(\)/g, "")
    .replace(/\s+\./g, ".")
    .replace(/\/\./g, "/")
    .replace(/\.$/, "")
    .replace(/\/$/, "");
}

function templateContextFor(item) {
  const title = sanitizeSegment(item.title || fileStem(item.source_path) || "untitled");
  const series = sanitizeSegment(deriveSeriesTitle(item));
  const marker = episodeMarker(item.source_path) || episodeMarker(item.title) || {};
  const season = item.season_number || marker.season || 1;
  const ext = extensionOf(item.source_path);
  const year = item.year || "";
  return {
    title,
    series,
    season,
    year,
    ext,
    filename: basename(item.source_path),
    episode_label: episodeFileLabel(item),
  };
}

function renderPathTemplate(template, item) {
  const context = templateContextFor(item);
  return compactPath(
    String(template || "")
      .replace(/\{title}/g, context.title)
      .replace(/\{series}/g, context.series)
      .replace(/\{season}/g, context.season)
      .replace(/\{year}/g, context.year)
      .replace(/\{ext}/g, context.ext)
      .replace(/\{filename}/g, context.filename)
      .replace(/\{episode_label}/g, context.episode_label)
  );
}

function collectionPathFor(item) {
  if (isAnimeMovie(item)) {
    return pathWithoutFilename(renderPathTemplate(pathTemplates.animeMovie, item));
  }

  if (isAnimeSeries(item)) {
    return pathWithoutFilename(renderPathTemplate(pathTemplates.animeTv, item));
  }

  if (item.media_type === "series" || hasEpisodeMarker(item)) {
    return pathWithoutFilename(renderPathTemplate(pathTemplates.series, item));
  }

  if (item.media_type === "film") {
    return pathWithoutFilename(renderPathTemplate(pathTemplates.film, item));
  }

  const folder = item.folder_segment ?? folderSegmentFor(item.media_type);
  const title = sanitizeSegment(item.title || fileStem(item.source_path) || "Unbenannt");
  return title ? `${folder}/${title}` : folder;
}

function episodeFileLabel(item) {
  const marker = episodeMarker(item.source_path) || episodeMarker(item.title);
  if (!marker) {
    return sanitizeSegment(item.title || fileStem(item.source_path) || "Episode");
  }

  const start = String(marker.episodeStart).padStart(2, "0");
  const end = marker.episodeEnd && marker.episodeEnd !== marker.episodeStart
    ? `-${String(marker.episodeEnd).padStart(2, "0")}`
    : "";
  const episodeTitle = deriveEpisodeTitle(item);
  return episodeTitle ? `E${start}${end} - ${sanitizeSegment(episodeTitle)}` : `E${start}${end}`;
}

function projectItem(item) {
  const correction = corrections[item.source_path] ?? {};
  const metadata = apiMetadata[item.source_path] ?? {};
  const effective = { ...item };

  [
    "title",
    "title_original",
    "series_title",
    "description",
    "media_type",
    "format",
    "source",
    "country_of_origin",
    "hashtag",
    "cover_image_medium",
    "cover_image_large",
    "cover_image_extra_large",
    "cover_color",
    "banner_image",
    "airing_season",
    "anilist_url",
    "episode_title",
  ].forEach((field) => {
    if (typeof metadata[field] === "string") {
      effective[field] = metadata[field];
    }
  });

  [
    "year",
    "season_number",
    "episode_start",
    "episode_end",
    "episode_count",
    "runtime_minutes",
    "average_score",
    "mean_score",
    "popularity",
    "favourites",
    "anilist_id",
    "mal_id",
  ].forEach((field) => {
    if (typeof metadata[field] === "number") {
      effective[field] = metadata[field];
    }
  });

  [
    "genres",
    "synonyms",
    "tags",
    "studios",
    "relations",
    "characters",
    "staff",
    "reviews",
  ].forEach((field) => {
    if (Array.isArray(metadata[field])) {
      effective[field] = metadata[field];
    }
  });

  ["start_date", "end_date", "trailer"].forEach((field) => {
    if (metadata[field] && typeof metadata[field] === "object") {
      effective[field] = metadata[field];
    }
  });

  if (typeof correction.title === "string") {
    effective.title = correction.title;
  }
  if (typeof correction.series_title === "string") {
    effective.series_title = correction.series_title;
  }
  if (typeof correction.media_type === "string") {
    effective.media_type = canonicalMediaType(correction.media_type);
    const selectedFormat = formatForMediaSelection(correction.media_type);
    if (selectedFormat) {
      effective.format = selectedFormat;
    }
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
    ? folderSegmentFor(canonicalMediaType(correction.media_type))
    : effective.folder_segment ?? folderSegmentFor(effective.media_type);
  const marker = episodeMarker(effective.source_path) || episodeMarker(effective.title);
  if (marker) {
    effective.season_number = effective.season_number || marker.season;
    effective.episode_start = effective.episode_start || marker.episodeStart;
    effective.episode_end = effective.episode_end || marker.episodeEnd;
    if (effective.media_type === "film") {
      effective.media_type = candidateText(effective).toLowerCase().includes("anime")
        ? "anime"
        : "series";
      effective.folder_segment = folderSegmentFor(effective.media_type);
    }
  }
  effective.collection_path = collectionPathFor(effective);
  effective.target_path = buildTargetPath(effective);
  effective.sidecar_path = buildSidecarPath(effective.target_path);
  effective.sidecar_preview = yamlOverrides[item.source_path] || buildSidecarPreview(effective);
  return effective;
}

function projectPlan(plan) {
  const cloned = JSON.parse(JSON.stringify(plan));
  cloned.items = cloned.items.map((item) => projectItem(item));
  return cloned;
}

function buildTargetPath(item) {
  const mediaType = item.media_type ?? "unclassified";
  const title = sanitizeSegment(item.title || fileStem(item.source_path) || "untitled");
  const yearSuffix = item.year ? ` (${item.year})` : "";
  const extension = extensionOf(item.source_path);

  if (isAnimeMovie(item)) {
    return renderPathTemplate(pathTemplates.animeMovie, item);
  }

  if (isAnimeSeries(item)) {
    return renderPathTemplate(pathTemplates.animeTv, item);
  }

  if (mediaType === "series" || hasEpisodeMarker(item)) {
    return renderPathTemplate(pathTemplates.series, item);
  }

  if (mediaType === "film") {
    return renderPathTemplate(pathTemplates.film, item);
  }

  const folderSegment = item.folder_segment ?? folderSegmentFor(mediaType);
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

function appendYamlValue(lines, key, value, indent = 0) {
  if (value === null || typeof value === "undefined" || value === "") {
    return;
  }

  const prefix = " ".repeat(indent);
  if (Array.isArray(value)) {
    if (!value.length) {
      return;
    }
    lines.push(`${prefix}${key}:`);
    value.forEach((entry) => {
      if (entry && typeof entry === "object") {
        lines.push(`${prefix}  - ${JSON.stringify(entry)}`);
      } else {
        lines.push(`${prefix}  - ${yamlScalar(entry)}`);
      }
    });
    return;
  }

  if (typeof value === "object") {
    if (!Object.keys(value).length) {
      return;
    }
    lines.push(`${prefix}${key}: ${JSON.stringify(value)}`);
    return;
  }

  if (typeof value === "number" || typeof value === "boolean") {
    lines.push(`${prefix}${key}: ${value}`);
    return;
  }

  lines.push(`${prefix}${key}: ${yamlScalar(value)}`);
}

function yamlLineForTypedProperty(key, type, rawValue) {
  const name = sanitizeSegment(key).replace(/\s+/g, "_").toLowerCase();
  if (!name) {
    return "";
  }

  const value = String(rawValue ?? "").trim();
  switch (type) {
    case "number":
    case "rating": {
      const parsed = Number.parseFloat(value.replace(",", "."));
      return Number.isNaN(parsed) ? `${name}: ${yamlScalar(value)}` : `${name}: ${parsed}`;
    }
    case "date":
      return `${name}: ${yamlScalar(value)}`;
    case "tags": {
      const values = value
        .split(",")
        .map((part) => part.trim())
        .filter(Boolean);
      return values.length
        ? `${name}:\n${values.map((entry) => `  - ${yamlScalar(entry)}`).join("\n")}`
        : `${name}: []`;
    }
    case "boolean":
      return `${name}: ${["true", "ja", "yes", "1"].includes(value.toLowerCase())}`;
    case "link":
    case "text":
    default:
      return `${name}: ${yamlScalar(value)}`;
  }
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
  appendYamlValue(lines, "mal_id", item.mal_id);
  appendYamlValue(lines, "source", item.source);
  appendYamlValue(lines, "country_of_origin", item.country_of_origin);
  appendYamlValue(lines, "hashtag", item.hashtag);
  appendYamlValue(lines, "mean_score", item.mean_score);
  appendYamlValue(lines, "popularity", item.popularity);
  appendYamlValue(lines, "favourites", item.favourites);
  appendYamlValue(lines, "start_date", item.start_date);
  appendYamlValue(lines, "end_date", item.end_date);
  appendYamlValue(lines, "genres", item.genres);
  appendYamlValue(lines, "synonyms", item.synonyms);
  appendYamlValue(lines, "tags", item.tags);
  appendYamlValue(lines, "studios", item.studios);
  appendYamlValue(lines, "relations", item.relations);
  appendYamlValue(lines, "characters", item.characters);
  appendYamlValue(lines, "staff", item.staff);
  appendYamlValue(lines, "reviews", item.reviews);
  appendYamlValue(lines, "cover_image_medium", item.cover_image_medium);
  appendYamlValue(lines, "cover_image_large", item.cover_image_large);
  appendYamlValue(lines, "cover_image_extra_large", item.cover_image_extra_large);
  appendYamlValue(lines, "cover_color", item.cover_color);
  appendYamlValue(lines, "banner_image", item.banner_image);
  appendYamlValue(lines, "trailer", item.trailer);
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

function normalizeAniListMetadata(metadata, item) {
  const title = metadata.title_english || metadata.title_romaji || metadata.title_native;
  const marker = episodeMarker(item.source_path) || episodeMarker(item.title);
  const normalized = {
    title: title || item.title,
    title_original: metadata.title_native || null,
    series_title: title || item.series_title || item.title,
    description: metadata.description || null,
    media_type: "anime",
    year: metadata.season_year ?? item.year ?? null,
    start_date: metadata.start_date ?? null,
    end_date: metadata.end_date ?? null,
    episode_count: metadata.episodes ?? null,
    runtime_minutes: metadata.duration ?? null,
    average_score: metadata.average_score ?? null,
    mean_score: metadata.mean_score ?? null,
    popularity: metadata.popularity ?? null,
    favourites: metadata.favourites ?? null,
    format: metadata.format || null,
    source: metadata.source || null,
    country_of_origin: metadata.country_of_origin || null,
    hashtag: metadata.hashtag || null,
    airing_season: metadata.season || null,
    anilist_id: metadata.anilist_id ?? null,
    anilist_url: metadata.anilist_url || null,
    mal_id: metadata.mal_id ?? null,
    genres: Array.isArray(metadata.genres) ? metadata.genres : [],
    synonyms: Array.isArray(metadata.synonyms) ? metadata.synonyms : [],
    tags: Array.isArray(metadata.tags) ? metadata.tags : [],
    studios: Array.isArray(metadata.studios) ? metadata.studios : [],
    relations: Array.isArray(metadata.relations) ? metadata.relations : [],
    characters: Array.isArray(metadata.characters) ? metadata.characters : [],
    staff: Array.isArray(metadata.staff) ? metadata.staff : [],
    reviews: Array.isArray(metadata.reviews) ? metadata.reviews : [],
    cover_image_medium: metadata.cover_image_medium || null,
    cover_image_large: metadata.cover_image_large || null,
    cover_image_extra_large: metadata.cover_image_extra_large || null,
    cover_color: metadata.cover_color || null,
    banner_image: metadata.banner_image || null,
    trailer: metadata.trailer || null,
  };

  if (marker) {
    normalized.season_number = marker.season;
    normalized.episode_start = marker.episodeStart;
    normalized.episode_end = marker.episodeEnd;
    normalized.episode_title = deriveEpisodeTitle(item);
  }

  Object.keys(normalized).forEach((key) => {
    if (
      normalized[key] === null ||
      normalized[key] === "" ||
      (Array.isArray(normalized[key]) && !normalized[key].length)
    ) {
      delete normalized[key];
    }
  });

  return normalized;
}

function setApiFeedback(node, message, tone = "") {
  if (!node) {
    return;
  }
  node.className = `api-feedback${tone ? ` is-${tone}` : ""}`;
  node.textContent = message;
}

function aniListSummary(metadata) {
  const parts = [
    metadata.title || metadata.series_title,
    metadata.format ? `Format ${metadata.format}` : "",
    metadata.year ? `Jahr ${metadata.year}` : "",
    typeof metadata.episode_count === "number" ? `${metadata.episode_count} Folgen` : "",
    typeof metadata.average_score === "number" ? `Score ${Math.round(metadata.average_score)} / 100` : "",
    Array.isArray(metadata.tags) ? `${metadata.tags.length} Tags` : "",
    Array.isArray(metadata.relations) ? `${metadata.relations.length} Relationen` : "",
    Array.isArray(metadata.characters) ? `${metadata.characters.length} Charaktere` : "",
    Array.isArray(metadata.staff) ? `${metadata.staff.length} Staff` : "",
  ].filter(Boolean);
  return parts.join(" | ");
}

async function fetchAniListForItem(item, feedbackNode = null, targetItems = [item]) {
  const searchTitle =
    item.series_title ||
    item.title ||
    detailTitle?.value.trim() ||
    stripEpisodeMarkerText(fileStem(item.source_path));
  if (!searchTitle) {
    setApiFeedback(feedbackNode, `Kein Suchbegriff für ${item.source_path} vorhanden.`, "error");
    if (statusStrip) {
      statusStrip.textContent = "Kein Suchbegriff für AniList vorhanden.";
    }
    return;
  }

  if (statusStrip) {
    statusStrip.textContent = `AniList-Suche läuft für: ${searchTitle}`;
  }
  setApiFeedback(feedbackNode, `Suche AniList für ${item.source_path} mit "${searchTitle}"...`, "loading");

  const adult = canonicalMediaType(item.media_type) === "hentai-anime";
  const response = await fetch(`/api/anilist-search?title=${encodeURIComponent(searchTitle)}&adult=${adult ? "true" : "false"}`);
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  const payload = await response.json();
  if (!payload.metadata) {
    setApiFeedback(
      feedbackNode,
      `Kein AniList-Treffer für "${searchTitle}" bei ${item.source_path}. Titel korrigieren und erneut abrufen.`,
      "error"
    );
    throw new Error(payload.error || "Kein AniList-Treffer");
  }

  const normalized = normalizeAniListMetadata(payload.metadata, item);
  targetItems.forEach((target) => {
    apiMetadata[target.source_path] = normalized;
  });
  saveStoredJson(metadataKey, apiMetadata);
  currentPlan = projectPlan(sourcePlan);
  selectedItemKey = item.source_path;
  renderPlan(currentPlan);
  setApiFeedback(feedbackNode, `AniList übernommen: ${aniListSummary(normalized)}`, "success");
  updateAuditTrail(`AniList-Metadaten übernommen für ${item.source_path}.`);
  return normalized;
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
    if (currentActiveTab() !== "collections") {
      renderInspector(null);
    }
    return;
  }

  detailHeading.textContent = item.title || fileStem(item.source_path) || "Unbenannt";
  detailSubtitle.textContent = item.has_correction
    ? "Für diesen Eintrag existiert bereits eine lokale Korrektur."
    : "Die gewählte Datei wird hier mit Zielpfad, Sidecar-Vorschau und Korrekturfeldern angezeigt.";

  if (detailSourcePath) detailSourcePath.textContent = item.source_path;
  if (detailTargetPath) detailTargetPath.textContent = item.target_path || "-";
  if (detailMediaType) detailMediaType.textContent = mediaTypeLabel(mediaSelectionValue(item));
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
  if (detailMediaTypeInput) detailMediaTypeInput.value = mediaSelectionValue(item);
  if (detailYear) detailYear.value = item.year ?? "";
  if (detailStatus) detailStatus.value = item.status ?? (item.needs_review ? "needs-review" : "inbox");
  if (detailNotes) detailNotes.value = item.notes ?? "";
  if (detailSidecarPreview) detailSidecarPreview.textContent = item.sidecar_preview;
  if (currentActiveTab() !== "collections") {
    renderInspector(item);
  }
}

function propertyTypeLabel(key, value) {
  if (key.includes("url") || key.includes("link") || key.includes("image")) {
    return "Link";
  }
  if (key.includes("date")) {
    return "Datum";
  }
  if (key.includes("score") || key.includes("rating")) {
    return "Bewertung";
  }
  if (typeof value === "number") {
    return "Nummer";
  }
  if (Array.isArray(value)) {
    return key === "tags" || key === "genres" ? "Tags" : "Liste";
  }
  if (value && typeof value === "object") {
    return "Objekt";
  }
  return "Text";
}

function propertyDisplayValue(value) {
  if (Array.isArray(value)) {
    if (!value.length) {
      return "-";
    }
    return value
      .slice(0, 5)
      .map((entry) => {
        if (entry && typeof entry === "object") {
          return entry.name || entry.title || entry.character_name || entry.relation_type || JSON.stringify(entry);
        }
        return String(entry);
      })
      .join(", ");
  }

  if (value && typeof value === "object") {
    return JSON.stringify(value);
  }

  return value === null || typeof value === "undefined" || value === "" ? "-" : String(value);
}

function propertyEntriesFor(value) {
  const selection = normalizeSelection(value);
  if (!selection) {
    return [];
  }
  const item = selection.item ?? {};
  const baseEntries = selection.type === "node"
    ? [
        ["object_type", nodeTypeLabel(selection.node)],
        ["collection_path", selection.node.path || "Sammlungen"],
        ["items", selection.items.length],
        ["folders", sortedChildren(selection.node).length],
      ]
    : [
        ["source_path", item.source_path],
        ["target_path", item.target_path],
        ["sidecar_path", item.sidecar_path],
      ];

  return [
    ...baseEntries,
    ["media_type", mediaTypeLabel(mediaSelectionValue(item))],
    ["format", item.format],
    ["title", item.title],
    ["series_title", item.series_title],
    ["season_number", item.season_number],
    ["episode_start", item.episode_start],
    ["episode_end", item.episode_end],
    ["year", item.year],
    ["status", item.status ? statusLabel(item.status) : item.needs_review ? "Prüfung nötig" : null],
    ["anilist_id", item.anilist_id],
    ["anilist_url", item.anilist_url],
    ["mal_id", item.mal_id],
    ["average_score", item.average_score],
    ["mean_score", item.mean_score],
    ["popularity", item.popularity],
    ["favourites", item.favourites],
    ["genres", item.genres],
    ["tags", item.tags],
    ["studios", item.studios],
    ["relations", item.relations],
    ["characters", item.characters],
    ["staff", item.staff],
    ["reviews", item.reviews],
    ["cover_image_extra_large", item.cover_image_extra_large],
    ["banner_image", item.banner_image],
  ].filter(([, entryValue]) => {
    if (Array.isArray(entryValue)) {
      return entryValue.length;
    }
    return entryValue !== null && typeof entryValue !== "undefined" && entryValue !== "";
  });
}

function renderInspector(value) {
  const selection = normalizeSelection(value);
  inspectorSelection = selection;
  inspectorItemKey = selection?.key ?? "";
  const item = selection?.item ?? null;

  if (inspectorTitle) {
    inspectorTitle.textContent = selectionTitle(selection);
  }
  if (inspectorName) {
    inspectorName.value = selection ? selectionTitle(selection) : "";
  }
  if (inspectorMediaType) {
    inspectorMediaType.value = item ? mediaSelectionValue(item) : "unclassified";
  }
  if (inspectorYear) {
    inspectorYear.value = item?.year ?? "";
  }
  if (inspectorStatus) {
    inspectorStatus.value = item?.status ?? (item?.needs_review ? "needs-review" : "inbox");
  }

  if (inspectorProperties) {
    clearNode(inspectorProperties);
    const entries = propertyEntriesFor(selection);
    if (!entries.length) {
      const empty = document.createElement("div");
      empty.className = "property-empty";
      empty.textContent = "Wähle eine Datei aus, um Properties zu sehen.";
      inspectorProperties.appendChild(empty);
    } else {
      entries.forEach(([key, value]) => {
        const row = document.createElement("div");
        row.className = "property-row";

        const name = document.createElement("strong");
        name.textContent = key;
        const type = document.createElement("span");
        type.textContent = propertyTypeLabel(key, value);
        const body = document.createElement("p");
        body.textContent = propertyDisplayValue(value);

        row.appendChild(name);
        row.appendChild(type);
        row.appendChild(body);
        inspectorProperties.appendChild(row);
      });
    }
  }

  if (inspectorYaml) {
    inspectorYaml.value = selectionYamlPreview(selection);
    inspectorYaml.disabled = !selection;
  }
  if (inspectorYamlSave) {
    inspectorYamlSave.disabled = !selection;
  }
  if (inspectorYamlReset) {
    inspectorYamlReset.disabled = !selection || !yamlOverrides[selection.key];
  }
  [
    inspectorEditToggle,
    inspectorFetchMetadata,
    inspectorApply,
    inspectorPropertyAddToggle,
    inspectorPropertyKey,
    inspectorPropertyType,
    inspectorPropertyValue,
    inspectorPropertyAdd,
  ].forEach((control) => {
    if (control) {
      control.disabled = !selection;
    }
  });
  if (inspectorYamlHint) {
    inspectorYamlHint.textContent = selection
      ? `Lokale YAML-Fassung für ${selection.key}.`
      : "Wähle eine Datei oder Sammlung aus, um YAML zu bearbeiten.";
  }
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

function createCollectionNode(path, label, parent = null) {
  return {
    path,
    label,
    parent,
    children: new Map(),
    items: [],
    directItems: [],
  };
}

function buildCollectionTree(items) {
  const root = createCollectionNode("", "Sammlungen");

  items.forEach((item) => {
    const parts = String(item.collection_path || folderSegmentFor(item.media_type))
      .split("/")
      .map((part) => part.trim())
      .filter(Boolean);
    let node = root;
    node.items.push(item);

    parts.forEach((part, index) => {
      const path = parts.slice(0, index + 1).join("/");
      if (!node.children.has(part)) {
        node.children.set(part, createCollectionNode(path, part, node));
      }
      node = node.children.get(part);
      node.items.push(item);
    });

    node.directItems.push(item);
  });

  return root;
}

function findCollectionNode(root, path) {
  if (!path) {
    return root;
  }

  return path
    .split("/")
    .filter(Boolean)
    .reduce((node, part) => node?.children.get(part), root) ?? root;
}

function sortedChildren(node) {
  return Array.from(node.children.values()).sort((left, right) =>
    left.label.localeCompare(right.label, "de")
  );
}

function parentPath(path) {
  const parts = String(path || "").split("/").filter(Boolean);
  parts.pop();
  return parts.join("/");
}

function collectionNodeKind(node) {
  if (!node.path) {
    return "root";
  }
  if (node.path.includes("/Staffel ")) {
    return "season";
  }
  if (node.path.includes("/Filme/")) {
    return "movie";
  }
  if (node.path.endsWith("/Serien") || node.path.endsWith("/Filme")) {
    return "group";
  }
  if (sortedChildren(node).some((child) => child.label.startsWith("Staffel "))) {
    return "series";
  }
  return sortedChildren(node).length ? "folder" : "folder";
}

function nodeTypeLabel(node) {
  const kind = collectionNodeKind(node);
  switch (kind) {
    case "root":
      return "Vault";
    case "series":
      return "Serie";
    case "season":
      return "Staffel";
    case "movie":
      return "Film";
    case "group":
      return "Ordner";
    default:
      return "Sammlung";
  }
}

function representativeItem(node) {
  return node.items.find((item) => item.anilist_id || item.series_title) ?? node.items[0] ?? null;
}

function normalizeSelection(value) {
  if (!value) {
    return null;
  }
  if (value.source_path) {
    return {
      type: "file",
      key: value.source_path,
      item: value,
      items: [value],
      node: null,
    };
  }
  if (value.type === "node" && value.node) {
    return {
      type: "node",
      key: `node:${value.node.path || "root"}`,
      item: representativeItem(value.node),
      items: value.node.items,
      node: value.node,
    };
  }
  return null;
}

function selectionTitle(selection) {
  if (!selection) {
    return "Kein Eintrag ausgewählt";
  }
  if (selection.type === "node") {
    const primary = selection.item;
    const kind = collectionNodeKind(selection.node);
    if (kind === "series") {
      return primary?.series_title || selection.node.label;
    }
    return selection.node.label || primary?.title || "Sammlung";
  }
  return selection.item?.title || fileStem(selection.item?.source_path) || "Unbenannt";
}

function selectionYamlPreview(selection) {
  if (!selection) {
    return "";
  }
  if (yamlOverrides[selection.key]) {
    return yamlOverrides[selection.key];
  }
  if (selection.type === "file") {
    return selection.item?.sidecar_preview ?? "";
  }

  const primary = selection.item ?? {};
  const lines = [
    "---",
    `id: ${yamlScalar(selection.key)}`,
    `object_type: ${yamlScalar(nodeTypeLabel(selection.node).toLowerCase())}`,
    `collection_path: ${yamlScalar(selection.node.path || "Sammlungen")}`,
    `title: ${yamlScalar(selectionTitle(selection))}`,
    `media_type: ${yamlScalar(primary.media_type ?? "collection")}`,
  ];
  appendYamlValue(lines, "format", primary.format);
  appendYamlValue(lines, "year", primary.year);
  appendYamlValue(lines, "anilist_id", primary.anilist_id);
  appendYamlValue(lines, "anilist_url", primary.anilist_url);
  appendYamlValue(lines, "genres", primary.genres);
  appendYamlValue(lines, "tags", primary.tags);
  appendYamlValue(lines, "cover_image_extra_large", primary.cover_image_extra_large);
  appendYamlValue(lines, "banner_image", primary.banner_image);
  lines.push("---");
  return lines.join("\n");
}

function collectionDescriptionFor(node) {
  const kind = collectionNodeKind(node);
  if (kind === "root") {
    return "Wähle links einen Medienbereich. Dateien erscheinen erst auf Staffel- oder Filmebene.";
  }
  if (kind === "series") {
    return "Serienübersicht mit Staffeln. Wähle eine Staffel, um die Dateien zu sehen.";
  }
  if (kind === "season") {
    return "Staffelinhalt. Hier kannst du einzelne Dateien auswählen und bearbeiten.";
  }
  if (kind === "movie") {
    return "Filmübersicht. Die zugehörige Datei kann unten ausgewählt und korrigiert werden.";
  }
  return "Ordneransicht innerhalb der geplanten Vault-Struktur.";
}

function renderCollectionMeta(node) {
  if (!collectionMeta) {
    return;
  }

  clearNode(collectionMeta);

  const children = sortedChildren(node);
  const totalBytes = node.items.reduce((sum, item) => sum + (item.size_bytes || 0), 0);
  const reviewCount = node.items.filter((item) => item.needs_review).length;

  [
    ["Ordner", `${children.length}`],
    ["Dateien", `${node.items.length}`],
    ["Größe", formatBytes(totalBytes)],
    ["Review", `${reviewCount}`],
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

function renderCollectionNavigation(root, node) {
  if (!collectionSidebarList) {
    return;
  }

  clearNode(collectionSidebarList);

  if (node.path) {
    const up = document.createElement("button");
    up.type = "button";
    up.className = "collection-nav-item";
    up.addEventListener("click", () => {
      selectedCollectionKey = parentPath(node.path);
      selectedCollectionItemKey = "";
      renderCollections(currentPlan?.items ?? []);
    });

    const label = document.createElement("strong");
    label.textContent = "Eine Ebene zurück";
    const count = document.createElement("span");
    count.textContent = "";
    up.appendChild(label);
    up.appendChild(count);
    collectionSidebarList.appendChild(up);
  }

  sortedChildren(node).forEach((child) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "collection-nav-item";
    button.classList.toggle("is-active", child.path === selectedCollectionKey);
    button.addEventListener("click", () => {
      selectedCollectionKey = child.path;
      selectedCollectionItemKey = "";
      renderCollections(currentPlan?.items ?? []);
    });

    const label = document.createElement("strong");
    label.textContent = child.label;
    const count = document.createElement("span");
    count.textContent = `${child.items.length}`;

    button.appendChild(label);
    button.appendChild(count);
    collectionSidebarList.appendChild(button);
  });

  if (!sortedChildren(node).length && node.path) {
    const empty = document.createElement("div");
    empty.className = "collection-empty compact";
    empty.textContent = "Keine Unterordner.";
    collectionSidebarList.appendChild(empty);
  }
}

function renderCollectionBreadcrumb(node) {
  if (!collectionBreadcrumb) {
    return;
  }

  clearNode(collectionBreadcrumb);

  const rootButton = document.createElement("button");
  rootButton.type = "button";
  rootButton.textContent = "Sammlungen";
  rootButton.addEventListener("click", () => {
    selectedCollectionKey = "";
    selectedCollectionItemKey = "";
    renderCollections(currentPlan?.items ?? []);
  });
  collectionBreadcrumb.appendChild(rootButton);

  const parts = String(node.path || "").split("/").filter(Boolean);
  parts.forEach((part, index) => {
    const separator = document.createElement("span");
    separator.textContent = ">";
    collectionBreadcrumb.appendChild(separator);

    const path = parts.slice(0, index + 1).join("/");
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = part;
    button.classList.toggle("is-active", index === parts.length - 1);
    button.addEventListener("click", () => {
      selectedCollectionKey = path;
      selectedCollectionItemKey = "";
      renderCollections(currentPlan?.items ?? []);
    });
    collectionBreadcrumb.appendChild(button);
  });
}

function renderCollectionProfile(node) {
  if (!collectionProfile) {
    return;
  }

  clearNode(collectionProfile);

  const primary = representativeItem(node);
  if (!primary || ["root", "group", "folder"].includes(collectionNodeKind(node))) {
    return;
  }

  const profile = document.createElement("section");
  profile.className = "collection-profile-card";
  if (primary.banner_image) {
    profile.style.setProperty("--media-banner", `url("${primary.banner_image}")`);
    profile.classList.add("has-banner");
  }

  const cover = document.createElement("div");
  cover.className = "collection-cover";
  const coverUrl = primary.cover_image_extra_large || primary.cover_image_large || primary.cover_image_medium;
  if (coverUrl) {
    const image = document.createElement("img");
    image.src = coverUrl;
    image.alt = primary.series_title || primary.title || node.label;
    cover.appendChild(image);
  } else {
    cover.textContent = "MV";
  }

  const heading = document.createElement("div");
  heading.className = "collection-profile-heading";

  const label = document.createElement("span");
  label.textContent = nodeTypeLabel(node);
  const name = document.createElement("strong");
  name.textContent = primary.series_title || primary.title || node.label;
  const meta = document.createElement("p");
  const parts = [
    primary.format,
    primary.airing_season,
    primary.year,
    typeof primary.episode_count === "number" ? `${primary.episode_count} Folgen` : "",
    typeof primary.runtime_minutes === "number" ? `${primary.runtime_minutes} Min.` : "",
    typeof primary.average_score === "number" ? `${Math.round(primary.average_score)} / 100` : "",
  ].filter(Boolean);
  meta.textContent = parts.join(" · ") || "Noch keine externen Metadaten vorhanden.";

  const description = document.createElement("p");
  description.className = "collection-profile-description";
  description.textContent =
    primary.description?.replace(/<[^>]*>/g, "") ||
    "Noch keine Beschreibung vorhanden. Starte AniList in der rechten Seitenleiste, um Serien- oder Staffelinformationen zu ergänzen.";

  const tagBar = document.createElement("div");
  tagBar.className = "media-tagbar";
  (primary.genres || []).slice(0, 8).forEach((genre) => {
    const tag = document.createElement("span");
    tag.textContent = genre;
    tagBar.appendChild(tag);
  });

  heading.appendChild(label);
  heading.appendChild(name);
  heading.appendChild(meta);
  heading.appendChild(description);
  if (tagBar.childNodes.length) {
    heading.appendChild(tagBar);
  }

  profile.appendChild(cover);
  profile.appendChild(heading);

  if (primary.anilist_url) {
    const link = document.createElement("a");
    link.className = "collection-profile-link";
    link.href = primary.anilist_url;
    link.textContent = "AniList öffnen";
    profile.appendChild(link);
  }

  collectionProfile.appendChild(profile);
}

function createCollectionFolderRow(node) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "table-row table-row-button collection-folder-row";
  row.addEventListener("click", () => {
    selectedCollectionKey = node.path;
    selectedCollectionItemKey = "";
    renderCollections(currentPlan?.items ?? []);
  });

  [
    node.label,
    nodeTypeLabel(node),
    `${sortedChildren(node).length} Ordner / ${node.items.length} Dateien`,
  ].forEach((cell) => {
    const span = document.createElement("span");
    span.textContent = cell;
    row.appendChild(span);
  });

  return row;
}

function createCollectionItemRow(item) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "table-row table-row-button";
  row.classList.toggle("is-selected", item.source_path === selectedCollectionItemKey);
  row.dataset.sourcePath = item.source_path;
  row.addEventListener("click", () => {
    selectedItemKey = item.source_path;
    selectedCollectionItemKey = item.source_path;
    renderCollectionEditor(item);
    renderInspector(item);
    renderCollectionRows(findCollectionNode(buildCollectionTree(currentPlan?.items ?? []), selectedCollectionKey));
    if (statusStrip) {
      statusStrip.textContent = `Eintrag ausgewählt: ${item.source_path}`;
    }
  });

  [
    item.source_path,
    mediaTypeLabel(mediaSelectionValue(item)),
    item.target_path || statusLabel(item.status || "inbox"),
  ].forEach((cell) => {
    const span = document.createElement("span");
    span.textContent = cell;
    row.appendChild(span);
  });

  return row;
}

function renderCollectionRows(node) {
  if (!collectionRows) {
    return;
  }

  clearNode(collectionRows);

  sortedChildren(node).forEach((child) => {
    collectionRows.appendChild(createCollectionFolderRow(child));
  });

  if (!sortedChildren(node).length) {
    node.directItems.forEach((item) => {
      collectionRows.appendChild(createCollectionItemRow(item));
    });
  }

  if (!sortedChildren(node).length && !node.directItems.length) {
    const empty = document.createElement("div");
    empty.className = "collection-empty";
    empty.textContent = "Keine Einträge in dieser Sammlung.";
    collectionRows.appendChild(empty);
  }
}

function renderCollections(items) {
  const root = buildCollectionTree(items);
  const selected = findCollectionNode(root, selectedCollectionKey);

  if (!selected.items.length) {
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
    if (collectionBreadcrumb) {
      clearNode(collectionBreadcrumb);
    }
    renderCollectionEditor(null);
    return;
  }

  selectedCollectionKey = selected.path;

  if (collectionTitle) {
    collectionTitle.textContent = selected.label;
  }
  if (collectionDescription) {
    collectionDescription.textContent = collectionDescriptionFor(selected);
  }
  if (metricCollections) {
    metricCollections.textContent = `${root.items.length} Dateien`;
  }

  renderCollectionNavigation(root, selected);
  renderCollectionBreadcrumb(selected);
  renderCollectionMeta(selected);
  renderCollectionProfile(selected);
  renderCollectionRows(selected);

  const selectedItem = selected.directItems.find((item) => item.source_path === selectedCollectionItemKey);
  renderCollectionEditor(selectedItem ?? null);
  if (!selectedItem && currentActiveTab() === "collections") {
    renderInspector({ type: "node", node: selected });
  }
}

function renderCollectionEditor(item) {
  if (!collectionEditor) {
    return;
  }

  collectionEditor.classList.remove("is-active");

  if (!item) {
    if (collectionEditorTitle) collectionEditorTitle.textContent = "Eintrag auswählen";
    if (collectionEditorSubtitle) {
      collectionEditorSubtitle.textContent =
        "Wähle unten eine Datei, um Zielpfad, Metadaten und Sidecar zu prüfen.";
    }
    if (collectionEditorSource) collectionEditorSource.textContent = "-";
    if (collectionEditorTarget) collectionEditorTarget.textContent = "-";
    if (collectionEditorType) collectionEditorType.textContent = "-";
    if (collectionEditorSidecar) collectionEditorSidecar.textContent = "-";
    if (collectionEditorName) collectionEditorName.value = "";
    if (collectionEditorMediaType) collectionEditorMediaType.value = "unclassified";
    if (collectionEditorYear) collectionEditorYear.value = "";
    if (collectionEditorStatus) collectionEditorStatus.value = "inbox";
    if (collectionEditorNotes) collectionEditorNotes.value = "";
    if (collectionEditorSidecarPreview) {
      collectionEditorSidecarPreview.textContent = "Wähle einen Eintrag aus.";
    }
    return;
  }

  if (collectionEditorTitle) {
    collectionEditorTitle.textContent = item.title || fileStem(item.source_path) || "Unbenannt";
  }
  if (collectionEditorSubtitle) {
    collectionEditorSubtitle.textContent = item.collection_path || "Noch keiner Sammlung zugeordnet.";
  }
  if (collectionEditorSource) collectionEditorSource.textContent = item.source_path;
  if (collectionEditorTarget) collectionEditorTarget.textContent = item.target_path || "-";
  if (collectionEditorType) collectionEditorType.textContent = mediaTypeLabel(mediaSelectionValue(item));
  if (collectionEditorSidecar) collectionEditorSidecar.textContent = item.sidecar_path || "-";
  if (collectionEditorName) collectionEditorName.value = item.title ?? "";
  if (collectionEditorMediaType) collectionEditorMediaType.value = mediaSelectionValue(item);
  if (collectionEditorYear) collectionEditorYear.value = item.year ?? "";
  if (collectionEditorStatus) {
    collectionEditorStatus.value = item.status ?? (item.needs_review ? "needs-review" : "inbox");
  }
  if (collectionEditorNotes) collectionEditorNotes.value = item.notes ?? "";
  if (collectionEditorSidecarPreview) {
    collectionEditorSidecarPreview.textContent = item.sidecar_preview;
  }
  if (currentActiveTab() === "collections") {
    renderInspector(item);
  }
}

function upsertCollectionCorrection(item) {
  corrections[item.source_path] = {
    title: collectionEditorName?.value.trim() || null,
    media_type: collectionEditorMediaType?.value || item.media_type,
    year: normalizeYear(collectionEditorYear?.value),
    status: collectionEditorStatus?.value || null,
    notes: collectionEditorNotes?.value.trim() || null,
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

function currentCollectionEditorItem() {
  if (!currentPlan || !selectedCollectionItemKey) {
    return null;
  }

  return currentPlan.items.find((item) => item.source_path === selectedCollectionItemKey) ?? null;
}

function rerenderAfterCollectionEdit(item) {
  currentPlan = projectPlan(sourcePlan);
  const updated = currentPlan.items.find((candidate) => candidate.source_path === item.source_path);
  if (updated) {
    selectedCollectionItemKey = updated.source_path;
    selectedItemKey = updated.source_path;
    selectedCollectionKey = updated.collection_path || selectedCollectionKey;
  }
  renderPlan(currentPlan);
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

function syncTemplateInputs() {
  pathTemplates = normalizeTemplateConfig(pathTemplates);
  if (templateAnimeTv) templateAnimeTv.value = pathTemplates.animeTv;
  if (templateAnimeMovie) templateAnimeMovie.value = pathTemplates.animeMovie;
  if (templateSeries) templateSeries.value = pathTemplates.series;
  if (templateFilm) templateFilm.value = pathTemplates.film;
}

function readTemplateInputs() {
  return normalizeTemplateConfig({
    animeTv: templateAnimeTv?.value.trim(),
    animeMovie: templateAnimeMovie?.value.trim(),
    series: templateSeries?.value.trim(),
    film: templateFilm?.value.trim(),
  });
}

function populateSelectors() {
  [detailMediaTypeInput, collectionEditorMediaType, inspectorMediaType].forEach((select) => {
    if (!select) {
      return;
    }
    clearNode(select);
    mediaTypeOptions.forEach(([value, label]) => {
      const option = document.createElement("option");
      option.value = value;
      option.textContent = label;
      select.appendChild(option);
    });
  });

  [detailStatus, collectionEditorStatus, inspectorStatus].forEach((select) => {
    if (!select) {
      return;
    }
    clearNode(select);
    statusOptions.forEach(([value, label]) => {
      const option = document.createElement("option");
      option.value = value;
      option.textContent = label;
      select.appendChild(option);
    });
  });
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

function upsertSelectionCorrection(selection) {
  if (!selection) {
    return;
  }

  const titleValue = inspectorName?.value.trim() || null;
  selection.items.forEach((item) => {
    const existing = corrections[item.source_path] ?? {};
    const next = {
      ...existing,
      media_type: inspectorMediaType?.value || existing.media_type || mediaSelectionValue(item),
      year: normalizeYear(inspectorYear?.value),
      status: inspectorStatus?.value || existing.status || null,
      updated_at: new Date().toISOString(),
    };

    if (selection.type === "node") {
      next.series_title = titleValue;
      if (collectionNodeKind(selection.node) === "movie") {
        next.title = titleValue;
      }
    } else {
      next.title = titleValue;
    }

    if (!next.title) {
      delete next.title;
    }
    if (!next.series_title) {
      delete next.series_title;
    }
    if (next.year === null) {
      delete next.year;
    }
    if (!next.status) {
      delete next.status;
    }
    corrections[item.source_path] = next;
  });

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
    if (!document.body.classList.contains("is-vault-open")) {
      openVault(getVaultRoot());
    } else {
      loadPlan();
    }
  });
}

if (vaultOpenButton) {
  vaultOpenButton.addEventListener("click", () => {
    openVault(vaultOpenPath?.value || getVaultRoot());
  });
}

if (vaultCreateButton) {
  vaultCreateButton.addEventListener("click", () => {
    const path = vaultCreatePath?.value.trim();
    const name = vaultCreateName?.value.trim() || basename(path || "");
    openVault(path, name);
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

if (templatesSave) {
  templatesSave.addEventListener("click", () => {
    pathTemplates = readTemplateInputs();
    saveStoredJson(pathTemplatesKey, pathTemplates);
    currentPlan = sourcePlan ? projectPlan(sourcePlan) : currentPlan;
    if (currentPlan) {
      renderPlan(currentPlan);
    }
    if (templatesHint) {
      templatesHint.textContent =
        "Templates gespeichert. Zielpfade wurden neu berechnet; echte Verschiebungen werden später separat bestätigt.";
    }
  });
}

if (templatesReset) {
  templatesReset.addEventListener("click", () => {
    pathTemplates = defaultPathTemplates();
    saveStoredJson(pathTemplatesKey, pathTemplates);
    syncTemplateInputs();
    currentPlan = sourcePlan ? projectPlan(sourcePlan) : currentPlan;
    if (currentPlan) {
      renderPlan(currentPlan);
    }
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

if (detailFetchMetadata) {
  detailFetchMetadata.addEventListener("click", async () => {
    const item = getSelectedItem();
    if (!item) {
      return;
    }

    try {
      upsertCorrection(item);
      const draft = {
        ...item,
        title: detailTitle?.value.trim() || item.title,
        series_title: detailTitle?.value.trim() || item.series_title,
        media_type: detailMediaTypeInput?.value || item.media_type,
      };
      await fetchAniListForItem(draft, detailApiFeedback);
    } catch (error) {
      setApiFeedback(
        detailApiFeedback,
        `AniList konnte keine Daten liefern: ${error.message}. Titel prüfen und erneut abrufen.`,
        "error"
      );
      if (statusStrip) {
        statusStrip.textContent = `AniList konnte keine Daten liefern: ${error.message}`;
      }
    }
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

if (collectionEditorApply) {
  collectionEditorApply.addEventListener("click", () => {
    const item = currentCollectionEditorItem();
    if (!item) {
      return;
    }

    upsertCollectionCorrection(item);
    rerenderAfterCollectionEdit(item);
    updateAuditTrail(`Korrektur übernommen für ${item.source_path}.`);
  });
}

if (collectionEditorFetch) {
  collectionEditorFetch.addEventListener("click", async () => {
    const item = currentCollectionEditorItem();
    if (!item) {
      return;
    }

    try {
      upsertCollectionCorrection(item);
      const draft = {
        ...item,
        title: collectionEditorName?.value.trim() || item.title,
        series_title: collectionEditorName?.value.trim() || item.series_title,
        media_type: collectionEditorMediaType?.value || item.media_type,
      };
      await fetchAniListForItem(draft, collectionEditorApiFeedback);
      const updated = currentPlan?.items.find((candidate) => candidate.source_path === item.source_path);
      if (updated) {
        selectedCollectionKey = updated.collection_path || selectedCollectionKey;
        selectedCollectionItemKey = updated.source_path;
      }
      renderPlan(currentPlan);
    } catch (error) {
      setApiFeedback(
        collectionEditorApiFeedback,
        `AniList konnte keine Daten liefern: ${error.message}. Titel prüfen und erneut abrufen.`,
        "error"
      );
      if (statusStrip) {
        statusStrip.textContent = `AniList konnte keine Daten liefern: ${error.message}`;
      }
    }
  });
}

if (collectionEditorReset) {
  collectionEditorReset.addEventListener("click", () => {
    const item = currentCollectionEditorItem();
    if (!item) {
      return;
    }

    clearCorrection(item);
    rerenderAfterCollectionEdit(item);
    updateAuditTrail(`Lokale Korrektur zurückgesetzt für ${item.source_path}.`);
  });
}

if (inspectorToggle) {
  inspectorToggle.addEventListener("click", () => {
    document.body.classList.toggle("inspector-collapsed");
    inspectorToggle.textContent = document.body.classList.contains("inspector-collapsed")
      ? "Properties anzeigen"
      : "Properties ausblenden";
  });
}

if (inspectorEditToggle) {
  inspectorEditToggle.addEventListener("click", () => {
    document.body.classList.toggle("inspector-editing");
    inspectorEditToggle.textContent = document.body.classList.contains("inspector-editing")
      ? "Edit schließen"
      : "Bearbeiten";
  });
}

if (inspectorPropertyAddToggle) {
  inspectorPropertyAddToggle.addEventListener("click", () => {
    document.body.classList.toggle("property-add-open");
  });
}

if (inspectorApply) {
  inspectorApply.addEventListener("click", () => {
    if (!inspectorSelection) {
      return;
    }

    upsertSelectionCorrection(inspectorSelection);
    currentPlan = projectPlan(sourcePlan);
    if (inspectorSelection.type === "node") {
      const titleValue = inspectorName?.value.trim();
      if (titleValue) {
        const updated = currentPlan.items.find((item) => {
          return item.series_title === titleValue || item.title === titleValue;
        });
        selectedCollectionKey = updated?.collection_path || selectedCollectionKey;
      }
    } else {
      selectedItemKey = inspectorSelection.item.source_path;
    }
    renderPlan(currentPlan);
    updateAuditTrail(`Inspector-Korrektur übernommen für ${inspectorSelection.key}.`);
  });
}

if (inspectorFetchMetadata) {
  inspectorFetchMetadata.addEventListener("click", async () => {
    if (!inspectorSelection || !inspectorSelection.item) {
      return;
    }

    try {
      upsertSelectionCorrection(inspectorSelection);
      const draft = {
        ...inspectorSelection.item,
        title: inspectorName?.value.trim() || inspectorSelection.item.title,
        series_title: inspectorName?.value.trim() || inspectorSelection.item.series_title,
        media_type: inspectorMediaType?.value || mediaSelectionValue(inspectorSelection.item),
      };
      await fetchAniListForItem(draft, inspectorApiFeedback, inspectorSelection.items);
      renderPlan(currentPlan);
    } catch (error) {
      setApiFeedback(
        inspectorApiFeedback,
        `AniList konnte keine Daten liefern: ${error.message}. Titel prüfen und erneut abrufen.`,
        "error"
      );
    }
  });
}

if (inspectorYamlSave) {
  inspectorYamlSave.addEventListener("click", () => {
    if (!inspectorItemKey || !inspectorYaml) {
      return;
    }

    yamlOverrides[inspectorItemKey] = inspectorYaml.value;
    saveStoredJson(yamlOverridesKey, yamlOverrides);
    currentPlan = projectPlan(sourcePlan);
    renderPlan(currentPlan);
    updateAuditTrail(`YAML lokal gespeichert für ${inspectorItemKey}.`);
  });
}

if (inspectorYamlReset) {
  inspectorYamlReset.addEventListener("click", () => {
    if (!inspectorItemKey) {
      return;
    }

    delete yamlOverrides[inspectorItemKey];
    saveStoredJson(yamlOverridesKey, yamlOverrides);
    currentPlan = projectPlan(sourcePlan);
    renderPlan(currentPlan);
    updateAuditTrail(`YAML-Override zurückgesetzt für ${inspectorItemKey}.`);
  });
}

if (inspectorPropertyAdd) {
  inspectorPropertyAdd.addEventListener("click", () => {
    if (!inspectorItemKey || !inspectorYaml || !inspectorPropertyKey) {
      return;
    }

    const line = yamlLineForTypedProperty(
      inspectorPropertyKey.value,
      inspectorPropertyType?.value || "text",
      inspectorPropertyValue?.value || ""
    );
    if (!line) {
      return;
    }

    const current = inspectorYaml.value.trimEnd();
    inspectorYaml.value = `${current}\n${line}\n`;
    yamlOverrides[inspectorItemKey] = inspectorYaml.value;
    saveStoredJson(yamlOverridesKey, yamlOverrides);
    currentPlan = projectPlan(sourcePlan);
    renderPlan(currentPlan);
    updateAuditTrail(`Property ${inspectorPropertyKey.value} lokal ergänzt für ${inspectorItemKey}.`);

    inspectorPropertyKey.value = "";
    if (inspectorPropertyValue) {
      inspectorPropertyValue.value = "";
    }
  });
}

if (collectionBackDashboard) {
  collectionBackDashboard.addEventListener("click", () => {
    selectedCollectionKey = "";
    selectedCollectionItemKey = "";
    setActiveTab("overview");
  });
}

populateSelectors();
renderAuditTrail();
renderRecentVaults();
syncTemplateInputs();
syncVaultHint();
if (vaultOpenPath) {
  vaultOpenPath.value = localStorage.getItem(storageKey) ?? "";
}
