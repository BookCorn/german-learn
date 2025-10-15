<script lang="ts">
import PhoneFrame from '$lib/components/PhoneFrame.svelte';

const days = [
		{ label: '一', state: 'done' },
		{ label: '二', state: 'done' },
		{ label: '三', state: 'done' },
		{ label: '四', state: 'current' },
		{ label: '五', state: 'upcoming' },
		{ label: '六', state: 'upcoming' },
		{ label: '日', state: 'upcoming' }
	];

	const modules = [
		{
			title: '学习基本单词',
			image:
				'https://lh3.googleusercontent.com/aida-public/AB6AXuBb1hQPnof5rHxyQGeSJIqW3egwIaUV-l8rpu87UTmpx9pbiMh_Bqk_VWpzlZPbuULi5eIcPtZTCyw0pAYmmXlORABH0oWaLDEEsuJ4Gd78qUuW6mUQgosPHhgPW-YwlQQX0Qq2Tb50-MkVtF3QmAVXmNLeicNuHyJrR-u3qTAOQ0XO4vHeFLhIpB2Lm8Xo1XgHbPL3py4UJrsvrTg9hRSF2xtFSOE1dMI2cEfN17nN-Q296BeYSOvi19JqK7YTFF-QWZxQAmhOLuIA'
		},
		{
			title: '学习常用语句',
			image:
				'https://lh3.googleusercontent.com/aida-public/AB6AXuBJrR9_U9KYgjoE7VetMdBh639wy0DS_pSRbneT7pxWbo9C3sdS3cdVwJrYVcjiDnSVlL5OJTpcKX-2RVAZ8RvZvgreWnD7moVzu8IIkyEyBLdV_zt0kbnQLqD3HfwnFgVwk9Ij8wPbAQUN_XVjxwjT5bmrOrGF9Kp_nYT2v2_ya1hIouTTJ9w1ZA-tid29ipF-CGfIlUXoHhd3XinS9WX7rDYX8NhC5tphppzmYfcuaRr1_NoYHMKZaNRmZyVwff5y4O7jJteCvB_8'
		},
		{
			title: '学习语法',
			image:
				'https://lh3.googleusercontent.com/aida-public/AB6AXuDlVzjGTzNBWW0MMCHY804N3XZK6fmewuHdyFLsrA8v6vY3U4KBYmUF-2mT7YefwcbYO_vkGaIm7hfLt0FMpMj4vmee8LZzfN0Jkb5nBl3N7V8xVaoBxTJbS3_Dv1BUnWG9BC0E3Av9Y1v60JFIt2zqN0_1LyuFh_qjrmspEVtOarkCM40AJj4v2rG3i5I_zBR3HoQES-sAnT7fiTLdn525Fa--qcqG52yNqmdF00CK_Lk6NiC6dFnbg764mvqAChXGjNKN-OLp_K3D'
		}
	];

const tasks = [
	{ title: '完成每天的学习任务', xp: '1000 XP' },
	{ title: '完成每天的学习任务', xp: '1000 XP', variant: 'secondary' }
];

const navItems = [
	{ id: 'home', href: '/', label: '主页' },
	{ id: 'learning', href: '/learning', label: '学习模块' },
	{ id: 'profile', href: '/profile', label: '个人中心' }
];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>主页 - German Learn</title>
	<link href="https://fonts.googleapis.com" rel="preconnect" />
	<link crossorigin="" href="https://fonts.gstatic.com" rel="preconnect" />
	<link
		href="https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<PhoneFrame {navItems} navActive="home">
	<svelte:fragment slot="header">
		<div class="top-bar">
			<span aria-hidden="true" class="ghost-button"></span>
			<h1 class="top-bar-title">主页</h1>
			<button aria-label="打开设置" class="icon-button">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path
						d="M216.7,138.78l-14.94-18.67a8,8,0,0,0-6.84-2.88,74.5,74.5,0,0,1-10.37,0,8,8,0,0,0-6.84,2.88L163.6,138.78A91.81,91.81,0,0,1,148.8,172l-2.61,23.49a8,8,0,0,0,3.94,6.72,107.13,107.13,0,0,0,26.24,10.87,8,8,0,0,0,7.06-1.49l18.64-14.92a8,8,0,0,0,2.88-6.84,74.5,74.5,0,0,1,0-10.37,8,8,0,0,0-2.88-6.84ZM128,80a48,48,0,1,0,48,48A48.05,48.05,0,0,0,128,80Zm0,80a32,32,0,1,1,32-32A32,32,0,0,1,128,160Zm35.6-84.3L178.52,78a8,8,0,0,0-5.1-2.64,74.11,74.11,0,0,1-6.14-6.14,8,8,0,0,0-2.64-5.1l-2.51-22.58a91.32,91.32,0,0,1-15-6.23l-17.74,14.19a8,8,0,0,0-5.48,1.74,73.93,73.93,0,0,1-8.68,0,8,8,0,0,0-5.48-1.74L100.45,40.2a91.57,91.57,0,0,1-15,6.23L82.89,69a8,8,0,0,0-2.64,5.1,74.11,74.11,0,0,1-6.14,6.14,8,8,0,0,0-5.1,2.64l-22.58,2.51a91.32,91.32,0,0,1-6.23,15L54.36,100.5A8,8,0,0,0,56.1,106a73.93,73.93,0,0,1,0,8.68,8,8,0,0,0-1.74,5.48L40.2,138a91.57,91.57,0,0,1,6.23,15l22.58-2.51a8,8,0,0,0,5.1-2.64,74.11,74.11,0,0,1,6.14-6.14,8,8,0,0,0,2.64-5.1L85.4,113.43a91.32,91.32,0,0,1,15-6.23L118.14,93a8,8,0,0,0,5.48-1.74,73.93,73.93,0,0,1,8.68,0,8,8,0,0,0,5.48,1.74L155.55,107.8a91.57,91.57,0,0,1,15,6.23L173.11,91.45a8,8,0,0,0,2.64-5.1A74.11,74.11,0,0,1,182,79.52a8,8,0,0,0,5.1-2.64l22.58-2.51a91.32,91.32,0,0,1,6.23,15l-14.19,17.74a8,8,0,0,0-1.74,5.48,73.93,73.93,0,0,1,0,8.68A8,8,0,0,0,201.4,138Z"
					/>
				</svg>
			</button>
		</div>
	</svelte:fragment>

	<section class="profile">
		<img
			alt="Lukas 头像"
			class="avatar"
			src="https://lh3.googleusercontent.com/aida-public/AB6AXuCUTaJw1VMMAo6XMzmOdx_fKnNE0uSbom6YRmG-GHaMAfLDYK2kDlLE7AT8dG362aorOdnPLuQMRKJAzrUbJTq93XFGgBYm0mEhGPjS3cTQfq1A4q1J_QFQ-RW0fDt7b7qN3rV4wvuifD1Q8kXxgHx_DuCJYz9FFby1mlomFv11IaR4kRLsDR6mhdmXUrfT5sv6o3-GeUWBxjefstw63XRJZGbQGgYA0nWyQZ7JnXWkqBBbE4dA2hwvb9P_GjNZDfqDlQ62c_8GA0tS"
		/>
		<div class="profile-info">
			<p class="name">Lukas</p>
			<p class="meta">Level 1</p>
			<p class="meta">1000 XP</p>
		</div>
	</section>

	<section class="card daily-goal">
		<header class="daily-goal-header">
			<h2 class="section-heading">每日学习目标</h2>
			<button class="link-button" type="button">设定目标</button>
		</header>

		<ul class="goal-days">
			{#each days as day, index (day.label + index)}
				<li class={`goal-day ${day.state}`}>
					<span class="day-label">{day.label}</span>
					<span class="day-indicator">
						{#if day.state === 'done'}
							<svg fill="currentColor" height="18" viewBox="0 0 256 256" width="18" xmlns="http://www.w3.org/2000/svg">
								<path
									d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm45.66,85.66-56,56a8,8,0,0,1-11.32,0l-24-24a8,8,0,1,1,11.32-11.32L112,148.69l50.34-50.35a8,8,0,0,1,11.32,11.32Z"
								/>
							</svg>
						{:else if day.state === 'current'}
							<span class="current-dot"></span>
						{/if}
					</span>
				</li>
			{/each}
		</ul>

	<p class="goal-message">太棒了！您已连续学习3天！</p>
</section>

	<section class="cta-card">
		<div class="cta-text">
			<h2 class="cta-title">继续记忆词汇</h2>
			<p class="cta-subtitle">打开闪卡快速复习今天的重点词汇。</p>
		</div>
		<a class="cta-button" href="/flashcards">开始复习</a>
	</section>

	<section class="modules">
		<h2 class="section-heading">学习模块</h2>
		<div class="modules-grid">
			{#each modules as module, index (module.title + index)}
				<div class="module-card">
					<div class="module-image">
						<img alt={module.title} src={module.image} />
					</div>
					<p>{module.title}</p>
				</div>
			{/each}
		</div>
	</section>

	<section class="plan">
		<h2 class="section-heading">学习计划</h2>
		<ul class="plan-list">
			{#each tasks as task, index (`task-${index}`)}
				<li class={`task-card ${task.variant ?? ''}`}>
					<div class="task-icon">
						{#if task.variant === 'secondary'}
							<svg fill="currentColor" height="28" viewBox="0 0 256 256" width="28" xmlns="http://www.w3.org/2000/svg">
								<path
									d="M183.89,153.34a57.6,57.6,0,0,1-46.56,46.55A8.75,8.75,0,0,1,136,200a8,8,0,0,1-1.32-15.89c16.57-2.79,30.63-16.85,33.44-33.45a8,8,0,1,1,15.78,2.68ZM216,144a88,88,0,0,1-176,0c0-27.92,11-56.47,32.66-84.85a8,8,0,0,1,11.93-.89l24.12,23.41,22-60.41a8,8,0,0,1,12.63-3.41C165.21,36,216,84.55,216,144Zm-16,0c0-46.09-35.79-85.92-58.21-106.33L119.52,98.74a8,8,0,0,1-13.09,3L80.06,76.16C64.09,99.21,56,122,56,144a72,72,0,0,0,144,0Z"
								/>
							</svg>
						{:else}
							<svg fill="currentColor" height="28" viewBox="0 0 256 256" width="28" xmlns="http://www.w3.org/2000/svg">
								<path
									d="M208,32H184V24a8,8,0,0,0-16,0v8H88V24a8,8,0,0,0-16,0v8H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM72,48v8a8,8,0,0,0,16,0V48h80v8a8,8,0,0,0,16,0V48h24V80H48V48ZM208,208H48V96H208V208Zm-96-88v64a8,8,0,0,1-16,0V132.94l-4.42,2.22a8,8,0,0,1-7.16-14.32l16-8A8,8,0,0,1,112,120Zm59.16,30.45L152,176h16a8,8,0,0,1,0,16H136a8,8,0,0,1-6.4-12.8l28.78-38.37A8,8,0,1,0,145.07,132a8,8,0,1,1-13.85-8A24,24,0,0,1,176,136,23.76,23.76,0,0,1,171.16,150.45Z"
								/>
							</svg>
						{/if}
					</div>
					<div>
						<p class="task-title">{task.title}</p>
						<p class="task-meta">{task.xp}</p>
					</div>
				</li>
			{/each}
		</ul>
	</section>
</PhoneFrame>

<style>
	.profile {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		margin-top: 0.6rem;
	}

	.avatar {
		width: 6.25rem;
		height: 6.25rem;
		border-radius: 50%;
		object-fit: cover;
		box-shadow: 0 12px 24px rgba(14, 43, 78, 0.16);
	}

	.profile-info {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
	}

	.name {
		margin: 0;
		font-size: 1.9rem;
		font-weight: 700;
		color: #1b2638;
	}

	.meta {
		margin: 0;
		font-size: 1rem;
		color: #7c8da6;
		font-weight: 500;
	}

	.daily-goal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1.4rem;
	}

	.link-button {
		border: none;
		background: none;
		color: #1470ff;
		font-weight: 600;
		font-size: 0.95rem;
		cursor: pointer;
	}

	.goal-days {
		list-style: none;
		display: flex;
		justify-content: space-between;
		align-items: flex-end;
		padding: 0;
		margin: 0 0 1.4rem;
	}

	.goal-day {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.6rem;
	}

	.day-label {
		font-weight: 600;
		color: #223349;
	}

	.day-indicator {
		width: 2.8rem;
		height: 2.8rem;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.goal-day.done .day-indicator {
		background: linear-gradient(145deg, #1c88ff 0%, #1469ff 100%);
		color: white;
		box-shadow: 0 12px 22px rgba(20, 115, 255, 0.33);
	}

	.goal-day.current .day-indicator {
		border: 2px dashed #1c7aff;
		background: rgba(28, 122, 255, 0.1);
	}

	.goal-day.current .current-dot {
		display: block;
		width: 0.7rem;
		height: 0.7rem;
		border-radius: 50%;
		background: #1c7aff;
	}

	.goal-day.upcoming .day-indicator {
		border: 2px solid #d4dbea;
	}

	.goal-message {
		margin: 0;
		background: #edf4ff;
		border-radius: 16px;
		padding: 0.9rem 1rem;
		color: #1b4a87;
		font-weight: 600;
		font-size: 0.98rem;
	}

	.cta-card {
		margin-top: 1.4rem;
		padding: 1.2rem 1.4rem;
		border-radius: 28px;
		background: linear-gradient(135deg, #5f7bff, #7c5bff);
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1.1rem;
		color: #f4f6ff;
		box-shadow: 0 18px 32px rgba(74, 96, 255, 0.28);
	}

	.cta-text {
		display: flex;
		flex-direction: column;
		gap: 0.35rem;
		min-width: 200px;
	}

	.cta-title {
		margin: 0;
		font-size: 1.35rem;
		font-weight: 700;
		letter-spacing: 0.01em;
	}

	.cta-subtitle {
		margin: 0;
		font-size: 0.92rem;
		color: rgba(243, 246, 255, 0.8);
	}

	.cta-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.75rem 1.4rem;
		border-radius: 999px;
		background: #fefefe;
		color: #3d4aad;
		font-weight: 700;
		text-decoration: none;
		box-shadow: 0 14px 26px rgba(26, 32, 72, 0.22);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.cta-button:hover {
		transform: translateY(-1px);
		box-shadow: 0 18px 32px rgba(26, 32, 72, 0.28);
	}

	.modules {
		display: flex;
		flex-direction: column;
		gap: 1.1rem;
	}

	.modules-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1.1rem;
	}

	.module-card {
		background: #ffffff;
		border-radius: 26px;
		padding: 0.75rem;
		box-shadow: 0 14px 28px rgba(18, 46, 84, 0.08);
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
	}

	.module-image {
		position: relative;
		width: 100%;
		padding-top: 100%;
		border-radius: 22px;
		overflow: hidden;
	}

	.module-image img {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.module-card p {
		margin: 0;
		font-weight: 600;
		color: #1f2d43;
		font-size: 1rem;
	}

	.plan {
		display: flex;
		flex-direction: column;
		gap: 1.1rem;
	}

	.plan-list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.task-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.1rem 1.3rem;
		border-radius: 22px;
		background: #ffffff;
		box-shadow: 0 12px 24px rgba(15, 43, 82, 0.08);
	}

	.task-card.secondary {
		background: #f8fbff;
		box-shadow: none;
		border: 1px solid #e1e9f5;
	}

	.task-icon {
		width: 3.2rem;
		height: 3.2rem;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #edf2fb;
		color: #5c6d86;
		box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.4);
	}

	.task-card.secondary .task-icon {
		background: #f3f7ff;
		color: #7085a4;
	}

	.task-title {
		margin: 0;
		font-weight: 600;
		color: #1c2b3f;
	}

	.task-meta {
		margin: 0.35rem 0 0;
		color: #96a4b9;
		font-size: 0.95rem;
	}
</style>
