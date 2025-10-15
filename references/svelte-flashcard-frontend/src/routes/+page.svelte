<script>
	import { onMount } from 'svelte';

	const API_URL = 'http://localhost:8080/api';

	let stats = { mastered: 0, unmastered: 0, remaining: 0, total: 0 };
	let currentWord = null;
	let isLoading = true;
	let error = null;
	let showMeaning = false;

	// 新增状态
	let showHelp = false;
	let darkMode = false;
	let ttsEnabled = false;
	let newWord = '';
	let newMeaning = '';

	onMount(async () => {
		// 主题优先级：localStorage -> 系统偏好
		const savedTheme = localStorage.getItem('darkMode');
		if (savedTheme !== null) {
			darkMode = savedTheme === 'true';
		} else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
			darkMode = true;
		}
		updateTheme();

		await fetchStats();
		await fetchNextWord();
		isLoading = false;
	});

	function updateTheme() {
		if (darkMode) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
		document.documentElement.style.setProperty('color-scheme', darkMode ? 'dark' : 'light');
		localStorage.setItem('darkMode', String(darkMode));
	}

	async function fetchStats() {
		try {
			const res = await fetch(`${API_URL}/stats`);
			if (!res.ok) throw new Error('Could not fetch stats');
			stats = await res.json();
		} catch (e) {
			error = e.message;
		}
	}

	async function fetchNextWord() {
		showMeaning = false;
		try {
			const res = await fetch(`${API_URL}/word`);
			if (!res.ok) throw new Error('Could not fetch the next word');
			const data = await res.json();
			if (data.word) {
				currentWord = data;
			} else {
				currentWord = null;
			}
		} catch (e) {
			error = e.message;
		}
	}

	async function classifyWord(status) {
		if (!currentWord) return;
		try {
			const res = await fetch(`${API_URL}/classify`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ status })
			});
			if (!res.ok) {
				const errData = await res.json();
				throw new Error(errData.error || 'Classification failed');
			}
			await Promise.all([fetchStats(), fetchNextWord()]);
		} catch (e) {
			error = e.message;
		}
	}

	// 新增：跳过
	async function skipWord() {
		try {
			const res = await fetch(`${API_URL}/skip`, { method: 'POST' });
			if (!res.ok) throw new Error('Skip failed');
			await fetchNextWord();
		} catch (e) {
			error = e.message;
		}
	}

	// 新增：撤销
	async function undo() {
		try {
			const res = await fetch(`${API_URL}/undo`, { method: 'POST' });
			const data = await res.json();
			if (!res.ok) throw new Error(data.error || 'Undo failed');
			await Promise.all([fetchStats(), fetchNextWord()]);
		} catch (e) {
			error = e.message;
		}
	}

	// 新增：打乱
	async function shuffle() {
		try {
			const res = await fetch(`${API_URL}/shuffle`, { method: 'POST' });
			if (!res.ok) throw new Error('Shuffle failed');
			await fetchNextWord();
		} catch (e) {
			error = e.message;
		}
	}

	// 新增：添加新词
	async function addNewWord() {
		if (!newWord.trim() && !newMeaning.trim()) return;
		try {
			const res = await fetch(`${API_URL}/word`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ word: newWord.trim(), meaning: newMeaning.trim() })
			});
			const data = await res.json();
			if (!res.ok) throw new Error(data.error || 'Add word failed');
			newWord = '';
			newMeaning = '';
			await fetchStats();
			if (!currentWord) await fetchNextWord();
		} catch (e) {
			error = e.message;
		}
	}

	// 朗读
	function speak() {
		if (!ttsEnabled || !currentWord) return;
		try {
			const u = new SpeechSynthesisUtterance(currentWord.word);
			u.lang = 'en-US';
			window.speechSynthesis.cancel();
			window.speechSynthesis.speak(u);
		} catch {}
	}

	// 键盘快捷键
	function handleKey(event) {
		const key = event.key.toLowerCase();
		const target = event.target;
		if (target && (target.tagName === 'INPUT' || target.isContentEditable)) return;

		if (key === 'y' || key === 'arrowright') {
			classifyWord('mastered');
		} else if (key === 'n' || key === 'arrowleft') {
			classifyWord('unmastered');
		} else if (key === 'f' || key === ' ' || key === 'enter') {
			event.preventDefault();
			showMeaning = !showMeaning;
		} else if (key === 's') {
			skipWord();
		} else if (key === 'u') {
			undo();
		} else if (key === 'r') {
			shuffle();
		} else if (key === 'd') {
			darkMode = !darkMode; updateTheme();
		} else if (key === 'v') {
			speak();
		} else if (key === 'h') {
			showHelp = !showHelp;
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKey);
		return () => window.removeEventListener('keydown', handleKey);
	});
</script>

<main>
    <header class="topbar">
        <div class="left">
            <button class="link" on:click={() => (showHelp = !showHelp)} aria-expanded={showHelp} aria-controls="help-panel">❓ 快捷键</button>
        </div>
        <h1 class="brand">Vocabulary Flashcard</h1>
        <div class="right">
            <label class="switch" title="朗读">
                <input type="checkbox" bind:checked={ttsEnabled} aria-label="切换朗读" />
                <span class="slider">🔊</span>
            </label>
            <label class="switch" title="深色模式">
                <input type="checkbox" bind:checked={darkMode} on:change={updateTheme} aria-label="切换深色模式" />
                <span class="slider">🌙</span>
            </label>
        </div>
    </header>

    {#if showHelp}
    <div class="help" id="help-panel">
        <p>空格/回车：翻面</p>
        <p>→：已掌握，←：未掌握</p>
        <p>Y / N：已掌握 / 未掌握</p>
        <p>S：跳过，U：撤销，R：打乱</p>
        <p>D：切换深色，V：朗读，H：显示/隐藏帮助</p>
    </div>
    {/if}

	{#if isLoading}
		<p>正在加载词汇...</p>
	{:else if error}
		<p class="error">发生错误: {error}</p>
	{:else}
		<div class="progress-container">
			<div class="progress-bar">
				<div
					class="progress mastered"
					style="width: {stats.total ? (stats.mastered / stats.total) * 100 : 0}%"
					title="已掌握: {stats.mastered}"
				></div>
				<div
					class="progress unmastered"
					style="width: {stats.total ? (stats.unmastered / stats.total) * 100 : 0}%"
					title="未掌握: {stats.unmastered}"
				></div>
			</div>
			<div class="stats">
				<span>已掌握: {stats.mastered}</span>
				<span>未掌握: {stats.unmastered}</span>
				<span>剩余: {stats.remaining}</span>
				<span>总计: {stats.total}</span>
			</div>
		</div>

		<div class="card-area">
			{#if currentWord}
				<div class="card-container" on:click={() => (showMeaning = !showMeaning)}>
					<div class="card" class:flipped={showMeaning}>
						<div class="card-face front">
							<h2>{currentWord.word}</h2>
						</div>
						<div class="card-face back">
							<p>{currentWord.meaning || '（无释义）'}</p>
						</div>
					</div>
				</div>

				<div class="controls">
					<button class="btn-secondary" on:click={undo}>↶ 撤销 (U)</button>
					<button class="btn-secondary" on:click={skipWord}>⤼ 跳过 (S)</button>
					<button class="btn-secondary" on:click={shuffle}>🔀 打乱 (R)</button>
					<button class="btn-secondary" on:click={speak} disabled={!ttsEnabled}>🔊 朗读 (V)</button>
				</div>

				<div class="controls">
					<button class="btn-unmastered" on:click={() => classifyWord('unmastered')}>
						- 未掌握 (N / ←)
					</button>
					<button class="btn-mastered" on:click={() => classifyWord('mastered')}>
						✓ 已掌握 (Y / →)
					</button>
				</div>

				<div class="adder">
					<input placeholder="新单词" bind:value={newWord} />
					<input placeholder="释义（可选）" bind:value={newMeaning} />
					<button class="btn-primary" on:click={addNewWord}>添加</button>
				</div>
			{:else}
				<div class="congrats">
					<h2>🎉 恭喜!</h2>
					<p>您已完成所有词汇的学习！</p>
					<div class="controls">
						<button class="btn-secondary" on:click={shuffle}>🔀 打乱</button>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</main>

    <style>
        :root {
                --bg-1: #e6e8ef;
                --bg-2: #cfd4e6;
                --surface: #ffffff;
                --surface-hi: #f5f7fa;
                --ink: #1e2231;
                --ink-dim: #6b7280;
                --accent-1: #667eea;
                --accent-2: #764ba2;
                --radius-xl: 22px;
                --radius-lg: 16px;
                --shadow-outer: 0 18px 40px rgba(0,0,0,.15), 0 6px 18px rgba(0,0,0,.1);
                --shadow-inner: inset 0 2px 4px rgba(255,255,255,.5), inset 0 -3px 8px rgba(0,0,0,.1);
                --card-bg-start: #fafbff;
                --card-bg-end: #c9d3e2;
                --control-bg-start: #f0f2fa;
                --control-bg-end: #e2e6ef;
                --primary-color: var(--accent-1);
                --primary-600: #556cd6;
                --mastered-color: #22c55e;
                --unmastered-color: #f59e0b;
                --text-color: var(--ink);
                --muted-text: var(--ink-dim);
                --border-color: #d1d5db;
                --bg-color: linear-gradient(180deg, var(--bg-2), var(--bg-1));
                --bg-accent: radial-gradient(1200px 800px at 60% -10%, rgba(0,0,0,.02), transparent 60%);
        }

        /* 深色模式覆盖 */
        :root.dark {
                --bg-1: #1f2230;
                --bg-2: #2a2d3d;
                --surface: #34384b;
                --surface-hi: #3c4156;
                --ink: #e6e8ef;
                --ink-dim: #bcc1d6;
                --shadow-outer: 0 18px 40px rgba(0,0,0,.45), 0 6px 18px rgba(0,0,0,.35);
                --shadow-inner: inset 0 2px 4px rgba(255,255,255,.08), inset 0 -3px 8px rgba(0,0,0,.35);
                --card-bg-start: var(--surface);
                --card-bg-end: #2c3041;
                --control-bg-start: var(--surface-hi);
                --control-bg-end: var(--surface);
                --text-color: var(--ink);
                --muted-text: var(--ink-dim);
                --border-color: rgba(255,255,255,.06);
                --primary-color: var(--accent-1);
                --primary-600: #556cd6;
                --bg-color: linear-gradient(180deg, var(--bg-2), var(--bg-1));
                --bg-accent: radial-gradient(1200px 800px at 60% -10%, rgba(255,255,255,.05), transparent 60%);
        }

        * { box-sizing: border-box; }
        body, html {
                background: var(--bg-color);
                background-image: var(--bg-accent);
                transition: background-color .2s ease, color .2s ease;
        }

        main {
                font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
                max-width: 820px;
                margin: 2rem auto;
                padding: 0 1rem 2rem;
                text-align: center;
                color: var(--text-color);
        }

	.brand { color: var(--primary-color); font-size: 1.4rem; margin: 0; }
	.topbar {
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		align-items: center;
		margin-bottom: 1rem;
	}
	.topbar .left { justify-self: start; }
	.topbar .right { justify-self: end; display: flex; gap: .5rem; align-items: center; }

	.link {
		background: transparent;
		border: 1px solid transparent;
		color: var(--primary-color);
		cursor: pointer;
		padding: 6px 10px;
		border-radius: 8px;
	}
	.link:hover { background: rgba(90, 121, 255, 0.08); }

	/* 开关样式 */
	.switch { position: relative; width: 52px; height: 28px; display: inline-flex; align-items: center; }
	.switch input { display: none; }
	.switch .slider {
		position: relative; width: 100%; height: 100%; border-radius: 999px; line-height: 28px;
		background: var(--border-color); color: var(--muted-text); font-size: 14px;
		transition: background .2s ease;
		padding: 0 8px; text-align: right;
	}
	.switch input:checked + .slider { background: var(--primary-color); color: #fff; text-align: left; }
	.switch .slider::after {
		content: ''; position: absolute; top: 3px; left: 3px; width: 22px; height: 22px; border-radius: 50%;
		background: #fff; transition: transform .2s ease; box-shadow: 0 2px 6px rgba(0,0,0,.15);
	}
	.switch input:checked + .slider::after { transform: translateX(24px); }

	.help {
		background: var(--card-bg);
		border: 1px solid var(--border-color);
		padding: .75rem 1rem;
		border-radius: 12px;
		color: var(--text-color);
		text-align: left;
		margin-bottom: 1rem;
		box-shadow: 0 6px 18px rgba(0,0,0,.08);
	}

	.progress-container { margin-bottom: 1rem; }
	.progress-bar {
		display: flex;
		height: 16px;
		background-color: var(--border-color);
		border-radius: 999px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}
	.progress { transition: width 0.35s ease-in-out; }
	.progress.mastered { background-image: linear-gradient(90deg, var(--mastered-color), #16a34a); }
	.progress.unmastered { background-image: linear-gradient(90deg, var(--unmastered-color), #d97706); }

	.stats {
		display: grid; grid-template-columns: repeat(4, 1fr); gap: .5rem;
		font-size: 0.9em; color: var(--muted-text);
	}
	.stats span {
		background: rgba(127,127,127,0.08);
		border: 1px solid var(--border-color);
		padding: 6px 10px; border-radius: 10px;
	}

        .card-area { min-height: 260px; }

        .card-container {
                width: 100%; height: 240px; perspective: 1000px; margin-bottom: 1rem; cursor: pointer;
        }
        .card {
                width: 100%; height: 100%; position: relative; transition: transform 0.6s; transform-style: preserve-3d;
                border-radius: var(--radius-xl);
        }
        .card.flipped { transform: rotateY(180deg); }
        .card-face {
                position: absolute; inset: 0; backface-visibility: hidden; display: flex; justify-content: center; align-items: center;
                padding: 24px; border-radius: var(--radius-xl); border: 1px solid var(--border-color);
                box-shadow: var(--shadow-outer), var(--shadow-inner);
        }
        .card-face.front {
                font-size: 2rem; font-weight: 700;
                background: linear-gradient(145deg, var(--card-bg-start), var(--card-bg-end));
                color: var(--text-color);
        }
        .card-face.back {
                transform: rotateY(180deg); font-size: 1.1rem;
                background: linear-gradient(145deg, var(--accent-1), var(--accent-2));
                color: #fff;
        }

        .controls {
                display: flex; justify-content: center; gap: .75rem; margin-bottom: .75rem; flex-wrap: wrap;
                background: linear-gradient(145deg, var(--control-bg-start), var(--control-bg-end));
                border: 1px solid var(--border-color); border-radius: var(--radius-lg);
                box-shadow: var(--shadow-outer), var(--shadow-inner);
                padding: 14px 16px;
        }

        button {
                padding: 10px 18px; border-radius: 10px; font-size: 0.95em; font-weight: 600; color: #1e2231; cursor: pointer;
                background: linear-gradient(145deg, #f5f7fa, #c3cfe2);
                border: 1px solid rgba(0,0,0,.35);
                box-shadow: 0 8px 16px rgba(0,0,0,.15), inset 0 1px 2px rgba(255,255,255,.8);
                transition: filter .1s ease, transform .1s ease, box-shadow .1s ease;
        }
        button:hover { filter: brightness(1.05); }
        button:active { transform: translateY(3px); box-shadow: 0 4px 10px rgba(0,0,0,.35), inset 0 2px 4px rgba(0,0,0,.35); }

        .btn-primary { background: linear-gradient(145deg, var(--primary-color), var(--primary-600)); color: #fff; }
        .btn-mastered { background: linear-gradient(145deg, var(--mastered-color), #16a34a); color: #fff; }
        .btn-unmastered { background: linear-gradient(145deg, var(--unmastered-color), #d97706); color: #fff; }
        .btn-secondary { background: linear-gradient(145deg, #6c757d, #5a636b); color: #fff; }

	.adder { display: flex; gap: .5rem; justify-content: center; align-items: center; margin-top: .5rem; flex-wrap: wrap; }
	.adder input {
		padding: 10px 12px; border-radius: 10px; border: 1px solid var(--border-color); background: var(--card-bg); color: var(--text-color);
		outline: none; min-width: 200px;
	}
	.adder input:focus { box-shadow: 0 0 0 3px rgba(88,166,255,.25); border-color: var(--primary-color); }

	.congrats { padding: 2.5rem 0; }
	.error { color: #ef4444; }
    </style>
