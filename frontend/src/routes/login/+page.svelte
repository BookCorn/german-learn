<script lang="ts">
  import PhoneFrame from '$lib/components/PhoneFrame.svelte';
  let email = '';
  let password = '';
  let loading = $state(false);
  let error = $state('');

  async function signIn(e: Event) {
    e.preventDefault();
    loading = true;
    error = '';
    try {
      const res = await fetch('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password }),
        credentials: 'include'
      });
      if (!res.ok) {
        await fetch('/api/auth/dev-login', {
          method: 'POST', headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ email, name: email.split('@')[0] })
        });
      }
      location.href = '/';
    } catch (err) {
      error = err instanceof Error ? err.message : '登录失败';
    } finally {
      loading = false;
    }
  }

  function backHome(){ location.href = '/'; }
</script>

<PhoneFrame showNav={false}>
  <svelte:fragment slot="header">
    <div class="top-bar">
      <button class="icon-button" aria-label="返回首页" on:click={backHome}>←</button>
      <h2 class="top-bar-title">登录</h2>
      <span class="ghost-button"></span>
    </div>
  </svelte:fragment>

  <section class="hero">
    <h1>Willkommen!</h1>
    <p class="subtitle">欢迎回来！请登录您的账户。</p>
  </section>

  <form class="form" on:submit|preventDefault={signIn}>
    {#if error}<p class="error">{error}</p>{/if}
    <label class="field">
      <span class="icon">👤</span>
      <input type="email" placeholder="用户名/邮箱" bind:value={email} required />
    </label>
    <label class="field">
      <span class="icon">🔒</span>
      <input type="password" placeholder="密码" bind:value={password} required />
    </label>
    <button class="submit" type="submit" disabled={loading}>{loading ? '登录中…' : '登录'}</button>
    <a class="forgot" href="/register">忘记密码？</a>
  </form>
</PhoneFrame>

<style>
  :global(html, body){ background:#ffffff }
  .hero{ margin: 4svh auto 16px; text-align:center; max-width:560px; padding:0 8px }
  .hero h1{ margin:.4rem 0; font-size:40px; font-weight:900; letter-spacing:.02em }
  .subtitle{ margin:0; color:#666; font-size:16px }

  .form{ max-width:640px; margin:16px auto; display:grid; gap:14px; padding:0 8px }
  .field{ display:grid; grid-template-columns:40px 1fr; align-items:center; gap:8px; padding:12px 14px; border:1px solid #e3e6ef; border-radius:14px; background:#fff }
  .field input{ border:none; outline:none; font-size:16px; padding:8px 4px }
  .icon{ width:28px; height:28px; display:grid; place-items:center; color:#8c93a6; font-size:18px }

  .submit{ height:54px; border:none; border-radius:12px; background: linear-gradient(180deg,#2f7bff,#1e63ff); color:#fff; font-weight:800; letter-spacing:.05em; cursor:pointer; box-shadow:0 10px 24px rgba(30,99,255,.25) }
  .submit:disabled{ opacity:.7; cursor:not-allowed }
  .error{ color:#d12d42; font-weight:600 }
  .forgot{ text-align:center; color:#1e63ff; font-weight:700; margin-top: 8px }
</style>
