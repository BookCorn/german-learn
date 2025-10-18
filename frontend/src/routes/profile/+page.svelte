<script lang="ts">
  import PhoneFrame from '$lib/components/PhoneFrame.svelte';
  import { onMount } from 'svelte';

  const navItems = [
    { id: 'home', href: '/', label: '首页' },
    { id: 'learning', href: '/learning', label: '学习' },
    { id: 'profile', href: '/profile', label: '我的' }
  ];

  const AVATAR_URL = 'https://lh3.googleusercontent.com/aida-public/AB6AXuDnLnp7iffK2EJGBHr32RYNlV4zTKcCV-enBl9pfqlWCMuIAAOwFUuCjYOwb0BNKW9jN106HNVYptnzZ-9fQRJLn-W8G-8Y-i3GpZBKNfOLfOnNrWwMOPoKz9TACmBy57QTRh7pJ5b_A0JXT__FkyvUzSB6VysegynxaPstkd9bcfh6H7u8SIscZrJo1-oaylcsVhATjyVbU-hDeg2GzfJlraMoOzpw4hJ3SPyAX-KxkRAAfNskbiLuT_c_hgxd7tSpaQNzwnfyi3UO';

  type Me = { user_id: string; email?: string|null; name?: string|null };
  let me = $state<Me | null>(null);
  let error = $state('');

  onMount(async () => {
    try {
      const res = await fetch('/api/auth/me', { method: 'POST', credentials: 'include' });
      if (res.status === 401) { location.href = '/login'; return; }
      me = await res.json();
    } catch (e) {
      error = '加载用户信息失败';
    }
  });

  function displayName() {
    if (!me) return '';
    if (me.name && me.name.trim()) return me.name;
    if (me.email) return me.email.split('@')[0];
    return me.user_id;
  }

  async function logout() {
    await fetch('/api/auth/logout', { method: 'POST', credentials: 'include' });
    location.href = '/login';
  }
</script>

<svelte:head>
  <title>个人中心</title>
</svelte:head>

<PhoneFrame {navItems} navActive="profile">
  <section class="center">
    <div class="avatar">
      <img alt="avatar" src={AVATAR_URL} />
    </div>
    <h1 class="name">{displayName()}</h1>
    {#if error}<p class="err">{error}</p>{/if}
    <button class="logout" on:click={logout}>退出登录</button>
  </section>
</PhoneFrame>

<style>
  .center{ padding: 40px 20px; display:flex; flex-direction:column; align-items:center; gap: 16px }
  .avatar{ width: 112px; height:112px; border-radius:999px; overflow:hidden; box-shadow:0 8px 24px rgba(0,0,0,.2) }
  .avatar img{ width:100%; height:100%; object-fit:cover }
  .name{ margin: 0; font-size: 20px; font-weight: 800 }
  .err{ color:#c00 }
  .logout{ margin-top: 8px; padding:.6rem 1rem; border:none; border-radius:12px; background:#ffe3e7; color:#c12741; font-weight:700 }
</style>
