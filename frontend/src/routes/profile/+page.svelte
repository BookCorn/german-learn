<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];

	const highlights = [
		{ label: '连续学习', value: '7 天' },
		{ label: '总时长', value: '120 分钟' },
		{ label: '掌握词汇', value: '340 个' }
	];

	const settings = [
		{ label: '通知设置', hint: '每天 20:00 提醒', action: '调整' },
		{ label: '账号安全', hint: '未绑定手机号', action: '去绑定' },
		{ label: '语言偏好', hint: '德语 / 中文', action: '切换' }
	];

	const badges = ['坚持 7 天', '发音达人', '词汇大师'];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>个人中心 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="profile">
	<svelte:fragment slot="header">
		<div class="profile-top">
			<button aria-label="返回" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M224,128a8,8,0,0,1-8,8H59.31l58.35,58.34a8,8,0,0,1-11.32,11.32l-72-72a8,8,0,0,1,0-11.32l72-72a8,8,0,0,1,11.32,11.32L59.31,120H216A8,8,0,0,1,224,128Z" />
				</svg>
			</button>
			<h1 class="top-bar-title">个人中心</h1>
			<button aria-label="设置" class="icon-button ghost">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path d="M232.37,101.61l-19.1-11a80.46,80.46,0,0,0-3.28-7.94l6.7-20.53a8,8,0,0,0-7.62-10.54H187a80.63,80.63,0,0,0-7.52-4.32L170,28.63a8,8,0,0,0-10.53-7.63L138.94,31.7a80.66,80.66,0,0,0-9.88,0l-20.53-6.7a8,8,0,0,0-10.53,7.63L94.54,47.3a80.63,80.63,0,0,0-7.52,4.32H47.93a8,8,0,0,0-7.62,10.54l6.7,20.53a80.46,80.46,0,0,0-3.28,7.94l-19.1,11a8,8,0,0,0-3.24,11l10.78,18.66a80.29,80.29,0,0,0,0,15.88L21.39,165.71a8,8,0,0,0,3.24,11l19.1,11a80.46,80.46,0,0,0,3.28,7.94l-6.7,20.53a8,8,0,0,0,7.62,10.54H87a80.63,80.63,0,0,0,7.52,4.32l6.53,19.67a8,8,0,0,0,10.53,7.63l20.53-6.7a80.66,80.66,0,0,0,9.88,0l20.53,6.7a8,8,0,0,0,10.53-7.63l-6.53-19.67a80.63,80.63,0,0,0,7.52-4.32h23.57a8,8,0,0,0,7.62-10.54l-6.7-20.53a80.46,80.46,0,0,0,3.28-7.94l19.1-11a8,8,0,0,0,3.24-11L221.59,129.2a80.29,80.29,0,0,0,0-15.88l10.78-18.66A8,8,0,0,0,232.37,101.61ZM128,168a40,40,0,1,1,40-40A40,40,0,0,1,128,168Z" />
				</svg>
			</button>
		</div>
	</svelte:fragment>

	<section class="avatar-card">
		<div class="bg-blur"></div>
		<figure class="avatar">
			<img alt="Sophia 的头像" src="https://lh3.googleusercontent.com/aida-public/AB6AXuDnLnp7iffK2EJGBHr32RYNlV4zTKcCV-enBl9pfqlWCMuIAAOwFUuCjYOwb0BNKW9jN106HNVYptnzZ-9fQRJLn-W8G-8Y-i3GpZBKNfOLfOnNrWwMOPoKz9TACmBy57QTRh7pJ5b_A0JXT__FkyvUzSB6VysegynxaPstkd9bcfh6H7u8SIscZrJo1-oaylcsVhATjyVbU-hDeg2GzfJlraMoOzpw4hJ3SPyAX-KxkRAAfNskbiLuT_c_hgxd7tSpaQNzwnfyi3UO" />
		</figure>
		<h2>Sophia</h2>
		<p class="meta">Level 10 · 1000 XP</p>
		<div class="badges">
			{#each badges as badge, index (`badge-${index}`)}
				<span>{badge}</span>
			{/each}
		</div>
		<button class="edit" type="button">编辑资料</button>
	</section>

	<section class="highlight-grid">
		{#each highlights as item, index (`highlight-${index}`)}
			<div class="highlight">
				<span class="value">{item.value}</span>
				<p>{item.label}</p>
			</div>
		{/each}
	</section>

	<section class="settings">
		<h3>设置</h3>
		<ul>
			{#each settings as item, index (`setting-${index}`)}
				<li>
					<div class="info">
						<p>{item.label}</p>
						<span>{item.hint}</span>
					</div>
					<button type="button">{item.action}</button>
				</li>
			{/each}
		</ul>
	</section>

	<section class="actions">
		<button class="premium" type="button">开通高级会员</button>
		<button class="logout" type="button">退出登录</button>
	</section>
</PhoneFrame>

<style>
	.profile-top {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 2.2rem 1.8rem 1.2rem;
	}

	.icon-button.ghost {
		background: rgba(17, 32, 51, 0.08);
		box-shadow: none;
		width: 2.75rem;
		height: 2.75rem;
		border-radius: 999px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		color: #112033;
	}

	.icon-button.ghost:hover {
		background: rgba(17, 32, 51, 0.14);
	}

	.avatar-card {
		position: relative;
		padding: 2.2rem 1.8rem 1.9rem;
		border-radius: 28px;
		overflow: hidden;
		color: #ffffff;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 0.75rem;
		box-shadow: 0 26px 48px rgba(20, 46, 90, 0.22);
		margin-bottom: 2rem;
		isolation: isolate;
		width: 100%;
	}

	.bg-blur {
		position: absolute;
		top: -25%;
		left: -20%;
		width: 140%;
		height: 150%;
		background: conic-gradient(from 210deg, #0c52d4, #1a6dff, #54c4ff, #174dff, #0c52d4);
		filter: blur(46px) saturate(1.35);
		opacity: 0.85;
		transform: rotate(-6deg);
	}

	.avatar-card::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		background: linear-gradient(150deg, rgba(7, 22, 52, 0.55) 15%, rgba(0, 0, 0, 0) 70%);
		z-index: 0;
	}

	.avatar-card::after {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		background: linear-gradient(185deg, rgba(13, 30, 64, 0.08) 0%, rgba(3, 10, 24, 0.82) 100%);
		box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.08);
		z-index: 0;
	}

	.avatar-card > * {
		position: relative;
		z-index: 1;
	}

	.avatar {
		width: 116px;
		height: 116px;
		border-radius: 50%;
		margin: 0;
		padding: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.7), rgba(255, 255, 255, 0.2));
		box-shadow: 0 20px 36px rgba(6, 18, 40, 0.4);
	}

	.avatar img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		border-radius: 50%;
	}

	.avatar-card h2 {
		margin: 0;
		font-size: 1.7rem;
		font-weight: 800;
		letter-spacing: 0.01em;
	}

	.avatar-card .meta {
		margin: 0;
		font-size: 0.96rem;
		color: rgba(255, 255, 255, 0.78);
	}

	.badges {
		display: inline-flex;
		gap: 0.5rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	.badges span {
		padding: 0.35rem 0.85rem;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.2);
		font-size: 0.78rem;
		font-weight: 600;
		backdrop-filter: blur(18px);
		border: 1px solid rgba(255, 255, 255, 0.18);
	}

	.edit {
		margin-top: 0.4rem;
		padding: 0.68rem 1.5rem;
		border-radius: 18px;
		border: 1px solid rgba(255, 255, 255, 0.24);
		background: rgba(255, 255, 255, 0.18);
		color: #ffffff;
		font-weight: 700;
		cursor: pointer;
		box-shadow: 0 10px 18px rgba(8, 20, 46, 0.35);
		transition: transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease;
	}

	.edit:hover {
		transform: translateY(-1px);
		background: rgba(255, 255, 255, 0.22);
		box-shadow: 0 14px 24px rgba(8, 20, 46, 0.42);
	}

	.highlight-grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 0.8rem;
		margin-top: 1.6rem;
	}

	.highlight {
		background: #ffffff;
		border-radius: 22px;
		padding: 1.1rem 0.8rem;
		text-align: center;
		box-shadow: 0 16px 28px rgba(16, 32, 60, 0.12);
		border: 1px solid rgba(16, 32, 60, 0.06);
	}

	.highlight .value {
		display: block;
		font-size: 1.1rem;
		font-weight: 700;
		color: #111b2c;
	}

	.highlight p {
		margin: 0.35rem 0 0;
		font-size: 0.84rem;
		color: #677187;
	}

	.settings {
		margin-top: 2rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.settings h3 {
		margin: 0;
		font-size: 1.08rem;
		font-weight: 700;
		color: #111b2c;
	}

	.settings ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.8rem;
	}

	.settings li {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.95rem 1.2rem;
		border-radius: 18px;
		background: #ffffff;
		border: 1px solid rgba(16, 32, 60, 0.05);
		box-shadow: 0 14px 26px rgba(16, 32, 60, 0.1);
	}

	.settings .info p {
		margin: 0;
		font-weight: 600;
		color: #152038;
	}

	.settings .info span {
		display: block;
		margin-top: 0.2rem;
		font-size: 0.85rem;
		color: #7c8598;
	}

	.settings button {
		border: none;
		padding: 0.45rem 1.05rem;
		border-radius: 14px;
		background: rgba(24, 111, 255, 0.14);
		color: #1769ff;
		font-weight: 600;
		cursor: pointer;
	}

	.actions {
		margin-top: 2.4rem;
		display: flex;
		flex-direction: column;
		gap: 0.95rem;
	}

	.actions button {
		width: 100%;
		padding: 1rem 1.2rem;
		border-radius: 18px;
		border: none;
		font-weight: 700;
		cursor: pointer;
	}

	.actions .premium {
		background: linear-gradient(135deg, #ffd400 0%, #ffb300 100%);
		color: #11151f;
		box-shadow: 0 20px 36px rgba(255, 182, 24, 0.28);
	}

	.actions .logout {
		background: rgba(230, 46, 77, 0.12);
		color: #d32c4f;
	}
</style>
