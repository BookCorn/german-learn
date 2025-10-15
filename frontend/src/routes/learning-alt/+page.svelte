<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const units = [
		{
			title: '单元 1: 德语基础',
			progress: 20,
			items: [
				{ title: '基本语法', description: '学习德语基本语法和常用词汇' },
				{ title: '常用词汇', description: '学习德语常用词汇和表达' },
				{ title: '基本对话', description: '学习德语基本对话和常用表达' }
			]
		},
		{
			title: '单元 2: 德语语法',
			progress: 50,
			items: [
				{ title: '名词', description: '学习德语名词和其变化' },
				{ title: '动词', description: '学习德语动词和其变位' },
				{ title: '形容词', description: '学习德语形容词和其变化' }
			]
		}
	];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>学习路径 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="learning">
	<svelte:fragment slot="header">
		<div class="learning-top-bar">
			<button aria-label="返回" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M224,128a8,8,0,0,1-8,8H59.31l58.35,58.34a8,8,0,0,1-11.32,11.32l-72-72a8,8,0,0,1,0-11.32l72-72a8,8,0,0,1,11.32,11.32L59.31,120H216A8,8,0,0,1,224,128Z"></path>
				</svg>
			</button>
			<h1 class="top-bar-title">学习</h1>
			<span class="icon-button ghost" aria-hidden="true"></span>
		</div>
	</svelte:fragment>

	{#each units as unit, index (`unit-${index}`)}
		<section class="unit">
			<h2>{unit.title}</h2>
			<div class="unit-progress">
				<div class="meta">
					<p>完成度</p>
					<p class="value">{unit.progress}%</p>
				</div>
				<div class="bar">
					<div class="fill" style={`--value:${unit.progress}%`} />
				</div>
			</div>

			<ul class="lesson-list">
				{#each unit.items as item, inner (`${unit.title}-${inner}`)}
					<li>
						<div class="flag-strip" />
						<div class="lesson">
							<p class="lesson-title">{item.title}</p>
							<p class="lesson-desc">{item.description}</p>
						</div>
						<svg fill="currentColor" height="18" viewBox="0 0 256 256" width="18" xmlns="http://www.w3.org/2000/svg">
							<path d="M181.66,133.66l-80,80a8,8,0,0,1-11.32-11.32L164.69,128,90.34,53.66a8,8,0,0,1,11.32-11.32l80,80A8,8,0,0,1,181.66,133.66Z"></path>
						</svg>
					</li>
				{/each}
			</ul>
		</section>
	{/each}
</PhoneFrame>

<style>
	.learning-top-bar {
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
		color: #121722;
	}

	.icon-button.ghost:hover {
		background: rgba(18, 23, 34, 0.08);
	}

	.unit {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.unit + .unit {
		margin-top: 1.4rem;
	}

	.unit h2 {
		margin: 0;
		font-size: 1.4rem;
		font-weight: 800;
		color: #0f172a;
	}

	.unit-progress {
		background: #ffffff;
		border-radius: 24px;
		padding: 1.2rem 1.4rem;
		box-shadow: 0 16px 30px rgba(0, 0, 0, 0.08);
		border: 1px solid rgba(15, 15, 15, 0.06);
		display: flex;
		flex-direction: column;
		gap: 0.9rem;
	}

	.unit-progress .meta {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.unit-progress .meta p {
		margin: 0;
		font-size: 0.92rem;
		color: #636c7c;
		font-weight: 600;
	}

	.unit-progress .value {
		color: #d60000;
		font-size: 1.05rem;
		font-weight: 700;
	}

	.unit-progress .bar {
		height: 0.95rem;
		border-radius: 999px;
		background: rgba(255, 214, 0, 0.32);
		overflow: hidden;
	}

	.unit-progress .fill {
		width: var(--value);
		height: 100%;
		background: linear-gradient(90deg, #000000 0%, #d60000 50%, #ffd400 100%);
		border-radius: inherit;
	}

	.lesson-list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
	}

	.lesson-list li {
		display: flex;
		align-items: center;
		gap: 1rem;
		background: #ffffff;
		border-radius: 22px;
		padding: 0.9rem 1.2rem;
		box-shadow: 0 14px 28px rgba(12, 25, 55, 0.1);
		border: 1px solid rgba(12, 12, 24, 0.05);
	}

	.flag-strip {
		width: 6px;
		height: 48px;
		border-radius: 6px;
		background: linear-gradient(180deg, #000000 0%, #dd0000 50%, #ffcc00 100%);
	}

	.lesson {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
	}

	.lesson-title {
		margin: 0;
		font-size: 1.05rem;
		font-weight: 700;
		color: #111827;
	}

	.lesson-desc {
		margin: 0;
		font-size: 0.9rem;
		color: #6b7280;
	}

	.lesson-list svg {
		color: #9ca3af;
	}
</style>
