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
			color: '#f20d0d',
			items: [
				{ title: '基本语法', description: '学习德语基本语法和常用词汇' },
				{ title: '常用词汇', description: '学习德语常用词汇和表达' },
				{ title: '基本对话', description: '学习德语基本对话和常用表达' }
			]
		},
		{
			title: '单元 2: 德语语法',
			progress: 50,
			color: '#f20d0d',
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
	<title>学习 - German Learn</title>
</svelte:head>

<PhoneFrame navItems={navItems} navActive="learning">
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
			<div class="progress-card">
				<div class="progress-header">
					<p>完成度</p>
					<p class="value" style={`color: ${unit.color};`}>{unit.progress}%</p>
				</div>
				<div class="progress-bar">
					<div class="track" style={`--progress-color: ${unit.color};`}>
						<div class="indicator" style={`--value: ${unit.progress}%;`}></div>
					</div>
				</div>
			</div>

			<ul class="lesson-list">
				{#each unit.items as item, subIndex (`${unit.title}-${subIndex}`)}
					<li>
						<div>
							<p class="lesson-title">{item.title}</p>
							<p class="lesson-desc">{item.description}</p>
						</div>
						<svg fill="currentColor" height="20" viewBox="0 0 256 256" width="20" xmlns="http://www.w3.org/2000/svg">
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
		color: #1f3552;
		box-shadow: none;
		width: 2.75rem;
		height: 2.75rem;
		border-radius: 50%;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.icon-button.ghost:hover {
		background: rgba(31, 53, 82, 0.08);
	}

	.unit {
		display: flex;
		flex-direction: column;
		gap: 1.2rem;
	}

	.unit + .unit {
		margin-top: 1.4rem;
	}

	.unit h2 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 800;
		color: #111827;
	}

	.progress-card {
		background: #ffffff;
		border-radius: 26px;
		padding: 1.4rem 1.6rem;
		box-shadow: 0 14px 28px rgba(18, 46, 84, 0.08);
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.progress-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.progress-header p {
		margin: 0;
		font-size: 0.95rem;
		font-weight: 600;
		color: #6d788c;
	}

	.progress-header .value {
		font-size: 1.1rem;
		font-weight: 700;
	}

	.progress-bar {
		width: 100%;
	}

	.track {
		position: relative;
		width: 100%;
		height: 0.9rem;
		border-radius: 999px;
		background: #fdeebe;
		overflow: hidden;
		box-shadow: inset 0 0 2px rgba(0, 0, 0, 0.05);
	}

	.indicator {
		position: absolute;
		inset: 0;
		width: var(--value);
		background: var(--progress-color);
		border-radius: inherit;
		transition: width 0.3s ease;
	}

	.lesson-list {
		list-style: none;
		margin: 0;
		padding: 0;
		background: #ffffff;
		border-radius: 26px;
		box-shadow: 0 14px 28px rgba(18, 46, 84, 0.08);
		overflow: hidden;
	}

	.lesson-list li {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 1.2rem 1.5rem;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	.lesson-list li + li {
		border-top: 1px solid rgba(216, 226, 236, 0.6);
	}

	.lesson-list li:hover {
		background: rgba(242, 13, 13, 0.05);
	}

	.lesson-title {
		margin: 0;
		font-size: 1.05rem;
		font-weight: 700;
		color: #1d2432;
	}

	.lesson-desc {
		margin: 0.3rem 0 0;
		font-size: 0.92rem;
		color: #6c768a;
	}

	.lesson-list svg {
		color: rgba(118, 128, 146, 0.9);
	}
</style>
