<script lang="ts">
	import PhoneFrame from '$lib/components/PhoneFrame.svelte';

	const navItems = [
		{ id: 'home', href: '/', label: '主页' },
		{ id: 'learning', href: '/learning', label: '学习' },
		{ id: 'profile', href: '/profile', label: '个人中心' }
	];

	const stages = [
		{
			label: '入门',
			image:
				'https://lh3.googleusercontent.com/aida-public/AB6AXuAgZObLv6BpIoYYsYRkYKRGTUx37fE5Rr11pzCZFnOT1KHDjmmuUh5_9enFfQ-DWbFFzT-6JlLwFLlqu7oSHqq43BqCxh3T7NR8pmW7mCOTL3vc4wxCOOVADfSigHojaSvG5bphBxZIiCj23iFVlL8Juwnte0FxNwYyp8itVAHPG8mTw92RVoKSi-gpKxJX-k9W_m1IWz4T5jFnjpZgGSg7E3uHQScSQSbVVu5F9BBecRM8gjhrCtR7-DOUPdHs0hRHwWJsSgWvqFti'
		},
		{
			label: '初级',
			image:
				'https://lh3.googleusercontent.com/aida-public/AB6AXuC8Z801QLHU8uZ10GvCT5pInlMTIXSfxRFGOGWK0vBjgtonhgYo126UmSfjFfHGxdafR3hE8Uetwyka0dkBrNWs8B2fjHm5b6I8Q03ZWLioIXBPlAlxBgCr70RDv-_eAqO41lcZOps9QAEtauvM97WA1723c5H2RaBGmHsPEup1t5oRlL8vBMNfWJWuehXIBqvi4SU3VCgnMUN1Mt7xzfeJo1H6ccaFSNElsNVxBqSyRTfRH0_b9SPmIo9irtLHIBEtnw_1nRiV1mJj'
		},
		{
			label: '中级',
			image:
				'https://lh3.googleusercontent.com/aida-public/AB6AXuAfwfPOc73vVWrnHH9xpvIjoo1c9dlwAe9nnJaABW9c8eG428k70gCFmOdnv-_lN8nt4I3DnjWt_SbO_va-uBEVEKaIuv5DE53cYQWSDcO5T05kG2vS66K9QlUUlH9HCjEPgXyaK8asT5mDZF1AtnfoutGfFAnCskGotp5nP5ltnx6bh213M52BXlVIxzDVAyM5DGh2-j4Uj6AJPUOCtbKo_s8mpA1bFCoSE9gS3ZC0dMG1P4FeP7mISz3liP1oaP2ZsEIp2ZcAkmTH'
		}
	];
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta content="width=device-width, initial-scale=1.0" name="viewport" />
	<title>学习路线 - German Learn</title>
</svelte:head>

<PhoneFrame {navItems} navActive="home">
	<svelte:fragment slot="header">
		<div class="top-bar">
			<span class="ghost-button" aria-hidden="true"></span>
			<h1 class="top-bar-title">首页</h1>
			<button aria-label="打开设置" class="icon-button">
				<svg fill="currentColor" height="22" viewBox="0 0 256 256" width="22" xmlns="http://www.w3.org/2000/svg">
					<path
						d="M128,80a48,48,0,1,0,48,48A48.05,48.05,0,0,0,128,80Zm0,80a32,32,0,1,1,32-32A32,32,0,0,1,128,160Zm88-29.84q.06-2.16,0-4.32l14.92-18.64a8,8,0,0,0,1.48-7.06,107.21,107.21,0,0,0-10.88-26.25,8,8,0,0,0-6-3.93l-23.72-2.64q-1.48-1.56-3-3L186,40.54a8,8,0,0,0-3.94-6,107.71,107.71,0,0,0-26.25-10.87,8,8,0,0,0-7.06,1.49L130.16,40Q128,40,125.84,40L107.2,25.11a8,8,0,0,0-7.06-1.48A107.6,107.6,0,0,0,73.89,34.51a8,8,0,0,0-3.93,6L67.32,64.27q-1.56,1.49-3,3L40.54,70a8,8,0,0,0-6,3.94,107.71,107.71,0,0,0-10.87,26.25,8,8,0,0,0,1.49,7.06L40,125.84Q40,128,40,130.16L25.11,148.8a8,8,0,0,0-1.48,7.06,107.21,107.21,0,0,0,10.88,26.25,8,8,0,0,0,6,3.93l23.72,2.64q1.49,1.56,3,3L70,215.46a8,8,0,0,0,3.94,6,107.71,107.71,0,0,0,26.25,10.87,8,8,0,0,0,7.06-1.49L125.84,216q2.16.06,4.32,0l18.64,14.92a8,8,0,0,0,7.06,1.48,107.21,107.21,0,0,0,26.25-10.88,8,8,0,0,0,3.93-6l2.64-23.72q1.56-1.48,3-3L215.46,186a8,8,0,0,0,6-3.94,107.71,107.71,0,0,0,10.87-26.25,8,8,0,0,0-1.49-7.06Zm-16.1-6.5a73.93,73.93,0,0,1,0,8.68,8,8,0,0,0,1.74,5.48l14.19,17.73a91.57,91.57,0,0,1-6.23,15L187,173.11a8,8,0,0,0-5.1,2.64,74.11,74.11,0,0,1-6.14,6.14,8,8,0,0,0-2.64,5.1l-2.51,22.58a91.32,91.32,0,0,1-15,6.23l-17.74-14.19a8,8,0,0,0-5-1.75h-.48a73.93,73.93,0,0,1-8.68,0,8,8,0,0,0-5.48,1.74L100.45,215.8a91.57,91.57,0,0,1-15-6.23L82.89,187a8,8,0,0,0-2.64-5.1,74.11,74.11,0,0,1-6.14-6.14,8,8,0,0,0-5.1-2.64L46.43,170.6a91.32,91.32,0,0,1-6.23-15l14.19-17.74a8,8,0,0,0,1.74-5.48,73.93,73.93,0,0,1,0-8.68,8,8,0,0,0-1.74-5.48L40.2,100.45a91.57,91.57,0,0,1,6.23-15L69,82.89a8,8,0,0,0,5.1-2.64,74.11,74.11,0,0,1,6.14-6.14A8,8,0,0,0,82.89,69L85.4,46.43a91.32,91.32,0,0,1,15-6.23l17.74,14.19a8,8,0,0,0,5.48,1.74,73.93,73.93,0,0,1,8.68,0,8,8,0,0,0,5.48-1.74L155.55,40.2a91.57,91.57,0,0,1,15,6.23L173.11,69a8,8,0,0,0,2.64,5.1,74.11,74.11,0,0,1,6.14,6.14,8,8,0,0,0,5.1,2.64l22.58,2.51a91.32,91.32,0,0,1,6.23,15l-14.19,17.74A8,8,0,0,0,199.87,123.66Z"
					/>
				</svg>
			</button>
		</div>
	</svelte:fragment>

	<section class="hero">
		<div class="portrait" />
		<h2>学习路线</h2>
	</section>

	<section class="timeline">
		<div class="line" />
		{#each stages as stage, index (`stage-${index}`)}
			<div class={`stage ${index === stages.length - 1 ? 'last' : ''}`}>
				<div class="stage-icon" style={`background-image:url('${stage.image}')`} />
				<div class="stage-card">
					<p>{stage.label}</p>
				</div>
			</div>
		{/each}
	</section>

	<button class="primary-action" type="button">开始答题</button>
</PhoneFrame>

<style>
	.hero {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 1rem;
	}

	.portrait {
		width: 180px;
		height: 180px;
		border-radius: 50%;
		background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuDT3XxmDXPiFzapVCy7BxODgu3eKNRDyA5ZJhNauToGQOXc7EvWGFu_M405UCPlpqcqL02ong-9hO1Tg-W-t1EmNFnSvqiEF8GbkuHN1WC2a2Js1ZS4MFwUqDfPW2rIJAJfvkw6_Kb20pRPhn5rNWhvPdbBmZUwy1B2E22QQexEHhxOcmt07GJmRJ2FfURWFD0cFIAgeFW8TXqawUk7-s1EpmXM-OZEwQxNUPjP1h3fpaDn3Ng_3rSgrf7Nl-F7s3M6bJB0quGtwzOX');
		background-size: cover;
		background-position: center;
		box-shadow: 0 18px 42px rgba(0, 0, 0, 0.18);
	}

	.hero h2 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 800;
		color: #101827;
	}

	.timeline {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 1.8rem;
		padding-left: 3rem;
		margin-top: 1rem;
	}

	.timeline .line {
		position: absolute;
		top: 0;
		left: 1.4rem;
		width: 4px;
		height: calc(100% - 2.2rem);
		background: linear-gradient(180deg, #ffc700 0%, rgba(255, 199, 0, 0) 100%);
	}

	.stage {
		position: relative;
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.stage.last .stage-card {
		margin-bottom: 0;
	}

	.stage-icon {
		position: absolute;
		left: -2.8rem;
		width: 44px;
		height: 44px;
		border-radius: 50%;
		background-size: cover;
		background-position: center;
		box-shadow: 0 10px 24px rgba(0, 0, 0, 0.18);
	}

	.stage-card {
		flex: 1;
		background: #f9f2ec;
		border-radius: 18px;
		padding: 1rem 1.3rem;
		box-shadow: 0 12px 28px rgba(240, 164, 92, 0.18);
		border: 1px solid rgba(255, 206, 0, 0.35);
	}

	.stage-card p {
		margin: 0;
		font-weight: 700;
		color: #1c2335;
		font-size: 1.05rem;
	}

	.primary-action {
		margin-top: 2rem;
		width: 100%;
		padding: 1rem 1.4rem;
		border: none;
		border-radius: 999px;
		background: linear-gradient(135deg, #ff3d2e 0%, #d50000 100%);
		color: #ffffff;
		font-weight: 700;
		font-size: 1.05rem;
		cursor: pointer;
		box-shadow: 0 18px 34px rgba(213, 0, 0, 0.32);
	}
</style>
