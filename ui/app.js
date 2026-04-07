const historyState = {
  latencyMs: [],
  throughput: [],
  maxPoints: 80,
};

let timer = null;

const nodes = {
  pollInterval: document.getElementById("poll-interval-ms"),
  startBtn: document.getElementById("start-btn"),
  stopBtn: document.getElementById("stop-btn"),
  healthDot: document.getElementById("health-dot"),
  healthText: document.getElementById("health-text"),
  lastUpdated: document.getElementById("last-updated"),
  foodRate: document.getElementById("metric-food-rate"),
  stepAvgMs: document.getElementById("metric-step-avg-ms"),
  stability: document.getElementById("metric-runtime-stability"),
  antsActive: document.getElementById("metric-ants-active"),
  runtimeMailbox: document.getElementById("runtime-mailbox-depth"),
  runtimeDropped: document.getElementById("runtime-dropped-total"),
  runtimeRestarts: document.getElementById("runtime-restarts-total"),
  runtimeSupervision: document.getElementById("runtime-supervision-total"),
  runtimeServerUptime: document.getElementById("runtime-server-uptime"),
  latencyCanvas: document.getElementById("latency-chart"),
  throughputCanvas: document.getElementById("throughput-chart"),
};

nodes.startBtn.addEventListener("click", startPolling);
nodes.stopBtn.addEventListener("click", stopPolling);

startPolling();

function startPolling() {
  stopPolling();
  fetchAndRender();
  const interval = Math.max(200, Number(nodes.pollInterval.value) || 1000);
  timer = setInterval(fetchAndRender, interval);
}

function stopPolling() {
  if (timer !== null) {
    clearInterval(timer);
    timer = null;
  }
}

async function fetchAndRender() {
  try {
    const [healthText, metricsText] = await Promise.all([
      fetch("/api/healthz").then((res) => res.text()),
      fetch("/api/metrics").then((res) => res.text()),
    ]);
    const healthy = healthText.trim().toLowerCase() === "ok";
    setHealth(healthy);

    const metrics = parsePrometheus(metricsText);
    updateCards(metrics);
    updateRuntimeTable(metrics);
    appendSeries(metrics);
    drawLineChart(nodes.latencyCanvas, historyState.latencyMs, "#6cf6d8");
    drawLineChart(nodes.throughputCanvas, historyState.throughput, "#7eb8ff");
    nodes.lastUpdated.textContent = `Last updated: ${new Date().toLocaleTimeString()}`;
  } catch (_) {
    setHealth(false);
  }
}

function setHealth(healthy) {
  nodes.healthDot.classList.toggle("healthy", healthy);
  nodes.healthDot.classList.toggle("degraded", !healthy);
  nodes.healthText.textContent = `Health: ${healthy ? "ok" : "degraded"}`;
}

function updateCards(m) {
  const foodRate = number(m.empireants_food_collected_total) / Math.max(0.001, number(m.empireants_simulation_elapsed_seconds));
  const avgMs = number(m.empireants_step_latency_avg_microseconds) / 1000;
  const activeAnts = number(m.empireants_ants_searching) + number(m.empireants_ants_returning);
  const failureTotal =
    number(m.empireants_runtime_dropped_messages_total) +
    number(m.empireants_runtime_restarts_total) +
    number(m.empireants_runtime_supervision_events_total);
  const stability = 1 / (1 + failureTotal);

  nodes.foodRate.textContent = foodRate.toFixed(2);
  nodes.stepAvgMs.textContent = avgMs.toFixed(2);
  nodes.antsActive.textContent = Math.round(activeAnts).toString();
  nodes.stability.textContent = stability.toFixed(4);
}

function updateRuntimeTable(m) {
  nodes.runtimeMailbox.textContent = fmt(m.empireants_runtime_mailbox_depth, 0);
  nodes.runtimeDropped.textContent = fmt(m.empireants_runtime_dropped_messages_total, 0);
  nodes.runtimeRestarts.textContent = fmt(m.empireants_runtime_restarts_total, 0);
  nodes.runtimeSupervision.textContent = fmt(m.empireants_runtime_supervision_events_total, 0);
  nodes.runtimeServerUptime.textContent = fmt(m.empireants_server_uptime_seconds, 2);
}

function appendSeries(m) {
  const latency = number(m.empireants_step_latency_avg_microseconds) / 1000;
  const throughput = number(m.empireants_food_collected_total) / Math.max(0.001, number(m.empireants_simulation_elapsed_seconds));
  push(historyState.latencyMs, latency);
  push(historyState.throughput, throughput);
}

function push(target, value) {
  target.push(value);
  if (target.length > historyState.maxPoints) {
    target.shift();
  }
}

function drawLineChart(canvas, points, color) {
  const ctx = canvas.getContext("2d");
  const w = canvas.width;
  const h = canvas.height;
  ctx.clearRect(0, 0, w, h);

  if (points.length === 0) {
    return;
  }

  const min = Math.min(...points);
  const max = Math.max(...points);
  const range = Math.max(0.0001, max - min);

  ctx.strokeStyle = "rgba(255,255,255,0.08)";
  ctx.lineWidth = 1;
  for (let i = 1; i <= 3; i++) {
    const y = (h / 4) * i;
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(w, y);
    ctx.stroke();
  }

  ctx.strokeStyle = color;
  ctx.lineWidth = 2;
  ctx.beginPath();
  points.forEach((value, idx) => {
    const x = (idx / Math.max(1, points.length - 1)) * (w - 16) + 8;
    const y = h - ((value - min) / range) * (h - 20) - 10;
    if (idx === 0) {
      ctx.moveTo(x, y);
    } else {
      ctx.lineTo(x, y);
    }
  });
  ctx.stroke();
}

function parsePrometheus(text) {
  const out = {};
  for (const line of text.split("\n")) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const [name, value] = trimmed.split(/\s+/, 2);
    out[name] = Number(value);
  }
  return out;
}

function number(value) {
  if (Number.isFinite(value)) return value;
  return 0;
}

function fmt(value, digits) {
  return number(value).toFixed(digits);
}
