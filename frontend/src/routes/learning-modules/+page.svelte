<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const categories = [
		{ title: 'Wortschatz', subtitle: '词汇', icon: 'book', link: '/flashcards' },
		{ title: 'Grammatik', subtitle: '语法', icon: 'text' },
		{ title: 'Hörverstehen', subtitle: '听力', icon: 'headphones' },
		{ title: 'Sprechen', subtitle: '口语', icon: 'mic' }
	];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>学习模块 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="modules-top-bar">
			<button aria-label="返回" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M224,128a8,8,0,0,1-8,8H59.31l58.35,58.34a8,8,0,0,1-11.32,11.32l-72-72a8,8,0,0,1,0-11.32l72-72a8,8,0,0,1,11.32,11.32L59.31,120H216A8,8,0,0,1,224,128Z"></path>
				</svg>
			</button>
			<h1 class="top-bar-title">学习</h1>
			<span class="icon-button ghost" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	<section class="modules-hero">
		<h2>Lernkategorien</h2>
		<p>选择你想专注的学习方向</p>
	</section>

	<section class="modules-grid">
		{#each categories as cat, index (`cat-${index}`)}
			<svelte:element
				this={cat.link ? 'a' : 'div'}
				class="category-card"
				href={cat.link ?? undefined}
			>
				<span class="icon-wrap">
					{#if cat.icon === 'book'}
						<svg fill="none" height="28" viewBox="0 0 24 24" width="28" xmlns="http://www.w3.org/2000/svg">
							<path d="M6 4h11a1 1 0 0 1 1 1v13.5a.5.5 0 0 1-.76.43l-1.48-.89a1 1 0 0 0-1.02 0l-1.48.89a.5.5 0 0 1-.76-.43V6H6a1 1 0 0 1 0-2Z" fill="#1a6dff" />
							<path d="M4 6h10a1 1 0 0 1 1 1v12.5a.5.5 0 0 1-.76.43l-1.48-.89a1 1 0 0 0-1.02 0l-1.48.89a.5.5 0 0 1-.76-.43V8H4a1 1 0 1 1 0-2Z" fill="#3a87ff" />
						</svg>
					{:else if cat.icon === 'text'}
						<svg fill="none" height="28" viewBox="0 0 24 24" width="28" xmlns="http://www.w3.org/2000/svg">
							<path d="M5 6h14" stroke="#1a6dff" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
							<path d="M8 12h8" stroke="#1a6dff" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
							<path d="M10 18h4" stroke="#1a6dff" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
						</svg>
					{:else if cat.icon === 'headphones'}
						<svg fill="none" height="28" viewBox="0 0 24 24" width="28" xmlns="http://www.w3.org/2000/svg">
							<path d="M5 17v-2a7 7 0 0 1 14 0v2" stroke="#1a6dff" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
							<rect fill="#3a87ff" height="6" rx="2" width="4" x="3" y="15" />
							<rect fill="#3a87ff" height="6" rx="2" width="4" x="17" y="15" />
						</svg>
					{:else}
						<svg fill="none" height="28" viewBox="0 0 24 24" width="28" xmlns="http://www.w3.org/2000/svg">
							<path d="M12 3a3 3 0 0 1 3 3v5a3 3 0 0 1-6 0V6a3 3 0 0 1 3-3Z" fill="#3a87ff" />
							<path d="M8 11v1a4 4 0 0 0 8 0v-1" stroke="#1a6dff" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
							<path d="M12 19v2" stroke="#1a6dff" stroke-linecap="round" stroke-width="2" />
						</svg>
					{/if}
				</span>
				<span class="title">{cat.title}</span>
				<span class="subtitle">{cat.subtitle}</span>
			</svelte:element>
		{/each}
	</section>
</PhoneFrame>

<style>
	.modules-top-bar {
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

	.modules-hero {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.modules-hero h2 {
		margin: 0;
		font-size: 1.8rem;
		font-weight: 800;
		color: #101827;
	}

	.modules-hero p {
		margin: 0;
		font-size: 0.95rem;
		color: #5f6c7f;
	}

	.modules-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1.1rem;
	}

	.modules-grid .category-card {
		border: none;
		border-radius: 28px;
		background: #e3eeff;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.65rem;
		padding: 1.6rem 1rem;
		cursor: pointer;
		box-shadow: 0 14px 28px rgba(17, 67, 165, 0.08);
		color: #101b2f;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
		text-decoration: none;
	}

	.modules-grid .category-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 20px 32px rgba(17, 67, 165, 0.12);
	}

	.icon-wrap {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		border-radius: 18px;
		background: rgba(25, 112, 255, 0.12);
	}

	.title {
		font-size: 1.1rem;
		font-weight: 700;
	}

	.subtitle {
		font-size: 0.88rem;
		color: #3f4a5c;
	}
</style>
