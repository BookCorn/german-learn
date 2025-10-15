<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>短语练习 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="practice-top-bar">
			<button aria-label="返回" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M224,128a8,8,0,0,1-8,8H59.31l58.35,58.34a8,8,0,0,1-11.32,11.32l-72-72a8,8,0,0,1,0-11.32l72-72a8,8,0,0,1,11.32,11.32L59.31,120H216A8,8,0,0,1,224,128Z"></path>
				</svg>
			</button>
			<h1 class="top-bar-title">练习</h1>
			<span class="icon-button ghost" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	<section class="prompt">
		<h2>Guten Tag</h2>
		<p>你好</p>
	</section>

	<section class="player">
		<button aria-label="播放示例" class="play">
			<svg fill="currentColor" height="28" viewBox="0 0 256 256" width="28" xmlns="http://www.w3.org/2000/svg">
				<path d="M228.45,112.6,92.42,28.5A16,16,0,0,0,67.42,42V214a16,16,0,0,0,25,13.5l136-84.1a16,16,0,0,0,0-26.8Z"></path>
			</svg>
		</button>
		<div class="waveform" aria-hidden="true">
			{#each Array(12) as _, index}
				<div style={`animation-delay:${index * 0.08}s`} />
			{/each}
		</div>
	</section>

	<button aria-label="开始录音" class="record">
		<svg fill="currentColor" height="42" viewBox="0 0 256 256" width="42" xmlns="http://www.w3.org/2000/svg">
			<path d="M128,176a48,48,0,0,0,48-48V80a48,48,0,0,0-96,0v48A48,48,0,0,0,128,176Zm64-48a64.07,64.07,0,0,1-56,63.49V216h32a8,8,0,0,1,0,16H88a8,8,0,0,1,0-16h32V191.49A64.07,64.07,0,0,1,64,128a8,8,0,0,1,16,0,48,48,0,0,0,96,0,8,8,0,0,1,16,0Z"></path>
		</svg>
	</button>

	<section class="score">
		<div class="meta">
			<span>你的发音</span>
			<strong>85% 匹配</strong>
		</div>
		<div class="meter">
			<div class="fill" style="width:85%" />
		</div>
	</section>

	<button class="secondary" type="button">更多句子</button>
</PhoneFrame>

<style>
	.practice-top-bar {
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
		color: #11243f;
	}

	.icon-button.ghost:hover {
		background: rgba(17, 36, 63, 0.08);
	}

	.prompt {
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}

	.prompt h2 {
		margin: 0;
		font-size: 2.4rem;
		font-weight: 800;
		color: #101827;
	}

	.prompt p {
		margin: 0;
		font-size: 1.05rem;
		color: #5a6476;
	}

	.player {
		margin-top: 2.2rem;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1.4rem;
	}

	.play {
		border: none;
		width: 66px;
		height: 66px;
		border-radius: 50%;
		background: linear-gradient(135deg, #1a72ff 0%, #0c52d4 100%);
		color: #ffffff;
		box-shadow: 0 16px 34px rgba(16, 82, 212, 0.28);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.waveform {
		display: flex;
		align-items: center;
		height: 60px;
		gap: 6px;
		color: rgba(16, 82, 212, 0.35);
	}

	.waveform div {
		width: 5px;
		border-radius: 999px;
		background: currentColor;
		animation: wave 1.2s ease-in-out infinite;
	}

	@keyframes wave {
		0%, 100% {
			height: 6px;
		}
		50% {
			height: 100%;
		}
	}

	.record {
		margin: 2.6rem auto 0;
		width: 110px;
		height: 110px;
		border-radius: 50%;
		border: none;
		background: linear-gradient(135deg, #ff2f54 0%, #c60028 100%);
		color: #ffffff;
		box-shadow: 0 24px 48px rgba(198, 0, 40, 0.35);
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: transform 0.2s ease;
	}

	.record:hover {
		transform: translateY(-3px);
	}

	.score {
		margin-top: 2.8rem;
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
	}

	.score .meta {
		display: flex;
		justify-content: space-between;
		font-size: 0.95rem;
		color: #60697a;
		font-weight: 600;
	}

	.score .meta strong {
		color: #0c52d4;
	}

	.meter {
		height: 0.9rem;
		border-radius: 999px;
		background: rgba(16, 82, 212, 0.2);
		overflow: hidden;
	}

	.meter .fill {
		height: 100%;
		background: linear-gradient(90deg, #0c52d4 0%, #1a72ff 100%);
		border-radius: inherit;
	}

	.secondary {
		margin-top: 2.4rem;
		width: 100%;
		padding: 0.9rem 1.2rem;
		border-radius: 16px;
		border: none;
		background: rgba(16, 82, 212, 0.08);
		color: #0c52d4;
		font-weight: 700;
		font-size: 0.98rem;
		cursor: pointer;
	}
</style>
