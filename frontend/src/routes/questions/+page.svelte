<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const options = [
		{ title: '轻松 (10 分钟)', subtitle: 'Leicht · 10 Minuten', selected: false },
		{ title: '常规 (20 分钟)', subtitle: 'Normal · 20 Minuten', selected: true },
		{ title: '认真 (30 分钟)', subtitle: 'Ernsthaft · 30 Minuten', selected: false },
		{ title: '强化 (45 分钟)', subtitle: 'Intensiv · 45 Minuten', selected: false }
	];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>设定学习目标 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<div class="overlay">
		<div class="dialog">
			<div class="icon">
				<svg fill="currentColor" height="32" viewBox="0 0 256 256" width="32" xmlns="http://www.w3.org/2000/svg">
					<path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm4,168a4,4,0,0,1-8,0V152a4,4,0,0,1,8,0Zm-4-56a24,24,0,0,1-20.39-36.62l15.17-25.3a6,6,0,1,1,10.32,6.24l-15.17,25.3a12,12,0,1,0,20.78,12A6,6,0,0,1,151,122.62,24,24,0,0,1,128,136Z"></path>
				</svg>
			</div>
			<h1>设定每日学习目标</h1>
			<p>Stelle dein tägliches Lernziel ein</p>

			<ul>
				{#each options as option, index (`goal-${index}`)}
					<li class={option.selected ? 'selected' : ''}>
						<div class="label">
							<strong>{option.title}</strong>
							<span>{option.subtitle}</span>
						</div>
						<span class={`radio ${option.selected ? 'active' : ''}`} aria-hidden="true" />
					</li>
				{/each}
			</ul>

			<button class="confirm" type="button">继续</button>
		</div>
	</div>
</PhoneFrame>

<style>
	.overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(8, 16, 28, 0.35);
		backdrop-filter: blur(6px);
		padding: 1.5rem;
	}

	.dialog {
		width: min(360px, 100%);
		background: #f7f9fc;
		border-radius: 24px;
		padding: 2rem 1.6rem;
		box-shadow: 0 30px 60px rgba(12, 24, 48, 0.28);
		display: flex;
		flex-direction: column;
		gap: 1.3rem;
		position: relative;
	}

	.icon {
		width: 72px;
		height: 72px;
		border-radius: 18px;
		background: rgba(24, 112, 255, 0.12);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #186fff;
		margin: 0 auto;
	}

	h1 {
		margin: 0;
		font-size: 1.4rem;
		text-align: center;
		font-weight: 800;
		color: #101c30;
	}

	p {
		margin: 0;
		text-align: center;
		color: #6d7687;
		font-size: 0.95rem;
	}

	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
	}

	li {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.95rem 1rem;
		border-radius: 18px;
		border: 1px solid rgba(24, 36, 60, 0.08);
		background: #ffffff;
		box-shadow: 0 14px 28px rgba(12, 24, 48, 0.08);
		cursor: pointer;
		transition: border 0.2s ease, background 0.2s ease;
	}

	li.selected {
		border-color: rgba(24, 112, 255, 0.6);
		background: rgba(24, 112, 255, 0.12);
	}

	.label {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.label strong {
		font-size: 1rem;
		color: #11243f;
	}

	.label span {
		font-size: 0.85rem;
		color: #6d7687;
	}

	.radio {
		width: 18px;
		height: 18px;
		border-radius: 50%;
		border: 2px solid rgba(24, 36, 60, 0.25);
		position: relative;
	}

	.radio.active {
		border-color: rgba(24, 112, 255, 0.8);
	}

	.radio.active::after {
		content: '';
		position: absolute;
		inset: 3px;
		border-radius: 50%;
		background: #186fff;
	}

	.confirm {
		width: 100%;
		padding: 0.95rem 1.2rem;
		border-radius: 16px;
		border: none;
		background: linear-gradient(135deg, #186fff 0%, #0c52d4 100%);
		color: #ffffff;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		box-shadow: 0 18px 36px rgba(12, 82, 212, 0.28);
	}
</style>
