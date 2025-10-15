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

	type FlashcardMetadata =
		| {
				type: 'noun';
				gender?: string | null;
				plural?: string | null;
				suffix?: string | null;
			}
		| {
				type: 'verb';
				present_form?: string | null;
				preterite_form?: string | null;
				perfect_form?: string | null;
				properties?: string | null;
				noun_form?: string | null;
			}
		| {
				type: 'adjective_adverb';
				attribute?: string | null;
				comparison_forms: string[];
			};

	type NounMetadata = Extract<FlashcardMetadata, { type: 'noun' }>;
	type VerbMetadata = Extract<FlashcardMetadata, { type: 'verb' }>;
	type AdjectiveMetadata = Extract<FlashcardMetadata, { type: 'adjective_adverb' }>;

	type Flashcard = {
		entry_id: number;
		word: string;
		part_of_speech: string;
		meaning?: string | null;
		english?: string | null;
		examples?: string | null;
		themes?: string | null;
		status?: string | null;
		times_seen: number;
		times_mastered: number;
		last_seen_at?: string | null;
		metadata?: FlashcardMetadata | null;
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

	let stats = $state<Stats | null>(null);
	let card = $state<Flashcard | null>(null);
	let loadingStats = $state(false);
	let loadingCard = $state(false);
	let submitting = $state(false);
	let showBack = $state(false);
	let errorMessage = $state('');
	let selectedPart = $state<PartOption['value']>('all');
	let selectedStatus = $state<StatusOption['value']>('all');

	const metadata = $derived(card?.metadata ?? null);
	const nounMetadata = $derived(
		metadata?.type === 'noun' ? (metadata as NounMetadata) : null
	);
	const verbMetadata = $derived(
		metadata?.type === 'verb' ? (metadata as VerbMetadata) : null
	);
	const adjectiveMetadata = $derived(
		metadata?.type === 'adjective_adverb' ? (metadata as AdjectiveMetadata) : null
	);
	const verbProperties = $derived(splitKeywords(verbMetadata?.properties));
	const adjectiveComparisons = $derived(
		buildAdjectiveComparisons(card?.word ?? '', adjectiveMetadata?.comparison_forms ?? [])
	);
	const themes = $derived(parseThemes(card?.themes));
	const examples = $derived(parseExamples(card?.examples));
	const partMeta = $derived(card ? getPartMeta(card.part_of_speech) : null);
	const showNounDetails = $derived(
		Boolean(noneEmpty(nounMetadata?.gender, nounMetadata?.plural, nounMetadata?.suffix))
	);
	const showVerbDetails = $derived(
		Boolean(
			verbMetadata &&
			(
				verbMetadata.present_form ||
				verbMetadata.preterite_form ||
				verbMetadata.perfect_form ||
				verbMetadata.properties ||
				verbMetadata.noun_form
			)
		)
	);
	const showAdjectiveDetails = $derived(
		Boolean(
			adjectiveMetadata &&
			(adjectiveMetadata.attribute || adjectiveMetadata.comparison_forms.length > 0)
		)
	);

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
		} catch (error) {
			handleError(error);
		} finally {
			loadingCard = false;
		}
	}

	function buildQuery() {
		const params = new URLSearchParams();
		if (selectedPart !== 'all') params.set('part_of_speech', selectedPart);
		if (selectedStatus !== 'all') params.set('status', selectedStatus);
		return params.toString();
	}

	function toggleFace() {
		if (card) {
			showBack = !showBack;
		}
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

	async function skipCard() {
		if (!card || loadingCard || submitting) return;
		await review('learning', { skip: true });
	}

	async function review(
		result: 'mastered' | 'learning',
		options?: { skip?: boolean }
	) {
		if (!card || submitting) return;
		submitting = true;
		try {
			const res = await fetch(`${API_BASE}/${card.entry_id}/review`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ result })
			});
			if (!res.ok) {
				const errorPayload = await res.json().catch(() => ({}));
				throw new Error(errorPayload.error ?? '提交复习结果失败');
			}
			if (options?.skip) {
				void loadStats();
			} else {
				await loadStats();
			}
			await loadCard();
		} catch (error) {
			handleError(error);
		} finally {
			submitting = false;
		}
	}

	function handleError(error: unknown) {
		const message =
			error instanceof Error ? error.message : typeof error === 'string' ? error : '发生未知错误';
		errorMessage = message;
	}

	function clearError() {
		errorMessage = '';
	}

	function getPartMeta(part: string) {
		switch (part) {
			case 'noun':
				return { label: '名词', accent: 'noun', hint: 'Nomen · 名词' };
			case 'verb':
				return { label: '动词', accent: 'verb', hint: 'Verb · 动词' };
			case 'adjective_adverb':
				return { label: '形容 / 副词', accent: 'adjective', hint: 'Adjektiv / Adverb' };
			default:
				return { label: part, accent: 'default', hint: part };
		}
	}

	function parseExamples(value?: string | null) {
		if (!value) return [];
		return value
			.split(/[\r\n\u{FF1B};]+/u)
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
			// ignore
		}

		return value
			.split(/[,\u{FF0C}]+/u)
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

	function splitKeywords(value?: string | null) {
		if (!value) return [];
		return value
			.split(/[;\u{FF1B},\u{FF0C}、·•]+/u)
			.map((item) => item.trim())
			.filter(Boolean);
	}

	function noneEmpty(...values: Array<string | null | undefined>) {
		return values.some((value) => value && value.trim());
	}

	function formatGender(value?: string | null) {
		if (!value) return '';
		const normalized = value.trim().toLowerCase();
		if (!normalized) return '';
		return normalized.charAt(0).toUpperCase() + normalized.slice(1);
	}

	function buildAdjectiveComparisons(base: string, forms: string[]) {
		const comparisons: Array<{ label: string; value: string }> = [];
		const [comparative, superlative, ...rest] = forms;
		if (base) comparisons.push({ label: '原级', value: base });
		if (comparative) comparisons.push({ label: '比较级', value: comparative });
		if (superlative) comparisons.push({ label: '最高级', value: superlative });
		rest.forEach((extra, index) => {
			if (extra) {
				comparisons.push({ label: `其他形式 ${index + 1}`, value: extra });
			}
		});
		return comparisons;
	}
</script>


<PhoneFrame {navItems} navActive="learning">
	<div class="page">
		{#if errorMessage}
			<div class="alert" role="alert">
				<p>{errorMessage}</p>
				<button type="button" onclick={clearError}>知道了</button>
			</div>
		{/if}

		<section class="stats-panel">
			{#if loadingStats}
				<div class="metric loading">
					<span class="metric-value">统计加载中…</span>
				</div>
			{:else if stats}
				<div class="metric highlight">
					<span class="metric-label">总词汇</span>
					<span class="metric-value">{stats.total}</span>
				</div>
				<div class="metric">
					<span class="metric-label">未学习</span>
					<span class="metric-value">{stats.new}</span>
				</div>
				<div class="metric">
					<span class="metric-label">学习中</span>
					<span class="metric-value">{stats.learning}</span>
				</div>
				<div class="metric">
					<span class="metric-label">已掌握</span>
					<span class="metric-value">{stats.mastered}</span>
				</div>
			{/if}
		</section>

		<section class="filters">
			<div class="filter-row primary">
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
			<div class="filter-row secondary">
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

		<section class="card-stage">
			{#if loadingCard}
				<div class="card-placeholder">
					<span class="gloss"></span>
					<p>正在挑选适合你的卡片…</p>
				</div>
			{:else if !card}
				<div class="card-placeholder empty">
					<span class="gloss"></span>
					<p>当前筛选条件下没有更多卡片啦 🎉</p>
				</div>
			{:else}
				<div class="stack">
					<span class="layer layer-b"></span>
					<span class="layer layer-a"></span>
					<article class={`flashcard ${showBack ? 'is-flipped' : ''}`} data-part={card.part_of_speech}>
						<button class="card-body" type="button" onclick={toggleFace} aria-pressed={showBack}>
							<div class="face front">
								<div class="card-bg"></div>
								<header class="card-head">
									{#if partMeta}
										<span class={`pos-chip ${partMeta.accent}`}>{partMeta.label}</span>
									{/if}
									<span class="status-pill">{formatStatus(card.status)}</span>
								</header>
								<div class="word-block">
									<h2 class="word">{card.word}</h2>
									{#if card.english}
										<p class="translation">{card.english}</p>
									{:else if partMeta}
										<p class="translation faint">{partMeta.hint}</p>
									{/if}
								</div>
								{#if themes.length}
									<ul class="theme-chips">
										{#each themes.slice(0, 3) as theme (theme)}
											<li>{theme}</li>
										{/each}
										{#if themes.length > 3}
											<li class="more">+{themes.length - 3}</li>
										{/if}
									</ul>
								{/if}
								<p class="flip-hint">点击翻转查看详解</p>
							</div>

							<div class="face back">
								<div class="card-bg"></div>
								<header class="card-head back">
									{#if partMeta}
										<span class={`pos-chip ${partMeta.accent}`}>{partMeta.label}</span>
									{/if}
									<div class="headline">
										<h2>{card.word}</h2>
										<span class="status-pill ghost">{formatStatus(card.status)}</span>
									</div>
								</header>

								{#if card.meaning || card.english}
									<section class="meaning">
										{#if card.meaning}
											<p class="primary">{card.meaning}</p>
										{/if}
										{#if card.english}
											<p class="secondary">{card.english}</p>
										{/if}
									</section>
								{/if}

								{#if showNounDetails && nounMetadata}
									<section class="morphology noun">
										<h3>名词提示</h3>
										<div class="grid">
											{#if nounMetadata.gender}
												<div>
													<span class="label">冠词</span>
													<span class={`value gender ${nounMetadata.gender?.toLowerCase() ?? ''}`}>{formatGender(nounMetadata.gender)}</span>
												</div>
											{/if}
											{#if nounMetadata.plural}
												<div>
													<span class="label">复数</span>
													<span class="value">{nounMetadata.plural}</span>
												</div>
											{/if}
											{#if nounMetadata.suffix}
												<div>
													<span class="label">常见词尾</span>
													<span class="value">{nounMetadata.suffix}</span>
												</div>
											{/if}
										</div>
									</section>
							{:else if showVerbDetails && verbMetadata}
								<section class="morphology verb">
									<h3>动词变位</h3>
									<ul class="grid">
										{#if verbMetadata.present_form}
											<li>
												<span class="label">现在时</span>
												<span class="value">{verbMetadata.present_form}</span>
											</li>
										{/if}
										{#if verbMetadata.preterite_form}
											<li>
												<span class="label">过去时</span>
												<span class="value">{verbMetadata.preterite_form}</span>
											</li>
										{/if}
										{#if verbMetadata.perfect_form}
											<li>
												<span class="label">完成时</span>
												<span class="value">{verbMetadata.perfect_form}</span>
											</li>
										{/if}
									</ul>

									{#if verbProperties.length}
										<div class="tag-row">
											{#each verbProperties as property (property)}
												<span>{property}</span>
											{/each}
										</div>
									{/if}

									{#if verbMetadata.noun_form}
										<p class="note">相关名词：{verbMetadata.noun_form}</p>
									{/if}
								</section>
							{:else if showAdjectiveDetails && adjectiveMetadata}
								<section class="morphology adjective">
									<h3>比较层级</h3>
									{#if adjectiveMetadata.attribute}
										<p class="note">{adjectiveMetadata.attribute}</p>
									{/if}
									{#if adjectiveComparisons.length}
										<ul class="grid">
											{#each adjectiveComparisons as item (item.label)}
												<li>
													<span class="label">{item.label}</span>
													<span class="value">{item.value}</span>
												</li>
											{/each}
										</ul>
									{/if}
								</section>
							{/if}

							{#if examples.length}
								<section class="examples">
									<div class="examples-head">
										<h3>例句</h3>
										<span>{examples.length}</span>
									</div>
									<ul>
										{#each examples as example (example)}
											<li>{example}</li>
										{/each}
									</ul>
								</section>
							{/if}

							<footer class="card-footer">
								<div class="review-metrics">
									<span>复习 {card.times_seen} 次</span>
									<span>掌握 {card.times_mastered} 次</span>
								</div>
								<span class="last-reviewed">上次复习：{formatDate(card.last_seen_at)}</span>
							</footer>
						</div>
					</button>
				</article>
			</div>

			<div class="review-controls">
				<button
					class="control-btn outline"
					type="button"
					onclick={skipCard}
					disabled={loadingCard || submitting}
				>
					<span class="icon">⏭</span>
					<span>跳过</span>
				</button>
				<button
					class="control-btn ghost"
					type="button"
					onclick={() => review('learning')}
					disabled={submitting}
				>
					<span class="icon">↻</span>
					<span>再巩固一下</span>
				</button>
				<button
					class="control-btn solid"
					type="button"
					onclick={() => review('mastered')}
					disabled={submitting}
				>
					<span class="icon">✓</span>
					<span>已经掌握</span>
				</button>
			</div>
		{/if}
		</section>
	</div>
</PhoneFrame>

<style>
.page {
	display: flex;
	flex-direction: column;
	gap: 1.8rem;
	padding: 1.4rem 1.4rem 2.4rem;
	background: rgba(18, 22, 36, 0.55);
	border-radius: 32px 32px 0 0;
	box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06), 0 -20px 60px rgba(0, 0, 0, 0.35);
}

	.alert {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.8rem 1rem;
		border-radius: 16px;
		background: rgba(220, 68, 78, 0.14);
		color: #ffb1ae;
		font-size: 0.88rem;
		border: 1px solid rgba(255, 109, 118, 0.2);
	}

	.alert button {
		border: none;
		background: transparent;
		color: inherit;
		font-weight: 600;
		cursor: pointer;
	}

	.stats-panel {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
		gap: 0.75rem;
		align-items: stretch;
	}

	.metric {
		position: relative;
		padding: 0.75rem 0.95rem;
		border-radius: 16px;
		background: linear-gradient(145deg, rgba(46, 50, 74, 0.78), rgba(26, 29, 44, 0.86));
		color: rgba(229, 235, 255, 0.86);
		border: 1px solid rgba(90, 116, 188, 0.18);
		box-shadow: 0 16px 32px rgba(8, 16, 38, 0.42), inset 0 1px 2px rgba(255, 255, 255, 0.06);
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		gap: 0.6rem;
	}

	.metric-label {
		font-size: 0.78rem;
		opacity: 0.78;
		letter-spacing: 0.035em;
	}

	.metric-value {
		font-size: 1.18rem;
		font-weight: 700;
	}

	.metric.highlight {
		background: linear-gradient(135deg, #4a66ff, #4ec8ff);
		color: #ffffff;
	}

	.metric.highlight .metric-label {
		opacity: 0.9;
	}

	.metric.loading {
		grid-column: 1 / -1;
		justify-content: center;
		align-items: center;
		background: rgba(32, 36, 56, 0.85);
	}

	.filters {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.filter-row {
		display: flex;
		gap: 0.55rem;
		overflow-x: auto;
		padding-bottom: 0.15rem;
	}

	.filter-row button {
		white-space: nowrap;
		border-radius: 999px;
		padding: 0.48rem 1.05rem;
		border: 1px solid rgba(88, 120, 240, 0.18);
		background: rgba(25, 34, 58, 0.7);
		color: rgba(228, 234, 255, 0.78);
		font-weight: 600;
		cursor: pointer;
		backdrop-filter: blur(12px);
		transition: background 0.18s ease, transform 0.18s ease, box-shadow 0.18s ease;
	}

	.filter-row button.selected {
		background: linear-gradient(135deg, #6079ff, #6ed5ff);
		color: #ffffff;
		border-color: transparent;
		box-shadow: 0 12px 24px rgba(64, 125, 255, 0.32);
		transform: translateY(-1px);
	}

	.filter-row.secondary button {
		background: rgba(27, 36, 56, 0.6);
	}

	.filter-row.secondary button.selected {
		background: rgba(255, 255, 255, 0.16);
		border-color: rgba(255, 255, 255, 0.24);
	}

	.card-stage {
		display: flex;
		flex-direction: column;
		gap: 1.6rem;
	}

	.card-placeholder {
		position: relative;
		padding: 2.4rem 1.4rem;
		border-radius: 26px;
		background: linear-gradient(145deg, rgba(28, 32, 52, 0.9), rgba(17, 21, 34, 0.92));
		color: rgba(225, 234, 255, 0.82);
		text-align: center;
		border: 1px solid rgba(80, 120, 220, 0.2);
		box-shadow: 0 22px 36px rgba(8, 16, 32, 0.45), inset 0 1px 1px rgba(255, 255, 255, 0.08);
		overflow: hidden;
	}

	.card-placeholder .gloss {
		position: absolute;
		top: -40%;
		left: -10%;
		width: 140%;
		height: 120%;
		background: radial-gradient(circle at 50% 0%, rgba(86, 120, 255, 0.28), rgba(86, 120, 255, 0));
		filter: blur(48px);
		opacity: 0.85;
	}

	.card-placeholder p {
		margin: 0;
		position: relative;
		z-index: 1;
		font-size: 0.96rem;
	}

	.stack {
		position: relative;
		width: 100%;
		min-height: 280px;
		perspective: 1400px;
	}

	.layer {
		position: absolute;
		inset: 0;
		border-radius: 28px;
		background: linear-gradient(145deg, rgba(46, 52, 74, 0.8), rgba(19, 21, 32, 0.9));
		box-shadow: 0 18px 36px rgba(0, 0, 0, 0.45), inset 0 1px 1px rgba(255, 255, 255, 0.05);
	}

	.layer.layer-a {
		transform: translateY(16px) translateZ(-30px) rotateX(5deg);
		opacity: 0.55;
	}

	.layer.layer-b {
		transform: translateY(28px) translateZ(-60px) rotateX(8deg);
		opacity: 0.35;
	}

	.flashcard {
		position: relative;
		width: 100%;
		min-height: 280px;
		transform-style: preserve-3d;
		cursor: pointer;
	}

	.card-body {
		position: relative;
		width: 100%;
		min-height: 280px;
		padding: 0;
		border: none;
		background: transparent;
		transform-style: preserve-3d;
		transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.flashcard.is-flipped .card-body {
		transform: rotateY(180deg);
	}

	.face {
		position: absolute;
		inset: 0;
		border-radius: 28px;
		padding: 1.8rem 1.6rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		color: #202438;
		backface-visibility: hidden;
		border: 1px solid rgba(0, 0, 0, 0.08);
		box-shadow: 0 22px 44px rgba(9, 16, 36, 0.45), inset 0 2px 4px rgba(255, 255, 255, 0.4), inset 0 -4px 8px rgba(0, 0, 0, 0.3);
		overflow: hidden;
	}

	.face .card-bg {
		position: absolute;
		inset: 0;
		border-radius: inherit;
		background: linear-gradient(145deg, #f5f7fc, #cfd8ec);
		z-index: -1;
	}

	.face.front .card-bg::after {
		content: '';
		position: absolute;
		inset: 0;
		background: repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(0, 0, 0, 0.05) 2px, rgba(0, 0, 0, 0.05) 4px);
		opacity: 0.12;
	}

	.face.back {
		transform: rotateY(180deg);
		color: #f4f6ff;
	}

	.face.back .card-bg {
		background: linear-gradient(145deg, #5f7bff, #8e59ff);
	}

	.card-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.card-head.back {
		margin-bottom: 1.2rem;
	}

	.pos-chip {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.3rem 0.9rem;
		border-radius: 999px;
		font-size: 0.78rem;
		font-weight: 700;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		background: rgba(85, 102, 255, 0.16);
		color: #273067;
	}

	.pos-chip.noun {
		background: rgba(255, 174, 102, 0.22);
		color: #7b3d08;
	}

	.pos-chip.verb {
		background: rgba(122, 231, 186, 0.22);
		color: #0f4b3d;
	}

	.pos-chip.adjective {
		background: rgba(178, 162, 255, 0.22);
		color: #352d75;
	}

	.status-pill {
		display: inline-flex;
		align-items: center;
		padding: 0.24rem 0.72rem;
		border-radius: 999px;
		font-size: 0.76rem;
		font-weight: 600;
		background: rgba(15, 25, 45, 0.08);
		color: rgba(34, 44, 72, 0.7);
	}

	.status-pill.ghost {
		background: rgba(255, 255, 255, 0.18);
		color: #f4f6ff;
	}

	.word-block {
		display: flex;
		flex-direction: column;
		gap: 0.45rem;
		margin-top: 1.6rem;
	}

	.word {
		margin: 0;
		font-size: 2.25rem;
		font-weight: 800;
		letter-spacing: 0.02em;
	}

	.translation {
		margin: 0;
		font-size: 1.02rem;
		color: rgba(38, 44, 68, 0.78);
	}

	.translation.faint {
		opacity: 0.7;
	}

	.theme-chips {
		margin: 1.5rem 0 0;
		padding: 0;
		list-style: none;
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.theme-chips li {
		padding: 0.32rem 0.8rem;
		border-radius: 999px;
		background: rgba(30, 40, 68, 0.12);
		color: rgba(26, 38, 70, 0.68);
		font-size: 0.78rem;
		font-weight: 600;
	}

	.theme-chips li.more {
		background: rgba(90, 110, 255, 0.24);
		color: #ffffff;
	}

	.flip-hint {
		margin-top: auto;
		font-size: 0.82rem;
		color: rgba(38, 48, 78, 0.58);
		text-align: right;
	}

	.headline {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.headline h2 {
		margin: 0;
		font-size: 1.8rem;
		font-weight: 800;
		color: #f4f6ff;
	}

	.meaning {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 1.2rem;
	}

	.meaning .primary {
		margin: 0;
		font-size: 1.08rem;
		font-weight: 600;
		color: #ffffff;
	}

	.meaning .secondary {
		margin: 0;
		font-size: 0.9rem;
		color: rgba(229, 235, 255, 0.78);
	}

	.morphology {
		border-radius: 18px;
		padding: 1rem 1.1rem;
		margin-bottom: 1rem;
		background: rgba(17, 24, 48, 0.52);
		border: 1px solid rgba(112, 148, 255, 0.18);
		box-shadow: inset 0 1px 1px rgba(255, 255, 255, 0.08);
	}

	.morphology h3 {
		margin: 0 0 0.55rem;
		font-size: 0.9rem;
		font-weight: 700;
		color: rgba(226, 233, 255, 0.9);
	}

	.morphology .grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.5rem 0.8rem;
	}

	.morphology .label {
		display: block;
		font-size: 0.72rem;
		font-weight: 600;
		color: rgba(198, 210, 255, 0.72);
	}

	.morphology .value {
		font-size: 0.95rem;
		font-weight: 700;
		color: #ffffff;
	}

	.morphology .value.gender.der {
		color: #ffd3be;
	}

	.morphology .value.gender.die {
		color: #e5d8ff;
	}

	.morphology .value.gender.das {
		color: #b6ffdf;
	}

	.morphology .tag-row {
		display: flex;
		gap: 0.45rem;
		flex-wrap: wrap;
		margin-top: 0.6rem;
	}

	.morphology .tag-row span {
		padding: 0.28rem 0.7rem;
		border-radius: 999px;
		font-size: 0.74rem;
		font-weight: 600;
		background: rgba(78, 218, 170, 0.18);
		color: #84ffd7;
	}

	.morphology .note {
		margin: 0.55rem 0 0;
		font-size: 0.8rem;
		color: rgba(205, 220, 255, 0.76);
	}

	.examples {
		margin: 1.1rem 0 0.8rem;
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
	}

	.examples-head {
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		color: rgba(228, 235, 255, 0.84);
	}

	.examples-head span {
		font-size: 0.78rem;
		opacity: 0.72;
	}

	.examples ul {
		margin: 0;
		padding-left: 1rem;
		color: rgba(224, 232, 255, 0.88);
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
		font-size: 0.9rem;
	}

	.card-footer {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		padding-top: 1rem;
		font-size: 0.82rem;
		color: rgba(205, 216, 242, 0.72);
	}

	.review-metrics {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
	}

	.last-reviewed {
		font-size: 0.8rem;
	}

	.review-controls {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.control-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		border-radius: 16px;
		font-weight: 700;
		font-size: 0.94rem;
		cursor: pointer;
		border: 1px solid rgba(255, 255, 255, 0.18);
		transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease;
	}

	.control-btn .icon {
		font-size: 1rem;
	}

	.control-btn.ghost {
		background: rgba(255, 255, 255, 0.16);
		color: #ffffff;
		box-shadow: 0 16px 28px rgba(16, 24, 48, 0.32);
	}

	.control-btn.outline {
		background: rgba(20, 30, 54, 0.6);
		color: rgba(225, 236, 255, 0.88);
		border-color: rgba(255, 255, 255, 0.22);
		box-shadow: 0 14px 24px rgba(10, 24, 48, 0.3);
	}

	.control-btn.outline:hover:not(:disabled) {
		background: rgba(24, 36, 64, 0.78);
	}

	.control-btn.solid {
		background: linear-gradient(135deg, #3dd79d, #2dc0ff);
		color: #0a1a2e;
		box-shadow: 0 20px 36px rgba(44, 180, 225, 0.4);
		border-color: transparent;
	}

	.control-btn:hover:not(:disabled) {
		transform: translateY(-1px);
	}

	.control-btn:disabled {
		opacity: 0.65;
		cursor: not-allowed;
		transform: none;
	}

	@media (max-width: 420px) {
		.word {
			font-size: 2rem;
		}

		.review-metrics {
			flex-direction: column;
			align-items: flex-start;
		}
	}

	@media (min-width: 520px) {
		.stack {
			min-height: 320px;
		}

		.card-body {
			min-height: 320px;
		}

		.control-btn {
			gap: 0.6rem;
		}
	}
</style>
