/* ─────────────────────────────────────────
   speedtest-rs — script.js
───────────────────────────────────────── */

'use strict';

// ─── Nav scroll ───────────────────────────
const navbar = document.getElementById('navbar');
window.addEventListener('scroll', () => {
    navbar.classList.toggle('scrolled', window.scrollY > 40);
});

// ─── Hamburger ────────────────────────────
const hamburger = document.getElementById('hamburger');
const mobileMenu = document.getElementById('mobile-menu');


document.querySelectorAll('.mobile-link').forEach(link => {
    link.addEventListener('click', () => {
        hamburger.classList.remove('active');
        mobileMenu.classList.remove('open');
    });
});

// ─── Copy to clipboard ────────────────────
function setupCopyButtons() {
    document.querySelectorAll('.copyable').forEach(el => {
        el.addEventListener('click', (e) => {
            const text = el.dataset.copy || el.querySelector('code')?.textContent;
            if (!text) return;
            navigator.clipboard.writeText(text.trim()).then(() => {
                const toast = el.querySelector('.copy-toast');
                if (toast) {
                    toast.classList.add('show');
                    setTimeout(() => toast.classList.remove('show'), 1600);
                }
            });
        });

        // Prevent btn from double-firing
        const btn = el.querySelector('.copy-icon-btn');
        if (btn) {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                const text = el.dataset.copy || el.querySelector('code')?.textContent;
                if (!text) return;
                navigator.clipboard.writeText(text.trim()).then(() => {
                    const toast = el.querySelector('.copy-toast');
                    if (toast) {
                        toast.classList.add('show');
                        setTimeout(() => toast.classList.remove('show'), 1600);
                    }
                });
            });
        }
    });
}

setupCopyButtons();

// ─── Reveal on scroll ─────────────────────
const revealEls = document.querySelectorAll(
    '.feature-row, .arch-module, .install-card, .flag-item'
);

revealEls.forEach(el => el.classList.add('reveal'));

const revealObserver = new IntersectionObserver((entries) => {
    entries.forEach((entry, i) => {
        if (entry.isIntersecting) {
            setTimeout(() => {
                entry.target.classList.add('visible');
            }, (i % 4) * 80);
            revealObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.1 });

revealEls.forEach(el => revealObserver.observe(el));

// ─── Hero Terminal Animation ───────────────
const BAR_CHARS = '▁▂▃▄▅▆▇█';

function randomBar(percent) {
    const len = 8;
    const filled = Math.round((percent / 100) * len);
    let bar = '';
    for (let i = 0; i < len; i++) {
        const charIdx = i < filled ? 7 : Math.floor(Math.random() * 3);
        bar += BAR_CHARS[charIdx];
    }
    return bar;
}

function lerp(a, b, t) { return a + (b - a) * t; }

// Canvas graph
const canvas = document.getElementById('term-graph');
const ctx = canvas ? canvas.getContext('2d') : null;

const GRAPH_HISTORY = 30;
const graphData = Array(GRAPH_HISTORY).fill(0);

function drawGraph(data) {
    if (!ctx) return;
    const W = canvas.width;
    const H = canvas.height;
    ctx.clearRect(0, 0, W, H);

    const max = Math.max(...data, 100);
    const padding = { left: 0, right: 4, top: 6, bottom: 0 };
    const gW = W - padding.left - padding.right;
    const gH = H - padding.top - padding.bottom;

    // Grid lines
    ctx.strokeStyle = 'rgba(249,115,22,0.08)';
    ctx.lineWidth = 1;
    for (let i = 1; i < 4; i++) {
        const y = padding.top + (gH / 4) * i;
        ctx.beginPath();
        ctx.moveTo(padding.left, y);
        ctx.lineTo(W - padding.right, y);
        ctx.stroke();
    }

    // Fill gradient
    const grad = ctx.createLinearGradient(0, padding.top, 0, H);
    grad.addColorStop(0, 'rgba(249,115,22,0.25)');
    grad.addColorStop(1, 'rgba(249,115,22,0.02)');

    ctx.beginPath();
    data.forEach((val, i) => {
        const x = padding.left + (i / (data.length - 1)) * gW;
        const y = padding.top + gH - (val / max) * gH;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
    });
    ctx.lineTo(W - padding.right, H);
    ctx.lineTo(padding.left, H);
    ctx.closePath();
    ctx.fillStyle = grad;
    ctx.fill();

    // Line
    ctx.beginPath();
    ctx.strokeStyle = '#f97316';
    ctx.lineWidth = 1.5;
    ctx.lineJoin = 'round';
    data.forEach((val, i) => {
        const x = padding.left + (i / (data.length - 1)) * gW;
        const y = padding.top + gH - (val / max) * gH;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
    });
    ctx.stroke();
}

// Hero live numbers
let dlTarget = 0, dlCurrent = 0;
let ulTarget = 0, ulCurrent = 0;
let phase = 'idle'; // idle → ramp-up → sustain → done
let tick = 0;

function heroTick() {
    tick++;

    if (tick < 10) {
        phase = 'ramp-up';
        dlTarget = lerp(0, 450, tick / 10);
        ulTarget = lerp(0, 130, tick / 10);
    } else if (tick < 40) {
        phase = 'sustain';
        dlTarget = 420 + Math.sin(tick * .3) * 40 + (Math.random() - .5) * 30;
        ulTarget = 110 + Math.sin(tick * .4) * 20 + (Math.random() - .5) * 15;
    } else {
        phase = 'done';
    }

    dlCurrent = lerp(dlCurrent, dlTarget, 0.2);
    ulCurrent = lerp(ulCurrent, ulTarget, 0.2);

    // Update DOM
    const dlEl = document.getElementById('dl-val');
    const ulEl = document.getElementById('ul-val');
    const pingEl = document.getElementById('ping-val');
    const dlBarEl = document.getElementById('dl-bar');
    const ulBarEl = document.getElementById('ul-bar');

    if (dlEl) dlEl.textContent = dlCurrent.toFixed(0) + ' Mbps';
    if (ulEl) ulEl.textContent = ulCurrent.toFixed(0) + ' Mbps';
    if (pingEl && tick > 2) pingEl.textContent = (8 + Math.floor(Math.random() * 3)) + ' ms';
    if (dlBarEl) dlBarEl.textContent = randomBar((dlCurrent / 500) * 100);
    if (ulBarEl) ulBarEl.textContent = randomBar((ulCurrent / 200) * 100);

    // Graph
    graphData.shift();
    graphData.push(dlCurrent);
    drawGraph(graphData);

    if (phase !== 'done') {
        setTimeout(heroTick, 200);
    } else {
        // Reset after pause
        setTimeout(() => {
            tick = 0; dlCurrent = 0; ulCurrent = 0; dlTarget = 0; ulTarget = 0;
            graphData.fill(0);
            heroTick();
        }, 3000);
    }
}

// Start after short delay
setTimeout(heroTick, 1200);

// ─── Animated ASCII chart (features section) ──
const asciiEl = document.getElementById('ascii-chart');
const gdFill = document.getElementById('gd-fill');
const gdUploadFill = document.getElementById('gd-upload-fill');
const gdMbps = document.getElementById('gd-mbps');
const gdUpload = document.getElementById('gd-upload');

const ASCII_FRAMES = [
`500 ┤
450 ┤
400 ┤────╮
350 ┤    ╰─
    └────────────────`,
`500 ┤
450 ┤      ╭──
400 ┤────╮╭╯
350 ┤    ╰╯
    └────────────────`,
`500 ┤         ╭──╮
450 ┤      ╭──╯  ╰
400 ┤────╮╭╯
350 ┤    ╰╯
    └────────────────`,
`500 ┤         ╭──╮
450 ┤      ╭──╯  ╰─╮
400 ┤────╮╭╯       ╰──
350 ┤    ╰╯
    └────────────────`,
];

let asciiFrame = 0;
let asciiVisible = false;

function animateAscii() {
    if (!asciiEl) return;
    asciiEl.textContent = ASCII_FRAMES[asciiFrame % ASCII_FRAMES.length];
    asciiFrame++;

    const dlPct = 65 + Math.sin(asciiFrame * .5) * 15;
    const ulPct = 35 + Math.sin(asciiFrame * .4) * 10;
    const dlVal = Math.round(350 + Math.sin(asciiFrame * .5) * 80);
    const ulVal = Math.round(110 + Math.sin(asciiFrame * .4) * 15);

    if (gdFill) gdFill.style.width = dlPct + '%';
    if (gdUploadFill) gdUploadFill.style.width = ulPct + '%';
    if (gdMbps) gdMbps.textContent = dlVal + ' Mbps';
    if (gdUpload) gdUpload.textContent = ulVal + ' Mbps';
}

// Trigger when visible
const asciiObserver = new IntersectionObserver(entries => {
    entries.forEach(entry => {
        if (entry.isIntersecting && !asciiVisible) {
            asciiVisible = true;
            setInterval(animateAscii, 900);
        }
    });
}, { threshold: .3 });

if (asciiEl) asciiObserver.observe(asciiEl);

// ─── Demo Section ─────────────────────────
const demoBody = document.getElementById('demo-body');
const demoRestartBtn = document.getElementById('demo-restart');
const demoJsonBtn = document.getElementById('demo-json');

let demoRunning = false;
let demoTimeout = null;

function clearDemo() {
    if (demoBody) demoBody.innerHTML = '';
    if (demoTimeout) clearTimeout(demoTimeout);
}

function appendLine(cls, text) {
    if (!demoBody) return;
    const el = document.createElement('span');
    el.className = 'do-line ' + cls;
    el.textContent = text;
    demoBody.appendChild(el);
    demoBody.appendChild(document.createElement('br'));
}

function appendBlank() {
    if (!demoBody) return;
    const el = document.createElement('span');
    el.className = 'do-blank';
    demoBody.appendChild(el);
}

function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }

async function runLiveDemo() {
    if (demoRunning) return;
    demoRunning = true;
    clearDemo();

    const lines = [
        { delay: 0,    cls: 'prompt', text: '$ speedtest-rs --live' },
        { delay: 400,  cls: 'dim',    text: '' },
        { delay: 500,  cls: 'dim',    text: 'Inicializando runtime assíncrono...' },
        { delay: 900,  cls: 'dim',    text: 'Obtendo localização por IP...' },
        { delay: 1400, cls: 'success',text: '✓ Localização: Rio de Janeiro, BR' },
        { delay: 1600, cls: 'dim',    text: 'Descobrindo servidores disponíveis...' },
        { delay: 2200, cls: 'dim',    text: '  Candidatos: São Paulo (12ms), Campinas (18ms), RJ (8ms)' },
        { delay: 2600, cls: 'success',text: '✓ Servidor selecionado: Rio de Janeiro — 8 ms' },
        { delay: 2800, cls: '', text: '' },
    ];

    for (const l of lines) {
        await sleep(l.delay === 0 ? 0 : 80);
        appendLine(l.cls, l.text);
    }

    await sleep(200);

    // Ping test
    appendLine('header', '── Ping ──────────────────────────────');
    await sleep(300);
    appendLine('dim', '  Enviando 10 pacotes ICMP...');
    await sleep(700);
    appendLine('speed', '  Mínimo: 7ms  Máximo: 11ms  Médio: 8ms  Jitter: 1.1ms');
    appendBlank();

    await sleep(300);

    // Download test
    appendLine('header', '── Download ──────────────────────────');
    await sleep(300);
    appendLine('dim', '  Iniciando 8 streams paralelos...');

    const dlSpeeds = [84, 156, 234, 312, 380, 418, 434, 441, 423];
    for (let i = 0; i < dlSpeeds.length; i++) {
        await sleep(220);
        const bar = randomBar((dlSpeeds[i] / 500) * 100);
        appendLine('speed', `  ${dlSpeeds[i].toString().padStart(3)} Mbps  ${bar}`);
    }

    await sleep(300);
    appendLine('success', '✓ Download: 423 Mbps');
    appendBlank();

    // Upload test
    appendLine('header', '── Upload ────────────────────────────');
    await sleep(300);
    appendLine('dim', '  Iniciando 4 streams paralelos...');

    const ulSpeeds = [28, 62, 89, 107, 114, 118, 116];
    for (let i = 0; i < ulSpeeds.length; i++) {
        await sleep(240);
        const bar = randomBar((ulSpeeds[i] / 200) * 100);
        appendLine('speed', `  ${ulSpeeds[i].toString().padStart(3)} Mbps  ${bar}`);
    }

    await sleep(300);
    appendLine('success', '✓ Upload: 118 Mbps');
    appendBlank();

    // Summary
    appendLine('header', '── Resultado ─────────────────────────');
    await sleep(300);
    appendLine('speed', '  Download : 423 Mbps');
    appendLine('speed', '  Upload   : 118 Mbps');
    appendLine('speed', '  Ping     :   8 ms');
    appendLine('speed', '  Jitter   : 1.1 ms');
    appendBlank();
    appendLine('success', '✓ Resultado salvo no histórico local.');

    demoRunning = false;
}

async function runJsonDemo() {
    if (demoRunning) return;
    demoRunning = true;
    clearDemo();

    appendLine('prompt', '$ speedtest-rs --json');
    await sleep(800);
    appendLine('dim', '# executando teste...');
    await sleep(1800);

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
    ];

    for (const line of jsonLines) {
        await sleep(80);
        const span = document.createElement('span');
        span.className = 'do-line json';

        // Colorize keys vs values
        if (line.includes('":')) {
            const colonIdx = line.indexOf('":');
            const key = line.substring(0, colonIdx + 2);
            const val = line.substring(colonIdx + 2);
            span.innerHTML =
                `<span style="color:var(--blue)">${escHtml(key)}</span>` +
                `<span style="color:var(--yellow)">${escHtml(val)}</span>`;
        } else {
            span.textContent = line;
        }

        demoBody.appendChild(span);
        demoBody.appendChild(document.createElement('br'));
    }

    demoRunning = false;
}

function escHtml(str) {
    return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

if (demoRestartBtn) demoRestartBtn.addEventListener('click', runLiveDemo);
if (demoJsonBtn) demoJsonBtn.addEventListener('click', runJsonDemo);

// Auto-start demo when visible
const demoObserver = new IntersectionObserver(entries => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            runLiveDemo();
            demoObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.2 });

if (demoBody) demoObserver.observe(demoBody);

// ─── Smooth scroll override ────────────────
document.querySelectorAll('a[href^="#"]').forEach(a => {
    a.addEventListener('click', e => {
        const target = document.querySelector(a.getAttribute('href'));
        if (target) {
            e.preventDefault();
            const offset = document.querySelector('nav')?.offsetHeight || 0;
            const y = target.getBoundingClientRect().top + window.scrollY - offset;
            window.scrollTo({ top: y, behavior: 'smooth' });
        }
    });
});

const heroCmd = document.getElementById('hero-cmd');

heroCmd.addEventListener('click', async () => {
    const text = heroCmd.getAttribute('data-copy');

    try {
        await navigator.clipboard.writeText(text);

        heroCmd.classList.add('copied');

        const original = heroCmd.querySelector('code').textContent;
        heroCmd.querySelector('code').textContent = 'Copied!';

        setTimeout(() => {
            heroCmd.querySelector('code').textContent = original;
            heroCmd.classList.remove('copied');
        }, 1200);

    } catch (err) {
        console.error('Copy failed:', err);
    }
});

