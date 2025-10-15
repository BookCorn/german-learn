<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';
	import logo from '$lib/logo.jpg';

	const navItems = [
		{ id: 'home', href: '/', label: '首页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '我的' }
	];
</script>

<PhoneFrame {navItems} navActive="home">
	<svelte:fragment slot="header">
		<div class="top-bar simple">
			<img alt="German Learn Logo" class="brand-logo" src={logo} />
		</div>
	</svelte:fragment>

	<section class="coming-soon">
		<img alt="German Learn Logo" class="soon-logo" src={logo} />
		<p class="soon-text">开发中...</p>
	</section>
</PhoneFrame>

<style>
	.top-bar.simple {
		justify-content: center;
	}

	.brand-logo {
		height: 2.4rem;
		border-radius: 12px;
		box-shadow: 0 6px 16px rgba(24, 48, 88, 0.18);
	}

	.coming-soon {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		margin-top: 4rem;
		gap: 1.2rem;
	}

	.soon-logo {
		width: 160px;
		height: 160px;
		object-fit: cover;
		border-radius: 28px;
		box-shadow: 0 24px 60px rgba(20, 46, 94, 0.18);
	}

	.soon-text {
		margin: 0;
		font-size: 1.4rem;
		font-weight: 700;
		color: #1f2d43;
		letter-spacing: 0.08em;
	}
</style>
