<script lang="ts">
  import PhoneFrame from '$lib/components/PhoneFrame.svelte';
  import FlashCard, { type Flashcard as Card } from '$lib/components/FlashCard.svelte';
  import { onMount } from 'svelte';

  const navItems = [
    { id: 'home', href: '/', label: '首页' },
    { id: 'learning', href: '/learning', label: '学习' },
    { id: 'profile', href: '/profile', label: '我的' }
  ];

  const API_BASE = '/api/v1/flashcards';

  type Part = 'all' | 'noun' | 'verb' | 'adjective_adverb';
  type Status = 'all' | 'new' | 'learning' | 'mastered';

  let card = $state<Card | null>(null);
  let loading = $state(false);
  let submitting = $state(false);
  let showExamples = $state(false);
  let errorMessage = $state('');
  let selectedPart = $state<Part>('all');
  let selectedStatus = $state<Status>('all');

  const examples = $derived(parseExamples(card?.examples));

  onMount(() => { loadCard(); });

  function buildQuery() {
    const p = new URLSearchParams();
    if (selectedPart !== 'all') p.set('part_of_speech', selectedPart);
    if (selectedStatus !== 'all') p.set('status', selectedStatus);
    return p.toString();
  }

  async function loadCard() {
    loading = true;
    try {
      const q = buildQuery();
      const res = await fetch(`${API_BASE}/next${q ? `?${q}` : ''}`, { credentials:'include' });
      if (!res.ok) throw new Error('无法获取下一张卡片');
      card = (await res.json()) as Card | null;
    } catch (e) { handleError(e); } finally { loading = false; }
  }

  async function review(result: 'mastered'|'learning') {
    if (!card || submitting) return;
    submitting = true;
    try {
      const res = await fetch(`${API_BASE}/${card.entry_id}/review`, {
        method: 'POST', headers: { 'Content-Type':'application/json' }, credentials:'include', body: JSON.stringify({ result })
      });
      if (!res.ok) throw new Error('提交复习结果失败');
      await loadCard();
    } catch (e) { handleError(e); } finally { submitting = false; }
  }

  function parseExamples(value?: string | null) {
    if (!value) return [] as string[];
    return value.split(/[\r\n\u{FF1B};]+/u).map((x)=>x.trim()).filter(Boolean);
  }

  function handleError(err: unknown) {
    errorMessage = err instanceof Error ? err.message : String(err ?? '未知错误');
  }
  function clearError(){ errorMessage='' }
</script>

<PhoneFrame {navItems} navActive="learning">
  <div class="wrap">
    {#if errorMessage}
      <div class="alert"><span>{errorMessage}</span><button onclick={clearError}>关闭</button></div>
    {/if}

    <section class="toolbar">
      <div class="row">
        <button class:selected={selectedPart==='all'} onclick={() => (selectedPart='all', loadCard())}>全部</button>
        <button class:selected={selectedPart==='noun'} onclick={() => (selectedPart='noun', loadCard())}>名词</button>
        <button class:selected={selectedPart==='verb'} onclick={() => (selectedPart='verb', loadCard())}>动词</button>
        <button class:selected={selectedPart==='adjective_adverb'} onclick={() => (selectedPart='adjective_adverb', loadCard())}>形/副词</button>
      </div>
      <div class="row">
        <button class:selected={selectedStatus==='all'} onclick={() => (selectedStatus='all', loadCard())}>不限</button>
        <button class:selected={selectedStatus==='new'} onclick={() => (selectedStatus='new', loadCard())}>未学习</button>
        <button class:selected={selectedStatus==='learning'} onclick={() => (selectedStatus='learning', loadCard())}>学习中</button>
        <button class:selected={selectedStatus==='mastered'} onclick={() => (selectedStatus='mastered', loadCard())}>已掌握</button>
      </div>
    </section>

    <section class="stage">
      {#if loading}
        <div class="placeholder">正在挑选适合你的卡片…</div>
      {:else if !card}
        <div class="placeholder empty">当前筛选条件下没有更多卡片啦 🎉</div>
      {:else}
        <FlashCard card={card} />

        <div class="actions">
          <button class="minor" onclick={() => (showExamples=true)} disabled={examples.length===0}>例句</button>
          <button class="minor" onclick={() => review('learning')} disabled={submitting}>再巩固一下</button>
          <button class="primary" onclick={() => review('mastered')} disabled={submitting}>已经掌握</button>
        </div>
      {/if}
    </section>
  </div>
</PhoneFrame>

{#if showExamples}
  <div class="backdrop" role="button" tabindex="0" onclick={() => (showExamples=false)} onkeydown={(e)=> (e.key==='Escape') && (showExamples=false)}>
    <div class="modal" onclick={(e)=>e.stopPropagation()}>
      <header><h3>例句（{examples.length}）</h3><button onclick={() => (showExamples=false)}>×</button></header>
      <ul>{#each examples as ex}<li>{ex}</li>{/each}</ul>
    </div>
  </div>
{/if}

<style>
.wrap { display:flex; flex-direction:column; gap:1rem; padding:1rem 1rem 2rem; }
.alert { background:#fff3f3; border:1px solid #ffd3d3; color:#ba2b2b; padding:.6rem .8rem; border-radius:12px; display:flex; justify-content:space-between; }
.alert button { border:none; background:transparent; font-weight:700; cursor:pointer; }

.toolbar { display:flex; flex-direction:column; gap:.5rem; }
.toolbar .row { display:flex; gap:.5rem; overflow:auto; }
.toolbar button { border:1px solid #d7e0f0; background:#fff; padding:.45rem .9rem; border-radius:999px; font-weight:700; color:#4b5d79; }
.toolbar button.selected { background:#edf4ff; color:#1870ff; border-color:rgba(24,112,255,.35); }

.stage { margin-top:.5rem; display:flex; flex-direction:column; gap:.9rem; }
.placeholder { background:#fff; border:1px solid #e5eef8; padding:1.2rem; border-radius:16px; color:#4b5d79; text-align:center; }

.actions { display:grid; grid-template-columns: 1fr 1fr; gap:.6rem; align-items:center; }
.actions .primary { grid-column: 1 / -1; background:linear-gradient(145deg,#1c88ff,#1469ff); color:#fff; border:none; border-radius:16px; padding:.85rem 1rem; font-weight:800; }
.actions .minor { background:#fff; border:1px solid #d7e0f0; border-radius:16px; padding:.75rem 1rem; font-weight:700; color:#1b2536; }

.backdrop { position:fixed; inset:0; background:rgba(0,0,0,.35); display:flex; align-items:center; justify-content:center; z-index:50; }
.modal { width:min(92vw,420px); background:#fff; border-radius:14px; box-shadow:0 18px 36px rgba(12,35,65,.18); overflow:hidden; }
.modal header { display:flex; justify-content:space-between; align-items:center; padding:.8rem 1rem; border-bottom:1px solid #e5eef8; }
.modal header button { border:none; background:#edf4ff; border-radius:999px; width:2rem; height:2rem; cursor:pointer; }
.modal ul { margin:0; padding: .8rem 1.2rem 1rem 1.6rem; }
.modal li { margin:.4rem 0; }
</style>

