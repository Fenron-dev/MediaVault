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

const labels = {
  overview: "Überblick",
  inbox: "Inbox",
  review: "Prüfung",
  collections: "Sammlungen",
  settings: "Einstellungen",
};

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

function createRow(cells) {
  const row = document.createElement("div");
  row.className = "table-row";

  cells.forEach((cell) => {
    const span = document.createElement("span");
    span.textContent = cell;
    row.appendChild(span);
  });

  return row;
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
    const target = item.duplicate_of
      ? "Inbox/_review_queue"
      : item.target_path ?? "Inbox/_review_queue";
    inboxRows.appendChild(createRow([item.source_path, status, target]));
  });
}

function renderReviewRows(items) {
  if (!reviewRows) {
    return;
  }

  clearNode(reviewRows);

  items
    .filter((item) => item.needs_review)
    .forEach((item) => {
      const status = item.duplicate_of
        ? "Duplikat"
        : item.manual_review
        ? "Review"
        : "Prüfung";
      const action = item.steps[item.steps.length - 1] ?? "Prüfen";
      reviewRows.appendChild(createRow([item.source_path, status, action]));
    });
}

function renderPlan(plan) {
  setMetrics(plan.summary);
  renderInboxRows(plan.items);
  renderReviewRows(plan.items);

  if (statusStrip) {
    statusStrip.textContent = `${plan.title} geladen: ${plan.summary.total_files} Dateien, ${plan.summary.items_needing_review} zur Prüfung, ${plan.summary.duplicates} Duplikat(e).`;
  }
}

async function loadPlan() {
  if (statusStrip) {
    statusStrip.textContent = "Import-Dry-Run wird geladen...";
  }

  try {
    const response = await fetch("/api/demo-plan");
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }

    const plan = await response.json();
    renderPlan(plan);
  } catch (error) {
    if (statusStrip) {
      statusStrip.textContent = `Demo-Dry-Run konnte nicht geladen werden: ${error.message}`;
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

loadPlan();
