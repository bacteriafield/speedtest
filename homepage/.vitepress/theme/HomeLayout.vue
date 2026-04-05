<template>
  <div class="site-root">
    <!-- Scanlines overlay -->
    <div class="scanlines" aria-hidden="true" />

    <!-- ─── NAV ─── -->
    <nav :class="{ scrolled: isScrolled }" id="navbar">
      <div class="nav-container">
        <a href="#" class="nav-logo">
          <span class="logo-prompt">$</span>
          <span class="logo-name">speedtest</span>
          <span class="logo-cursor">▊</span>
        </a>
        <div class="nav-links">
          <a href="#features">features</a>
          <a href="#demo">demo</a>
          <a href="https://github.com/bacteriafield/speedtest#installation">install</a>
          <a href="#arch">architecture</a>
          <a
            href="https://github.com/bacteriafield/speedtest"
            target="_blank"
            class="btn-gh"
          >
            <img :src="github_logo" alt="GitHub">
            GitHub
          </a>
        </div>
        <button
          class="hamburger"
          :class="{ active: menuOpen }"
          @click="menuOpen = !menuOpen"
          aria-label="Toggle menu"
        >
          <span /><span /><span />
        </button>
      </div>
    </nav>

    <!-- Mobile menu -->
    <div class="mobile-menu" :class="{ open: menuOpen }">
      <div class="mobile-menu-content">
        <a href="#features" class="mobile-link" @click="menuOpen = false">features</a>
        <a href="#demo"     class="mobile-link" @click="menuOpen = false">demo</a>
        <a href="https://github.com/bacteriafield/speedtest#installation" class="mobile-link" @click="menuOpen = false">install</a>
        <a href="#arch"     class="mobile-link" @click="menuOpen = false">architecture</a>
        <a href="https://github.com/bacteriafield/speedtest" target="_blank" class="mobile-link" @click="menuOpen = false">GitHub ↗</a>
      </div>
    </div>

    <!-- ─── HERO ─── -->
    <section class="hero" id="hero">
      <div class="hero-bg-grid" aria-hidden="true" />
      <div class="hero-glow"   aria-hidden="true" />

      <div class="hero-content">
        <div class="hero-badge">
          <span class="badge-dot" />
          v0.1.0 — open source
        </div>

        <h1 class="hero-title">
          <span class="title-line1">speedtest</span>
        </h1>

        <p class="hero-sub">
          Professional CLI speed test.<br>
          Async. Zero overhead. No external dependencies.
        </p>

        <div class="hero-pills">
          <span class="pill">⚡ Tokio async</span>
          <span class="pill">📊 Terminal graphs</span>
          <span class="pill">🔧 JSON output</span>
          <span class="pill">🌐 Multi-stream</span>
          <span class="pill">🦀 100% Rust</span>
        </div>

        <div class="hero-install">
          <div
            class="install-cmd-wrap"
            :class="{ copied: cmdCopied }"
            @click="copyCmd"
          >
            <span class="cmd-prefix">$</span>
            <code>{{ cmdCopied ? 'Copied!' : 'curl https://raw.githubusercontent.com/bacteriafield/speedtest/refs/heads/main/scripts/install.sh | sh' }}</code>
            <div class="copy-toast" :class="{ show: cmdCopied }">Copied!</div>
          </div>
          <a
            href="https://github.com/bacteriafield/speedtest#installation"
            target="_blank"
            class="hero-more-link"
          >more options →</a>
        </div>
      </div>

      <!-- Hero Terminal -->
      <div class="hero-terminal">
        <div class="terminal-chrome">
          <div class="chrome-dots">
            <span /><span /><span />
          </div>
          <span class="chrome-title">speedtest</span>
          <div style="width:42px" />
        </div>
        <div class="terminal-body">
          <span class="term-line dim">$ speedtest</span>
          <span class="term-line dim">Detecting nearest server...</span>
          <span class="term-line accent">✓ Server: São Paulo, BR — 8ms</span>

          <div class="term-blank" />

          <div class="term-speed-row">
            <span class="speed-label">Download</span>
            <span class="speed-val">{{ dlDisplay }} Mbps</span>
          </div>
          <div class="term-speed-row">
            <span class="speed-label">Upload</span>
            <span class="speed-val">{{ ulDisplay }} Mbps</span>
          </div>
          <div class="term-speed-row">
            <span class="speed-label">Ping</span>
            <span class="speed-val">{{ pingDisplay }} ms</span>
          </div>

          <div class="term-graph-wrap">
            <div class="graph-label">download throughput</div>
            <canvas ref="graphCanvas" id="term-graph" width="540" height="60" />
          </div>
        </div>
      </div>
    </section>

    <!-- ─── FEATURES ─── -->
    <section class="features" id="features">
      <div class="section-container">
        <div class="section-header reveal" ref="featHeader">
          <h2>Why speedtest?</h2>
          <p>Built for engineers who need real data, fast.</p>
        </div>

        <div v-for="(feat, i) in features" :key="i"
          class="feature-row reveal"
          :class="{ reverse: i % 2 === 1 }"
          :ref="el => featRefs[i] = el"
        >
          <div class="feature-text">
            <div class="feat-tag">{{ feat.tag }}</div>
            <h2 v-html="feat.title" />
            <p>{{ feat.desc }}</p>
            <ul v-if="feat.list" class="feat-list">
              <li v-for="item in feat.list" :key="item">{{ item }}</li>
            </ul>
          </div>
          <div class="feature-visual">
            <div class="fv-dots">
              <span /><span /><span />
            </div>
            <div class="fv-code" v-html="feat.code" />
          </div>
        </div>
      </div>
    </section>

    <!-- ─── DEMO ─── -->
    <section class="demo-section" id="demo">
      <div class="section-container">
        <div class="section-header reveal">
          <h2>Live simulation</h2>
          <p>See how speedtest runs inside your terminal.</p>
        </div>
        <div class="demo-terminal reveal">
          <div class="terminal-chrome">
            <div class="chrome-dots"><span /><span /><span /></div>
            <span class="chrome-title">speedtest — demo</span>
            <div style="width:42px" />
          </div>
          <div class="demo-body" ref="demoBody" />
          <div class="demo-controls">
            <button class="demo-btn" @click="runLiveDemo">▶ Run demo</button>
            <button class="demo-btn secondary" @click="runJsonDemo">{ } JSON mode</button>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── ARCH ─── -->
    <section class="arch-section" id="arch">
      <div class="section-container">
        <div class="section-header reveal">
          <h2>Modular architecture</h2>
          <p>Clear separation of responsibilities. Easy to extend.</p>
        </div>
        <div class="arch-grid">
          <div
            v-for="mod in modules"
            :key="mod.name"
            class="arch-module reveal"
          >
            <div class="arch-icon">{{ mod.icon }}</div>
            <div class="arch-name">{{ mod.name }}</div>
            <div class="arch-desc">{{ mod.desc }}</div>
            <span class="arch-stack">{{ mod.stack }}</span>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── FLAGS ─── -->
    <section class="install-section" id="install">
      <div class="section-container">
        <div class="flags-section">
          <h3 class="flags-title">Available flags</h3>
          <div class="flags-grid">
            <div v-for="flag in flags" :key="flag.cmd" class="flag-item reveal">
              <code class="flag-cmd">{{ flag.cmd }}</code>
              <span class="flag-desc">{{ flag.desc }}</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── FOOTER ─── -->
    <footer>
      <div class="footer-inner">
        <div class="footer-brand">
          <span class="logo-prompt">$</span>
          <span class="logo-name">speedtest</span>
        </div>
        <div class="footer-links">
          <a href="https://github.com/bacteriafield/speedtest" target="_blank">GitHub</a>
          <a href="https://github.com/bacteriafield/speedtest/releases" target="_blank">Releases</a>
          <a href="https://github.com/bacteriafield/speedtest/blob/master/LICENSE" target="_blank">License</a>
        </div>
        <div class="footer-credit">Built with 🦀 Rust · Open source · MIT</div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import github_logo from '../../public/github-icon.svg'

// ─── Nav scroll ───
const isScrolled = ref(false)
const menuOpen   = ref(false)

// ─── Copy command ───
const cmdCopied = ref(false)
const fullCmd   = 'curl https://raw.githubusercontent.com/bacteriafield/speedtest/refs/heads/main/scripts/install.sh | sh'

async function copyCmd() {
  try {
    await navigator.clipboard.writeText(fullCmd)
    cmdCopied.value = true
    setTimeout(() => { cmdCopied.value = false }, 1400)
  } catch {}
}

// ─── Hero terminal animation ───
const dlDisplay   = ref('0')
const ulDisplay   = ref('0')
const pingDisplay = ref('--')
const graphCanvas = ref<HTMLCanvasElement | null>(null)

const GRAPH_LEN = 30
const graphData = Array(GRAPH_LEN).fill(0)
let heroTick  = 0
let dlCurrent = 0
let ulCurrent = 0
let dlTarget  = 0
let ulTarget  = 0
let heroTimer: ReturnType<typeof setTimeout> | null = null

function lerp(a: number, b: number, t: number) { return a + (b - a) * t }

function drawGraph(data: number[]) {
  const canvas = graphCanvas.value
  if (!canvas) return
  const ctx = canvas.getContext('2d')!
  const W = canvas.width, H = canvas.height
  ctx.clearRect(0, 0, W, H)
  const max = Math.max(...data, 100)
  const pad = { l: 0, r: 4, t: 6, b: 0 }
  const gW  = W - pad.l - pad.r
  const gH  = H - pad.t - pad.b

  ctx.strokeStyle = 'rgba(249,115,22,0.08)'
  ctx.lineWidth = 1
  for (let i = 1; i < 4; i++) {
    const y = pad.t + (gH / 4) * i
    ctx.beginPath(); ctx.moveTo(pad.l, y); ctx.lineTo(W - pad.r, y); ctx.stroke()
  }

  const grad = ctx.createLinearGradient(0, pad.t, 0, H)
  grad.addColorStop(0, 'rgba(249,115,22,0.25)')
  grad.addColorStop(1, 'rgba(249,115,22,0.02)')

  ctx.beginPath()
  data.forEach((v, i) => {
    const x = pad.l + (i / (data.length - 1)) * gW
    const y = pad.t + gH - (v / max) * gH
    i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y)
  })
  ctx.lineTo(W - pad.r, H); ctx.lineTo(pad.l, H); ctx.closePath()
  ctx.fillStyle = grad; ctx.fill()

  ctx.beginPath()
  ctx.strokeStyle = '#f97316'; ctx.lineWidth = 1.5; ctx.lineJoin = 'round'
  data.forEach((v, i) => {
    const x = pad.l + (i / (data.length - 1)) * gW
    const y = pad.t + gH - (v / max) * gH
    i === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y)
  })
  ctx.stroke()
}

function runHeroTick() {
  heroTick++
  let phase = 'idle'

  if (heroTick < 10) {
    phase = 'ramp-up'
    dlTarget = lerp(0, 450, heroTick / 10)
    ulTarget = lerp(0, 130, heroTick / 10)
  } else if (heroTick < 40) {
    phase = 'sustain'
    dlTarget = 420 + Math.sin(heroTick * .3) * 40 + (Math.random() - .5) * 30
    ulTarget = 110 + Math.sin(heroTick * .4) * 20 + (Math.random() - .5) * 15
  } else {
    phase = 'done'
  }

  dlCurrent = lerp(dlCurrent, dlTarget, 0.2)
  ulCurrent = lerp(ulCurrent, ulTarget, 0.2)

  dlDisplay.value   = dlCurrent.toFixed(0)
  ulDisplay.value   = ulCurrent.toFixed(0)
  if (heroTick > 2) pingDisplay.value = String(8 + Math.floor(Math.random() * 3))

  graphData.shift()
  graphData.push(dlCurrent)
  drawGraph(graphData)

  if (phase !== 'done') {
    heroTimer = setTimeout(runHeroTick, 200)
  } else {
    heroTimer = setTimeout(() => {
      heroTick = 0; dlCurrent = 0; ulCurrent = 0; dlTarget = 0; ulTarget = 0
      graphData.fill(0); runHeroTick()
    }, 3000)
  }
}

// ─── Features data ───
const featRefs = ref<any[]>([])
const features = [
  {
    tag: 'performance',
    title: 'Zero overhead.<br>Real throughput.',
    desc: 'Multiple simultaneous TCP/HTTP connections via Tokio async ensure real throughput measurement — not theoretical.',
    list: ['8 parallel download streams', '4 parallel upload streams', 'Zero-copy buffer strategy'],
    code: `<span class="fc-line"><span class="fc-c">// spawn parallel streams</span></span>
<span class="fc-line"><span class="fc-kw">let</span> handles: Vec&lt;_&gt; = (0..streams)</span>
<span class="fc-line fc-indent"><span class="fc-fn">.map</span>(|_| tokio::spawn(</span>
<span class="fc-line fc-indent2"><span class="fc-fn">download_chunk</span>(client.<span class="fc-fn">clone</span>(), url)</span>
<span class="fc-line fc-indent">))</span>
<span class="fc-line fc-indent"><span class="fc-fn">.collect</span>();</span>
<span class="fc-blank"/>
<span class="fc-line"><span class="fc-kw">let</span> results = <span class="fc-fn">join_all</span>(handles).<span class="fc-fn">await</span>;</span>`
  },
  {
    tag: 'visualization',
    title: 'Live graphs<br>inside your terminal.',
    desc: 'Real-time rendering using Unicode blocks and Braille characters. Smooth updates with no flicker.',
    list: ['Braille dot rendering (2560 dpi equivalent)', 'No flicker via full-frame diff', 'Adaptive scale'],
    code: `<span class="fc-line"><span class="fc-c">// braille rasterizer</span></span>
<span class="fc-line"><span class="fc-kw">fn</span> <span class="fc-fn">render_graph</span>(points: &amp;[f64]) -&gt; String {</span>
<span class="fc-line fc-indent"><span class="fc-kw">let</span> canvas = BrailleCanvas::<span class="fc-fn">new</span>(</span>
<span class="fc-line fc-indent2">points.<span class="fc-fn">len</span>() * <span class="fc-n">2</span>, <span class="fc-n">4</span></span>
<span class="fc-line fc-indent">);</span>
<span class="fc-line fc-indent">canvas.<span class="fc-fn">plot</span>(points).<span class="fc-fn">render</span>()</span>
<span class="fc-line">}</span>`
  },
  {
    tag: 'server selection',
    title: 'Best server.<br>Automatically selected.',
    desc: 'Automatic nearest-server discovery based on geolocation, latency, and availability.',
    list: ['IP-based geolocation', 'Parallel latency probing', 'Fallback chain'],
    code: `<span class="fc-line"><span class="fc-kw">async fn</span> <span class="fc-fn">best_server</span>(</span>
<span class="fc-line fc-indent">candidates: Vec&lt;Server&gt;</span>
<span class="fc-line">) -&gt; Server {</span>
<span class="fc-line fc-indent"><span class="fc-kw">let</span> pings = <span class="fc-fn">probe_all</span>(&amp;candidates).<span class="fc-fn">await</span>;</span>
<span class="fc-line fc-indent">pings.<span class="fc-fn">into_iter</span>()</span>
<span class="fc-line fc-indent2">.<span class="fc-fn">min_by_key</span>(|s| s.latency_ms)</span>
<span class="fc-line fc-indent2">.<span class="fc-fn">unwrap</span>()</span>
<span class="fc-line">}</span>`
  },
  {
    tag: 'structured output',
    title: 'Native JSON.<br>Ready for metrics.',
    desc: 'Structured output for Prometheus, Grafana, dashboards, and observability pipelines.',
    list: ['--json flag for machine output', 'Compatible with jq, Prometheus', '--csv for spreadsheet export'],
    code: `<span class="fc-line"><span class="fc-str">{</span></span>
<span class="fc-line fc-indent"><span class="fc-str">"download_mbps"</span>: <span class="fc-n">423</span>,</span>
<span class="fc-line fc-indent"><span class="fc-str">"upload_mbps"</span>:   <span class="fc-n">118</span>,</span>
<span class="fc-line fc-indent"><span class="fc-str">"ping_ms"</span>:        <span class="fc-n">8</span>,</span>
<span class="fc-line fc-indent"><span class="fc-str">"jitter_ms"</span>:      <span class="fc-n">1.1</span>,</span>
<span class="fc-line fc-indent"><span class="fc-str">"server"</span>: <span class="fc-str">"br-rj-01"</span>,</span>
<span class="fc-line fc-indent"><span class="fc-str">"timestamp"</span>: <span class="fc-str">"2026-04-04T12:00:00Z"</span></span>
<span class="fc-line"><span class="fc-str">}</span></span>`
  }
]

// ─── Architecture modules ───
const modules = [
  { icon: '⚡', name: 'async-runtime',  desc: 'Tokio-powered async executor. Manages connection pools and stream concurrency.',    stack: 'tokio / futures' },
  { icon: '🌐', name: 'server-discovery', desc: 'IP geolocation + parallel ICMP probing to select the fastest available server.',  stack: 'reqwest / trust-dns' },
  { icon: '📡', name: 'measurement',    desc: 'Download and upload engine with configurable stream count and test duration.',       stack: 'hyper / bytes' },
  { icon: '📊', name: 'renderer',       desc: 'Terminal UI with Braille canvas, Unicode bars, and full-frame diff for no flicker.', stack: 'crossterm / unicode' },
  { icon: '🔧', name: 'output',         desc: 'Structured result serialization to JSON, CSV, and human-readable terminal output.',  stack: 'serde / csv' },
  { icon: '💾', name: 'history',        desc: 'Local SQLite store for result history. Query and compare past tests with --history.', stack: 'rusqlite' },
]

// ─── Flags ───
const flags = [
  { cmd: '--live',           desc: 'Real-time terminal graph' },
  { cmd: '--json',           desc: 'Structured JSON output' },
  { cmd: '--server <id>',    desc: 'Select server manually' },
  { cmd: '--duration <s>',   desc: 'Test duration in seconds' },
  { cmd: '--streams <n>',    desc: 'Parallel streams count' },
  { cmd: '--csv',            desc: 'Export results to CSV' },
  { cmd: '--history',        desc: 'View previous test history' },
  { cmd: '--no-color',       desc: 'Disable colored output' },
]

// ─── Demo section ───
const demoBody  = ref<HTMLElement | null>(null)
let demoRunning = false

const BAR_CHARS = '▁▂▃▄▅▆▇█'

function randomBar(pct: number) {
  const len = 8, filled = Math.round((pct / 100) * len)
  return Array.from({ length: len }, (_, i) =>
    i < filled ? BAR_CHARS[7] : BAR_CHARS[Math.floor(Math.random() * 3)]
  ).join('')
}

function sleep(ms: number) { return new Promise(r => setTimeout(r, ms)) }

function appendLine(cls: string, text: string) {
  if (!demoBody.value) return
  const el = document.createElement('span')
  el.className = 'do-line ' + cls
  el.textContent = text
  demoBody.value.appendChild(el)
  demoBody.value.appendChild(document.createElement('br'))
  demoBody.value.scrollTop = demoBody.value.scrollHeight
}

function appendBlank() {
  if (!demoBody.value) return
  const el = document.createElement('span'); el.className = 'do-blank'
  demoBody.value.appendChild(el)
}

async function runLiveDemo() {
  if (demoRunning) return
  demoRunning = true
  if (demoBody.value) demoBody.value.innerHTML = ''

  const intro = [
    { cls: 'prompt',  text: '$ speedtest --live' },
    { cls: 'dim',     text: '' },
    { cls: 'dim',     text: 'Initializing async runtime...' },
    { cls: 'dim',     text: 'Obtaining IP geolocation...' },
    { cls: 'success', text: '✓ Location: Rio de Janeiro, BR' },
    { cls: 'dim',     text: 'Discovering available servers...' },
    { cls: 'dim',     text: '  Candidates: São Paulo (12ms), Campinas (18ms), RJ (8ms)' },
    { cls: 'success', text: '✓ Selected server: Rio de Janeiro — 8 ms' },
    { cls: '',        text: '' },
  ]
  for (const l of intro) { await sleep(80); appendLine(l.cls, l.text) }

  await sleep(200)
  appendLine('header', '── Ping ──────────────────────────────')
  await sleep(300); appendLine('dim', '  Sending 10 ICMP packets...')
  await sleep(700); appendLine('speed', '  Min: 7ms  Max: 11ms  Avg: 8ms  Jitter: 1.1ms')
  appendBlank(); await sleep(300)

  appendLine('header', '── Download ──────────────────────────')
  await sleep(300); appendLine('dim', '  Starting 8 parallel streams...')
  for (const sp of [84, 156, 234, 312, 380, 418, 434, 441, 423]) {
    await sleep(220)
    appendLine('speed', `  ${String(sp).padStart(3)} Mbps  ${randomBar((sp / 500) * 100)}`)
  }
  await sleep(300); appendLine('success', '✓ Download: 423 Mbps'); appendBlank()

  appendLine('header', '── Upload ────────────────────────────')
  await sleep(300); appendLine('dim', '  Starting 4 parallel streams...')
  for (const sp of [28, 62, 89, 107, 114, 118, 116]) {
    await sleep(240)
    appendLine('speed', `  ${String(sp).padStart(3)} Mbps  ${randomBar((sp / 200) * 100)}`)
  }
  await sleep(300); appendLine('success', '✓ Upload: 118 Mbps'); appendBlank()

  appendLine('header', '── Result ────────────────────────────')
  await sleep(300)
  appendLine('speed', '  Download : 423 Mbps')
  appendLine('speed', '  Upload   : 118 Mbps')
  appendLine('speed', '  Ping     :   8 ms')
  appendLine('speed', '  Jitter   : 1.1 ms')
  appendBlank()
  appendLine('success', '✓ Result saved to local history.')
  demoRunning = false
}

async function runJsonDemo() {
  if (demoRunning) return
  demoRunning = true
  if (demoBody.value) demoBody.value.innerHTML = ''

  appendLine('prompt', '$ speedtest --json')
  await sleep(800); appendLine('dim', '# running test...')
  await sleep(1800)

  const jsonLines = [
    '{',
    '  "server": {',
    '    "id": "br-rj-01",',
    '    "name": "Rio de Janeiro, BR",',
    '    "latency_ms": 8',
    '  },',
    '  "ping_ms": 8,',
    '  "jitter_ms": 1.1,',
    '  "download_mbps": 423,',
    '  "upload_mbps": 118,',
    '  "streams": 8,',
    '  "timestamp": "2026-04-04T12:00:00Z"',
    '}',
  ]

  function escHtml(s: string) {
    return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')
  }

  for (const line of jsonLines) {
    await sleep(80)
    if (!demoBody.value) continue
    const span = document.createElement('span')
    span.className = 'do-line json'
    if (line.includes('":')) {
      const idx = line.indexOf('":')
      span.innerHTML =
        `<span style="color:var(--blue)">${escHtml(line.slice(0, idx + 2))}</span>` +
        `<span style="color:var(--yellow)">${escHtml(line.slice(idx + 2))}</span>`
    } else { span.textContent = line }
    demoBody.value.appendChild(span)
    demoBody.value.appendChild(document.createElement('br'))
    demoBody.value.scrollTop = demoBody.value.scrollHeight
  }
  demoRunning = false
}

// ─── Scroll reveal ───
let revealObserver: IntersectionObserver | null = null

// ─── Lifecycle ───
onMounted(() => {
  // Nav scroll
  const onScroll = () => { isScrolled.value = window.scrollY > 40 }
  window.addEventListener('scroll', onScroll)

  // Hero animation
  setTimeout(runHeroTick, 1200)

  // Scroll reveal
  revealObserver = new IntersectionObserver((entries) => {
    entries.forEach((entry, i) => {
      if (entry.isIntersecting) {
        setTimeout(() => {
          (entry.target as HTMLElement).classList.add('visible')
        }, (i % 4) * 80)
        revealObserver!.unobserve(entry.target)
      }
    })
  }, { threshold: 0.1 })

  document.querySelectorAll('.reveal').forEach(el => revealObserver!.observe(el))

  // Auto-start demo
  const demoObs = new IntersectionObserver((entries) => {
    entries.forEach(e => {
      if (e.isIntersecting) { runLiveDemo(); demoObs.unobserve(e.target) }
    })
  }, { threshold: 0.2 })
  if (demoBody.value) demoObs.observe(demoBody.value)

  onUnmounted(() => {
    window.removeEventListener('scroll', onScroll)
    if (heroTimer) clearTimeout(heroTimer)
    revealObserver?.disconnect()
  })
})
</script>

<style scoped>
.site-root { min-height: 100vh; }

/* ─── Scanlines ─── */
.scanlines {
  pointer-events: none;
  position: fixed; inset: 0; z-index: 9999;
  background: repeating-linear-gradient(
    to bottom,
    transparent, transparent 2px,
    rgba(0,0,0,.03) 2px, rgba(0,0,0,.03) 4px
  );
}

/* ─── Nav ─── */
nav {
  position: fixed; top: 0; left: 0; right: 0;
  z-index: 200; height: var(--nav-h);
  background: rgba(13,13,13,.92);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border);
  transition: border-color .3s;
}
nav.scrolled { border-bottom-color: var(--border2); }

.nav-container {
  max-width: 1200px; margin: 0 auto; padding: 0 24px;
  height: 100%; display: flex; align-items: center; justify-content: space-between;
}

.nav-logo {
  display: flex; align-items: center; gap: 4px;
  font-size: 1.1rem; font-weight: 700; color: var(--text);
}
.logo-prompt { color: var(--orange); }
.logo-cursor { color: var(--orange); animation: blink 1.2s step-end infinite; }

.nav-links { display: flex; align-items: center; gap: 28px; }
.nav-links a { color: var(--muted2); font-size: .85rem; transition: color .2s; }
.nav-links a:hover { color: var(--text); }

.btn-gh {
  display: flex; align-items: center; gap: 6px;
  background: var(--card); border: 1px solid var(--border2);
  padding: 7px 14px; border-radius: var(--radius);
  color: var(--text) !important; font-size: .82rem;
  transition: background .2s, border-color .2s !important;
}
.btn-gh:hover { background: var(--bg3); border-color: #3a3a3a; color: var(--text) !important; }

/* ─── Hamburger ─── */
.hamburger {
  display: none; flex-direction: column; gap: 5px;
  background: transparent; border: none; cursor: pointer; padding: 8px;
}
.hamburger span { width: 22px; height: 2px; background: var(--orange); border-radius: 2px; transition: all .3s; display: block; }
.hamburger.active span:nth-child(1) { transform: rotate(45deg) translate(5px,5px); }
.hamburger.active span:nth-child(2) { opacity: 0; }
.hamburger.active span:nth-child(3) { transform: rotate(-45deg) translate(5px,-5px); }

.mobile-menu {
  display: none; position: fixed; top: var(--nav-h); left: 0; right: 0;
  background: var(--bg2); border-bottom: 1px solid var(--border);
  max-height: 0; overflow: hidden; transition: max-height .35s ease; z-index: 199;
}
.mobile-menu.open { max-height: 360px; }
.mobile-menu-content { display: flex; flex-direction: column; padding: 12px 16px; }
.mobile-link { padding: 13px 12px; color: var(--text); font-size: .9rem; border-radius: var(--radius); transition: background .2s; }
.mobile-link:hover { background: var(--card); color: var(--orange); }

/* ─── Hero ─── */
.hero {
  min-height: 100vh; padding: calc(var(--nav-h) + 60px) 24px 80px;
  display: flex; flex-direction: column; align-items: center; text-align: center;
  position: relative; overflow: hidden;
}

.hero-bg-grid {
  position: absolute; inset: 0;
  background-image:
    linear-gradient(var(--border) 1px, transparent 1px),
    linear-gradient(90deg, var(--border) 1px, transparent 1px);
  background-size: 40px 40px; opacity: .35;
  mask-image: radial-gradient(ellipse 80% 60% at 50% 0%, black 30%, transparent 80%);
}

.hero-glow {
  position: absolute; top: 20%; left: 50%; transform: translate(-50%,-50%);
  width: 600px; height: 300px;
  background: radial-gradient(ellipse, var(--orange-glow) 0%, transparent 70%);
  pointer-events: none; filter: blur(40px);
}

.hero-content { position: relative; z-index: 2; max-width: 780px; animation: fadeUp .7s ease both; }

.hero-badge {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--orange-dim); border: 1px solid rgba(249,115,22,.25);
  color: var(--orange); font-size: .8rem; padding: 5px 14px;
  border-radius: 999px; margin-bottom: 28px; letter-spacing: .04em;
}
.badge-dot {
  width: 7px; height: 7px; border-radius: 50%;
  background: var(--orange); box-shadow: 0 0 8px var(--orange);
  animation: pulse 2s ease-in-out infinite;
}

.hero-title {
  font-size: clamp(3rem, 10vw, 6.5rem); font-weight: 800;
  line-height: 1; letter-spacing: -.03em; margin-bottom: 20px;
  color: var(--text);
}

.hero-sub {
  color: var(--muted2); font-size: clamp(.95rem, 2.5vw, 1.15rem);
  line-height: 1.7; margin-bottom: 28px;
}

.hero-pills { display: flex; flex-wrap: wrap; justify-content: center; gap: 10px; margin-bottom: 36px; }
.pill {
  background: var(--card); border: 1px solid var(--border2);
  color: var(--muted2); padding: 6px 14px; border-radius: var(--radius);
  font-size: .8rem; letter-spacing: .02em;
}

.hero-install {
  display: flex; flex-wrap: wrap; align-items: center; justify-content: center; gap: 14px;
}

.install-cmd-wrap {
  display: inline-flex; align-items: center; gap: 10px;
  background: var(--card); border: 1px solid var(--border2);
  border-radius: var(--radius); padding: 10px 16px;
  cursor: pointer; transition: border-color .2s, background .2s; position: relative;
  max-width: 600px;
}
.install-cmd-wrap:hover  { border-color: var(--orange); background: var(--orange-dim); }
.install-cmd-wrap.copied { opacity: .85; }
.install-cmd-wrap code   { background: transparent; padding: 0; color: var(--text); font-size: .82rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

.cmd-prefix { color: var(--orange); font-weight: 700; font-size: .9rem; flex-shrink: 0; }

.copy-toast {
  position: absolute; top: -36px; left: 50%; transform: translateX(-50%);
  background: var(--orange); color: var(--bg); font-size: .78rem; font-weight: 700;
  padding: 5px 12px; border-radius: var(--radius);
  opacity: 0; pointer-events: none; transition: opacity .25s; white-space: nowrap;
}
.copy-toast.show { opacity: 1; }

.hero-more-link { color: var(--muted2); font-size: .88rem; transition: color .2s; }
.hero-more-link:hover { color: var(--orange); }

/* ─── Hero Terminal ─── */
.hero-terminal {
  position: relative; z-index: 2; width: 100%; max-width: 620px; margin-top: 52px;
  border-radius: 10px; overflow: hidden;
  box-shadow: 0 0 0 1px var(--border), 0 20px 60px rgba(0,0,0,.6), 0 0 80px var(--orange-dim);
  animation: fadeUp .9s .2s ease both;
}

.terminal-chrome {
  height: 38px; background: var(--bg2); border-bottom: 1px solid var(--border);
  display: flex; align-items: center; padding: 0 14px; gap: 10px;
}
.chrome-dots { display: flex; gap: 7px; }
.chrome-dots span { width: 12px; height: 12px; border-radius: 50%; }
.chrome-dots span:nth-child(1) { background: #ff5f57; }
.chrome-dots span:nth-child(2) { background: #febc2e; }
.chrome-dots span:nth-child(3) { background: #28c840; }
.chrome-title { color: var(--muted); font-size: .8rem; flex: 1; text-align: center; }

.terminal-body { background: #0a0a0a; padding: 20px 20px 24px; font-size: .82rem; line-height: 1.6; }

.term-line       { display: block; white-space: pre; }
.term-line.dim   { color: var(--muted); }
.term-line.accent{ color: var(--green); }
.term-blank      { height: 10px; }

.term-speed-row  { display: flex; align-items: center; gap: 12px; margin-bottom: 6px; }
.speed-label     { color: var(--muted2); min-width: 68px; }
.speed-val       { color: var(--orange); font-weight: 700; min-width: 90px; }

.term-graph-wrap { margin-top: 14px; border-top: 1px solid var(--border); padding-top: 12px; }
.graph-label     { color: var(--muted); font-size: .75rem; margin-bottom: 6px; }
#term-graph      { display: block; width: 100%; }

/* ─── Sections ─── */
.section-container { max-width: 1140px; margin: 0 auto; padding: 0 24px; }
.section-header    { text-align: center; margin-bottom: 56px; }
.section-header h2 { font-size: clamp(1.6rem, 4vw, 2.4rem); font-weight: 800; color: var(--text); margin-bottom: 12px; }
.section-header p  { color: var(--muted2); font-size: .95rem; }

/* ─── Features ─── */
.features { padding: 100px 0; }

.feature-row {
  display: grid; grid-template-columns: 1fr 1fr; gap: 64px;
  align-items: center; padding: 64px 0; border-bottom: 1px solid var(--border);
}
.feature-row:last-child { border-bottom: none; }
.feature-row.reverse    { direction: rtl; }
.feature-row.reverse > * { direction: ltr; }

.feat-tag {
  font-size: .72rem; font-weight: 700; text-transform: uppercase;
  letter-spacing: .12em; color: var(--orange); margin-bottom: 14px;
  display: flex; align-items: center; gap: 8px;
}
.feat-tag::before { content: ''; width: 20px; height: 2px; background: var(--orange); border-radius: 1px; }

.feature-text h2 {
  font-size: clamp(1.4rem, 3vw, 2rem); font-weight: 800;
  line-height: 1.2; color: var(--text); margin-bottom: 16px;
}
.feature-text p { color: var(--muted2); font-size: .9rem; line-height: 1.8; margin-bottom: 22px; }

.feat-list { list-style: none; }
.feat-list li {
  color: var(--muted2); font-size: .88rem; padding: 7px 0;
  display: flex; align-items: center; gap: 10px;
  border-bottom: 1px solid var(--border);
}
.feat-list li:last-child { border-bottom: none; }
.feat-list li::before { content: '▸'; color: var(--orange); flex-shrink: 0; }

/* ─── Feature Visual ─── */
.feature-visual {
  background: var(--card); border: 1px solid var(--border);
  border-radius: 8px; padding-top: 28px; position: relative; overflow: hidden;
}
.feature-visual::before {
  content: ''; position: absolute; top: 0; left: 0; right: 0;
  height: 28px; background: var(--bg2); border-bottom: 1px solid var(--border);
}
.fv-dots { position: absolute; top: 9px; left: 10px; display: flex; gap: 5px; z-index: 1; }
.fv-dots span { width: 9px; height: 9px; border-radius: 50%; }
.fv-dots span:nth-child(1) { background: #ff5f57; }
.fv-dots span:nth-child(2) { background: #febc2e; }
.fv-dots span:nth-child(3) { background: #28c840; }

.fv-code { padding: 20px 22px 22px; font-size: .8rem; line-height: 1.75; }

/* These classes are used inside v-html */
:deep(.fc-line)  { display: block; white-space: pre; color: var(--muted2); }
:deep(.fc-blank) { height: 8px; display: block; }
:deep(.fc-indent)  { padding-left: 16px; }
:deep(.fc-indent2) { padding-left: 32px; }
:deep(.fc-c)  { color: #555; }
:deep(.fc-kw) { color: #c084fc; }
:deep(.fc-fn) { color: var(--blue); }
:deep(.fc-str){ color: var(--green); }
:deep(.fc-n)  { color: var(--yellow); }

/* ─── Demo ─── */
.demo-section {
  padding: 100px 0; background: var(--bg2);
  border-top: 1px solid var(--border); border-bottom: 1px solid var(--border);
}
.demo-terminal {
  max-width: 740px; margin: 0 auto; border-radius: 10px; overflow: hidden;
  box-shadow: 0 0 0 1px var(--border), 0 20px 60px rgba(0,0,0,.5);
}
.demo-body {
  background: #080808; padding: 20px 22px; min-height: 280px; max-height: 420px;
  overflow-y: auto; font-size: .82rem; line-height: 1.7; font-family: var(--mono);
}
.demo-controls {
  background: var(--bg2); border-top: 1px solid var(--border);
  padding: 12px 16px; display: flex; gap: 10px;
}
.demo-btn {
  background: var(--orange); border: none; color: var(--bg);
  font-family: var(--mono); font-size: .8rem; font-weight: 700;
  padding: 8px 18px; border-radius: var(--radius); cursor: pointer; transition: opacity .2s;
}
.demo-btn:hover { opacity: .85; }
.demo-btn.secondary {
  background: transparent; border: 1px solid var(--border2); color: var(--muted2);
}
.demo-btn.secondary:hover { border-color: var(--orange); color: var(--orange); opacity: 1; }

/* Demo output line styles (injected via JS, so can't be scoped) */
:deep(.do-line)         { display: block; color: var(--muted2); }
:deep(.do-line.prompt)  { color: var(--orange); }
:deep(.do-line.success) { color: var(--green); }
:deep(.do-line.header)  { color: var(--text); font-weight: 700; }
:deep(.do-line.speed)   { color: var(--orange); }
:deep(.do-line.json)    { color: var(--green); }
:deep(.do-line.dim)     { color: var(--muted); }
:deep(.do-blank)        { height: 8px; display: block; }

/* ─── Arch ─── */
.arch-section { padding: 100px 0; }
.arch-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(230px, 1fr)); gap: 16px; }
.arch-module {
  background: var(--card); border: 1px solid var(--border); border-radius: 8px;
  padding: 22px; transition: border-color .2s, transform .2s; cursor: default;
}
.arch-module:hover { border-color: var(--orange); transform: translateY(-3px); }
.arch-icon  { font-size: 1.6rem; margin-bottom: 10px; }
.arch-name  { color: var(--orange); font-weight: 700; font-size: .9rem; margin-bottom: 8px; }
.arch-desc  { color: var(--muted2); font-size: .8rem; line-height: 1.6; margin-bottom: 12px; }
.arch-stack {
  background: var(--bg); border: 1px solid var(--border);
  color: var(--muted); font-size: .72rem; padding: 4px 10px;
  border-radius: 4px; display: inline-block;
}

/* ─── Flags ─── */
.install-section { padding: 100px 0; background: var(--bg2); border-top: 1px solid var(--border); }
.flags-section   { max-width: 860px; margin: 0 auto; }
.flags-title     { color: var(--muted2); font-size: .82rem; text-transform: uppercase; letter-spacing: .1em; margin-bottom: 20px; text-align: center; }
.flags-grid      { display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 10px; }
.flag-item {
  display: flex; align-items: center; gap: 12px;
  background: var(--card); border: 1px solid var(--border); border-radius: var(--radius); padding: 12px 16px;
}
.flag-cmd  { background: transparent; color: var(--orange); font-size: .8rem; min-width: 120px; white-space: nowrap; }
.flag-desc { color: var(--muted2); font-size: .78rem; }

/* ─── Footer ─── */
footer { padding: 48px 24px; border-top: 1px solid var(--border); }
.footer-inner {
  max-width: 1140px; margin: 0 auto;
  display: flex; flex-direction: column; align-items: center; gap: 20px; text-align: center;
}
.footer-brand { display: flex; align-items: center; gap: 4px; font-size: 1rem; font-weight: 700; }
.footer-links { display: flex; flex-wrap: wrap; justify-content: center; gap: 20px 32px; }
.footer-links a { color: var(--muted); font-size: .85rem; transition: color .2s; }
.footer-links a:hover { color: var(--orange); }
.footer-credit { color: var(--muted); font-size: .8rem; }

/* ─── Reveal animation ─── */
.reveal { opacity: 0; transform: translateY(20px); transition: opacity .6s ease, transform .6s ease; }
.reveal.visible { opacity: 1; transform: none; }

/* ─── Responsive ─── */
@media (max-width: 900px) {
  .feature-row { grid-template-columns: 1fr; gap: 36px; }
  .feature-row.reverse { direction: ltr; }
}
@media (max-width: 768px) {
  .nav-links    { display: none; }
  .hamburger    { display: flex; }
  .mobile-menu  { display: block; }
  .hero         { padding: calc(var(--nav-h) + 32px) 16px 48px; }
  .hero-title   { font-size: 2.8rem; }
  .hero-install { flex-direction: column; width: 100%; }
  .install-cmd-wrap { width: 100%; justify-content: center; }
  .hero-terminal { margin-top: 36px; }
  .features, .arch-section, .install-section, .demo-section { padding: 64px 0; }
  .feature-row  { padding: 48px 0; }
  .arch-grid    { grid-template-columns: 1fr 1fr; }
}
@media (max-width: 480px) {
  .arch-grid   { grid-template-columns: 1fr; }
  .flags-grid  { grid-template-columns: 1fr; }
}

.btn-gh img {
  width: 16px;
  height: 16px;
  object-fit: contain;
  background: white;
  padding: 2px;
  border-radius: 3px;
  flex-shrink: 0;
}
</style>