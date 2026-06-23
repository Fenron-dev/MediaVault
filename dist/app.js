const tabs = Array.from(document.querySelectorAll("[data-tab]"));
const views = Array.from(document.querySelectorAll("[data-view]"));
const vaultGate = document.getElementById("vault-gate");
const vaultOpenButton = document.getElementById("vault-open-button");
const vaultCreateName = document.getElementById("vault-create-name");
const vaultCreateSubmit = document.getElementById("vault-create-submit");
const recentVaultsList = document.getElementById("recent-vaults-list");
const title = document.getElementById("view-title");
const statusStrip = document.getElementById("status-strip");
const demoButton = document.getElementById("run-demo");
const applyImportButton = document.getElementById("apply-import");
const openVaultFolderButton = document.getElementById("open-vault-folder");
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
const collectionCards = document.getElementById("collection-cards");
const collectionRows = document.getElementById("collection-rows");
const collectionMultiToggle = document.getElementById("collection-multi-toggle");
const collectionViewBar = document.getElementById("collection-viewbar");
const collectionViewToggle = document.getElementById("collection-view-toggle");
const collectionSortName = document.getElementById("collection-sort-name");
const collectionSortType = document.getElementById("collection-sort-type");
const collectionSortTarget = document.getElementById("collection-sort-target");
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
const inspectorPlay = document.getElementById("inspector-play");
const inspectorFetchMetadata = document.getElementById("inspector-fetch-metadata");
const inspectorNotDuplicate = document.getElementById("inspector-not-duplicate");
const inspectorTrash = document.getElementById("inspector-trash");
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
const appModal = document.getElementById("app-modal");
const trashDialog = document.getElementById("trash-dialog");
const trashDialogTitle = document.getElementById("trash-dialog-title");
const trashDialogBody = document.getElementById("trash-dialog-body");
const trashDialogConfirm = document.getElementById("trash-dialog-confirm");
const trashDialogCancel = document.getElementById("trash-dialog-cancel");
const anilistDialog = document.getElementById("anilist-dialog");
const anilistDialogTitle = document.getElementById("anilist-dialog-title");
const anilistDialogQuery = document.getElementById("anilist-dialog-query");
const anilistDialogSearch = document.getElementById("anilist-dialog-search");
const anilistDialogCancel = document.getElementById("anilist-dialog-cancel");
const anilistDialogFeedback = document.getElementById("anilist-dialog-feedback");
const anilistDialogResults = document.getElementById("anilist-dialog-results");
const imageDialog = document.getElementById("image-dialog");
const imageDialogTitle = document.getElementById("image-dialog-title");
const imageDialogPreview = document.getElementById("image-dialog-preview");
const imageDialogClose = document.getElementById("image-dialog-close");

// Audiobookshelf sync
const absUrlInput = document.getElementById("abs-url-input");
const absKeyInput = document.getElementById("abs-key-input");
const absTestBtn = document.getElementById("abs-test-btn");
const absLibrariesBtn = document.getElementById("abs-libraries-btn");
const absFeedback = document.getElementById("abs-feedback");
const absLibrariesSection = document.getElementById("abs-libraries-section");
const absLibrarySelect = document.getElementById("abs-library-select");
const absImportBtn = document.getElementById("abs-import-btn");

// Playlists
const playlistList = document.getElementById("playlist-list");
const playlistNew = document.getElementById("playlist-new");
const playlistDetailEmpty = document.getElementById("playlist-detail-empty");
const playlistDetailContent = document.getElementById("playlist-detail-content");
const playlistDetailKindLabel = document.getElementById("playlist-detail-kind-label");
const playlistDetailTitle = document.getElementById("playlist-detail-title");
const playlistDetailCount = document.getElementById("playlist-detail-count");
const playlistPlayAll = document.getElementById("playlist-play-all");
const playlistDelete = document.getElementById("playlist-delete");
const playlistItemsRows = document.getElementById("playlist-items-rows");
const playlistItemsHint = document.getElementById("playlist-items-hint");
const playlistCreateDialog = document.getElementById("playlist-create-dialog");
const playlistCreateName = document.getElementById("playlist-create-name");
const playlistCreateConfirm = document.getElementById("playlist-create-confirm");
const playlistCreateCancel = document.getElementById("playlist-create-cancel");

// Player
const playerDialog = document.getElementById("player-dialog");
const playerTitle = document.getElementById("player-title");
const playerStage = document.getElementById("player-stage");
const playerVideo = document.getElementById("player-video");
const playerAudio = document.getElementById("player-audio");
const playerAudioArt = document.getElementById("player-audio-art");
const playerCoverArt = document.getElementById("player-cover-art");
const playerSubtitleDisplay = document.getElementById("player-subtitle-display");
const playerPdfStage = document.getElementById("player-pdf-stage");
const playerPdfFrame = document.getElementById("player-pdf-frame");
const playerMangaStage = document.getElementById("player-manga-stage");
const playerMangaImg = document.getElementById("player-manga-img");
const playerMangaPrev = document.getElementById("player-manga-prev");
const playerMangaNext = document.getElementById("player-manga-next");
const playerMangaCounter = document.getElementById("player-manga-counter");
const playerClose = document.getElementById("player-close");
const playerPlayPause = document.getElementById("player-play-pause");
const playerSeek = document.getElementById("player-seek");
const playerTimeCurrent = document.getElementById("player-time-current");
const playerTimeTotal = document.getElementById("player-time-total");
const playerSpeed = document.getElementById("player-speed");
const playerSleepTimer = document.getElementById("player-sleep-timer");
const playerSkipBack = document.getElementById("player-skip-back");
const playerSkipFwd = document.getElementById("player-skip-fwd");
const playerOpenSystem = document.getElementById("player-open-system");

const storageKey = "mediavault.vaultRoot";
const recentVaultsKey = "mediavault.recentVaults";
const pathTemplatesKey = "mediavault.pathTemplates";
const correctionsKey = "mediavault.reviewCorrections";
const metadataKey = "mediavault.apiMetadata";
const duplicateOverridesKey = "mediavault.duplicateOverrides";
const yamlOverridesKey = "mediavault.yamlOverrides";
const auditTrailKey = "mediavault.auditTrail";
const collectionViewKey = "mediavault.collectionView";
const recentCollectionsKey = "mediavault.recentCollections";
const trashKey = "mediavault.trash";

const labels = {
  overview: "Überblick",
  inbox: "Inbox",
  review: "Prüfung",
  collections: "Sammlungen",
  playlists: "Wiedergabelisten",
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
let duplicateOverrides = loadStoredJson(duplicateOverridesKey, {});
let yamlOverrides = loadStoredJson(yamlOverridesKey, {});
let pathTemplates = normalizeTemplateConfig(loadStoredJson(pathTemplatesKey, defaultPathTemplates()));
let auditTrail = loadStoredJson(auditTrailKey, []);
let collectionView = localStorage.getItem(collectionViewKey) || "cover";
let collectionSortKey = "name";
let collectionSortDirection = "asc";
let inspectorItemKey = "";
let inspectorSelection = null;
let recentCollections = loadStoredJson(recentCollectionsKey, []);
let trashedEntries = loadStoredJson(trashKey, {});
let isMultiEdit = false;
let multiSelectedKeys = new Set();
let pendingTrashSelection = null;
let pendingAniListSelection = null;
let pendingAniListTargets = [];
let pendingAniListFeedback = null;

function setActiveTab(tab, options = {}) {
  document.body.classList.remove("inspector-editing", "property-add-open");

  if (tab === "collections" && options.resetCollections) {
    selectedCollectionKey = "";
    selectedCollectionItemKey = "";
    isMultiEdit = false;
    multiSelectedKeys.clear();
  }

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

  // Hide the right-hand inspector on tabs that don't use it so they get
  // the full workspace width.  Restore it when switching to collections.
  const noInspector = tab === "inbox" || tab === "review";
  document.body.classList.toggle("inspector-hidden", noInspector);

  // Clear the inspector when leaving the collections tab so stale
  // property values don't bleed through to other tabs.
  if (noInspector) {
    renderInspector(null);
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

function saveDuplicateOverrides() {
  saveStoredJson(duplicateOverridesKey, duplicateOverrides);
}

async function selectFolder() {
  const response = await fetch("/api/select-folder");
  let payload = null;
  try {
    payload = await response.json();
  } catch {
    payload = null;
  }

  if (!response.ok || payload?.error) {
    throw new Error(payload?.error || `HTTP ${response.status}`);
  }

  return payload?.path || "";
}

async function loadVaultRootState() {
  const response = await fetch("/api/vault-root");
  let payload = null;
  try {
    payload = await response.json();
  } catch {
    payload = null;
  }

  if (!response.ok || payload?.error) {
    throw new Error(payload?.error || `HTTP ${response.status}`);
  }

  return payload?.root || "";
}

async function persistVaultRootState(path) {
  const response = await fetch(`/api/vault-root?root=${encodeURIComponent(String(path || "").trim())}`);
  let payload = null;
  try {
    payload = await response.json();
  } catch {
    payload = null;
  }

  if (!response.ok || payload?.error) {
    throw new Error(payload?.error || `HTTP ${response.status}`);
  }

  return payload?.root || "";
}

async function createVault(parent, name) {
  const response = await fetch(
    `/api/create-vault?parent=${encodeURIComponent(parent)}&name=${encodeURIComponent(name)}`
  );

  let payload = null;
  try {
    payload = await response.json();
  } catch {
    payload = null;
  }

  if (!response.ok || payload?.error) {
    throw new Error(payload?.error || `HTTP ${response.status}`);
  }

  return payload;
}

function defaultPathTemplates() {
  return {
    animeTv: "Anime/Serien/{series}/Staffel {season}/{episode_label}.{ext}",
    animeMovie: "Anime/Filme/{title} ({year})/{title} ({year}).{ext}",
    series: "Serien/{series}/Staffel {season}/{episode_label}.{ext}",
    film: "Filme/{title} ({year})/{title} ({year}).{ext}",
  };
}

function normalizeTemplateConfig(value) {
  const config = value && typeof value === "object" ? value : {};
  const defaults = defaultPathTemplates();
  return {
    ...defaults,
    ...config,
    animeTv:
      config.animeTv === "Anime/Serien/{series}/Staffel {season}/{series} - {episode_label}.{ext}"
        ? defaults.animeTv
        : config.animeTv || defaults.animeTv,
    series:
      config.series === "Serien/{series}/Staffel {season}/{series} - {episode_label}.{ext}"
        ? defaults.series
        : config.series || defaults.series,
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

async function openVault(path, name = "") {
  const value = String(path || "").trim();
  if (!value) {
    if (statusStrip) {
      statusStrip.textContent = "Bitte einen Vault im Finder auswählen.";
    }
    return;
  }

  localStorage.setItem(storageKey, value);
  try {
    await persistVaultRootState(value);
  } catch {
    // Best effort persistence only.
  }
  if (vaultRootInput) {
    vaultRootInput.value = value;
  }
  rememberVault(value, name);
  document.body.classList.add("is-vault-open");
  syncVaultHint();
  loadPlan();
}

async function bootstrapVault() {
  const localRoot = String(localStorage.getItem(storageKey) ?? "").trim();
  let persistedRoot = "";

  try {
    persistedRoot = String(await loadVaultRootState()).trim();
  } catch {
    persistedRoot = "";
  }

  const root = localRoot || persistedRoot;
  if (root) {
    localStorage.setItem(storageKey, root);
    if (vaultRootInput) {
      vaultRootInput.value = root;
    }
    await openVault(root);
    return;
  }

  renderRecentVaults();
  syncVaultHint();
}

function rememberCollection(node) {
  if (!node?.path) {
    return;
  }

  const entry = {
    path: node.path,
    label: node.label,
    at: new Date().toISOString(),
  };
  recentCollections = [
    entry,
    ...recentCollections.filter((candidate) => candidate.path !== node.path),
  ].slice(0, 5);
  saveStoredJson(recentCollectionsKey, recentCollections);
}

function validRecentCollections(root) {
  return recentCollections
    .map((entry) => ({ ...entry, node: findCollectionNode(root, entry.path) }))
    .filter((entry) => entry.node?.path === entry.path);
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

// Numeric-aware sort so "Part9" comes before "Part21".
function naturalCompare(a, b) {
  const re = /(\d+)|(\D+)/g;
  const at = String(a).match(re) ?? [];
  const bt = String(b).match(re) ?? [];
  for (let i = 0; i < Math.max(at.length, bt.length); i++) {
    const as = at[i] ?? "";
    const bs = bt[i] ?? "";
    const an = parseInt(as, 10);
    const bn = parseInt(bs, 10);
    if (!Number.isNaN(an) && !Number.isNaN(bn)) {
      if (an !== bn) return an - bn;
    } else {
      const c = as.localeCompare(bs, "de");
      if (c !== 0) return c;
    }
  }
  return 0;
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

function scoreLabel(value) {
  return typeof value === "number" ? `${(value / 10).toFixed(1)}` : "-";
}

function mediaFileUrlFor(item) {
  const path = item?.source_path || item?.target_path;
  if (!path) {
    return "";
  }
  const root = getVaultRoot();
  const rootQuery = root ? `&root=${encodeURIComponent(root)}` : "";
  return `/api/media-file?path=${encodeURIComponent(path)}${rootQuery}`;
}

function isLocalImageItem(item) {
  const type = mediaSelectionValue(item);
  return type === "photo" || type === "image";
}

function coverUrlFor(item) {
  return (
    item?.cover_url ||
    item?.cover_image_extra_large ||
    item?.cover_image_large ||
    item?.cover_image_medium ||
    (isLocalImageItem(item) ? mediaFileUrlFor(item) : "")
  );
}

function datePartsLabel(value) {
  if (!value || typeof value !== "object") {
    return "";
  }
  return [value.year, value.month, value.day].filter(Boolean).join("-");
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

function isAnimeContext(item) {
  const haystack = [
    item?.collection_path,
    item?.source_path,
    item?.series_title,
    item?.title,
  ]
    .filter(Boolean)
    .join(" ")
    .toLowerCase();

  return haystack.includes("anime");
}

function inferredMediaType(item) {
  const raw = item?.media_type ?? "unclassified";

  if (raw === "anime") {
    return String(item?.format ?? "").toUpperCase() === "MOVIE" ? "anime-movie" : "anime-tv";
  }

  if (raw === "series" && isAnimeContext(item)) {
    return String(item?.format ?? "").toUpperCase() === "MOVIE" ? "anime-movie" : "anime-tv";
  }

  if (raw === "film" && isAnimeContext(item) && String(item?.format ?? "").toUpperCase() === "MOVIE") {
    return "anime-movie";
  }

  return raw;
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
  return inferredMediaType(item);
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
      return "Filme";
    case "series":
      return "Serien";
    case "anime":
    case "hentai-anime":
      return "Anime";
    case "photo":
      return "Fotos";
    case "image":
      return "Bilder";
    case "music-album":
    case "music-track":
      return "Musik";
    case "book":
    case "ebook":
      return "Bücher";
    case "manga":
      return "Manga";
    case "comic":
      return "Comics";
    case "audiobook":
      return "Hörbücher";
    case "video-game":
      return "Games";
    case "rpg":
      return "TTRPG";
    case "board-game":
      return "Brettspiele";
    case "document":
      return "Dokumente";
    case "archive":
      return "Archive";
    case "software":
      return "Software";
    case "3d-model":
      return "3D-Modelle";
    case "video-misc":
      return "Videos";
    case "font":
      return "Schriften";
    default:
      return "Unklassifiziert";
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
  if (item.series_title && !episodeMarker(item.series_title) && !/^staffel\s+\d+$/i.test(cleanTitleText(item.series_title))) {
    return cleanTitleText(item.series_title);
  }

  if (item.title && !episodeMarker(item.title) && !/^staffel\s+\d+$/i.test(cleanTitleText(item.title))) {
    return cleanTitleText(item.title);
  }

  const pathParts = String(item.source_path ?? "").split("/").map(cleanTitleText).filter(Boolean);

  // Check for explicit "anime" folder
  const animeIndex = pathParts.findIndex((part) => part.toLowerCase() === "anime");
  if (animeIndex >= 0 && pathParts[animeIndex + 1]) {
    const candidate = pathParts[animeIndex + 1];
    if (!/^(?:staffel|season)\s*\d+$/i.test(candidate) && !/^s\d+$/i.test(candidate)) {
      return candidate;
    }
  }

  // Find the folder directly above "Staffel X" / "Season X" in the path.
  // e.g. "Inbox/Elfen Lied/Staffel 1/S01E02.mkv" → "Elfen Lied"
  for (let i = 1; i < pathParts.length; i++) {
    if (/^(?:staffel|season)\s*\d+$/i.test(pathParts[i]) || /^s\d{1,2}$/i.test(pathParts[i])) {
      const candidate = pathParts[i - 1];
      if (candidate && candidate.toLowerCase() !== "inbox" && !episodeMarker(candidate)) {
        return candidate;
      }
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
  const yearSuffix = item.year ? ` (${item.year})` : "";
  return title ? `${folder}/${title}${yearSuffix}` : folder;
}

function episodeFileLabel(item) {
  const marker = episodeMarker(item.source_path) || episodeMarker(item.title);
  if (!marker) {
    return sanitizeSegment(item.title || fileStem(item.source_path) || "Episode");
  }

  const season = String(item.season_number || marker.season || 1).padStart(2, "0");
  const start = String(marker.episodeStart).padStart(2, "0");
  const end = marker.episodeEnd && marker.episodeEnd !== marker.episodeStart
    ? `-E${String(marker.episodeEnd).padStart(2, "0")}`
    : "";
  const episodeTitle = deriveEpisodeTitle(item);
  const label = `S${season}E${start}${end}`;
  return episodeTitle ? `${label} - ${sanitizeSegment(episodeTitle)}` : label;
}

function needsReviewInUi(item) {
  return Boolean((item.needs_review || item.duplicate_of) && !item.has_correction);
}

function projectItem(item) {
  const correction = corrections[item.source_path] ?? {};
  const metadata = normalizeRestoredMetadata(
    apiMetadata[item.source_path] ?? parseSidecarPreviewMetadata(item.sidecar_preview)
  );
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
    "status",
    "notes",
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
  if (duplicateOverrides[item.source_path]) {
    effective.duplicate_of = null;
  }
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
  cloned.items = cloned.items
    .filter((item) => !trashedEntries[item.source_path])
    .map((item) => projectItem(item));
  if (cloned.summary) {
    cloned.summary.total_files = cloned.items.length;
    cloned.summary.items_needing_review = cloned.items.filter((item) => needsReviewInUi(item)).length;
    cloned.summary.duplicates = cloned.items.filter((item) => item.duplicate_of).length;
  }
  return cloned;
}

function effectiveAppliedStatus(item) {
  const status = String(item.status || "").trim().toLowerCase();
  if (!status || status === "inbox" || status === "needs-review") {
    return "in-library";
  }
  return status;
}

function isReadyForImport(item) {
  if (!item || !item.target_path) {
    return false;
  }
  if (!String(item.source_path || "").startsWith("Inbox/")) {
    return false;
  }
  if (trashedEntries[item.source_path]) {
    return false;
  }
  if (item.duplicate_of || needsReviewInUi(item)) {
    return false;
  }
  return !String(item.target_path).startsWith("Inbox/");
}

function buildApplyImportItem(item) {
  const prepared = {
    ...item,
    status: effectiveAppliedStatus(item),
  };
  prepared.target_path = buildTargetPath(prepared);
  prepared.sidecar_path = buildSidecarPath(prepared.target_path);
  prepared.sidecar_preview = yamlOverrides[item.source_path] || buildSidecarPreview(prepared);
  return {
    source_path: item.source_path,
    target_path: prepared.target_path,
    sidecar_preview: prepared.sidecar_preview,
  };
}

async function persistSidecarsForItems(items) {
  const fileItems = (Array.isArray(items) ? items : [])
    .filter((item) => item && typeof item.source_path === "string")
    .map((item) => ({
      media_path: item.source_path,
      sidecar_preview:
        yamlOverrides[item.source_path] ||
        buildSidecarPreview({
          ...item,
          target_path: item.source_path,
        }),
    }));

  if (!fileItems.length) {
    return;
  }

  const response = await fetch("/api/save-sidecars", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      vault_root: getVaultRoot(),
      items: fileItems,
    }),
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  const result = await response.json();
  if (result.error) {
    throw new Error(result.error);
  }

  return result;
}

function clearAppliedLocalState(sourcePaths) {
  sourcePaths.forEach((sourcePath) => {
    delete corrections[sourcePath];
    delete apiMetadata[sourcePath];
    delete duplicateOverrides[sourcePath];
    delete yamlOverrides[sourcePath];
    delete trashedEntries[sourcePath];
  });
  saveStoredJson(correctionsKey, corrections);
  saveStoredJson(metadataKey, apiMetadata);
  saveStoredJson(duplicateOverridesKey, duplicateOverrides);
  saveStoredJson(yamlOverridesKey, yamlOverrides);
  saveStoredJson(trashKey, trashedEntries);
}

async function applyReadyImports() {
  const readyItems = (currentPlan?.items ?? []).filter((item) => isReadyForImport(item));
  if (!readyItems.length) {
    const reviewCount = (currentPlan?.items ?? []).filter((item) => needsReviewInUi(item)).length;
    const hint = reviewCount
      ? ` ${reviewCount} Eintrag(e) warten noch auf Prüfung.`
      : " Alle Einträge sind bereits in der Vault oder werden als Review markiert.";
    if (statusStrip) {
      statusStrip.textContent = "Kein Eintrag bereit zum Verschieben." + hint;
      statusStrip.style.color = "var(--accent-strong)";
      setTimeout(() => {
        statusStrip.style.color = "";
      }, 4000);
    }
    return;
  }

  if (statusStrip) {
    statusStrip.textContent = `${readyItems.length} Eintrag(e) werden verschoben...`;
  }

  const response = await fetch("/api/apply-import", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      vault_root: getVaultRoot(),
      items: readyItems.map((item) => buildApplyImportItem(item)),
    }),
  });

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  const result = await response.json();
  if (result.error) {
    throw new Error(result.error);
  }

  const applied = Array.isArray(result.applied) ? result.applied : [];
  const skipped = Array.isArray(result.skipped) ? result.skipped : [];
  clearAppliedLocalState(applied);
  updateAuditTrail(`${applied.length} Eintrag(e) in die Vault-Struktur verschoben.`);

  if (statusStrip) {
    statusStrip.textContent = skipped.length
      ? `${applied.length} Eintrag(e) verschoben, ${skipped.length} übersprungen.`
      : `${applied.length} Eintrag(e) erfolgreich verschoben.`;
    if (skipped.length && skipped[0]?.reason) {
      statusStrip.textContent += ` Erstes Problem: ${skipped[0].reason}`;
    }
  }

  await loadPlan();
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

  // Multi-file audiobooks: all parts share a folder named after the audiobook.
  // Use the parent directory name from source_path as the folder title and keep
  // each file's original filename so parts don't collide.
  if (mediaType === "audiobook" && (item.audiobook_parts?.length > 0 || item.is_audiobook_part)) {
    const srcParts = (item.source_path || "").split("/");
    const parentName = srcParts.length >= 2 ? srcParts[srcParts.length - 2] : null;
    const INBOX_SEGMENTS = new Set(["inbox", "hörbücher", "horbücher"]);
    const audioTitle = sanitizeSegment(
      parentName && !INBOX_SEGMENTS.has(parentName.toLowerCase())
        ? parentName
        : item.title || fileStem(item.source_path) || "untitled"
    );
    const folderSegment = item.folder_segment ?? folderSegmentFor(mediaType);
    const origFilename = basename(item.source_path);
    return `${folderSegment}/${audioTitle}${yearSuffix}/${origFilename}`;
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
  return `${prefix}${stem}.mediavault.yaml`;
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

function parseYamlScalarValue(value) {
  const trimmed = String(value || "").trim();
  if (!trimmed) {
    return "";
  }
  if ((trimmed.startsWith("{") && trimmed.endsWith("}")) || (trimmed.startsWith("[") && trimmed.endsWith("]"))) {
    try {
      return JSON.parse(trimmed);
    } catch (_error) {
      return trimmed;
    }
  }
  if (trimmed.startsWith("\"") && trimmed.endsWith("\"")) {
    try {
      return JSON.parse(trimmed);
    } catch (_error) {
      return trimmed.slice(1, -1);
    }
  }
  if (/^-?\d+(\.\d+)?$/.test(trimmed)) {
    return Number(trimmed);
  }
  if (trimmed === "true") {
    return true;
  }
  if (trimmed === "false") {
    return false;
  }
  return trimmed;
}

function parseSidecarPreviewMetadata(raw) {
  if (!raw || typeof raw !== "string") {
    return {};
  }

  const metadata = {};
  const lines = raw.split(/\r?\n/);
  for (let index = 0; index < lines.length; index += 1) {
    const line = lines[index];
    const trimmed = line.trim();
    if (!trimmed || trimmed === "---") {
      continue;
    }

    const separator = trimmed.indexOf(":");
    if (separator <= 0) {
      continue;
    }

    const key = trimmed.slice(0, separator).trim();
    const value = trimmed.slice(separator + 1).trim();
    if (!key) {
      continue;
    }

    if (!value) {
      const entries = [];
      while (index + 1 < lines.length && lines[index + 1].trimStart().startsWith("- ")) {
        index += 1;
        entries.push(parseYamlScalarValue(lines[index].trimStart().slice(2)));
      }
      metadata[key] = entries;
      continue;
    }

    metadata[key] = parseYamlScalarValue(value);
  }

  return metadata;
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
    tags: normalizeAniListTags(metadata.tags),
    studios: normalizeAniListStudios(metadata.studios),
    relations: normalizeAniListRelations(metadata.relations),
    characters: normalizeAniListCharacters(metadata.characters),
    staff: normalizeAniListStaff(metadata.staff),
    reviews: normalizeAniListReviews(metadata.reviews),
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

function normalizeAniListTags(tags) {
  if (!Array.isArray(tags)) {
    return [];
  }

  return tags
    .map((tag) => {
      if (typeof tag === "string") {
        return tag;
      }
      if (!tag || typeof tag !== "object") {
        return "";
      }
      return tag.name || "";
    })
    .filter(Boolean);
}

function normalizeAniListStudios(studios) {
  if (!Array.isArray(studios)) {
    return [];
  }

  return studios
    .map((studio) => {
      if (!studio || typeof studio !== "object") {
        return null;
      }
      return {
        id: studio.id ?? null,
        name: studio.name || "",
        is_animation_studio: Boolean(studio.is_animation_studio),
        site_url: studio.site_url || null,
      };
    })
    .filter((studio) => studio?.name);
}

function normalizeAniListRelations(relations) {
  if (!Array.isArray(relations)) {
    return [];
  }

  return relations
    .map((relation) => {
      if (!relation || typeof relation !== "object") {
        return null;
      }
      return {
        id: relation.id ?? null,
        relation_type: relation.relation_type || null,
        title: relation.title || null,
        media_type: relation.media_type || null,
        format: relation.format || null,
        site_url: relation.site_url || null,
        cover_image_medium: relation.cover_image_medium || null,
        cover_image_large: relation.cover_image_large || null,
        cover_image_extra_large: relation.cover_image_extra_large || null,
      };
    })
    .filter((relation) => relation?.title);
}

function normalizeAniListCharacters(characters) {
  if (!Array.isArray(characters)) {
    return [];
  }

  return characters
    .map((character) => {
      if (!character || typeof character !== "object") {
        return null;
      }
      const firstVoiceActor = Array.isArray(character.voice_actors)
        ? character.voice_actors[0]
        : character.voice_actor && typeof character.voice_actor === "object"
          ? character.voice_actor
          : null;
      return {
        role: character.role || null,
        character_id: character.character_id ?? null,
        character_name: character.character_name || character.name || null,
        character_image: character.character_image || character.character?.image || null,
        voice_actor_name: character.voice_actor_name || firstVoiceActor?.name || null,
        voice_actor_native_name:
          character.voice_actor_native_name || firstVoiceActor?.native_name || null,
        voice_actor_language: character.voice_actor_language || firstVoiceActor?.language || null,
        voice_actor_image: character.voice_actor_image || firstVoiceActor?.image || null,
      };
    })
    .filter((character) => character?.character_name);
}

function normalizeAniListStaff(staff) {
  if (!Array.isArray(staff)) {
    return [];
  }

  return staff
    .map((entry) => {
      if (!entry || typeof entry !== "object") {
        return null;
      }
      const person = entry.person && typeof entry.person === "object" ? entry.person : entry;
      return {
        role: entry.role || null,
        id: person.id ?? null,
        name: entry.name || person.name || null,
        native_name: entry.native_name || person.native_name || null,
        language: entry.language || person.language || null,
        image: entry.image || person.image || null,
      };
    })
    .filter((entry) => entry?.name);
}

function normalizeAniListReviews(reviews) {
  if (!Array.isArray(reviews)) {
    return [];
  }

  return reviews
    .map((review) => {
      if (!review || typeof review !== "object") {
        return null;
      }
      return {
        id: review.id ?? null,
        summary: review.summary || null,
        rating: review.rating ?? null,
        rating_amount: review.rating_amount ?? null,
        site_url: review.site_url || null,
        user_name: review.user_name || null,
      };
    })
    .filter((review) => review?.summary || review?.user_name);
}

function normalizeRestoredMetadata(metadata) {
  if (!metadata || typeof metadata !== "object") {
    return {};
  }

  return {
    ...metadata,
    tags: normalizeAniListTags(metadata.tags),
    studios: normalizeAniListStudios(metadata.studios),
    relations: normalizeAniListRelations(metadata.relations),
    characters: normalizeAniListCharacters(metadata.characters),
    staff: normalizeAniListStaff(metadata.staff),
    reviews: normalizeAniListReviews(metadata.reviews),
  };
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

function aniListDisplayTitle(metadata) {
  return (
    metadata.title_english ||
    metadata.title_romaji ||
    metadata.title_native ||
    metadata.series_title ||
    metadata.title ||
    "Unbekannt"
  );
}

function closeActionModal() {
  pendingTrashSelection = null;
  pendingAniListSelection = null;
  pendingAniListTargets = [];
  pendingAniListFeedback = null;

  hideModalCards();
  if (appModal) {
    appModal.classList.remove("is-active");
    appModal.hidden = true;
  }
}

function hideModalCards() {
  [trashDialog, anilistDialog, imageDialog].forEach((dialog) => {
    if (dialog) {
      dialog.hidden = true;
    }
  });
}

function showModalCard(dialog) {
  if (!dialog) {
    return;
  }

  hideModalCards();
  dialog.hidden = false;
  if (appModal) {
    appModal.hidden = false;
    appModal.classList.add("is-active");
  }
}

function openImagePreview(item) {
  const imageUrl = coverUrlFor(item);
  if (!imageDialog || !imageDialogPreview || !imageUrl) {
    return;
  }

  hideModalCards();
  if (imageDialogTitle) {
    imageDialogTitle.textContent = item?.title || fileStem(item?.source_path || "") || "Bildansicht";
  }
  imageDialogPreview.src = imageUrl;
  imageDialogPreview.alt = item?.title || "Bildvorschau";
  showModalCard(imageDialog);
}

// ---------------------------------------------------------------------------
// Dashboard
// ---------------------------------------------------------------------------

const dashboardResumeSection = document.getElementById("dashboard-resume-section");
const dashboardResumeCards = document.getElementById("dashboard-resume-cards");
const dashboardRecentSection = document.getElementById("dashboard-recent-section");
const dashboardRecentCards = document.getElementById("dashboard-recent-cards");
const dashboardEmptyHint = document.getElementById("dashboard-empty-hint");

function mediaTypeEmoji(type) {
  switch (type) {
    case "film": return "🎬";
    case "series": return "📺";
    case "anime": case "hentai-anime": return "🌸";
    case "audiobook": return "🎧";
    case "music-album": case "music-track": return "🎵";
    case "book": case "ebook": return "📖";
    case "manga": return "📚";
    case "comic": return "💬";
    case "video-misc": return "🎞";
    default: return "📁";
  }
}

function createDashboardCard(item, onPlay) {
  const card = document.createElement("div");
  card.className = "dashboard-card";
  card.title = item.title;

  const coverWrap = document.createElement("div");
  coverWrap.className = "dashboard-card-cover";
  if (item.cover_url) {
    const img = document.createElement("img");
    img.src = item.cover_url;
    img.alt = item.title;
    img.onerror = () => { img.style.display = "none"; };
    coverWrap.appendChild(img);
  } else {
    coverWrap.textContent = mediaTypeEmoji(item.media_type);
  }

  const titleEl = document.createElement("div");
  titleEl.className = "dashboard-card-title";
  titleEl.textContent = item.title;

  const meta = document.createElement("div");
  meta.className = "dashboard-card-meta";
  meta.textContent = item.year ? String(item.year) : mediaTypeLabel(item.media_type);

  card.appendChild(coverWrap);

  if (typeof item.progress_fraction === "number") {
    const barWrap = document.createElement("div");
    barWrap.className = "dashboard-progress-bar";
    const fill = document.createElement("div");
    fill.className = "dashboard-progress-fill";
    fill.style.width = `${Math.round(item.progress_fraction * 100)}%`;
    barWrap.appendChild(fill);
    card.appendChild(barWrap);
  }

  card.appendChild(titleEl);
  card.appendChild(meta);

  card.addEventListener("click", () => {
    if (onPlay) onPlay(item);
  });

  return card;
}

async function loadDashboard() {
  const root = getVaultRoot();
  if (!root) return;

  const rootQuery = `root=${encodeURIComponent(root)}`;

  // Load in-progress items
  try {
    const res = await fetch(`mediavault://localhost/api/in-progress?${rootQuery}`);
    const data = await res.json();
    if (dashboardResumeCards) clearNode(dashboardResumeCards);
    if (data.items && data.items.length > 0) {
      if (dashboardResumeSection) dashboardResumeSection.hidden = false;
      if (dashboardEmptyHint) dashboardEmptyHint.hidden = true;
      data.items.forEach((item) => {
        const card = createDashboardCard(item, (it) => {
          openPlayer({ source_path: it.vault_path, target_path: it.vault_path, title: it.title });
        });
        dashboardResumeCards?.appendChild(card);
      });
    } else {
      if (dashboardResumeSection) dashboardResumeSection.hidden = true;
    }
  } catch {
    if (dashboardResumeSection) dashboardResumeSection.hidden = true;
  }

  // Load recent items
  try {
    const res = await fetch(`mediavault://localhost/api/recent-items?${rootQuery}`);
    const data = await res.json();
    if (dashboardRecentCards) clearNode(dashboardRecentCards);
    if (data.items && data.items.length > 0) {
      if (dashboardRecentSection) dashboardRecentSection.hidden = false;
      if (dashboardEmptyHint) dashboardEmptyHint.hidden = true;
      data.items.forEach((item) => {
        const card = createDashboardCard(item, (it) => {
          const ext = playerFileExt(it.vault_path);
          if (isVideoFile(it.vault_path) || isAudioFile(it.vault_path) || PLAYER_UNSUPPORTED_EXTS.has(ext) || PLAYER_PDF_EXTS.has(ext) || PLAYER_IMAGE_EXTS.has(ext) || PLAYER_EPUB_EXTS.has(ext)) {
            openPlayer({ source_path: it.vault_path, target_path: it.vault_path, title: it.title });
          }
        });
        dashboardRecentCards?.appendChild(card);
      });
    } else {
      if (dashboardRecentSection) dashboardRecentSection.hidden = true;
    }
  } catch {
    if (dashboardRecentSection) dashboardRecentSection.hidden = true;
  }
}

// ---------------------------------------------------------------------------
// Media player
// ---------------------------------------------------------------------------

const PLAYER_SAVE_INTERVAL_MS = 5000;
const PLAYER_VIDEO_EXTS = new Set(["mp4", "m4v", "mov", "webm", "ogv"]);
const PLAYER_AUDIO_EXTS = new Set(["mp3", "m4a", "m4b", "aac", "ogg", "oga", "opus", "flac", "wav", "weba"]);
// MKV/AVI cannot be played by macOS WKWebView natively.
const PLAYER_UNSUPPORTED_EXTS = new Set(["mkv", "avi", "ts", "wmv", "rmvb"]);
const PLAYER_PDF_EXTS = new Set(["pdf"]);
const PLAYER_IMAGE_EXTS = new Set(["jpg", "jpeg", "png", "webp", "gif", "avif", "bmp"]);
// EPUB opens via system viewer; listed here so the inspector "Abspielen" button shows.
const PLAYER_EPUB_EXTS = new Set(["epub"]);

// mangaState holds the image list and current index when a manga/image sequence is open.
let mangaState = null; // { items: string[], index: number }

let playerState = null; // { mediaEl, item, vaultPath, sleepTimerId, saveTimerId, subtitleTrack }

function playerMediaElement() {
  return playerState?.mediaEl ?? null;
}

function playerFileExt(path) {
  return (path || "").split(".").pop().toLowerCase();
}

function isVideoFile(path) {
  return PLAYER_VIDEO_EXTS.has(playerFileExt(path));
}

function isAudioFile(path) {
  return PLAYER_AUDIO_EXTS.has(playerFileExt(path));
}

function formatTime(seconds) {
  if (!isFinite(seconds) || seconds < 0) return "—:——";
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  const mm = String(m).padStart(2, "0");
  const ss = String(s).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${m}:${ss}`;
}

function playerUpdateTime(el) {
  if (!playerTimeCurrent || !playerTimeTotal || !playerSeek) return;
  const cur = el.currentTime || 0;
  const dur = el.duration || 0;
  playerTimeCurrent.textContent = formatTime(cur);
  playerTimeTotal.textContent = formatTime(dur);
  if (dur > 0) {
    playerSeek.value = String(Math.round((cur / dur) * 1000));
  }
}

function playerSaveProgress() {
  if (!playerState) return;
  const el = playerState.mediaEl;
  if (!el || !isFinite(el.duration) || el.duration <= 0) return;

  const vaultPath = playerState.vaultPath;
  const root = getVaultRoot();
  const completed = el.currentTime / el.duration >= 0.90;

  const progressType = isVideoFile(vaultPath) ? "video" : "audio";
  const body = JSON.stringify({
    vault_root: root || null,
    vault_path: vaultPath,
    progress: {
      type: progressType,
      position_seconds: el.currentTime,
      duration_seconds: el.duration,
    },
    completed,
  });

  fetch("mediavault://localhost/api/progress/save", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body,
  }).catch(() => {});
}

async function playerLoadProgress(vaultPath) {
  const root = getVaultRoot();
  const rootQuery = root ? `&root=${encodeURIComponent(root)}` : "";
  try {
    const res = await fetch(`mediavault://localhost/api/progress/load?path=${encodeURIComponent(vaultPath)}${rootQuery}`);
    const data = await res.json();
    return data?.record ?? null;
  } catch {
    return null;
  }
}

function playerStop() {
  if (!playerState) return;
  const { mediaEl, sleepTimerId, saveTimerId } = playerState;

  playerSaveProgress();

  if (sleepTimerId) clearTimeout(sleepTimerId);
  if (saveTimerId) clearInterval(saveTimerId);

  if (mediaEl) {
    mediaEl.pause();
    mediaEl.src = "";
    mediaEl.load();
  }

  if (playerVideo) { playerVideo.src = ""; }
  if (playerAudio) { playerAudio.src = ""; }
  if (playerSubtitleDisplay) playerSubtitleDisplay.textContent = "";
  if (playerPdfFrame) playerPdfFrame.src = "";
  if (playerPdfStage) playerPdfStage.hidden = true;
  if (playerMangaStage) playerMangaStage.hidden = true;
  mangaState = null;

  playerState = null;
}

// ── SRT subtitle parsing ────────────────────────────────────────────────────

function parseSrt(text) {
  const cues = [];
  const blocks = text.replace(/\r\n/g, "\n").trim().split(/\n\n+/);
  for (const block of blocks) {
    const lines = block.split("\n");
    if (lines.length < 3) continue;
    const timeLine = lines.find((l) => l.includes("-->"));
    if (!timeLine) continue;
    const [startStr, endStr] = timeLine.split("-->").map((s) => s.trim());
    const toSec = (ts) => {
      const [hms, ms] = ts.replace(",", ".").split(".");
      const [h, m, s] = hms.split(":").map(Number);
      return h * 3600 + m * 60 + s + Number(`0.${ms || 0}`);
    };
    const start = toSec(startStr);
    const end = toSec(endStr);
    const textIdx = lines.indexOf(timeLine) + 1;
    const cueText = lines.slice(textIdx).join("\n").replace(/<[^>]+>/g, "");
    cues.push({ start, end, text: cueText });
  }
  return cues;
}

let subtitleCues = [];
let subtitleRafId = null;

function startSubtitleLoop(mediaEl) {
  if (subtitleRafId) cancelAnimationFrame(subtitleRafId);
  function tick() {
    if (!playerState || !subtitleCues.length) {
      if (playerSubtitleDisplay) playerSubtitleDisplay.textContent = "";
      return;
    }
    const t = mediaEl.currentTime;
    const cue = subtitleCues.find((c) => t >= c.start && t <= c.end);
    if (playerSubtitleDisplay) playerSubtitleDisplay.textContent = cue?.text ?? "";
    subtitleRafId = requestAnimationFrame(tick);
  }
  subtitleRafId = requestAnimationFrame(tick);
}

async function loadSubtitles(vaultPath) {
  subtitleCues = [];
  const srtPath = vaultPath.replace(/\.[^.]+$/, ".srt");
  const root = getVaultRoot();
  const rootQuery = root ? `&root=${encodeURIComponent(root)}` : "";
  try {
    const res = await fetch(
      `mediavault://localhost/api/media-file?path=${encodeURIComponent(srtPath)}${rootQuery}`
    );
    if (!res.ok) return;
    const text = await res.text();
    subtitleCues = parseSrt(text);
  } catch {
    subtitleCues = [];
  }
}

// ── Manga/image sequence ────────────────────────────────────────────────────

function mangaSiblings(vaultPath) {
  if (!currentPlan) return [vaultPath];
  const dir = vaultPath.includes("/") ? vaultPath.substring(0, vaultPath.lastIndexOf("/")) : "";
  return currentPlan.items
    .filter((it) => {
      const p = it.source_path || "";
      const ext = p.split(".").pop().toLowerCase();
      if (!PLAYER_IMAGE_EXTS.has(ext)) return false;
      const d = p.includes("/") ? p.substring(0, p.lastIndexOf("/")) : "";
      return d === dir;
    })
    .map((it) => it.source_path)
    .sort();
}

function mangaShowIndex(index) {
  if (!mangaState || !playerMangaImg || !playerMangaCounter) return;
  const items = mangaState.items;
  const clamped = Math.max(0, Math.min(items.length - 1, index));
  mangaState.index = clamped;
  const vaultPath = items[clamped];
  const root = getVaultRoot();
  const rootQuery = root ? `&root=${encodeURIComponent(root)}` : "";
  playerMangaImg.src = `mediavault://localhost/api/media-file?path=${encodeURIComponent(vaultPath)}${rootQuery}`;
  playerMangaCounter.textContent = `${clamped + 1} / ${items.length}`;
  if (playerMangaPrev) playerMangaPrev.disabled = clamped === 0;
  if (playerMangaNext) playerMangaNext.disabled = clamped === items.length - 1;
}

function playerSetPlayPause(el) {
  if (!playerPlayPause) return;
  playerPlayPause.textContent = el.paused ? "▶" : "⏸";
}

async function openPlayer(item) {
  if (!playerDialog || !item) return;

  playerStop();

  const sourcePath = item.source_path || item.target_path || "";
  if (!sourcePath) return;

  const ext = playerFileExt(sourcePath);
  const isVideo = isVideoFile(sourcePath);
  const isAudio = isAudioFile(sourcePath);
  const isPdf = PLAYER_PDF_EXTS.has(ext);
  const isImage = PLAYER_IMAGE_EXTS.has(ext);
  const isEpub = PLAYER_EPUB_EXTS.has(ext);
  const unsupported = PLAYER_UNSUPPORTED_EXTS.has(ext) || isEpub;

  if (!isVideo && !isAudio && !isPdf && !isImage && !unsupported) return;

  // Unsupported formats (MKV, AVI, TS, ePub…): open directly in system player
  // without showing the internal player dialog.
  if (unsupported) {
    const root = getVaultRoot();
    const rootQuery = root ? `&root=${encodeURIComponent(root)}` : "";
    fetch(
      `mediavault://localhost/api/open-external?path=${encodeURIComponent(sourcePath)}${rootQuery}`
    ).catch(() => {});
    return;
  }

  const fileUrl = mediaFileUrlFor(item);
  const title = item.title || fileStem(sourcePath) || "Wiedergabe";

  if (playerTitle) playerTitle.textContent = title;

  // Show the player — it lives outside #app-modal so it is always visible
  // regardless of whether any other modal is open. Close any other open
  // modal card so the two overlays don't stack.
  if (appModal && !appModal.hidden) {
    appModal.hidden = true;
    appModal.classList.remove("is-active");
  }
  playerDialog.hidden = false;

  // ── PDF ──────────────────────────────────────────────────────────────────
  if (isPdf) {
    if (playerStage) playerStage.hidden = true;
    if (playerAudioArt) playerAudioArt.hidden = true;
    if (playerPdfStage) playerPdfStage.hidden = false;
    if (playerMangaStage) playerMangaStage.hidden = true;
    if (playerPdfFrame) playerPdfFrame.src = fileUrl;
    // PDF has no playback controls; disable them
    if (playerPlayPause) playerPlayPause.disabled = true;
    // Store minimal state so closePlayer works
    playerState = { mediaEl: null, item, vaultPath: sourcePath, sleepTimerId: null, saveTimerId: null };
    return;
  }

  // ── Image / manga sequence ───────────────────────────────────────────────
  if (isImage) {
    if (playerStage) playerStage.hidden = true;
    if (playerAudioArt) playerAudioArt.hidden = true;
    if (playerPdfStage) playerPdfStage.hidden = true;
    if (playerMangaStage) playerMangaStage.hidden = false;
    if (playerPlayPause) playerPlayPause.disabled = true;
    const siblings = mangaSiblings(sourcePath);
    const startIdx = Math.max(0, siblings.indexOf(sourcePath));
    mangaState = { items: siblings, index: startIdx };
    mangaShowIndex(startIdx);
    playerState = { mediaEl: null, item, vaultPath: sourcePath, sleepTimerId: null, saveTimerId: null };
    return;
  }

  // ── Audio / Video ────────────────────────────────────────────────────────
  const mediaEl = isVideo ? playerVideo : playerAudio;
  if (!mediaEl) return;

  if (playerStage) playerStage.hidden = !isVideo;
  if (playerAudioArt) playerAudioArt.hidden = isVideo;
  if (playerPdfStage) playerPdfStage.hidden = true;
  if (playerMangaStage) playerMangaStage.hidden = true;
  if (playerCoverArt && !isVideo) {
    const cover = coverUrlFor(item);
    if (cover) {
      playerCoverArt.src = cover;
      playerCoverArt.hidden = false;
      playerCoverArt.onerror = () => {
        playerCoverArt.hidden = true;
        playerCoverArt.onerror = null;
      };
    } else {
      // Fall back to cover image files in the same directory
      const root = getVaultRoot();
      const rootQuery = root ? `&root=${encodeURIComponent(root)}` : "";
      const dir = sourcePath.includes("/") ? sourcePath.slice(0, sourcePath.lastIndexOf("/")) : "";
      const candidates = dir
        ? ["cover.jpg", "folder.jpg", "cover.png", "poster.jpg"].map(
            (n) =>
              `mediavault://localhost/api/media-file?path=${encodeURIComponent(
                dir + "/" + n
              )}${rootQuery}`
          )
        : [];
      if (candidates.length > 0) {
        let ci = 0;
        const tryNextCover = () => {
          if (ci < candidates.length) {
            playerCoverArt.src = candidates[ci++];
          } else {
            playerCoverArt.src = "";
            playerCoverArt.hidden = true;
            playerCoverArt.onerror = null;
          }
        };
        playerCoverArt.hidden = false;
        playerCoverArt.onerror = tryNextCover;
        tryNextCover();
      } else {
        playerCoverArt.src = "";
        playerCoverArt.hidden = true;
      }
    }
  }

  if (playerPlayPause) playerPlayPause.disabled = false;
  mediaEl.src = fileUrl;
  mediaEl.playbackRate = parseFloat(playerSpeed?.value ?? "1");

  const saveTimerId = setInterval(playerSaveProgress, PLAYER_SAVE_INTERVAL_MS);
  playerState = { mediaEl, item, vaultPath: sourcePath, sleepTimerId: null, saveTimerId };

  // Restore resume position
  const record = await playerLoadProgress(sourcePath);
  if (record?.progress?.position_seconds && isFinite(record.progress.position_seconds)) {
    mediaEl.currentTime = record.progress.position_seconds;
  }

  // Load .srt subtitle companion for video files
  if (isVideo) {
    loadSubtitles(sourcePath).then(() => {
      if (playerState && subtitleCues.length > 0) startSubtitleLoop(mediaEl);
    });
  }

  mediaEl.play().catch(() => {});
  playerSetPlayPause(mediaEl);
}

function closePlayer() {
  playerStop();
  if (playerDialog) playerDialog.hidden = true;
}

// ─── Audiobookshelf (ABS) sync ────────────────────────────────────────────

const absSettingsKey = "mediavault.absSettings";

function absGetSettings() {
  return loadStoredJson(absSettingsKey, { url: "", key: "" });
}

function absSaveSettings(url, key) {
  saveStoredJson(absSettingsKey, { url, key });
}

function absUrl() {
  return absUrlInput?.value.trim() || absGetSettings().url;
}

function absKey() {
  return absKeyInput?.value.trim() || absGetSettings().key;
}

async function absTest() {
  const url = absUrl();
  const key = absKey();
  if (!url) {
    if (absFeedback) absFeedback.textContent = "Bitte Server-URL eingeben.";
    return false;
  }
  absSaveSettings(url, key);
  try {
    const res = await fetch(
      `mediavault://localhost/api/abs/test?url=${encodeURIComponent(url)}&key=${encodeURIComponent(key)}`
    );
    const data = await res.json();
    if (data.ok) {
      if (absFeedback) absFeedback.textContent = "Verbindung erfolgreich.";
      return true;
    }
    if (absFeedback) absFeedback.textContent = `Fehler: ${data.error || "Unbekannt"}`;
    return false;
  } catch (e) {
    if (absFeedback) absFeedback.textContent = `Verbindungsfehler: ${e.message}`;
    return false;
  }
}

async function absLoadLibraries() {
  const url = absUrl();
  const key = absKey();
  if (!url) {
    if (absFeedback) absFeedback.textContent = "Bitte Server-URL eingeben.";
    return;
  }
  absSaveSettings(url, key);
  if (absFeedback) absFeedback.textContent = "Lade Bibliotheken…";
  try {
    const res = await fetch(
      `mediavault://localhost/api/abs/libraries?url=${encodeURIComponent(url)}&key=${encodeURIComponent(key)}`
    );
    const data = await res.json();
    if (data.error) {
      if (absFeedback) absFeedback.textContent = `Fehler: ${data.error}`;
      return;
    }
    if (!absLibrarySelect || !absLibrariesSection) return;
    absLibrarySelect.innerHTML = "";
    (data.libraries || []).forEach((lib) => {
      const opt = document.createElement("option");
      opt.value = lib.id;
      opt.textContent = `${lib.name} (${lib.media_type})`;
      absLibrarySelect.appendChild(opt);
    });
    absLibrariesSection.hidden = data.libraries?.length === 0;
    if (absFeedback) absFeedback.textContent = `${data.libraries?.length || 0} Bibliothek(en) gefunden.`;
  } catch (e) {
    if (absFeedback) absFeedback.textContent = `Fehler: ${e.message}`;
  }
}

async function absImportLibrary() {
  const url = absUrl();
  const key = absKey();
  const libraryId = absLibrarySelect?.value;
  if (!url || !libraryId) return;
  if (absFeedback) absFeedback.textContent = "Lade Bibliotheksinhalte…";
  try {
    const res = await fetch(
      `mediavault://localhost/api/abs/library-items?url=${encodeURIComponent(url)}&key=${encodeURIComponent(key)}&library=${encodeURIComponent(libraryId)}`
    );
    const data = await res.json();
    if (data.error) {
      if (absFeedback) absFeedback.textContent = `Fehler: ${data.error}`;
      return;
    }
    const count = data.items?.length || 0;
    if (absFeedback) {
      absFeedback.textContent = `${count} Einträge gefunden. Import-Funktion wird in einem späteren Release implementiert.`;
    }
  } catch (e) {
    if (absFeedback) absFeedback.textContent = `Fehler: ${e.message}`;
  }
}

function initAbsSettings() {
  const stored = absGetSettings();
  if (absUrlInput && stored.url) absUrlInput.value = stored.url;
  if (absKeyInput && stored.key) absKeyInput.value = stored.key;
  if (absTestBtn) absTestBtn.addEventListener("click", absTest);
  if (absLibrariesBtn) absLibrariesBtn.addEventListener("click", absLoadLibraries);
  if (absImportBtn) absImportBtn.addEventListener("click", absImportLibrary);
}

// ─── Playlist management ──────────────────────────────────────────────────

let activePlaylists = [];
let activePlaylistId = null;

function playlistKindLabel(kind) {
  if (kind === "smart") return "Smart Playlist";
  if (kind === "series") return "Serie";
  return "Manuell";
}

function playlistKindIcon(kind) {
  if (kind === "smart") return "✦";
  if (kind === "series") return "▶▶";
  return "☰";
}

async function loadPlaylists() {
  const root = getVaultRoot();
  if (!root) return;
  try {
    const res = await fetch(
      `mediavault://localhost/api/playlist/list?root=${encodeURIComponent(root)}`
    );
    const data = await res.json();
    activePlaylists = data.playlists || [];
    renderPlaylistList();
  } catch {
    activePlaylists = [];
  }
}

function renderPlaylistList() {
  if (!playlistList) return;
  playlistList.innerHTML = "";
  if (activePlaylists.length === 0) {
    const hint = document.createElement("p");
    hint.className = "body-copy";
    hint.style.padding = "8px 4px";
    hint.style.color = "var(--text-muted)";
    hint.textContent = "Noch keine Playlists.";
    playlistList.appendChild(hint);
    return;
  }
  for (const pl of activePlaylists) {
    const row = document.createElement("div");
    row.className = "playlist-row" + (pl.id === activePlaylistId ? " is-active" : "");
    row.dataset.id = pl.id;

    const icon = document.createElement("span");
    icon.className = "playlist-row-icon";
    icon.textContent = playlistKindIcon(pl.kind);

    const name = document.createElement("span");
    name.className = "playlist-row-name";
    name.textContent = pl.name;

    const count = document.createElement("span");
    count.className = "playlist-row-count";
    count.textContent = pl.items?.length ?? 0;

    row.append(icon, name, count);
    row.addEventListener("click", () => selectPlaylist(pl.id));
    playlistList.appendChild(row);
  }
}

function selectPlaylist(id) {
  activePlaylistId = id;
  renderPlaylistList();
  const pl = activePlaylists.find((p) => p.id === id);
  if (!pl) return;
  renderPlaylistDetail(pl);
}

function renderPlaylistDetail(pl) {
  if (!playlistDetailEmpty || !playlistDetailContent) return;
  playlistDetailEmpty.hidden = true;
  playlistDetailContent.hidden = false;

  if (playlistDetailKindLabel) playlistDetailKindLabel.textContent = playlistKindLabel(pl.kind);
  if (playlistDetailTitle) playlistDetailTitle.textContent = pl.name;

  const count = pl.items?.length ?? 0;
  if (playlistDetailCount) playlistDetailCount.textContent = `${count} Eintr${count === 1 ? "ag" : "äge"}`;

  if (playlistPlayAll) playlistPlayAll.hidden = count === 0 || pl.kind === "smart";
  if (playlistItemsHint) playlistItemsHint.hidden = count > 0;

  if (!playlistItemsRows) return;
  playlistItemsRows.innerHTML = "";
  if (count === 0) return;

  for (let i = 0; i < pl.items.length; i++) {
    const vaultPath = pl.items[i];
    const row = document.createElement("div");
    row.className = "playlist-item-row";

    const nameSpan = document.createElement("span");
    nameSpan.style.fontSize = "0.875rem";
    nameSpan.style.overflow = "hidden";
    nameSpan.style.textOverflow = "ellipsis";
    nameSpan.style.whiteSpace = "nowrap";
    const stem = vaultPath.split("/").pop()?.replace(/\.[^.]+$/, "") || vaultPath;
    nameSpan.textContent = stem;
    nameSpan.title = vaultPath;

    const typeSpan = document.createElement("span");
    typeSpan.style.fontSize = "0.75rem";
    typeSpan.style.color = "var(--text-muted)";
    const ext = vaultPath.split(".").pop()?.toLowerCase() || "";
    typeSpan.textContent = ext.toUpperCase();

    const playBtn = document.createElement("button");
    playBtn.className = "action-button icon-button playlist-item-play";
    playBtn.title = "Abspielen";
    playBtn.textContent = "▶";
    playBtn.addEventListener("click", () => {
      openPlayer({ source_path: vaultPath, target_path: vaultPath });
    });

    row.append(nameSpan, typeSpan, playBtn);
    playlistItemsRows.appendChild(row);
  }
}

async function saveNewPlaylist(name) {
  const root = getVaultRoot();
  if (!root) return;
  const id = `pl_${Date.now()}`;
  const playlist = {
    id,
    name: name.trim(),
    kind: "manual",
    items: [],
    created_at: Math.floor(Date.now() / 1000),
    updated_at: Math.floor(Date.now() / 1000),
  };
  try {
    await fetch("mediavault://localhost/api/playlist/save", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ vault_root: root, playlist }),
    });
    await loadPlaylists();
    selectPlaylist(id);
  } catch (e) {
    if (statusStrip) statusStrip.textContent = `Fehler beim Erstellen: ${e.message}`;
  }
}

function createNewPlaylist() {
  if (playlistCreateDialog) {
    if (playlistCreateName) playlistCreateName.value = "";
    showModalCard(playlistCreateDialog);
    setTimeout(() => playlistCreateName?.focus(), 50);
  }
}

async function deleteActivePlaylist() {
  if (!activePlaylistId) return;
  const pl = activePlaylists.find((p) => p.id === activePlaylistId);
  if (!pl) return;
  if (!confirm(`Playlist „${pl.name}" wirklich löschen?`)) return;
  const root = getVaultRoot();
  if (!root) return;
  try {
    await fetch("mediavault://localhost/api/playlist/delete", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ vault_root: root, id: activePlaylistId }),
    });
    activePlaylistId = null;
    if (playlistDetailEmpty) playlistDetailEmpty.hidden = false;
    if (playlistDetailContent) playlistDetailContent.hidden = true;
    await loadPlaylists();
  } catch (e) {
    if (statusStrip) statusStrip.textContent = `Fehler beim Löschen: ${e.message}`;
  }
}

function initPlaylists() {
  if (playlistNew) playlistNew.addEventListener("click", createNewPlaylist);
  if (playlistDelete) playlistDelete.addEventListener("click", deleteActivePlaylist);
  if (playlistPlayAll) {
    playlistPlayAll.addEventListener("click", () => {
      const pl = activePlaylists.find((p) => p.id === activePlaylistId);
      if (!pl || !pl.items?.length) return;
      openPlayer({ source_path: pl.items[0], target_path: pl.items[0] });
    });
  }

  if (playlistCreateConfirm) {
    playlistCreateConfirm.addEventListener("click", async () => {
      const name = playlistCreateName?.value.trim();
      if (!name) return;
      hideModalCards();
      await saveNewPlaylist(name);
    });
  }

  if (playlistCreateCancel) {
    playlistCreateCancel.addEventListener("click", () => {
      hideModalCards();
    });
  }

  if (playlistCreateName) {
    playlistCreateName.addEventListener("keydown", async (e) => {
      if (e.key === "Enter") {
        const name = playlistCreateName.value.trim();
        if (!name) return;
        hideModalCards();
        await saveNewPlaylist(name);
      } else if (e.key === "Escape") {
        hideModalCards();
      }
    });
  }
}

// Wire up player controls once DOM is ready.
function initPlayer() {
  if (playerClose) {
    playerClose.addEventListener("click", closePlayer);
  }

  if (playerPlayPause) {
    playerPlayPause.addEventListener("click", () => {
      const el = playerMediaElement();
      if (!el) return;
      if (el.paused) { el.play().catch(() => {}); } else { el.pause(); }
      playerSetPlayPause(el);
    });
  }

  if (playerSkipBack) {
    playerSkipBack.addEventListener("click", () => {
      const el = playerMediaElement();
      if (el) el.currentTime = Math.max(0, el.currentTime - 10);
    });
  }

  if (playerSkipFwd) {
    playerSkipFwd.addEventListener("click", () => {
      const el = playerMediaElement();
      if (el) el.currentTime = Math.min(el.duration || Infinity, el.currentTime + 30);
    });
  }

  if (playerSeek) {
    playerSeek.addEventListener("input", () => {
      const el = playerMediaElement();
      if (!el || !isFinite(el.duration)) return;
      el.currentTime = (parseInt(playerSeek.value, 10) / 1000) * el.duration;
    });
  }

  if (playerSpeed) {
    playerSpeed.addEventListener("change", () => {
      const el = playerMediaElement();
      if (el) el.playbackRate = parseFloat(playerSpeed.value);
    });
  }

  if (playerSleepTimer) {
    playerSleepTimer.addEventListener("change", () => {
      if (!playerState) return;
      const { sleepTimerId } = playerState;
      if (sleepTimerId) clearTimeout(sleepTimerId);

      const minutes = parseInt(playerSleepTimer.value, 10);
      if (minutes > 0) {
        playerState.sleepTimerId = setTimeout(() => {
          const el = playerMediaElement();
          if (el) el.pause();
          if (playerPlayPause) playerPlayPause.textContent = "▶";
          if (playerSleepTimer) playerSleepTimer.value = "0";
        }, minutes * 60 * 1000);
      } else {
        playerState.sleepTimerId = null;
      }
    });
  }

  if (playerOpenSystem) {
    playerOpenSystem.addEventListener("click", async () => {
      if (!playerState) return;
      const root = getVaultRoot();
      const rootQuery = root ? `&root=${encodeURIComponent(root)}` : "";
      const path = playerState.vaultPath;
      try {
        await fetch(`mediavault://localhost/api/open-external?path=${encodeURIComponent(path)}${rootQuery}`);
      } catch {
        // Ignore — the system open is fire-and-forget
      }
    });
  }

  // Manga navigation buttons
  if (playerMangaPrev) {
    playerMangaPrev.addEventListener("click", () => {
      if (mangaState) mangaShowIndex(mangaState.index - 1);
    });
  }
  if (playerMangaNext) {
    playerMangaNext.addEventListener("click", () => {
      if (mangaState) mangaShowIndex(mangaState.index + 1);
    });
  }

  // Keep time display and seek bar in sync.
  for (const mediaEl of [playerVideo, playerAudio]) {
    if (!mediaEl) continue;
    mediaEl.addEventListener("timeupdate", () => playerUpdateTime(mediaEl));
    mediaEl.addEventListener("loadedmetadata", () => playerUpdateTime(mediaEl));
    mediaEl.addEventListener("play", () => playerSetPlayPause(mediaEl));
    mediaEl.addEventListener("pause", () => playerSetPlayPause(mediaEl));
    mediaEl.addEventListener("ended", () => {
      if (playerPlayPause) playerPlayPause.textContent = "▶";
      playerSaveProgress();
      playerAdvancePlaylist();
    });
  }

  // Close on Escape
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && playerDialog && !playerDialog.hidden) {
      e.preventDefault();
      closePlayer();
    }
    if (playerDialog && !playerDialog.hidden) {
      if (e.key === " " || e.key === "k") {
        e.preventDefault();
        playerPlayPause?.click();
      }
      if (e.key === "ArrowLeft") {
        if (mangaState) { e.preventDefault(); mangaShowIndex(mangaState.index - 1); }
        else { e.preventDefault(); playerSkipBack?.click(); }
      }
      if (e.key === "ArrowRight") {
        if (mangaState) { e.preventDefault(); mangaShowIndex(mangaState.index + 1); }
        else { e.preventDefault(); playerSkipFwd?.click(); }
      }
    }
  });
}

function playerAdvancePlaylist() {
  if (!activePlaylistId || !currentPlan) return;
  const pl = activePlaylists.find((p) => p.id === activePlaylistId);
  if (!pl || !pl.items?.length || !playerState) return;
  const cur = playerState.vaultPath;
  const idx = pl.items.indexOf(cur);
  if (idx < 0 || idx >= pl.items.length - 1) return;
  const next = currentPlan.items.find((it) => it.source_path === pl.items[idx + 1]);
  if (next) openPlayer(next);
}

function selectionSearchTitle(selection) {
  if (!selection) {
    return "";
  }

  const item = selection.item ?? {};
  if (selection.type === "node") {
    const kind = collectionNodeKind(selection.node);
    if (kind === "movie") {
      return item.title || selection.node.label || "";
    }
    if (kind === "series" || kind === "season") {
      return item.series_title || deriveSeriesTitle(item) || selection.node.label || "";
    }
    return selection.node.label || item.series_title || item.title || "";
  }

  const rawTitle =
    item.series_title ||
    item.title ||
    stripEpisodeMarkerText(fileStem(item.source_path)) ||
    "";
  return rawTitle.replace(/^[\s\-–—_:]+/, "").trim();
}

function showAniListResults(results, query) {
  if (!anilistDialogResults) {
    return;
  }

  clearNode(anilistDialogResults);
  const list = Array.isArray(results) ? results : [];
  if (!list.length) {
    const empty = document.createElement("div");
    empty.className = "anilist-empty";
    empty.textContent = `Keine Treffer für "${query}". Titel anpassen und erneut suchen.`;
    anilistDialogResults.appendChild(empty);
    return;
  }

  list.forEach((result) => {
    const card = document.createElement("article");
    card.className = "anilist-result";

    const cover = document.createElement("div");
    cover.className = "anilist-result-cover";
    const coverUrl = coverUrlFor(result);
    if (coverUrl) {
      const image = document.createElement("img");
      image.src = coverUrl;
      image.alt = aniListDisplayTitle(result);
      cover.appendChild(image);
    }

    const meta = document.createElement("div");
    meta.className = "anilist-result-meta";
    const title = document.createElement("strong");
    title.textContent = aniListDisplayTitle(result);
    const subtitle = document.createElement("span");
    subtitle.textContent = [
      result.format || "",
      result.season_year || result.start_date?.year || "",
      typeof result.episodes === "number" ? `${result.episodes} Folgen` : "",
      typeof result.average_score === "number" ? `Score ${Math.round(result.average_score)} / 100` : "",
    ]
      .filter(Boolean)
      .join(" · ");
    const description = document.createElement("p");
    description.textContent = String(result.description || "")
      .replace(/<[^>]*>/g, " ")
      .replace(/\s+/g, " ")
      .trim()
      .slice(0, 220);
    meta.appendChild(title);
    meta.appendChild(subtitle);
    if (description.textContent) {
      meta.appendChild(description);
    }

    const actions = document.createElement("div");
    actions.className = "anilist-result-actions";
    const applyButton = document.createElement("button");
    applyButton.type = "button";
    applyButton.className = "action-button primary";
    applyButton.textContent = "Übernehmen";
    applyButton.addEventListener("click", () => {
      applyAniListResult(result);
    });
    actions.appendChild(applyButton);

    card.appendChild(cover);
    card.appendChild(meta);
    card.appendChild(actions);
    anilistDialogResults.appendChild(card);
  });
}

function openAniListDialog(selection, feedbackNode = null, targetItems = []) {
  const resolvedSelection =
    selection && Array.isArray(selection.items) && typeof selection.key === "string"
      ? selection
      : normalizeSelection(selection);
  if (!resolvedSelection || !resolvedSelection.item) {
    return;
  }

  pendingAniListSelection = resolvedSelection;
  pendingAniListTargets = targetItems.length ? targetItems : resolvedSelection.items;
  pendingAniListFeedback = feedbackNode;

  if (anilistDialogTitle) {
    anilistDialogTitle.textContent = `AniList-Treffer für ${selectionTitle(resolvedSelection)}`;
  }
  if (anilistDialogQuery) {
    anilistDialogQuery.value = selectionSearchTitle(resolvedSelection);
  }
  if (anilistDialogFeedback) {
    anilistDialogFeedback.textContent = "";
    anilistDialogFeedback.className = "api-feedback";
  }
  if (anilistDialogResults) {
    clearNode(anilistDialogResults);
  }
  if (trashDialog) {
    trashDialog.hidden = true;
  }
  showModalCard(anilistDialog);
  if (statusStrip) {
    statusStrip.textContent = `AniList-Treffer für ${selectionTitle(resolvedSelection)} werden geladen.`;
  }
}

async function runAniListDialogSearch() {
  if (!pendingAniListSelection) {
    return;
  }

  const query = anilistDialogQuery?.value.trim() || "";
  if (!query) {
    setApiFeedback(anilistDialogFeedback, "Bitte zuerst einen Suchbegriff eintragen.", "error");
    return;
  }

  const adult = canonicalMediaType(pendingAniListSelection.item?.media_type) === "hentai-anime";
  if (statusStrip) {
    statusStrip.textContent = `AniList-Suche läuft für: ${query}`;
  }
  setApiFeedback(anilistDialogFeedback, `Suche AniList für "${query}"...`, "loading");

  const response = await fetch(
    `/api/anilist-search?title=${encodeURIComponent(query)}&adult=${adult ? "true" : "false"}&limit=10`
  );
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  const payload = await response.json();
  const results = Array.isArray(payload.results) ? payload.results : payload.metadata ? [payload.metadata] : [];
  showAniListResults(results, query);

  if (!results.length) {
    setApiFeedback(
      anilistDialogFeedback,
      `Keine Treffer für "${query}". Titel korrigieren und erneut suchen.`,
      "error"
    );
    return;
  }

  const countText = `${results.length} Treffer gefunden`;
  setApiFeedback(anilistDialogFeedback, countText, "success");
}

async function fetchAniListForItem(item, feedbackNode = null, targetItems = [item], selection = null) {
  const resolvedSelection =
    selection && Array.isArray(selection.items) && typeof selection.key === "string"
      ? selection
      : selection
        ? normalizeSelection(selection)
        : normalizeSelection(item);
  if (!resolvedSelection) {
    throw new Error("Kein gültiger Eintrag für AniList vorhanden");
  }

  openAniListDialog(resolvedSelection, feedbackNode, targetItems);
  try {
    await runAniListDialogSearch();
  } catch (error) {
    setApiFeedback(
      anilistDialogFeedback,
      `AniList konnte keine Daten liefern: ${error.message}. Titel prüfen und erneut suchen.`,
      "error"
    );
    if (statusStrip) {
      statusStrip.textContent = `AniList konnte keine Daten liefern: ${error.message}`;
    }
    throw error;
  }
}

function applyAniListResult(metadata) {
  if (!pendingAniListSelection) {
    return;
  }

  const searchTitle = anilistDialogQuery?.value.trim() || selectionSearchTitle(pendingAniListSelection);
  applySelectionCorrection(pendingAniListSelection, {
    titleValue: searchTitle,
    mediaTypeValue: pendingAniListSelection.item?.media_type || mediaSelectionValue(pendingAniListSelection.item ?? {}),
    yearValue: pendingAniListSelection.item?.year ?? null,
    statusValue:
      pendingAniListSelection.item?.status ||
      (pendingAniListSelection.item?.needs_review ? "needs-review" : null),
  });

  const normalized = normalizeAniListMetadata(metadata, pendingAniListSelection.item);
  pendingAniListTargets.forEach((target) => {
    apiMetadata[target.source_path] = normalized;
  });
  saveStoredJson(metadataKey, apiMetadata);
  currentPlan = projectPlan(sourcePlan);
  const firstTarget = pendingAniListTargets[0];
  if (pendingAniListSelection.type === "node") {
    selectedCollectionKey = pendingAniListSelection.node?.path || selectedCollectionKey;
    selectedItemKey = "";
    selectedCollectionItemKey = "";
  } else if (firstTarget?.source_path) {
    selectedItemKey = firstTarget.source_path;
    selectedCollectionItemKey = firstTarget.source_path;
  }
  renderPlan(currentPlan);
  setApiFeedback(pendingAniListFeedback, `AniList übernommen: ${aniListSummary(normalized)}`, "success");
  updateAuditTrail(`AniList-Metadaten übernommen für ${selectionTitle(pendingAniListSelection)}.`);
  void persistSidecarsForItems(
    pendingAniListTargets
      .map((target) => currentPlan.items.find((item) => item.source_path === target.source_path))
      .filter(Boolean)
  ).catch((error) => {
    if (statusStrip) {
      statusStrip.textContent = `AniList übernommen, aber Sidecar konnte nicht geschrieben werden: ${error.message}`;
    }
  });
  closeActionModal();
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
    const text = String(cell ?? "");
    span.textContent = text;
    span.title = text;
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
    // Audiobook sub-parts are hidden by default under their group representative.
    if (item.is_audiobook_part) {
      return;
    }
    const status = item.duplicate_of
      ? "Duplikat"
      : needsReviewInUi(item)
      ? "Review"
      : item.audiobook_parts
      ? `Hörbuch (${item.audiobook_parts.length} Teile)`
      : "klar";
    const target = item.duplicate_of || item.needs_review
      ? "Inbox/_review_queue"
      : item.target_path ?? "Inbox/_review_queue";
    const displayName = item.title || basename(item.source_path);
    const displayTarget = target.startsWith("Inbox/") ? target : basename(target);
    inboxRows.appendChild(createRow(item, [displayName, status, displayTarget], false));
  });
}

function renderReviewRows(items) {
  if (!reviewRows) {
    return;
  }

  clearNode(reviewRows);

  const reviewItems = items.filter((item) => needsReviewInUi(item));

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
      createRow(item, [item.title || basename(item.source_path), status, action], item.source_path === selectedItemKey)
    );
  });

  if (!selectedItemKey && reviewItems.length) {
    selectedItemKey = reviewItems[0].source_path;
  }
}

function syncCollectionSortButtons() {
  [
    [collectionSortName, "name"],
    [collectionSortType, "type"],
    [collectionSortTarget, "target"],
  ].forEach(([button, key]) => {
    if (!button) {
      return;
    }
    button.classList.toggle("is-active", collectionSortKey === key);
    button.dataset.direction = collectionSortKey === key ? collectionSortDirection : "";
  });
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
  if (detailStatusLabel) detailStatusLabel.textContent = statusLabel(item.status || (needsReviewInUi(item) ? "needs-review" : "inbox"));
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
  if (detailStatus) detailStatus.value = item.status ?? (needsReviewInUi(item) ? "needs-review" : "inbox");
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

function propertyTypeIcon(key, value) {
  const type = propertyTypeLabel(key, value);
  switch (type) {
    case "Link":
      return "↗";
    case "Datum":
      return "◷";
    case "Bewertung":
      return "★";
    case "Nummer":
      return "#";
    case "Tags":
      return "⌘";
    case "Liste":
      return "☰";
    case "Objekt":
      return "{}";
    default:
      return "Aa";
  }
}

function propertyDisplayValue(value) {
  if (Array.isArray(value)) {
    if (!value.length) {
      return "-";
    }
    return value
      .slice(0, 5)
      .map((entry) => summarizePropertyObject(entry))
      .join(", ");
  }

  if (value && typeof value === "object") {
    return summarizePropertyObject(value);
  }

  return value === null || typeof value === "undefined" || value === "" ? "-" : String(value);
}

function summarizePropertyObject(value) {
  if (!value || typeof value !== "object") {
    return value === null || typeof value === "undefined" ? "-" : String(value);
  }

  if (value.character_name || value.voice_actor_name) {
    return [value.character_name || value.name, value.voice_actor_name ? `VA ${value.voice_actor_name}` : ""]
      .filter(Boolean)
      .join(" · ");
  }
  if (value.role && (value.name || value.person?.name)) {
    return `${value.role}: ${value.name || value.person?.name}`;
  }
  if (value.relation_type || value.title) {
    return [value.relation_type, value.title || value.name].filter(Boolean).join(": ");
  }
  if (value.summary || value.user_name) {
    return [value.user_name, value.summary].filter(Boolean).join(": ");
  }
  if (value.name || value.title) {
    return value.name || value.title;
  }

  return Object.entries(value)
    .slice(0, 3)
    .map(([key, entry]) => `${key}: ${typeof entry === "object" ? "…" : String(entry)}`)
    .join(" | ");
}

function propertyEntriesFor(value) {
  const selection = normalizeSelection(value);
  if (!selection) {
    return [];
  }
  const item = selection.item ?? {};
  const baseEntries = selection.type === "bulk"
    ? [
        ["object_type", "Mehrfachauswahl"],
        ["items", selection.items.length],
      ]
    : selection.type === "node"
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
    ["status", item.status ? statusLabel(item.status) : needsReviewInUi(item) ? "Prüfung nötig" : null],
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
  const previousInspectorKey = inspectorItemKey;
  const selection = normalizeSelection(value);
  inspectorSelection = selection;
  inspectorItemKey = selection?.key ?? "";
  if (previousInspectorKey && previousInspectorKey !== inspectorItemKey) {
    document.body.classList.remove("inspector-editing", "property-add-open");
  }
  const item = selection?.item ?? null;
  const editable = selectionEditable(selection);
  const metadataAllowed = selectionSupportsMetadata(selection);
  const yamlEditable = editable && selection?.type !== "bulk";

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
    inspectorStatus.value = item?.status ?? (item && needsReviewInUi(item) ? "needs-review" : "inbox");
  }

  if (inspectorProperties) {
    clearNode(inspectorProperties);
    const entries = propertyEntriesFor(selection);
    if (!entries.length) {
      const empty = document.createElement("div");
      empty.className = "property-empty";
      empty.textContent = "Wähle eine Datei, Serie, Staffel oder Mehrfachauswahl aus.";
      inspectorProperties.appendChild(empty);
    } else {
      entries.forEach(([key, value]) => {
        const row = document.createElement("div");
        row.className = "property-row";

        const meta = document.createElement("div");
        meta.className = "property-row-meta";
        const label = document.createElement("div");
        label.className = "property-label";
        const icon = document.createElement("span");
        icon.className = "property-type-icon";
        icon.textContent = propertyTypeIcon(key, value);
        const name = document.createElement("strong");
        name.textContent = key;
        const body = document.createElement("p");
        body.textContent = propertyDisplayValue(value);

        label.appendChild(icon);
        label.appendChild(name);
        meta.appendChild(label);
        row.appendChild(meta);
        row.appendChild(body);
        inspectorProperties.appendChild(row);
      });
    }
  }

  if (inspectorYaml) {
    inspectorYaml.value = selectionYamlPreview(selection);
    inspectorYaml.disabled = !yamlEditable;
  }
  if (inspectorYamlSave) {
    inspectorYamlSave.disabled = !yamlEditable;
  }
  if (inspectorYamlReset) {
    inspectorYamlReset.disabled = !yamlEditable || !yamlOverrides[selection.key];
  }
  if (inspectorEditToggle) {
    inspectorEditToggle.disabled = !editable;
    inspectorEditToggle.classList.toggle("is-active", document.body.classList.contains("inspector-editing"));
    inspectorEditToggle.textContent = "✎";
  }
  if (inspectorPlay) {
    const srcPath = item?.source_path || item?.target_path || "";
    const ext = playerFileExt(srcPath);
    const playable =
      isVideoFile(srcPath) ||
      isAudioFile(srcPath) ||
      PLAYER_UNSUPPORTED_EXTS.has(ext) ||
      PLAYER_PDF_EXTS.has(ext) ||
      PLAYER_IMAGE_EXTS.has(ext) ||
      PLAYER_EPUB_EXTS.has(ext);
    inspectorPlay.hidden = !item || !playable;
  }
  if (inspectorFetchMetadata) {
    inspectorFetchMetadata.disabled = !metadataAllowed;
  }
  if (inspectorNotDuplicate) {
    const duplicateRelevant = selectionHasDuplicateFlag(selection) || selectionHasDuplicateOverride(selection);
    inspectorNotDuplicate.disabled = !editable || !duplicateRelevant;
    inspectorNotDuplicate.textContent = selectionHasDuplicateOverride(selection)
      ? "Duplikat-Rücksetzung"
      : "Kein Duplikat";
  }
  [inspectorApply, inspectorTrash].forEach((control) => {
    if (control) {
      control.disabled = !editable;
    }
  });
  [
    inspectorPropertyAddToggle,
    inspectorPropertyKey,
    inspectorPropertyType,
    inspectorPropertyValue,
    inspectorPropertyAdd,
  ].forEach((control) => {
    if (control) {
      control.disabled = !yamlEditable;
    }
  });
  if (inspectorYamlHint) {
    inspectorYamlHint.textContent = yamlEditable
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

  const readyCount = plan.items.filter((item) => isReadyForImport(item)).length;
  if (applyImportButton) {
    applyImportButton.textContent =
      readyCount > 0 ? `Import anwenden (${readyCount})` : "Import anwenden";
  }

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
    naturalCompare(left.label, right.label)
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

function nodeDisplayTitle(node, item = null) {
  const kind = collectionNodeKind(node);
  if (["series", "season"].includes(kind)) {
    return item ? deriveSeriesTitle(item) : node.label;
  }
  if (kind === "movie") {
    return item?.title || node.label;
  }
  return node.label;
}

function representativeItem(node) {
  return node.items.find((item) => item.anilist_id || item.series_title) ?? node.items[0] ?? null;
}

function normalizeSelection(value) {
  if (!value) {
    return null;
  }
  if (value.type === "bulk" && Array.isArray(value.items)) {
    const items = value.items.filter(Boolean);
    return {
      type: "bulk",
      key: `bulk:${items.map((item) => item.source_path).join("|")}`,
      item: items[0] ?? null,
      items,
      node: null,
    };
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

function selectionEditable(selection) {
  if (!selection || !selection.items.length) {
    return false;
  }
  if (selection.type === "bulk" || selection.type === "file") {
    return true;
  }
  const kind = collectionNodeKind(selection.node);
  return ["series", "season", "movie"].includes(kind);
}

function selectionSupportsMetadata(selection) {
  return Boolean(selectionEditable(selection) && selection.type !== "bulk");
}

function selectionHasDuplicateFlag(selection) {
  return Boolean(selection && selection.items.some((item) => item.duplicate_of));
}

function selectionHasDuplicateOverride(selection) {
  return Boolean(selection && selection.items.some((item) => duplicateOverrides[item.source_path]));
}

function applyDuplicateOverride(selection) {
  if (!selectionEditable(selection)) {
    return false;
  }

  let changed = false;
  selection.items.forEach((item) => {
    if (!duplicateOverrides[item.source_path]) {
      duplicateOverrides[item.source_path] = true;
      changed = true;
    }
  });

  if (changed) {
    saveDuplicateOverrides();
  }

  return changed;
}

function clearDuplicateOverride(selection) {
  if (!selectionEditable(selection)) {
    return false;
  }

  let changed = false;
  selection.items.forEach((item) => {
    if (duplicateOverrides[item.source_path]) {
      delete duplicateOverrides[item.source_path];
      changed = true;
    }
  });

  if (changed) {
    saveDuplicateOverrides();
  }

  return changed;
}

function selectionTitle(selection) {
  if (!selection) {
    return "Kein Eintrag ausgewählt";
  }
  if (selection.type === "bulk") {
    return `${selection.items.length} Einträge ausgewählt`;
  }
  if (selection.type === "node") {
    const primary = selection.item;
    return nodeDisplayTitle(selection.node, primary) || "Sammlung";
  }
  const rawTitle = selection.item?.title || fileStem(selection.item?.source_path) || "Unbenannt";
  return rawTitle.replace(/^[\s\-–—_:]+/, "").trim() || rawTitle;
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
  if (selection.type === "bulk") {
    const lines = [
      "---",
      `id: ${yamlScalar(selection.key)}`,
      "object_type: mehrfachauswahl",
      `items: ${selection.items.length}`,
      `title: ${yamlScalar(selectionTitle(selection))}`,
      "---",
    ];
    return lines.join("\n");
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
  const reviewCount = node.items.filter((item) => needsReviewInUi(item)).length;

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

  if (!node.path) {
    const recent = validRecentCollections(root);
    if (recent.length) {
      const heading = document.createElement("p");
      heading.className = "panel-label collection-recent-label";
      heading.textContent = "Zuletzt angesehen";
      collectionSidebarList.appendChild(heading);
    }

    recent.forEach((entry) => {
      const button = document.createElement("button");
      button.type = "button";
      button.className = "collection-nav-item";
      button.classList.add("is-recent");
      button.addEventListener("click", () => {
        selectedCollectionKey = entry.path;
        selectedCollectionItemKey = "";
        renderCollections(currentPlan?.items ?? []);
      });

      const label = document.createElement("strong");
      label.textContent = entry.label;
      const count = document.createElement("span");
      count.textContent = entry.path;
      button.appendChild(label);
      button.appendChild(count);
      collectionSidebarList.appendChild(button);
    });
  }

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
  const coverUrl = coverUrlFor(primary);
  if (coverUrl) {
    const image = document.createElement("img");
    image.src = coverUrl;
    image.alt = primary.series_title || primary.title || node.label;
    cover.appendChild(image);
    if (isLocalImageItem(primary)) {
      cover.classList.add("is-clickable");
      cover.addEventListener("click", () => openImagePreview(primary));
    }
  } else {
    cover.textContent = "MV";
  }

  const heading = document.createElement("div");
  heading.className = "collection-profile-heading";

  const label = document.createElement("span");
  label.textContent = nodeTypeLabel(node);
  const name = document.createElement("strong");
  name.textContent = primary.series_title || primary.title || node.label;
  name.className = "media-title";
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

  const kind = collectionNodeKind(node);
  const children = sortedChildren(node);
  if (children.length) {
    const stack = document.createElement("div");
    stack.className = "media-section-stack";
    const seasons = createMediaSection("Staffeln", `${children.length} Einträge in dieser Serie`);
    const cards = document.createElement("div");
    cards.className = "season-card-row";
    children.forEach((child, index) => {
      cards.appendChild(createPosterCard(child, index + 1));
    });
    seasons.appendChild(cards);
    stack.appendChild(seasons);
    appendCollectionMetadataSections(stack, primary, description.textContent);
    collectionProfile.appendChild(stack);
  } else {
    const stack = document.createElement("div");
    stack.className = "media-section-stack";
    appendCollectionMetadataSections(stack, primary, description.textContent);
    collectionProfile.appendChild(stack);
  }
}

function appendCollectionMetadataSections(stack, item, synopsisText) {
  const synopsis = createMediaSection("Synopsis");
  const synopsisBody = document.createElement("p");
  synopsisBody.className = "media-synopsis";
  synopsisBody.textContent = synopsisText;
  synopsis.appendChild(synopsisBody);
  stack.appendChild(synopsis);

  const details = createMediaSection("Details");
  details.appendChild(createDetailsGrid(item));
  stack.appendChild(details);

  if (Array.isArray(item.relations) && item.relations.length) {
    const relations = createMediaSection("Relationen");
    const relationGrid = document.createElement("div");
    relationGrid.className = "relation-grid";
    item.relations.slice(0, 8).forEach((relation) => {
      const card = document.createElement("div");
      card.className = "relation-card";
      const imageUrl = relation.cover_image_large || relation.cover_image_medium || "";
      if (imageUrl) {
        const image = document.createElement("img");
        image.src = imageUrl;
        image.alt = relation.title || "Relation";
        card.appendChild(image);
      }
      const relationType = document.createElement("span");
      relationType.textContent = relation.relation_type || "Verknüpft";
      const title = document.createElement("strong");
      title.textContent = relation.title || "Unbekannt";
      const meta = document.createElement("p");
      meta.textContent = [relation.media_type, relation.format, relation.status].filter(Boolean).join(" · ");
      card.appendChild(relationType);
      card.appendChild(title);
      card.appendChild(meta);
      relationGrid.appendChild(card);
    });
    relations.appendChild(relationGrid);
    stack.appendChild(relations);
  }

  if (Array.isArray(item.characters) && item.characters.length) {
    const characters = createMediaSection("Charaktere");
    const list = document.createElement("div");
    list.className = "credit-grid";
    item.characters.slice(0, 8).forEach((character) => {
      list.appendChild(
        createCreditCard(
          character.character_name || character.name || "Unbekannt",
          character.voice_actor_name || character.voice_actor?.name || "",
          character.role || character.language || "",
          character.character_image || character.character?.image || "",
          character.voice_actor_image || character.voice_actor?.image || ""
        )
      );
    });
    characters.appendChild(list);
    stack.appendChild(characters);
  }

  if (Array.isArray(item.staff) && item.staff.length) {
    const staff = createMediaSection("Staff");
    const list = document.createElement("div");
    list.className = "credit-grid";
    item.staff.slice(0, 8).forEach((entry) => {
      list.appendChild(
        createCreditCard(
          entry.name || "Unbekannt",
          entry.role || "",
          entry.language || "",
          entry.image || "",
          ""
        )
      );
    });
    staff.appendChild(list);
    stack.appendChild(staff);
  }
}

function createCreditCard(name, subline, meta, leftImageUrl = "", rightImageUrl = "") {
  const card = document.createElement("div");
  card.className = "credit-card";
  card.classList.toggle("has-left-image", Boolean(leftImageUrl));
  card.classList.toggle("has-right-image", Boolean(rightImageUrl));

  if (leftImageUrl) {
    const left = document.createElement("img");
    left.src = leftImageUrl;
    left.alt = name;
    left.className = "credit-thumb";
    card.appendChild(left);
  }

  const text = document.createElement("div");
  text.className = "credit-card-copy";
  const title = document.createElement("strong");
  title.textContent = name;
  text.appendChild(title);
  if (subline) {
    const support = document.createElement("span");
    support.textContent = subline;
    text.appendChild(support);
  }
  if (meta) {
    const metaNode = document.createElement("p");
    metaNode.textContent = meta;
    text.appendChild(metaNode);
  }
  card.appendChild(text);

  if (rightImageUrl) {
    const right = document.createElement("img");
    right.src = rightImageUrl;
    right.alt = subline || name;
    right.className = "credit-thumb is-secondary";
    card.appendChild(right);
  }

  return card;
}

function createMediaSection(title, subtitle = "") {
  const section = document.createElement("section");
  section.className = "media-section";
  const heading = document.createElement("div");
  heading.className = "media-section-heading";
  const icon = document.createElement("span");
  icon.textContent = "◆";
  const text = document.createElement("div");
  const strong = document.createElement("strong");
  strong.textContent = title;
  text.appendChild(strong);
  if (subtitle) {
    const sub = document.createElement("p");
    sub.textContent = subtitle;
    text.appendChild(sub);
  }
  heading.appendChild(icon);
  heading.appendChild(text);
  section.appendChild(heading);
  return section;
}

function createDetailsGrid(item) {
  const grid = document.createElement("div");
  grid.className = "media-details-grid";
  [
    ["Typ", item.format || mediaTypeLabel(mediaSelectionValue(item))],
    ["Status", item.status ? statusLabel(item.status) : "-"],
    ["Staffel", item.airing_season || "-"],
    ["Ausgestrahlt", [datePartsLabel(item.start_date), datePartsLabel(item.end_date)].filter(Boolean).join(" - ")],
    ["Dauer", typeof item.runtime_minutes === "number" ? `${item.runtime_minutes}m` : "-"],
    ["Score", typeof item.average_score === "number" ? scoreLabel(item.average_score) : "-"],
    [
      "Studios",
      (item.studios || [])
        .map((studio) => (typeof studio === "string" ? studio : studio?.name))
        .filter(Boolean)
        .join(", "),
    ],
    ["Synonyme", (item.synonyms || []).slice(0, 4).join(", ")],
  ].forEach(([label, value]) => {
    const block = document.createElement("div");
    const span = document.createElement("span");
    span.textContent = label;
    const strong = document.createElement("strong");
    strong.textContent = value || "-";
    block.appendChild(span);
    block.appendChild(strong);
    grid.appendChild(block);
  });
  return grid;
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

function selectedBulkItems() {
  return (currentPlan?.items ?? []).filter((item) => multiSelectedKeys.has(item.source_path));
}

function renderBulkInspector() {
  renderInspector({ type: "bulk", items: selectedBulkItems() });
}

function toggleBulkItem(item) {
  if (!item?.source_path) {
    return;
  }
  if (multiSelectedKeys.has(item.source_path)) {
    multiSelectedKeys.delete(item.source_path);
  } else {
    multiSelectedKeys.add(item.source_path);
  }
  renderCollections(currentPlan?.items ?? []);
  renderBulkInspector();
}

function installLongPressSelection(element, item) {
  let timer = null;
  const clearTimer = () => {
    if (timer) {
      window.clearTimeout(timer);
      timer = null;
    }
  };

  element.addEventListener("pointerdown", () => {
    if (!item?.source_path) {
      return;
    }
    clearTimer();
    timer = window.setTimeout(() => {
      isMultiEdit = true;
      element.dataset.longPressFired = "true";
      toggleBulkItem(item);
    }, 450);
  });
  ["pointerup", "pointerleave", "pointercancel"].forEach((eventName) => {
    element.addEventListener(eventName, clearTimer);
  });
}

function createPosterCard(value, ordinal = null) {
  const isNode = !value.source_path;
  const node = isNode ? value : null;
  const item = isNode ? representativeItem(node) : value;
  const button = document.createElement("button");
  button.type = "button";
  button.className = "poster-card";
  if (!isNode && item?.source_path) {
    button.classList.toggle("is-bulk-selected", multiSelectedKeys.has(item.source_path));
    installLongPressSelection(button, item);
  }
  button.addEventListener("click", () => {
    if (button.dataset.longPressFired === "true") {
      button.dataset.longPressFired = "";
      return;
    }
    if (!isNode && isMultiEdit) {
      toggleBulkItem(item);
      return;
    }
    if (isNode) {
      selectedCollectionKey = node.path;
      selectedCollectionItemKey = "";
      renderCollections(currentPlan?.items ?? []);
    } else {
      selectedItemKey = item.source_path;
      selectedCollectionItemKey = item.source_path;
      renderInspector(item);
      renderCollectionRows(findCollectionNode(buildCollectionTree(currentPlan?.items ?? []), selectedCollectionKey));
      const srcPath = item.source_path || item.target_path || "";
      const cardExt = playerFileExt(srcPath);
      const cardPlayable =
        isVideoFile(srcPath) ||
        isAudioFile(srcPath) ||
        PLAYER_UNSUPPORTED_EXTS.has(cardExt) ||
        PLAYER_PDF_EXTS.has(cardExt) ||
        PLAYER_IMAGE_EXTS.has(cardExt) ||
        PLAYER_EPUB_EXTS.has(cardExt);
      if (cardPlayable) openPlayer(item);
    }
  });

  const artwork = document.createElement("div");
  artwork.className = "poster-art";
  const imageUrl = coverUrlFor(item);
  if (imageUrl) {
    const image = document.createElement("img");
    image.src = imageUrl;
    image.alt = item?.series_title || item?.title || node?.label || "Cover";
    artwork.appendChild(image);
  } else {
    artwork.textContent = ordinal || "MV";
  }

  const badge = document.createElement("span");
  badge.className = "poster-badge";
  badge.textContent = ordinal || (item ? mediaTypeLabel(mediaSelectionValue(item)) : nodeTypeLabel(node));
  artwork.appendChild(badge);

  if (typeof item?.average_score === "number") {
    const score = document.createElement("span");
    score.className = "poster-score";
    score.textContent = `★ ${scoreLabel(item.average_score)}`;
    artwork.appendChild(score);
  }

  const title = document.createElement("strong");
  title.textContent = isNode ? nodeDisplayTitle(node, item) : item.title || fileStem(item.source_path);
  const meta = document.createElement("span");
  meta.textContent = isNode
    ? [nodeTypeLabel(node), item?.year].filter(Boolean).join(" · ")
    : [mediaTypeLabel(mediaSelectionValue(item)), formatBytes(item.size_bytes)].filter(Boolean).join(" · ");

  button.appendChild(artwork);
  button.appendChild(title);
  button.appendChild(meta);
  return button;
}

function createCollectionItemRow(item) {
  const row = document.createElement("button");
  row.type = "button";
  row.className = "table-row table-row-button";
  row.classList.toggle("is-selected", item.source_path === selectedCollectionItemKey);
  row.classList.toggle("is-bulk-selected", multiSelectedKeys.has(item.source_path));
  row.dataset.sourcePath = item.source_path;
  installLongPressSelection(row, item);
  row.addEventListener("click", () => {
    if (row.dataset.longPressFired === "true") {
      row.dataset.longPressFired = "";
      return;
    }
    if (isMultiEdit) {
      toggleBulkItem(item);
      return;
    }
    selectedItemKey = item.source_path;
    selectedCollectionItemKey = item.source_path;
    renderInspector(item);
    renderCollectionRows(findCollectionNode(buildCollectionTree(currentPlan?.items ?? []), selectedCollectionKey));
    if (statusStrip) {
      statusStrip.textContent = `Eintrag ausgewählt: ${item.source_path}`;
    }
  });

  [
    item.title || basename(item.source_path),
    mediaTypeLabel(mediaSelectionValue(item)),
    item.target_path ? basename(item.target_path) : statusLabel(item.status || "inbox"),
  ].forEach((cell) => {
    const span = document.createElement("span");
    span.textContent = cell;
    row.appendChild(span);
  });

  const rowSrcPath = item.source_path || item.target_path || "";
  const rowExt = playerFileExt(rowSrcPath);
  const rowPlayable =
    isVideoFile(rowSrcPath) ||
    isAudioFile(rowSrcPath) ||
    PLAYER_UNSUPPORTED_EXTS.has(rowExt) ||
    PLAYER_PDF_EXTS.has(rowExt) ||
    PLAYER_IMAGE_EXTS.has(rowExt) ||
    PLAYER_EPUB_EXTS.has(rowExt);
  if (rowPlayable) {
    const playCell = document.createElement("button");
    playCell.type = "button";
    playCell.className = "action-button compact icon-button";
    playCell.textContent = "▶";
    playCell.title = "Abspielen";
    playCell.addEventListener("click", (e) => {
      e.stopPropagation();
      openPlayer(item);
    });
    row.appendChild(playCell);
  }

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
    sortedCollectionItems(node.directItems).forEach((item) => {
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

function renderCollectionCards(node) {
  if (!collectionCards) {
    return;
  }

  clearNode(collectionCards);
  sortedChildren(node).forEach((child, index) => {
    collectionCards.appendChild(createPosterCard(child, index + 1));
  });
  if (!sortedChildren(node).length) {
    node.directItems.forEach((item, index) => {
      collectionCards.appendChild(createPosterCard(item, index + 1));
    });
  }
  if (!collectionCards.childNodes.length) {
    const empty = document.createElement("div");
    empty.className = "collection-empty";
    empty.textContent = "Keine Einträge in dieser Sammlung.";
    collectionCards.appendChild(empty);
  }
}

function compareCollectionItem(left, right) {
  if (collectionSortKey === "type") {
    return mediaTypeLabel(mediaSelectionValue(left)).localeCompare(mediaTypeLabel(mediaSelectionValue(right)), "de");
  }

  if (collectionSortKey === "target") {
    return String(left.target_path || left.status || "")
      .localeCompare(String(right.target_path || right.status || ""), "de");
  }

  const leftSeason = left.season_number || episodeMarker(left.source_path)?.season || 1;
  const rightSeason = right.season_number || episodeMarker(right.source_path)?.season || 1;
  const leftEpisode = left.episode_start || episodeMarker(left.source_path)?.episodeStart || 0;
  const rightEpisode = right.episode_start || episodeMarker(right.source_path)?.episodeStart || 0;

  if (leftSeason !== rightSeason) {
    return leftSeason - rightSeason;
  }
  if (leftEpisode !== rightEpisode) {
    return leftEpisode - rightEpisode;
  }

  return naturalCompare(
    left.title || basename(left.source_path),
    right.title || basename(right.source_path)
  );
}

function sortedCollectionItems(items) {
  const sorted = [...items].sort(compareCollectionItem);
  return collectionSortDirection === "desc" ? sorted.reverse() : sorted;
}

function syncCollectionViewMode(node = null) {
  const kind = node ? collectionNodeKind(node) : null;
  const forceList = kind === "season" || kind === "movie";
  const hideContent = kind === "series";
  const effectiveView = forceList ? "list" : collectionView;

  document.body.classList.toggle("collection-list-view", effectiveView === "list");
  document.body.classList.toggle("collection-cover-view", effectiveView !== "list");
  document.body.classList.toggle("collection-force-list", forceList);
  document.body.classList.toggle("collection-series-view", hideContent);
  document.body.classList.toggle("collection-bulk-mode", isMultiEdit);
  if (collectionMultiToggle) {
    collectionMultiToggle.classList.toggle("is-active", isMultiEdit);
    collectionMultiToggle.textContent = isMultiEdit
      ? `Auswahl beenden (${multiSelectedKeys.size})`
      : "Mehrfachauswahl";
  }
  if (collectionViewBar) {
    collectionViewBar.hidden = hideContent;
  }
  if (collectionViewToggle) {
    collectionViewToggle.hidden = forceList || hideContent;
  }
  document.querySelectorAll("[data-collection-view]").forEach((button) => {
    button.classList.toggle("is-active", button.dataset.collectionView === effectiveView);
  });
  syncCollectionSortButtons();
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
    if (collectionProfile) {
      clearNode(collectionProfile);
    }
    if (collectionCards) {
      clearNode(collectionCards);
    }
    if (collectionEditor) {
      collectionEditor.hidden = true;
    }
    renderInspector(null);
    return;
  }

  selectedCollectionKey = selected.path;
  if (selected.path) {
    rememberCollection(selected);
  }

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
  const selectedKind = collectionNodeKind(selected);
  const hideSeasonContent = selectedKind === "series";
  if (collectionRows) {
    collectionRows.hidden = hideSeasonContent;
  }
  if (collectionEditor) {
    collectionEditor.hidden = true;
  }
  renderCollectionRows(selected);
  renderCollectionCards(selected);
  syncCollectionViewMode(selected);

  const selectedItem = selected.directItems.find((item) => item.source_path === selectedCollectionItemKey);
  if (isMultiEdit && currentActiveTab() === "collections") {
    renderBulkInspector();
  } else if (!selectedItem && currentActiveTab() === "collections") {
    renderInspector({ type: "node", node: selected });
  } else if (selectedItem && currentActiveTab() === "collections") {
    renderInspector(selectedItem);
  }
}

function renderCollectionEditor(item) {
  if (!collectionEditor) {
    return;
  }

  collectionEditor.hidden = false;
  collectionEditor.classList.toggle("is-active", Boolean(item));

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
  applySelectionCorrection(selection);
}

function applySelectionCorrection(selection, overrides = {}) {
  if (!selectionEditable(selection)) {
    return;
  }

  const titleValue = (typeof overrides.titleValue === "string"
    ? overrides.titleValue.trim()
    : inspectorName?.value.trim()) || null;
  const mediaTypeValue = overrides.mediaTypeValue || inspectorMediaType?.value || null;
  const yearValue =
    Object.prototype.hasOwnProperty.call(overrides, "yearValue")
      ? overrides.yearValue
      : normalizeYear(inspectorYear?.value);
  const statusValue = Object.prototype.hasOwnProperty.call(overrides, "statusValue")
    ? overrides.statusValue
    : inspectorStatus?.value || null;

  selection.items.forEach((item) => {
    const existing = corrections[item.source_path] ?? {};
    const next = {
      ...existing,
      media_type: mediaTypeValue || existing.media_type || mediaSelectionValue(item),
      year: yearValue,
      status: statusValue || existing.status || null,
      updated_at: new Date().toISOString(),
    };

    if (selection.type === "node") {
      const kind = collectionNodeKind(selection.node);
      if (["series", "season"].includes(kind)) {
        next.series_title = titleValue;
      } else if (kind === "movie") {
        next.title = titleValue;
      }
    } else if (selection.type === "bulk") {
      if (titleValue && selection.items.length === 1) {
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

function openTrashDialog(selection) {
  if (!selectionEditable(selection)) {
    return;
  }

  pendingTrashSelection = selection;
  pendingAniListSelection = null;
  pendingAniListTargets = [];
  pendingAniListFeedback = null;
  if (trashDialogTitle) {
    trashDialogTitle.textContent = `${selectionTitle(selection)} in den Papierkorb verschieben?`;
  }
  if (trashDialogBody) {
    trashDialogBody.textContent = `${selection.items.length} Eintrag(e) werden zunächst nur in den App-Papierkorb verschoben.`;
  }
  showModalCard(trashDialog);
}

function commitTrashSelection(selection) {
  if (!selectionEditable(selection)) {
    return;
  }

  const count = selection.items.length;
  const at = new Date().toISOString();
  selection.items.forEach((item) => {
    trashedEntries[item.source_path] = {
      source_path: item.source_path,
      title: item.title || item.series_title || fileStem(item.source_path),
      at,
    };
    delete corrections[item.source_path];
    delete apiMetadata[item.source_path];
  });
  saveStoredJson(trashKey, trashedEntries);
  saveStoredJson(correctionsKey, corrections);
  saveStoredJson(metadataKey, apiMetadata);
  multiSelectedKeys.clear();
  selectedCollectionItemKey = "";
  selectedItemKey = "";
  currentPlan = projectPlan(sourcePlan);
  renderPlan(currentPlan);
  updateAuditTrail(`${count} Eintrag(e) in den App-Papierkorb verschoben.`);
  if (statusStrip) {
    statusStrip.textContent = `${count} Eintrag(e) in den MediaVault-Papierkorb verschoben.`;
  }
  closeActionModal();
}

function trashSelection(selection) {
  openTrashDialog(selection);
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
    // Load dashboard asynchronously so it doesn't block the plan render.
    loadDashboard().catch(() => {});
  } catch (error) {
    if (statusStrip) {
      statusStrip.textContent = `Vault-Scan konnte nicht geladen werden: ${error.message}`;
    }
  }
}

tabs.forEach((button) => {
  button.addEventListener("click", () => {
    const tab = button.dataset.tab;
    setActiveTab(tab, { resetCollections: tab === "collections" });
    if (tab === "collections" && currentPlan) {
      renderCollections(currentPlan.items);
    }
    if (tab === "overview") {
      loadDashboard().catch(() => {});
    }
    if (tab === "playlists") {
      loadPlaylists().catch(() => {});
    }
  });
});

document.querySelectorAll("[data-collection-view]").forEach((button) => {
  button.addEventListener("click", () => {
    collectionView = button.dataset.collectionView || "cover";
    localStorage.setItem(collectionViewKey, collectionView);
    syncCollectionViewMode();
  });
});

if (collectionMultiToggle) {
  collectionMultiToggle.addEventListener("click", () => {
    isMultiEdit = !isMultiEdit;
    if (!isMultiEdit) {
      multiSelectedKeys.clear();
    }
    renderCollections(currentPlan?.items ?? []);
    if (isMultiEdit) {
      renderBulkInspector();
    }
  });
}

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

if (applyImportButton) {
  applyImportButton.addEventListener("click", async () => {
    try {
      await applyReadyImports();
    } catch (error) {
      if (statusStrip) {
        statusStrip.textContent = `Import konnte nicht angewendet werden: ${error.message}`;
      }
    }
  });
}

if (openVaultFolderButton) {
  openVaultFolderButton.addEventListener("click", async () => {
    const root = getVaultRoot();
    if (!root) return;
    try {
      await fetch(`mediavault://localhost/api/open-vault-root?root=${encodeURIComponent(root)}`);
    } catch {
      // fire-and-forget
    }
  });
}

const vaultSwitchBtn = document.getElementById("vault-switch");
if (vaultSwitchBtn) {
  vaultSwitchBtn.addEventListener("click", () => {
    localStorage.removeItem(storageKey);
    persistVaultRootState("").catch(() => {});
    document.body.classList.remove("is-vault-open");
    syncVaultHint();
    renderRecentVaults();
  });
}

if (vaultOpenButton) {
  vaultOpenButton.addEventListener("click", async () => {
    let selected = "";
    try {
      selected = await selectFolder();
    } catch (error) {
      if (statusStrip) {
        statusStrip.textContent = `Vault-Auswahl fehlgeschlagen: ${error.message}`;
      }
      return;
    }
    if (!selected) {
      if (statusStrip) {
        statusStrip.textContent = "Kein Vault ausgewählt.";
      }
      return;
    }
    if (statusStrip) {
      statusStrip.textContent = `Vault ausgewählt: ${selected}`;
    }
    openVault(selected);
  });
}

if (vaultCreateSubmit) {
  vaultCreateSubmit.addEventListener("click", async () => {
    const name = vaultCreateName?.value.trim();

    if (!name) {
      if (statusStrip) {
        statusStrip.textContent = "Bitte einen Vault-Namen eingeben.";
      }
      return;
    }

    try {
      if (statusStrip) {
        statusStrip.textContent = "Speicherort wählen...";
      }

      const parent = await selectFolder();
      if (!parent) {
        if (statusStrip) {
          statusStrip.textContent = "Kein Speicherort ausgewählt.";
        }
        return;
      }

      if (statusStrip) {
        statusStrip.textContent = "Vault wird angelegt...";
      }

      const result = await createVault(parent, name);
      const path = result?.path || compactPath(`${parent}/${sanitizeSegment(name)}`);
      openVault(path, name);
    } catch (error) {
      if (statusStrip) {
        statusStrip.textContent = `Vault konnte nicht angelegt werden: ${error.message}`;
      }
    }
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
      openVault(value).catch(() => {});
    } else {
      localStorage.removeItem(storageKey);
      persistVaultRootState("").catch(() => {});
      document.body.classList.remove("is-vault-open");
      renderRecentVaults();
    }

    syncVaultHint();
  });
}

if (vaultRootClear) {
  vaultRootClear.addEventListener("click", () => {
    if (vaultRootInput) {
      vaultRootInput.value = "";
    }

    localStorage.removeItem(storageKey);
    persistVaultRootState("").catch(() => {});
    document.body.classList.remove("is-vault-open");
    syncVaultHint();
    renderRecentVaults();
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
        "Templates gespeichert. Zielpfade wurden neu berechnet und gelten beim nächsten Import anwenden.";
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
      await fetchAniListForItem(draft, detailApiFeedback, [item], normalizeSelection(draft));
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
      await fetchAniListForItem(draft, collectionEditorApiFeedback, [item], normalizeSelection(draft));
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
    if (!selectionEditable(inspectorSelection)) {
      return;
    }
    document.body.classList.toggle("inspector-editing");
    inspectorEditToggle.classList.toggle("is-active", document.body.classList.contains("inspector-editing"));
  });
}

[
  [collectionSortName, "name"],
  [collectionSortType, "type"],
  [collectionSortTarget, "target"],
].forEach(([button, key]) => {
  if (!button) {
    return;
  }

  button.addEventListener("click", () => {
    if (collectionSortKey === key) {
      collectionSortDirection = collectionSortDirection === "asc" ? "desc" : "asc";
    } else {
      collectionSortKey = key;
      collectionSortDirection = key === "name" ? "asc" : "desc";
    }

    syncCollectionSortButtons();
    renderCollections(currentPlan?.items ?? []);
  });
});

if (inspectorPropertyAddToggle) {
  inspectorPropertyAddToggle.addEventListener("click", () => {
    document.body.classList.toggle("property-add-open");
  });
}

if (inspectorApply) {
  inspectorApply.addEventListener("click", async () => {
    if (!selectionEditable(inspectorSelection)) {
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
    try {
      await persistSidecarsForItems(
        inspectorSelection.items
          .map((item) => currentPlan.items.find((candidate) => candidate.source_path === item.source_path))
          .filter(Boolean)
      );
    } catch (error) {
      if (statusStrip) {
        statusStrip.textContent = `Änderungen übernommen, aber Sidecar konnte nicht geschrieben werden: ${error.message}`;
      }
    }
    updateAuditTrail(`Inspector-Korrektur übernommen für ${inspectorSelection.key}.`);
  });
}

if (inspectorPlay) {
  inspectorPlay.addEventListener("click", () => {
    const item = inspectorSelection?.item;
    if (item) openPlayer(item);
  });
}

if (inspectorFetchMetadata) {
  inspectorFetchMetadata.addEventListener("click", async () => {
    if (!selectionSupportsMetadata(inspectorSelection) || !inspectorSelection.item) {
      return;
    }

    try {
      upsertSelectionCorrection(inspectorSelection);
      const inspectorDraft = {
        ...inspectorSelection.item,
        title: inspectorName?.value.trim() || inspectorSelection.item.title,
        series_title: inspectorName?.value.trim() || inspectorSelection.item.series_title,
        media_type: inspectorMediaType?.value || mediaSelectionValue(inspectorSelection.item),
      };
      const selectionForSearch =
        inspectorSelection.type === "node"
          ? {
              ...inspectorSelection,
              item: inspectorDraft,
            }
          : normalizeSelection(inspectorDraft);
      await fetchAniListForItem(
        inspectorDraft,
        inspectorApiFeedback,
        inspectorSelection.items,
        selectionForSearch
      );
    } catch (error) {
      setApiFeedback(
        inspectorApiFeedback,
        `AniList konnte keine Daten liefern: ${error.message}. Titel prüfen und erneut abrufen.`,
        "error"
      );
    }
  });
}

if (inspectorNotDuplicate) {
  inspectorNotDuplicate.addEventListener("click", () => {
    if (!selectionEditable(inspectorSelection)) {
      return;
    }

    const currentlyOverridden = selectionHasDuplicateOverride(inspectorSelection);
    const changed = currentlyOverridden
      ? clearDuplicateOverride(inspectorSelection)
      : applyDuplicateOverride(inspectorSelection);

    if (!changed) {
      return;
    }

    currentPlan = projectPlan(sourcePlan);
    renderPlan(currentPlan);
    renderInspector(inspectorSelection);

    const count = inspectorSelection.items.length;
    const message = currentlyOverridden
      ? `${count} Eintrag(e) werden wieder als Duplikat behandelt.`
      : `${count} Eintrag(e) werden nicht mehr als Duplikat behandelt.`;
    updateAuditTrail(message);
    if (statusStrip) {
      statusStrip.textContent = message;
    }
  });
}

if (inspectorTrash) {
  inspectorTrash.addEventListener("click", () => {
    openTrashDialog(inspectorSelection);
  });
}

if (trashDialogConfirm) {
  trashDialogConfirm.addEventListener("click", () => {
    if (pendingTrashSelection) {
      commitTrashSelection(pendingTrashSelection);
    }
  });
}

if (trashDialogCancel) {
  trashDialogCancel.addEventListener("click", () => {
    closeActionModal();
  });
}

if (anilistDialogSearch) {
  anilistDialogSearch.addEventListener("click", async () => {
    try {
      await runAniListDialogSearch();
    } catch (error) {
      setApiFeedback(
        anilistDialogFeedback,
        `AniList konnte keine Daten liefern: ${error.message}. Titel prüfen und erneut suchen.`,
        "error"
      );
      if (statusStrip) {
        statusStrip.textContent = `AniList konnte keine Daten liefern: ${error.message}`;
      }
    }
  });
}

if (anilistDialogCancel) {
  anilistDialogCancel.addEventListener("click", () => {
    closeActionModal();
  });
}

if (imageDialogClose) {
  imageDialogClose.addEventListener("click", () => {
    closeActionModal();
  });
}

if (anilistDialogQuery) {
  anilistDialogQuery.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      anilistDialogSearch?.click();
    }
    if (event.key === "Escape") {
      closeActionModal();
    }
  });
}

if (appModal) {
  appModal.addEventListener("click", (event) => {
    if (event.target === appModal) {
      closeActionModal();
    }
  });
}

if (inspectorYamlSave) {
  inspectorYamlSave.addEventListener("click", async () => {
    if (!inspectorItemKey || !inspectorYaml) {
      return;
    }

    yamlOverrides[inspectorItemKey] = inspectorYaml.value;
    saveStoredJson(yamlOverridesKey, yamlOverrides);
    currentPlan = projectPlan(sourcePlan);
    renderPlan(currentPlan);
    try {
      await persistSidecarsForItems(
        inspectorSelection?.items
          ?.map((item) => currentPlan.items.find((candidate) => candidate.source_path === item.source_path))
          .filter(Boolean) || []
      );
    } catch (error) {
      if (statusStrip) {
        statusStrip.textContent = `YAML gespeichert, aber Sidecar konnte nicht geschrieben werden: ${error.message}`;
      }
    }
    updateAuditTrail(`YAML gespeichert für ${inspectorItemKey}.`);
  });
}

if (inspectorYamlReset) {
  inspectorYamlReset.addEventListener("click", async () => {
    if (!inspectorItemKey) {
      return;
    }

    delete yamlOverrides[inspectorItemKey];
    saveStoredJson(yamlOverridesKey, yamlOverrides);
    currentPlan = projectPlan(sourcePlan);
    renderPlan(currentPlan);
    try {
      await persistSidecarsForItems(
        inspectorSelection?.items
          ?.map((item) => currentPlan.items.find((candidate) => candidate.source_path === item.source_path))
          .filter(Boolean) || []
      );
    } catch (error) {
      if (statusStrip) {
        statusStrip.textContent = `YAML-Reset gespeichert, aber Sidecar konnte nicht geschrieben werden: ${error.message}`;
      }
    }
    updateAuditTrail(`YAML-Override zurückgesetzt für ${inspectorItemKey}.`);
  });
}

if (inspectorPropertyAdd) {
  inspectorPropertyAdd.addEventListener("click", async () => {
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
    try {
      await persistSidecarsForItems(
        inspectorSelection?.items
          ?.map((item) => currentPlan.items.find((candidate) => candidate.source_path === item.source_path))
          .filter(Boolean) || []
      );
    } catch (error) {
      if (statusStrip) {
        statusStrip.textContent = `Property ergänzt, aber Sidecar konnte nicht geschrieben werden: ${error.message}`;
      }
    }
    updateAuditTrail(`Property ${inspectorPropertyKey.value} ergänzt für ${inspectorItemKey}.`);

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
    isMultiEdit = false;
    multiSelectedKeys.clear();
    setActiveTab("overview");
  });
}

populateSelectors();
renderAuditTrail();
syncTemplateInputs();
initPlayer();
initPlaylists();
initAbsSettings();
bootstrapVault().catch(() => {
  renderRecentVaults();
  syncVaultHint();
});
