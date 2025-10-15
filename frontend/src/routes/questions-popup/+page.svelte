<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const answers = [
		{ text: 'Hallo', selected: true },
		{ text: 'Guten Tag', selected: false },
		{ text: 'Auf Wiedersehen', selected: false },
		{ text: 'Tschüss', selected: false }
	];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>练习题 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="quiz-top-bar">
			<button aria-label="返回" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M224,128a8,8,0,0,1-8,8H59.31l58.35,58.34a8,8,0,0,1-11.32,11.32l-72-72a8,8,0,0,1,0-11.32l72-72a8,8,0,0,1,11.32,11.32L59.31,120H216A8,8,0,0,1,224,128Z"></path>
				</svg>
			</button>
			<h1 class="top-bar-title">学习</h1>
			<span class="icon-button ghost" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	<section class="progress">
		<span>第 1 天</span>
		<div class="bar">
			<div class="fill" style="width:20%" />
		</div>
	</section>

	<section class="question">
		<h2>如何用德语说“你好”？</h2>
	</section>

	<section class="answers">
		{#each answers as answer, index (`answer-${index}`)}
			<button class={`option ${answer.selected ? 'selected' : ''}`} type="button">
				<span>{answer.text}</span>
				<span class={`radio ${answer.selected ? 'active' : ''}`} aria-hidden="true" />
			</button>
		{/each}
	</section>

	<button class="submit" type="button">提交</button>
</PhoneFrame>

<style>
	.quiz-top-bar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 2.2rem 1.8rem 1.4rem;
	}

	.icon-button.ghost {
		background: rgba(0, 0, 0, 0.06);
		color: #1f1405;
		box-shadow: none;
		width: 2.75rem;
		height: 2.75rem;
		border-radius: 50%;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.icon-button.ghost:hover {
		background: rgba(0, 0, 0, 0.12);
	}

	.progress {
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
		color: #66533a;
		font-weight: 600;
	}

	.progress span {
		font-size: 0.9rem;
	}

	.progress .bar {
		height: 0.8rem;
		border-radius: 999px;
		background: rgba(0, 0, 0, 0.08);
		overflow: hidden;
	}

	.progress .fill {
		height: 100%;
		background: linear-gradient(90deg, #000000 0%, #dd8e00 50%, #ffcf40 100%);
	}

	.question {
		margin-top: 1.6rem;
	}

	.question h2 {
		margin: 0;
		font-size: 2rem;
		font-weight: 800;
		color: #2c2315;
	}

	.answers {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 2rem;
	}

	.option {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.1rem 1.4rem;
		border-radius: 18px;
		border: 2px solid rgba(0, 0, 0, 0.08);
		background: rgba(255, 255, 255, 0.9);
		box-shadow: 0 14px 28px rgba(63, 47, 15, 0.12);
		color: #2c2315;
		font-size: 1.05rem;
		font-weight: 700;
		cursor: pointer;
		transition: border 0.2s ease, background 0.2s ease, transform 0.2s ease;
	}

	.option:hover {
		transform: translateY(-2px);
	}

	.option.selected {
		border-color: #f2a60d;
		background: rgba(242, 166, 13, 0.12);
	}

	.radio {
		width: 18px;
		height: 18px;
		border-radius: 50%;
		border: 2px solid rgba(44, 35, 21, 0.25);
		position: relative;
	}

	.radio.active {
		border-color: #cf7a05;
	}

	.radio.active::after {
		content: '';
		position: absolute;
		inset: 3px;
		border-radius: 50%;
		background: #cf7a05;
	}

	.submit {
		margin-top: auto;
		width: 100%;
		padding: 1rem 1.2rem;
		border-radius: 999px;
		border: none;
		background: linear-gradient(135deg, #000000 0%, #dd8e00 50%, #ffcf40 100%);
		color: #1f1405;
		font-weight: 800;
		font-size: 1.05rem;
		cursor: pointer;
		box-shadow: 0 20px 34px rgba(63, 47, 15, 0.25);
	}
</style>
