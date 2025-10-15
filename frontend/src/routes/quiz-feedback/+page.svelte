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
	<title>学习反馈 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="feedback-top-bar">
			<button aria-label="关闭" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M208,48a8,8,0,0,1,0,11.31L139.31,128,208,196.69A8,8,0,0,1,196.69,208L128,139.31,59.31,208A8,8,0,0,1,48,196.69L116.69,128,48,59.31A8,8,0,0,1,59.31,48L128,116.69,196.69,48A8,8,0,0,1,208,48Z"></path>
				</svg>
			</button>
			<h1 class="top-bar-title">学习</h1>
			<span class="icon-button ghost" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	<section class="feedback-hero">
		<div class="icon">
			<svg fill="#ffffff" height="36" viewBox="0 0 256 256" width="36" xmlns="http://www.w3.org/2000/svg">
				<path d="M205.66,194.34a8,8,0,0,1-11.32,11.32L128,139.31,61.66,205.66a8,8,0,0,1-11.32-11.32L116.69,128,50.34,61.66A8,8,0,0,1,61.66,50.34L128,116.69l66.34-66.35a8,8,0,0,1,11.32,11.32L139.31,128Z" />
			</svg>
		</div>
		<h2>回答错误</h2>
		<p>很遗憾，您的回答不正确。请再试一次，或查看下面的解释。</p>
	</section>

	<section class="explanation">
		<h3>解释</h3>
		<p>
			这个问题的正确答案是：<span>'Das ist ein Buch.'</span>
			这是一本书。
		</p>
	</section>

	<section class="actions">
		<button class="ghost" type="button">再试一次</button>
		<button class="primary" type="button">下一题</button>
	</section>
</PhoneFrame>

<style>
	.feedback-top-bar {
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

	.feedback-hero {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 1rem;
		margin-top: 0.5rem;
	}

	.feedback-hero .icon {
		width: 104px;
		height: 104px;
		border-radius: 999px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #ff2d2d 0%, #d60000 100%);
		box-shadow: 0 18px 36px rgba(214, 0, 0, 0.28);
	}

	.feedback-hero h2 {
		margin: 0;
		font-size: 2rem;
		font-weight: 800;
		color: #d60000;
	}

	.feedback-hero p {
		margin: 0;
		font-size: 1rem;
		color: #4a5564;
		line-height: 1.6;
		max-width: 280px;
	}

	.explanation {
		margin-top: 1.8rem;
		padding: 1.6rem;
		border-radius: 28px;
		background: #10131a;
		color: #ffffff;
		box-shadow: 0 16px 40px rgba(6, 14, 32, 0.35);
	}

	.explanation h3 {
		margin: 0 0 0.8rem;
		font-size: 1.2rem;
		font-weight: 700;
	}

	.explanation p {
		margin: 0;
		font-size: 1rem;
		line-height: 1.7;
		color: rgba(255, 255, 255, 0.88);
	}

	.explanation span {
		font-weight: 700;
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-top: auto;
	}

	.actions button {
		flex: 1;
		padding: 0.95rem 1rem;
		border-radius: 32px;
		font-weight: 700;
		font-size: 1rem;
		border: none;
		cursor: pointer;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.actions button:hover {
		transform: translateY(-2px);
	}

	.actions .ghost {
		background: rgba(255, 216, 73, 0.24);
		color: #47390a;
		box-shadow: inset 0 0 0 1px rgba(255, 204, 0, 0.35);
	}

	.actions .primary {
		background: linear-gradient(135deg, #ff2d2d 0%, #d60000 100%);
		color: #ffffff;
		box-shadow: 0 16px 34px rgba(214, 0, 0, 0.24);
	}
</style>
