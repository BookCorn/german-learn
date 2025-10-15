<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';
	import { onMount } from 'svelte';

	type PartOption = {
		value: 'all' | 'noun' | 'verb' | 'adjective_adverb';
		label: string;
	};

	type StatusOption = {
		value: 'all' | 'new' | 'learning' | 'mastered';
		label: string;
	};

	type Flashcard = {
		entry_id: number;
		word: string;
		part_of_speech: string;
		meaning: string | null;
		english: string | null;
		examples: string | null;
		themes: string | null;
		status: string | null;
		times_seen: number;
		times_mastered: number;
		last_seen_at: string | null;
	};

	type Stats = {
		total: number;
		mastered: number;
		learning: number;
		new: number;
		per_part_of_speech: Array<{
			part_of_speech: string;
			total: number;
			mastered: number;
			learning: number;
			new: number;
		}>;
	};

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const API_BASE = '/api/v1/flashcards';

	const partOptions: PartOption[] = [
		{ value: 'all', label: '全部' },
		{ value: 'noun', label: '名词' },
		{ value: 'verb', label: '动词' },
		{ value: 'adjective_adverb', label: '形容 / 副词' }
	];

	const statusOptions: StatusOption[] = [
		{ value: 'all', label: '不限' },
		{ value: 'new', label: '未学习' },
		{ value: 'learning', label: '学习中' },
		{ value: 'mastered', label: '已掌握' }
	];

	let stats: Stats | null = null;
	let card: Flashcard | null = null;
	let loadingStats = false;
	let loadingCard = false;
	let submitting = false;
	let showBack = false;
	let errorMessage = '';
	let selectedPart: PartOption['value'] = 'all';
	let selectedStatus: StatusOption['value'] = 'all';

	let themes: string[] = [];
	let examples: string[] = [];
	let partMeta: ReturnType<typeof getPartMeta> | null = null;

	onMount(async () => {
		await Promise.all([loadStats(), loadCard()]);
	});

	async function loadStats() {
		loadingStats = true;
		try {
			const res = await fetch(`${API_BASE}/stats`);
			if (!res.ok) throw new Error('无法获取统计信息');
			stats = await res.json();
		} catch (error) {
			handleError(error);
		} finally {
			loadingStats = false;
		}
	}

	async function loadCard() {
		loadingCard = true;
		showBack = false;
		try {
			const query = buildQuery();
			const res = await fetch(`${API_BASE}/next${query ? `?${query}` : ''}`);
			if (!res.ok) throw new Error('无法获取下一张卡片');
			const data = (await res.json()) as Flashcard | null;
			card = data ?? null;
			updateDerivedValues();
		} catch (error) {
			handleError(error);
		} finally {
			loadingCard = false;
		}
	}

	function updateDerivedValues() {
		themes = parseThemes(card?.themes);
		examples = parseExamples(card?.examples);
		partMeta = card ? getPartMeta(card.part_of_speech) : null;
	}

	function buildQuery() {
		const params = new URLSearchParams();
		if (selectedPart !== 'all') params.set('part_of_speech', selectedPart);
		if (selectedStatus !== 'all') params.set('status', selectedStatus);
		return params.toString();
	}

	function toggleFace() {
		if (!card) return;
		showBack = !showBack;
	}

	function selectPart(value: PartOption['value']) {
		if (selectedPart === value) return;
		selectedPart = value;
		void loadCard();
	}

	function selectStatus(value: StatusOption['value']) {
		if (selectedStatus === value) return;
		selectedStatus = value;
		void loadCard();
	}

	async function review(result: 'mastered' | 'learning') {
		if (!card) return;
		submitting = true;
		try {
			const res = await fetch(`${API_BASE}/${card.entry_id}/review`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ result })
			});
			if (!res.ok) {
				const payload = await res.json().catch(() => ({}));
				throw new Error(payload.error ?? '提交复习结果失败');
			}
			await loadStats();
			await loadCard();
		} catch (error) {
			handleError(error);
		} finally {
			submitting = false;
		}
	}

	function handleError(error: unknown) {
		errorMessage =
			error instanceof Error ? error.message : typeof error === 'string' ? error : '发生未知错误';
	}

	function clearError() {
		errorMessage = '';
	}

	function getPartMeta(part: string) {
		switch (part) {
			case 'noun':
				return { label: '名词', accent: 'noun', description: '着重记忆性别与复数形式', kind: 'noun' as const };
			case 'verb':
				return { label: '动词', accent: 'verb', description: '关注动词变位与搭配', kind: 'verb' as const };
			case 'adjective_adverb':
				return { label: '形容 / 副词', accent: 'adjective', description: '观察比较级和搭配', kind: 'adjective' as const };
			default:
				return { label: part, accent: 'default', description: '', kind: 'default' as const };
		}
	}

	function parseExamples(value?: string | null) {
		if (!value) return [];
		return value
			.split(/[\r\n；;]+/)
			.map((item) => item.trim())
			.filter(Boolean);
	}

	function parseThemes(value?: string | null) {
		if (!value) return [];
		try {
			const parsed = JSON.parse(value);
			if (Array.isArray(parsed)) {
				return parsed.map((item) => String(item));
			}
		} catch {
			/* fall through */
		}

		return value
			.split(/[,，]+/)
			.map((item) => item.trim())
			.filter(Boolean);
	}

	function formatStatus(status?: string | null) {
		if (!status) return '未学习';
		const lower = status.toLowerCase();
		if (lower === 'mastered') return '已掌握';
		if (lower === 'learning') return '学习中';
		return status;
	}

	function statusBadgeClass(status?: string | null) {
		if (!status) return 'status new';
		const lower = status.toLowerCase();
		if (lower === 'mastered') return 'status mastered';
		if (lower === 'learning') return 'status learning';
		return 'status other';
	}

	function formatDate(value?: string | null) {
		if (!value) return '尚未复习';
		const date = new Date(value);
		if (Number.isNaN(date.getTime())) return value;
		return new Intl.DateTimeFormat('zh-CN', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		}).format(date);
	}
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>词汇闪卡 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="header">
			<span class="ghost-button" aria-hidden="true"></span>
			<h1 class="top-bar-title">词汇闪卡</h1>
			<span class="ghost-button" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	{#if errorMessage}
		<div class="alert" role="alert">
			<p>{errorMessage}</p>
			<button type="button" onclick={clearError}>知道了</button>
		</div>
	{/if}

	<section class="stats">
		{#if loadingStats}
			<div class="stats-placeholder">统计加载中…</div>
		{:else if stats}
			<div class="stats-grid">
				<div class="stats-card emphasis">
					<span class="label">总词汇</span>
					<strong>{stats.total}</strong>
				</div>
				<div class="stats-card">
					<span class="label">未学习</span>
					<strong>{stats.new}</strong>
				</div>
				<div class="stats-card">
					<span class="label">学习中</span>
					<strong>{stats.learning}</strong>
				</div>
				<div class="stats-card">
					<span class="label">已掌握</span>
					<strong>{stats.mastered}</strong>
				</div>
			</div>
		{/if}
	</section>

	<section class="filters">
		<div class="filter-row">
			{#each partOptions as option (option.value)}
				<button
					class:selected={selectedPart === option.value}
					type="button"
					onclick={() => selectPart(option.value)}
				>
					{option.label}
				</button>
			{/each}
		</div>
		<div class="filter-row subtle">
			{#each statusOptions as option (option.value)}
				<button
					class:selected={selectedStatus === option.value}
					type="button"
					onclick={() => selectStatus(option.value)}
				>
					{option.label}
				</button>
			{/each}
		</div>
	</section>

	<section class="flashcard-section">
		{#if loadingCard}
			<div class="card-placeholder">正在挑选适合你的卡片…</div>
		{:else if !card}
			<div class="card-placeholder">当前筛选条件下没有更多卡片啦 🎉</div>
		{:else}
			<article class={`flashcard ${showBack ? 'is-flipped' : ''}`} data-part={card.part_of_speech}>
				<button
					class="card-body"
					type="button"
					onclick={toggleFace}
					aria-pressed={showBack ? 'true' : 'false'}
				>
					<div class="face front">
						{#if partMeta}
							<span class={`chip ${partMeta.accent}`}>{partMeta.label}</span>
						{/if}
						<h2 class="word">{card.word}</h2>
						{#if card.english}
							<p class="hint">{card.english}</p>
						{:else if partMeta}
							<p class="hint">{partMeta.description}</p>
						{/if}
						<p class="instruction">点击翻面查看详情</p>
					</div>
					<div class="face back">
						<header>
							{#if partMeta}
								<span class={`chip ${partMeta.accent}`}>{partMeta.label}</span>
							{/if}
							<h2>{card.word}</h2>
							<p class={statusBadgeClass(card.status)}>{formatStatus(card.status)}</p>
						</header>

						{#if partMeta?.kind === 'noun'}
							<section class="detail noun">
								<h3>名词释义</h3>
								<p class="primary">{card.meaning ?? '暂无释义'}</p>
								{#if card.english}
									<p class="secondary">{card.english}</p>
								{/if}
								<p class="note">提示：留意阴阳中性与复数的变化。</p>
							</section>
						{:else if partMeta?.kind === 'verb'}
							<section class="detail verb">
								<h3>动词说明</h3>
								<p class="primary">{card.meaning ?? '暂无释义'}</p>
								{#if card.english}
									<p class="secondary">{card.english}</p>
								{/if}
								<p class="note">掌握常用搭配与变位，更容易记牢。</p>
							</section>
						{:else if partMeta?.kind === 'adjective'}
							<section class="detail adjective">
								<h3>形容 / 副词</h3>
								<p class="primary">{card.meaning ?? '暂无释义'}</p>
								{#if card.english}
									<p class="secondary">{card.english}</p>
								{/if}
								<p class="note">结合例句对比原级、比较级、最高级。</p>
							</section>
						{:else}
							<section class="detail default">
								<h3>单词释义</h3>
								<p class="primary">{card.meaning ?? '暂无释义'}</p>
								{#if card.english}
									<p class="secondary">{card.english}</p>
								{/if}
							</section>
						{/if}

						{#if examples.length}
							<section class="examples">
								<h3>例句</h3>
								<ul>
									{#each examples as example (example)}
										<li>{example}</li>
									{/each}
								</ul>
							</section>
						{/if}

						<section class="meta">
							<p>已复习 {card.times_seen} 次 · 掌握 {card.times_mastered} 次</p>
							<p>上次复习：{formatDate(card.last_seen_at)}</p>
						</section>

						{#if themes.length}
							<section class="themes">
								{#each themes as theme (theme)}
									<span>{theme}</span>
								{/each}
							</section>
						{/if}
					</div>
				</button>
			</article>

			<div class="actions">
				<button class="ghost" type="button" onclick={() => review('learning')} disabled={submitting}>
					再巩固一下
				</button>
				<button class="primary" type="button" onclick={() => review('mastered')} disabled={submitting}>
					已经掌握
				</button>
			</div>
		{/if}
	</section>
</PhoneFrame>

<style>
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 2rem 1.8rem 1.2rem;
	}

	.alert {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.9rem 1.1rem;
		border-radius: 16px;
		background: rgba(220, 68, 78, 0.12);
		color: #b3261e;
		font-size: 0.9rem;
	}

	.alert button {
		border: none;
		background: transparent;
		color: inherit;
		font-weight: 600;
		cursor: pointer;
	}

	.stats {
		margin-top: 0.2rem;
	}

	.stats-placeholder {
		text-align: center;
		padding: 1.2rem;
		color: #6a7487;
		font-size: 0.95rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.8rem;
	}

	.stats-card {
		background: #ffffff;
		border-radius: 18px;
		padding: 1rem;
		box-shadow: 0 16px 24px rgba(17, 29, 49, 0.08);
		border: 1px solid rgba(17, 29, 49, 0.06);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.stats-card.emphasis {
		grid-column: span 2;
		background: linear-gradient(135deg, #186fff, #4bc0ff);
		color: #ffffff;
	}

	.stats-card .label {
		font-size: 0.86rem;
		color: inherit;
		opacity: 0.8;
	}

	.stats-card strong {
		font-size: 1.4rem;
		font-weight: 800;
	}

	.filters {
		margin-top: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
	}

	.filter-row {
		display: flex;
		gap: 0.6rem;
		overflow-x: auto;
		padding-bottom: 0.2rem;
	}

	.filter-row button {
		white-space: nowrap;
		border: none;
		border-radius: 999px;
		padding: 0.55rem 1.1rem;
		background: rgba(17, 32, 51, 0.08);
		color: #112033;
		font-weight: 600;
		cursor: pointer;
		transition: transform 0.2s ease, background 0.2s ease, color 0.2s ease;
	}

	.filter-row button.selected {
		background: #112033;
		color: #ffffff;
		transform: translateY(-2px);
		box-shadow: 0 12px 20px rgba(17, 32, 51, 0.25);
	}

	.filter-row.subtle button {
		background: rgba(17, 32, 51, 0.05);
		color: #5c6475;
	}

	.filter-row.subtle button.selected {
		background: rgba(17, 32, 51, 0.16);
		color: #112033;
		box-shadow: none;
		transform: none;
	}

	.flashcard-section {
		margin-top: 1.8rem;
		display: flex;
		flex-direction: column;
		gap: 1.2rem;
	}

	.card-placeholder {
		padding: 2.4rem 1.4rem;
		border-radius: 20px;
		background: #ffffff;
		text-align: center;
		color: #606a7c;
		box-shadow: 0 18px 32px rgba(17, 32, 51, 0.08);
		border: 1px solid rgba(17, 32, 51, 0.05);
	}

	.flashcard {
		position: relative;
		width: 100%;
		min-height: 260px;
		border-radius: 24px;
		perspective: 1200px;
	}

	.card-body {
		width: 100%;
		min-height: 260px;
		border: none;
		background: transparent;
		padding: 0;
		cursor: pointer;
		transform-style: preserve-3d;
		display: block;
		position: relative;
		transform: rotateY(0);
		transition: transform 0.55s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.flashcard.is-flipped .card-body {
		transform: rotateY(180deg);
	}

	.face {
		position: absolute;
		inset: 0;
		border-radius: 24px;
		backface-visibility: hidden;
		background: #ffffff;
		box-shadow: 0 22px 36px rgba(17, 32, 51, 0.14);
		border: 1px solid rgba(17, 32, 51, 0.06);
		padding: 1.6rem 1.4rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		justify-content: center;
	}

	.face.front {
		align-items: center;
		text-align: center;
		background: linear-gradient(135deg, #ffffff, #f6f9ff);
	}

	.face.back {
		transform: rotateY(180deg);
		align-items: stretch;
		text-align: left;
		gap: 1rem;
		overflow-y: auto;
		scrollbar-width: thin;
	}

	.flashcard[data-part='noun'] .face.front {
		background: linear-gradient(135deg, #fef4f5, #fff9fb);
	}

	.flashcard[data-part='verb'] .face.front {
		background: linear-gradient(135deg, #f4fbff, #f6fff9);
	}

	.flashcard[data-part='adjective_adverb'] .face.front {
		background: linear-gradient(135deg, #f8f4ff, #fff5fe);
	}

	.chip {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.25rem 0.75rem;
		border-radius: 999px;
		font-size: 0.78rem;
		font-weight: 700;
		text-transform: uppercase;
	}

	.chip.noun {
		background: rgba(238, 99, 82, 0.14);
		color: #c63a2f;
	}

	.chip.verb {
		background: rgba(49, 150, 112, 0.16);
		color: #1c8c5f;
	}

	.chip.adjective {
		background: rgba(95, 88, 220, 0.16);
		color: #4c48c9;
	}

	.word {
		font-size: 2.2rem;
		font-weight: 800;
		color: #111b2c;
		margin: 0;
	}

	.hint {
		margin: 0;
		font-size: 1rem;
		color: #6b7689;
	}

	.instruction {
		margin: 0;
		font-size: 0.85rem;
		color: #9aa4b8;
	}

	.status {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.3rem 0.7rem;
		border-radius: 12px;
		font-size: 0.78rem;
		font-weight: 600;
		margin-top: 0.3rem;
	}

	.status.mastered {
		background: rgba(35, 197, 102, 0.14);
		color: #1c864f;
	}

	.status.learning {
		background: rgba(255, 171, 62, 0.18);
		color: #a7671b;
	}

	.status.new {
		background: rgba(59, 118, 255, 0.14);
		color: #2556cc;
	}

	.status.other {
		background: rgba(0, 0, 0, 0.08);
		color: #444b5b;
	}

	.detail {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}

	.detail h3 {
		margin: 0;
		font-size: 0.96rem;
		color: #5d6a80;
		font-weight: 600;
	}

	.detail .primary {
		margin: 0;
		font-size: 1.05rem;
		font-weight: 600;
		color: #1c2536;
	}

	.detail .secondary {
		margin: 0.2rem 0 0;
		font-size: 0.9rem;
		color: #728093;
	}

	.detail .note {
		margin: 0.4rem 0 0;
		font-size: 0.82rem;
		color: #9ba6bb;
	}

	.examples h3 {
		margin: 0 0 0.4rem;
		font-size: 0.9rem;
		color: #929eb2;
		font-weight: 600;
	}

	.examples ul {
		margin: 0;
		padding-left: 1.1rem;
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		color: #1c2536;
	}

	.examples li {
		font-size: 0.92rem;
	}

	.meta {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
		color: #708093;
		font-size: 0.85rem;
	}

	.themes {
		display: flex;
		flex-wrap: wrap;
		gap: 0.4rem;
	}

	.themes span {
		padding: 0.3rem 0.6rem;
		border-radius: 12px;
		font-size: 0.78rem;
		background: rgba(17, 32, 51, 0.08);
		color: #2c3748;
	}

	.actions {
		display: flex;
		gap: 0.8rem;
	}

	.actions button {
		flex: 1;
		border-radius: 16px;
		border: none;
		padding: 0.9rem 1rem;
		font-weight: 700;
		font-size: 0.98rem;
		cursor: pointer;
	}

	.actions .ghost {
		background: rgba(17, 32, 51, 0.08);
		color: #112033;
	}

	.actions .primary {
		background: linear-gradient(135deg, #186fff, #3158ff);
		color: #ffffff;
		box-shadow: 0 16px 32px rgba(24, 111, 255, 0.28);
	}

	.actions button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
