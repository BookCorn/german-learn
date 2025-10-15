<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const cards = [
		{ title: '得分', value: '95/100', class: 'score' },
		{ title: '学习时间', value: '30 分钟', class: 'time' },
		{ title: '新掌握词汇', value: '15 个', class: 'words' }
	];

	const vocabulary = ['das Buch', 'die Katze', 'der Hund', 'das Haus', 'die Schule', 'der Lehrer'];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>课程总结（德式） - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
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
		<div class="badge">
			<img alt="奖杯" src="https://lh3.googleusercontent.com/aida-public/AB6AXuCBvO9xuwWK6vqvUF2BPiIueSWpLZr_uyZhlqT1ZJnZ8SeCy2Kgbj8wCqF0nmtP7A8CCG5o80gg_8pjdG5V77E7oAm1avkwcrkAbngV33lEqRAOx55H0XSiU-88ky5phsIzUjo56K1dyVf_a4g8ucOHW712KomFZaGg2Y2_NhidI-645AdOpw6T5t-lZJl4s9S2JZIboykGRJ9dpTR32I-tppWEwTTCNKYUEFklCHYPqDb_u-4pOTDxm8MAJRos4tGO1iajKGXtQ_cS" />
		</div>
		<h2>恭喜您完成了本课程！</h2>
		<p>您在本课程中表现出色，以下是您的学习总结。</p>
	</section>

	<section class="card-grid">
		{#each cards as card, index (`alt-card-${index}`)}
			<div class={`summary-card ${card.class}`}>
				<p>{card.title}</p>
				<strong>{card.value}</strong>
			</div>
		{/each}
	</section>

	<section class="vocab">
		<h3>新掌握词汇</h3>
		<ul>
			{#each vocabulary as word, index (`alt-word-${index}`)}
				<li>
					<div class="chip">
						<span class="flag" />
						<span class="icon">
							<svg fill="currentColor" height="18" viewBox="0 0 256 256" width="18" xmlns="http://www.w3.org/2000/svg">
								<path d="M216,40H72A32,32,0,0,0,40,72V216a8,8,0,0,0,13.12,6.19L96,189.47l42.88,32.72A8,8,0,0,0,152,216V144h64a8,8,0,0,0,8-8V48A8,8,0,0,0,216,40Z"></path>
							</svg>
						</span>
					</div>
					<span>{word}</span>
				</li>
			{/each}
		</ul>
	</section>

	<button class="cta" type="button">继续学习</button>
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
		color: #101722;
	}

	.icon-button.ghost:hover {
		background: rgba(16, 23, 34, 0.08);
	}

	.hero {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 0.8rem;
		margin-top: 0.6rem;
	}

	.badge {
		width: 120px;
		height: 120px;
		background: linear-gradient(135deg, #000000 0%, #dd0000 60%, #ffd000 100%);
		border-radius: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 14px;
		box-shadow: 0 22px 36px rgba(8, 20, 48, 0.25);
	}

	.badge img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		border-radius: 22px;
	}

	.hero h2 {
		margin: 0;
		font-size: 1.7rem;
		font-weight: 900;
		color: #0f1628;
	}

	.hero p {
		margin: 0;
		font-size: 0.95rem;
		color: #5e6677;
	}

	.card-grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 0.8rem;
	}

	.summary-card {
		padding: 1rem 1.1rem;
		border-radius: 20px;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		text-align: center;
		box-shadow: 0 14px 32px rgba(12, 25, 55, 0.12);
	}

	.summary-card p {
		margin: 0;
		font-size: 0.9rem;
		font-weight: 600;
		color: rgba(16, 23, 34, 0.7);
	}

	.summary-card strong {
		font-size: 1.4rem;
		font-weight: 800;
	}

	.summary-card.score {
		background: #f1f1f3;
		color: #101722;
	}

	.summary-card.time {
		background: rgba(221, 0, 0, 0.1);
		color: #bb0000;
	}

	.summary-card.words {
		background: rgba(255, 204, 0, 0.18);
		color: #e59c00;
	}

	.vocab {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 1.6rem;
	}

	.vocab h3 {
		margin: 0;
		font-size: 1.2rem;
		font-weight: 800;
		color: #121828;
	}

	.vocab ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.9rem;
	}

	.vocab li {
		display: flex;
		align-items: center;
		gap: 0.9rem;
		padding: 0.9rem 1.1rem;
		border-radius: 20px;
		background: #ffffff;
		box-shadow: 0 14px 30px rgba(8, 20, 48, 0.08);
		border: 1px solid rgba(12, 25, 50, 0.08);
	}

	.chip {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 42px;
		height: 42px;
		border-radius: 14px;
		background: rgba(221, 0, 0, 0.12);
	}

	.flag {
		position: absolute;
		bottom: -6px;
		width: 30px;
		height: 6px;
		border-radius: 99px;
		background: linear-gradient(90deg, #000000 0%, #dd0000 50%, #ffcc00 100%);
	}

	.icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		color: #d00000;
	}

	.vocab li span:last-child {
		font-size: 1rem;
		font-weight: 700;
		color: #141c2c;
	}

	.cta {
		margin-top: 1.6rem;
		width: 100%;
		padding: 1rem 1.3rem;
		border: none;
		border-radius: 999px;
		background: linear-gradient(135deg, #000000 0%, #dd0000 50%, #ffcc00 100%);
		color: #0f1218;
		font-weight: 800;
		font-size: 1.05rem;
		cursor: pointer;
		box-shadow: 0 18px 34px rgba(12, 20, 32, 0.24);
	}
</style>
