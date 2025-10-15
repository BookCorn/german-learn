<script lang="ts">
	let {
		navItems = [
			{ id: 'home', href: '/', label: '首页' },
			{ id: 'learning', href: '/learning', label: '学习' },
			{ id: 'profile', href: '/profile', label: '我的' }
		],
		navActive = 'home',
		showNav = true
	} = $props<{
		navItems?: Array<{ id: string; href: string; label: string }>;
		navActive?: (typeof navItems)[number]['id'];
		showNav?: boolean;
	}>();
</script>

<div class="frame-page">
	<div class="frame-phone">
		<div class="frame-top">
			<slot name="header" />
		</div>

		<div class="frame-scroll">
			<slot />
		</div>

		{#if showNav}
			<footer class="frame-bottom-nav">
				<nav>
					{#each navItems as item}
						<a class={`frame-nav-link ${navActive === item.id ? 'active' : ''}`} href={item.href}>
							<div class="frame-nav-icon">
								{#if item.id === 'home'}
									<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
										<path
											d="M224,115.55V208a16,16,0,0,1-16,16H168a16,16,0,0,1-16-16V168a8,8,0,0,0-8-8H112a8,8,0,0,0-8,8v40a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V115.55a16,16,0,0,1,5.17-11.78l80-75.48.11-.11a16,16,0,0,1,21.53,0,1.14,1.14,0,0,0,.11.11l80,75.48A16,16,0,0,1,224,115.55Z"
										/>
									</svg>
								{:else if item.id === 'modules'}
									<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
										<path
											d="M224,48H160a40,40,0,0,0-32,16A40,40,0,0,0,96,48H32A16,16,0,0,0,16,64V192a16,16,0,0,0,16,16H96a24,24,0,0,1,24,24,8,8,0,0,0,16,0,24,24,0,0,1,24-24h64a16,16,0,0,0,16-16V64A16,16,0,0,0,224,48ZM96,192H32V64H96a24,24,0,0,1,24,24V200A39.81,39.81,0,0,0,96,192Zm128,0H160a39.81,39.81,0,0,0-24,8V88a24,24,0,0,1,24-24h64Z"
										/>
									</svg>
								{:else if item.id === 'learning'}
									<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
										<path
											d="M224.11,75.94,136.11,35.94a16,16,0,0,0-13.56,0l-88,40A16,16,0,0,0,24,90.67V200a16,16,0,0,0,16,16L96,215.9v12.1a8,8,0,0,0,11.79,7.16L128,227.06l20.21,8.1A8,8,0,0,0,160,228V215.9l56,0a16,16,0,0,0,16-16V90.67A16,16,0,0,0,224.11,75.94ZM112,195.64,48,200V96l64-29.09Zm96,4.35-64-4.35V66.91L208,96Z"
										/>
									</svg>
								{:else}
									<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
										<path
											d="M230.92,212c-15.23-26.33-38.7-45.21-66.09-54.16a72,72,0,1,0-73.66,0C63.78,166.78,40.31,185.66,25.08,212a8,8,0,1,0,13.85,8c18.84-32.56,52.14-52,89.07-52s70.23,19.44,89.07,52a8,8,0,1,0,13.85-8ZM72,96a56,56,0,1,1,56,56A56.06,56.06,0,0,1,72,96Z"
										/>
									</svg>
								{/if}
							</div>
							<span>{item.label}</span>
						</a>
					{/each}
				</nav>
			</footer>
		{/if}
	</div>
</div>

<style>
	.frame-page {
		min-height: 100vh;
		display: flex;
		justify-content: center;
		align-items: flex-start;
		padding: calc(env(safe-area-inset-top, 0) + 3rem) 1.5rem
			calc(env(safe-area-inset-bottom, 0) + 3rem);
		box-sizing: border-box;
		background: var(--page-bg-top);
	}

	.frame-phone {
		width: min(430px, 100%);
		height: 932px;
		max-height: 100vh;
		background: var(--phone-bg);
		border-radius: 36px;
		box-shadow: 0 22px 60px rgba(12, 35, 64, 0.12);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.frame-top {
		flex-shrink: 0;
	}

	.frame-scroll {
		flex: 1;
		overflow-y: auto;
		padding: 0 1.8rem 2.2rem;
		display: flex;
		flex-direction: column;
		gap: 2rem;
		scrollbar-width: thin;
		scrollbar-color: rgba(144, 161, 187, 0.4) transparent;
	}

	.frame-scroll::-webkit-scrollbar {
		width: 6px;
	}

	.frame-scroll::-webkit-scrollbar-thumb {
		background: rgba(144, 161, 187, 0.4);
		border-radius: 999px;
	}

	.frame-bottom-nav {
		background: #ffffff;
		padding: 0.8rem 2.4rem 1.2rem;
		box-shadow: 0 -8px 30px rgba(12, 35, 65, 0.08);
		border-top-left-radius: 28px;
		border-top-right-radius: 28px;
		border-top: 1px solid var(--border-soft);
	}

	.frame-bottom-nav nav {
		display: flex;
		justify-content: space-between;
	}

	.frame-nav-link {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.3rem;
		text-decoration: none;
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--nav-inactive);
		transition: color 0.2s ease;
	}

	.frame-nav-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2.8rem;
		height: 2.8rem;
		border-radius: 50%;
	}

	.frame-nav-link.active {
		color: var(--nav-active);
	}

	.frame-nav-link.active .frame-nav-icon {
		background: var(--nav-icon-active-bg);
		color: var(--nav-active);
		box-shadow: 0 8px 18px rgba(24, 112, 255, 0.18);
	}

	:global(.top-bar) {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 2.2rem 1.8rem 1.4rem;
		color: #1f3552;
	}

	:global(.top-bar .top-bar-title) {
		margin: 0;
		font-size: 1.32rem;
		font-weight: 700;
		letter-spacing: 0.08em;
	}

	:global(.icon-button) {
		border: none;
		background: #e3eaf5;
		color: #4c5d76;
		width: 2.75rem;
		height: 2.75rem;
		border-radius: 999px;
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.6);
		cursor: pointer;
		transition: background 0.2s ease;
	}

	:global(.icon-button:hover) {
		background: #dbe5f5;
	}

	:global(.ghost-button) {
		width: 2.75rem;
		height: 2.75rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	:global(.section-heading) {
		margin: 0;
		font-size: 1.26rem;
		font-weight: 700;
		color: #1b2536;
	}

	:global(.card) {
		background: #ffffff;
		border-radius: 24px;
		padding: 1.6rem;
		box-shadow: 0 16px 40px rgba(21, 55, 108, 0.08);
	}

	:global(.soft-card) {
		background: #f8fbff;
		border-radius: 22px;
		padding: 1.2rem 1.4rem;
		border: 1px solid #e1e9f5;
	}

	@media (max-width: 500px) {
		.frame-page {
			padding: 0;
		}

		.frame-phone {
			width: 100%;
			height: 100vh;
			border-radius: 0;
			box-shadow: none;
		}

		.frame-bottom-nav {
			border-radius: 0;
		}
	}
</style>
