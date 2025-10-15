<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const categories = ['全部', '已学习', '未学习'];

	const words = [
		{ word: 'Hallo', meaning: '你好', example: 'Hallo, wie geht’s?' },
		{ word: 'Auf Wiedersehen', meaning: '再见', example: 'Auf Wiedersehen, bis bald!' },
		{ word: 'Danke', meaning: '谢谢', example: 'Danke für deine Hilfe.' },
		{ word: 'Bitte', meaning: '请 / 不客气', example: 'Bitte, nimm Platz.' },
		{ word: 'Ja', meaning: '是', example: 'Ja, das stimmt.' },
		{ word: 'Nein', meaning: '不', example: 'Nein, das ist nicht richtig.' }
	];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>词汇表 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="lexicon-top-bar">
			<button aria-label="返回" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M224,128a8,8,0,0,1-8,8H59.31l58.35,58.34a8,8,0,0,1-11.32,11.32l-72-72a8,8,0,0,1,0-11.32l72-72a8,8,0,0,1,11.32,11.32L59.31,120H216A8,8,0,0,1,224,128Z"></path>
				</svg>
			</button>
			<h1 class="top-bar-title">词汇表</h1>
			<span class="icon-button ghost" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	<div class="search">
		<span class="icon">
			<svg fill="currentColor" height="16" viewBox="0 0 256 256" width="16" xmlns="http://www.w3.org/2000/svg">
				<path d="M232.49,215.51l-50.25-50.25a88,88,0,1,0-16.97,16.97l50.25,50.25a12,12,0,0,0,16.97-16.97ZM44,112a68,68,0,1,1,68,68A68.08,68.08,0,0,1,44,112Z"></path>
			</svg>
		</span>
		<input placeholder="搜索词汇" type="search" />
	</div>

	<div class="filters">
		{#each categories as category, index (`cat-${index}`)}
			<button class={index === 0 ? 'active' : ''} type="button">
				<span>{category}</span>
				<svg fill="currentColor" height="16" viewBox="0 0 256 256" width="16" xmlns="http://www.w3.org/2000/svg">
					<path d="M213.66,98.34l-80,80a8,8,0,0,1-11.32,0l-80-80a8,8,0,0,1,11.32-11.32L128,162.75l74.34-75.73a8,8,0,1,1,11.32,11.32Z"></path>
				</svg>
			</button>
		{/each}
	</div>

	<section class="list">
		<h2>基础</h2>
		<ul>
			{#each words as word, index (`word-${index}`)}
				<li>
					<div class="details">
						<p class="term">{word.word}</p>
						<p class="meaning">{word.meaning}</p>
						<p class="example">{word.example}</p>
					</div>
					<button aria-label="播放发音" class="speak" type="button">
						<svg fill="currentColor" height="20" viewBox="0 0 256 256" width="20" xmlns="http://www.w3.org/2000/svg">
							<path d="M224,128a72,72,0,0,1-72,72,8,8,0,0,1,0-16,56,56,0,0,0,0-112,8,8,0,0,1,0-16A72,72,0,0,1,224,128ZM80,90.56L114.26,56H120a8,8,0,0,1,8,8V192a8,8,0,0,1-8,8h-5.74L80,165.44ZM56,96H32a8,8,0,0,0-8,8v48a8,8,0,0,0,8,8H56l48,48V48Z"></path>
						</svg>
					</button>
				</li>
			{/each}
		</ul>
	</section>
</PhoneFrame>

<style>
	.lexicon-top-bar {
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
		border-radius: 50%;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		color: #1a2336;
	}

	.icon-button.ghost:hover {
		background: rgba(26, 35, 54, 0.08);
	}

	.search {
		position: relative;
		display: flex;
		align-items: center;
	}

	.search .icon {
		position: absolute;
		left: 1.1rem;
		color: #7d8698;
	}

	.search input {
		width: 100%;
		height: 48px;
		border: none;
		border-radius: 18px;
		padding: 0 1.2rem 0 3.2rem;
		background: rgba(21, 36, 60, 0.08);
		color: #182433;
		font-size: 0.95rem;
	}

	.filters {
		display: flex;
		gap: 0.6rem;
		overflow-x: auto;
		padding-bottom: 0.3rem;
	}

	.filters button {
		border: none;
		border-radius: 18px;
		padding: 0.6rem 1rem;
		background: rgba(21, 36, 60, 0.08);
		color: #1a2336;
		font-weight: 600;
		display: inline-flex;
		align-items: center;
		gap: 0.35rem;
		cursor: pointer;
	}

	.filters button.active {
		background: #186fff;
		color: #ffffff;
		box-shadow: 0 14px 28px rgba(24, 111, 255, 0.25);
	}

	.list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.list h2 {
		margin: 0;
		font-size: 1.1rem;
		font-weight: 800;
		color: #111b2c;
	}

	.list ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
	}

	.list li {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.2rem;
		border-radius: 18px;
		background: #ffffff;
		box-shadow: 0 16px 32px rgba(17, 29, 49, 0.1);
		border: 1px solid rgba(17, 29, 49, 0.06);
	}

	.details {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.term {
		margin: 0;
		font-size: 1.05rem;
		font-weight: 700;
		color: #111b2c;
	}

	.meaning {
		margin: 0;
		font-size: 0.9rem;
		color: #5f6c80;
	}

	.example {
		margin: 0;
		font-size: 0.85rem;
		color: #8c95a6;
	}

	.speak {
		width: 42px;
		height: 42px;
		border-radius: 14px;
		border: none;
		background: rgba(24, 111, 255, 0.12);
		color: #186fff;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
	}
</style>
