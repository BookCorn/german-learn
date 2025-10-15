<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const summaryCards = [
		{
			title: '得分',
			value: '95/100',
			bg: '#e6e6e8',
			color: '#141827'
		},
		{
			title: '学习时间',
			value: '30 分钟',
			bg: '#ffe3e3',
			color: '#f20d0d'
		},
		{
			title: '新掌握词汇',
			value: '15 个',
			bg: '#fff5cb',
			color: '#f4b000'
		}
	];

	const vocabulary = ['das Buch', 'die Katze', 'der Hund', 'das Haus', 'die Schule', 'der Lehrer'];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>课程总结 - German Learn</title>
</svelte:head>

<PhoneFrame navItems={navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="summary-top-bar">
			<button aria-label="关闭" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M208,48a8,8,0,0,1,0,11.31L139.31,128,208,196.69A8,8,0,0,1,196.69,208L128,139.31,59.31,208A8,8,0,0,1,48,196.69L116.69,128,48,59.31A8,8,0,0,1,59.31,48L128,116.69,196.69,48A8,8,0,0,1,208,48Z"></path>
				</svg>
			</button>
			<h1 class="top-bar-title">课程总结</h1>
			<span class="icon-button ghost" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	<section class="hero">
		<img alt="课程奖杯" src="https://images.unsplash.com/photo-1618005198919-d3d4b5a92eee?auto=format&fit=crop&w=320&q=80" />
		<h2>恭喜您完成了本课程！</h2>
		<p>您在本课程中表现出色，以下是您的学习总结。</p>
	</section>

	<section class="card-stack">
		{#each summaryCards as card, index (`card-${index}`)}
			<div class="summary-card" style={`background:${card.bg}; color:${card.color};`}>
				<p>{card.title}</p>
				<strong>{card.value}</strong>
			</div>
		{/each}
	</section>

	<section class="vocab">
		<h3>新掌握词汇</h3>
		<ul>
			{#each vocabulary as word, index (`word-${index}`)}
				<li>
					<div class="badge">
						<svg fill="currentColor" height="18" viewBox="0 0 256 256" width="18" xmlns="http://www.w3.org/2000/svg">
							<path d="M216,40H72A32,32,0,0,0,40,72V216a8,8,0,0,0,13.12,6.19L96,189.47l42.88,32.72A8,8,0,0,0,152,216V144h64a8,8,0,0,0,8-8V48A8,8,0,0,0,216,40Z"></path>
						</svg>
					</div>
					<span>{word}</span>
				</li>
			{/each}
		</ul>
	</section>

	<button class="primary-button" type="button">继续学习</button>
</PhoneFrame>

<style>
	.summary-top-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 2.2rem 1.8rem 1.4rem;
	}

	.icon-button.ghost {
		background: transparent;
		box-shadow: none;
		width: 2.75rem;
		height: 2.75rem;
		border-radius: 999px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		color: #1f2b3f;
	}

	.icon-button.ghost:hover {
		background: rgba(31, 43, 63, 0.08);
	}

	.hero {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 1rem;
	}

	.hero img {
		width: 120px;
		height: 120px;
		border-radius: 24px;
		object-fit: cover;
		box-shadow: 0 18px 40px rgba(18, 46, 84, 0.14);
	}

	.hero h2 {
		margin: 0;
		font-size: 1.8rem;
		font-weight: 800;
		color: #0f172a;
	}

	.hero p {
		margin: 0;
		font-size: 0.95rem;
		color: #516072;
	}

	.card-stack {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.summary-card {
		padding: 1.4rem 1.6rem;
		border-radius: 28px;
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
		box-shadow: 0 14px 32px rgba(18, 46, 84, 0.08);
	}

	.summary-card p {
		margin: 0;
		font-size: 1.05rem;
		font-weight: 600;
		opacity: 0.7;
	}

	.summary-card strong {
		font-size: 1.7rem;
		font-weight: 800;
	}

	.vocab {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.vocab h3 {
		margin: 0;
		font-size: 1.3rem;
		font-weight: 800;
		color: #131b2c;
	}

	.vocab ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.vocab li {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.3rem;
		border-radius: 24px;
		background: #ffffff;
		box-shadow: 0 12px 28px rgba(18, 46, 84, 0.06);
	}

	.badge {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #ffd4d4;
		color: #d63636;
	}

	.primary-button {
		margin-top: 1.6rem;
		width: 100%;
		padding: 1.05rem 1.4rem;
		border: none;
		border-radius: 30px;
		background: linear-gradient(135deg, #ffd80a 0%, #ffb300 100%);
		color: #191b22;
		font-weight: 700;
		font-size: 1.05rem;
		cursor: pointer;
		box-shadow: 0 16px 34px rgba(255, 182, 25, 0.28);
	}

	.primary-button:hover {
		filter: brightness(1.05);
	}
</style>
