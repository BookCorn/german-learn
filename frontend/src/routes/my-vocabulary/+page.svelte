<script lang="ts">
  import { onMount } from 'svelte';

  type Extra = { gender?: string|null; plural?: string|null; suffix?: string|null } & Record<string, unknown>;
  type Entry = {
    entry_id: number;
    word: string;
    part_of_speech: 'noun'|'verb'|'adjective_adverb'|string;
    english?: string|null;
    meaning?: string|null;
    examples?: string|null;
    themes?: string|null; // 逗号分隔
    extra?: Extra | null;
  };

  let list = $state<Entry[]>([]);
  let q = $state('');
  let pos = $state<'noun'|'verb'|'adjective_adverb'>('noun');
  let saving = $state<Record<number, boolean>>({});
  let adding = $state(false);
  let error = $state('');
  // AI 生成对话框状态
  let showAI = $state(false);
  let aiWords = $state('');
  let aiModel = $state('');
  let aiLoading = $state(false);
  let aiMessage = $state('');
  let aiResults = $state<{word:string;status:string;message?:string}[]>([]);

  onMount(loadData);

  async function loadData() {
    error = '';
    const res = await fetch('/api/v1/entries/mine', { credentials: 'include' });
    if (res.status === 401) { location.href = '/login'; return; }
    const data: Entry[] = await res.json();
    list = data.sort((a,b)=>a.word.localeCompare(b.word));
  }

  const filtered = $derived(list.filter(e => {
    if (e.part_of_speech !== pos) return false;
    if (q && !(e.word?.toLowerCase().includes(q.toLowerCase()) || e.meaning?.includes(q) || e.english?.toLowerCase().includes(q.toLowerCase()))) return false;
    return true;
  }));

  function genderColor(g?: string|null){
    const v = (g||'').toLowerCase();
    if (v === 'der') return 'male';
    if (v === 'die') return 'female';
    if (v === 'das') return 'neuter';
    return 'neutral';
  }

  function updateLocal(id: number, patch: Partial<Entry>){
    const idx = list.findIndex(x=>x.entry_id===id);
    if (idx>=0) list[idx] = { ...list[idx], ...patch };
  }

  async function saveField(id:number, patch: Partial<Entry>){
    saving[id] = true;
    try {
      const body:any = {};
      for (const k of Object.keys(patch)) (body as any)[k] = (patch as any)[k];
      const res = await fetch(`/api/v1/entries/${id}`, {
        method: 'PATCH', credentials: 'include', headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
      });
      if (!res.ok) throw new Error('保存失败');
      updateLocal(id, patch);
    } catch (e){ error = e instanceof Error ? e.message : '保存失败'; }
    finally { saving[id] = false; }
  }

  async function addRow(){
    adding = true; error='';
    try{
      const base: any = { word:'', part_of_speech: pos, english:'', meaning:'', examples:'', themes:'' };
      let extra: any = {};
      if (pos==='noun') extra = { gender:'', plural:'', suffix:'' };
      if (pos==='verb') extra = { present_form:'', preterite_form:'', perfect_form:'', properties:'', noun_form:'' };
      if (pos==='adjective_adverb') extra = { attribute:'', comparison_forms:'' };
      const res = await fetch('/api/v1/entries', {
        method: 'POST', credentials: 'include', headers: {'Content-Type':'application/json'},
        body: JSON.stringify({ ...base, extra })
      });
      if (!res.ok) throw new Error('创建失败');
      await loadData();
    }catch(e){ error = e instanceof Error ? e.message : '创建失败'; }
    finally{ adding = false; }
  }

  // AI 生成
  async function openAI(){ showAI = true; aiWords=''; aiResults=[]; aiMessage=''; }
  function closeAI(){ if(!aiLoading){ showAI = false; } }
  async function runAI(e: Event){
    e.preventDefault();
    if (!aiWords.trim()) { aiMessage = '请输入要生成的单词（逗号/分号/换行分隔）'; return; }
    aiLoading = true; aiMessage='模型处理中，请稍候…'; aiResults=[];
    try{
      const res = await fetch('/api/v1/entries/ai-fill', {
        method:'POST', credentials:'include', headers:{'Content-Type':'application/json'},
        body: JSON.stringify({ part_of_speech: pos, words: aiWords, model: aiModel||undefined })
      });
      if (res.status === 401) { location.href = '/login'; return; }
      const data = await res.json();
      if (!res.ok || !data.ok) { throw new Error(data.message || '生成失败'); }
      aiResults = (data.items||[]).map((i:any)=>({word:i.word, status:i.status, message:i.message}));
      aiMessage = `模型 ${data.model} 返回 ${aiResults.length} 项`;
      await loadData();
    }catch(err){ aiMessage = err instanceof Error ? err.message : '生成失败'; }
    finally{ aiLoading = false; }
  }

  async function removeRow(id:number){
    if (!confirm('确定要删除该词条吗？')) return;
    const res = await fetch(`/api/v1/entries/${id}`, { method:'DELETE', credentials:'include' });
    if (res.ok) list = list.filter(x=>x.entry_id!==id);
  }

  function onCellBlur(e: Event, id:number, key: keyof Entry){
    const value = (e.target as HTMLInputElement).value;
    if (key==='extra') return; // not used here
    saveField(id, { [key]: value } as any);
  }

  function onGenderClick(e: Event, row: Entry){
    const current = (row.extra?.gender||'').toLowerCase();
    const order = ['der','die','das',''];
    const next = order[(order.indexOf(current)+1) % order.length];
    const extra = { ...(row.extra||{}), gender: next } as Extra;
    saveField(row.entry_id, { extra });
  }
</script>

<main class="page">
  <header class="toolbar">
    <div class="left">
      <h1>我的词库</h1>
      {#if error}<span class="err">{error}</span>{/if}
    </div>
    <div class="right">
      <input class="search" placeholder="搜索单词/释义/英文" bind:value={q} />
      <nav class="tabs">
        <button class={`tab ${pos==='noun'?'active':''}`} on:click={() => pos='noun'}>名词</button>
        <button class={`tab ${pos==='verb'?'active':''}`} on:click={() => pos='verb'}>动词</button>
        <button class={`tab ${pos==='adjective_adverb'?'active':''}`} on:click={() => pos='adjective_adverb'}>形/副词</button>
      </nav>
      <button class="soft" on:click={openAI}>用AI生成</button>
      <button class="new" on:click={addRow} disabled={adding}>新建</button>
    </div>
  </header>

  {#if pos==='noun'}
    <section class="table-wrap">
      <table class="table">
        <thead>
          <tr>
            <th class="col-gender">Genus</th>
            <th class="col-word">Wörter</th>
            <th class="col-plural">Plural</th>
            <th class="col-zh">释义</th>
            <th class="col-en">English</th>
            <th class="col-suffix">Suffix</th>
            <th class="col-themes">Themen</th>
            <th class="col-example">Beispiel</th>
            <th class="col-actions"></th>
          </tr>
        </thead>
        <tbody>
          {#each filtered as row (row.entry_id)}
            <tr>
              <td class="col-gender">
                <button class={`pill gender ${genderColor((row.extra as any)?.gender as any)}`} on:click={(e)=>onGenderClick(e,row)}>
                  {(row.extra?.gender||'').toString() || '—'}
                </button>
              </td>
              <td class="col-word"><input value={row.word} on:blur={(e)=>onCellBlur(e,row.entry_id,'word')} /></td>
              <td class="col-plural"><input value={(row.extra?.plural as any)||''} on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), plural: (e.target as HTMLInputElement).value }})} /></td>
              <td class="col-zh"><input value={row.meaning || ''} on:blur={(e)=>onCellBlur(e,row.entry_id,'meaning')} /></td>
              <td class="col-en"><input value={row.english || ''} on:blur={(e)=>onCellBlur(e,row.entry_id,'english')} /></td>
              <td class="col-suffix"><input value={(row.extra?.suffix as any)||''} on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), suffix: (e.target as HTMLInputElement).value }})} /></td>
              <td class="col-themes"><input value={row.themes || ''} placeholder="用逗号分隔" on:blur={(e)=>onCellBlur(e,row.entry_id,'themes')} /></td>
              <td class="col-example"><input value={row.examples || ''} on:blur={(e)=>onCellBlur(e,row.entry_id,'examples')} /></td>
              <td class="col-actions"><button class="del" title="删除" on:click={() => removeRow(row.entry_id)}>🗑</button></td>
            </tr>
          {/each}
        </tbody>
      </table>
    </section>
  {:else if pos==='verb'}
    <section class="table-wrap">
      <table class="table">
        <thead>
          <tr>
            <th class="col-word">Wörter</th>
            <th class="col-en">English</th>
            <th class="col-zh">释义</th>
            <th class="col-form">现在时</th>
            <th class="col-form">过去时</th>
            <th class="col-form">完成时</th>
            <th class="col-prop">属性</th>
            <th class="col-noun">名词形式</th>
            <th class="col-themes">Themen</th>
            <th class="col-example">Beispiel</th>
            <th class="col-actions"></th>
          </tr>
        </thead>
        <tbody>
          {#each filtered as row (row.entry_id)}
            <tr>
              <td class="col-word"><input value={row.word} on:blur={(e)=>onCellBlur(e,row.entry_id,'word')} /></td>
              <td class="col-en"><input value={row.english||''} on:blur={(e)=>onCellBlur(e,row.entry_id,'english')} /></td>
              <td class="col-zh"><input value={row.meaning||''} on:blur={(e)=>onCellBlur(e,row.entry_id,'meaning')} /></td>
              <td class="col-form"><input value={(row.extra as any)?.present_form||''} on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), present_form:(e.target as HTMLInputElement).value }})} /></td>
              <td class="col-form"><input value={(row.extra as any)?.preterite_form||''} on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), preterite_form:(e.target as HTMLInputElement).value }})} /></td>
              <td class="col-form"><input value={(row.extra as any)?.perfect_form||''} on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), perfect_form:(e.target as HTMLInputElement).value }})} /></td>
              <td class="col-prop"><input value={(row.extra as any)?.properties||''} placeholder="分号/逗号分隔" on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), properties:(e.target as HTMLInputElement).value }})} /></td>
              <td class="col-noun"><input value={(row.extra as any)?.noun_form||''} on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), noun_form:(e.target as HTMLInputElement).value }})} /></td>
              <td class="col-themes"><input value={row.themes || ''} on:blur={(e)=>onCellBlur(e,row.entry_id,'themes')} /></td>
              <td class="col-example"><input value={row.examples || ''} on:blur={(e)=>onCellBlur(e,row.entry_id,'examples')} /></td>
              <td class="col-actions"><button class="del" on:click={() => removeRow(row.entry_id)}>🗑</button></td>
            </tr>
          {/each}
        </tbody>
      </table>
    </section>
  {:else}
    <section class="table-wrap">
      <table class="table">
        <thead>
          <tr>
            <th class="col-word">Wörter</th>
            <th class="col-attr">属性</th>
            <th class="col-comp">比较级</th>
            <th class="col-zh">释义</th>
            <th class="col-en">English</th>
            <th class="col-themes">Themen</th>
            <th class="col-example">Beispiel</th>
            <th class="col-actions"></th>
          </tr>
        </thead>
        <tbody>
          {#each filtered as row (row.entry_id)}
            <tr>
              <td class="col-word"><input value={row.word} on:blur={(e)=>onCellBlur(e,row.entry_id,'word')} /></td>
              <td class="col-attr"><input value={(row.extra as any)?.attribute||''} on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), attribute:(e.target as HTMLInputElement).value }})} /></td>
              <td class="col-comp"><input value={(row.extra as any)?.comparison_forms||''} placeholder="逗号/分号分隔" on:blur={(e)=>saveField(row.entry_id,{ extra:{ ...(row.extra||{}), comparison_forms:(e.target as HTMLInputElement).value }})} /></td>
              <td class="col-zh"><input value={row.meaning||''} on:blur={(e)=>onCellBlur(e,row.entry_id,'meaning')} /></td>
              <td class="col-en"><input value={row.english||''} on:blur={(e)=>onCellBlur(e,row.entry_id,'english')} /></td>
              <td class="col-themes"><input value={row.themes || ''} on:blur={(e)=>onCellBlur(e,row.entry_id,'themes')} /></td>
              <td class="col-example"><input value={row.examples || ''} on:blur={(e)=>onCellBlur(e,row.entry_id,'examples')} /></td>
              <td class="col-actions"><button class="del" on:click={() => removeRow(row.entry_id)}>🗑</button></td>
            </tr>
          {/each}
        </tbody>
      </table>
    </section>
  {/if}
  {#if showAI}
    <div class="dialog-mask" on:click={(e)=>{ if(e.target===e.currentTarget) closeAI(); }}>
      <form class="dialog" on:submit|preventDefault={runAI}>
        <h3>用 AI 生成（{pos==='noun'?'名词':pos==='verb'?'动词':'形/副词'}）</h3>
        <p class="hint">输入多个单词，使用逗号/分号/换行分隔。如：Haus, Stadt, Freude</p>
        <textarea bind:value={aiWords} placeholder="多个单词，用逗号/分号/换行分隔" />
        <div class="row">
          <input placeholder="可选：模型（如 gpt-4o-mini）" bind:value={aiModel} />
          <button class="run" type="submit" disabled={aiLoading}>{aiLoading? '生成中…' : '开始生成'}</button>
        </div>
        {#if aiMessage}<p class="msg">{aiMessage}</p>{/if}
        {#if aiResults.length}
          <ul class="result">
            {#each aiResults as r}
              <li><b>{r.word}</b> — {r.status}{#if r.message}（{r.message}）{/if}</li>
            {/each}
          </ul>
        {/if}
        <button class="close" type="button" on:click={closeAI} disabled={aiLoading}>关闭</button>
      </form>
    </div>
  {/if}
</main>

<style>
  :global(html, body){ background:#fbfbfc; }
  .page{ max-width: 1200px; margin: 24px auto; padding: 0 16px; display: grid; gap: 16px; }
  .toolbar{ display:flex; align-items:center; justify-content:space-between; gap:12px; }
  .toolbar h1{ margin:0; font-size:24px; font-weight:800; }
  .toolbar .right{ display:flex; gap:8px; align-items:center }
  .search{ width: 240px; padding:.5rem .6rem; border:1px solid #ddd; border-radius:8px; }
  .tabs{ display:flex; gap:6px; background:#f1f2f7; padding:4px; border-radius:10px }
  .tab{ border:none; background:transparent; padding:.42rem .7rem; border-radius:8px; font-weight:700; color:#4b4f66; cursor:pointer }
  .tab.active{ background:#fff; color:#1c233b; box-shadow:0 1px 2px rgba(0,0,0,.06) }
  .soft{ padding:.5rem .9rem; border:1px solid #d9def0; border-radius:8px; background:#fff; color:#1c233b; font-weight:700 }
  .new{ padding:.5rem .9rem; border:none; border-radius:8px; background:#4a66ff; color:#fff; font-weight:700 }
  .err{ color:#c00; margin-left: 12px; }

  .table-wrap{ overflow:auto; background:#fff; border:1px solid #e7e7ef; border-radius:12px; }
  table.table{ width:100%; border-collapse:separate; border-spacing:0; }
  thead th{ position:sticky; top:0; background:#fff; text-align:left; font-weight:700; font-size:13px; color:#666; padding:12px; border-bottom:1px solid #eee }
  tbody td{ padding:10px 12px; border-bottom:1px solid #f3f3f7; }
  tbody tr:hover{ background:#fafaff }
  input{ width:100%; border:1px solid transparent; outline:none; background:transparent; padding:6px 8px; border-radius:8px; }
  input:focus{ background:#fff; border-color:#cfd7ff }

  .pill.gender{ min-width:64px; text-transform: lowercase; padding:6px 10px; border-radius:999px; border:1px solid #e6e6ef; background:#f8f8fb; font-weight:700 }
  .pill.gender.male{ background:#e7ecff; color:#1b42a8; border-color:#cfd7ff }
  .pill.gender.female{ background:#ffe7f0; color:#a81b63; border-color:#ffcfe0 }
  .pill.gender.neuter{ background:#e7fff7; color:#1b7a66; border-color:#c7f3e7 }
  .pill.gender.neutral{ background:#f0f0f4; color:#666; }

  .col-gender{ width:100px }
  .col-word{ width:220px }
  .col-plural{ width:180px }
  .col-zh{ width:260px }
  .col-en{ width:260px }
  .col-suffix{ width:140px }
  .col-themes{ width:220px }
  .col-example{ width:420px }
  .col-form{ width:180px }
  .col-prop{ width:220px }
  .col-noun{ width:180px }
  .col-attr{ width:220px }
  .col-comp{ width:260px }
  .col-actions{ width:60px; text-align:right }
  .del{ border:none; background:transparent; cursor:pointer; font-size:18px }

  .dialog-mask{ position:fixed; inset:0; background:rgba(0,0,0,.35); display:grid; place-items:center; z-index:50 }
  .dialog{ width:min(640px, 92vw); background:#fff; border-radius:14px; padding:16px; display:grid; gap:10px; box-shadow:0 20px 60px rgba(0,0,0,.25) }
  .dialog h3{ margin:0; font-size:18px }
  .dialog .hint{ margin:0; color:#666; font-size:12px }
  .dialog textarea{ min-height:120px; border:1px solid #e3e6ef; border-radius:8px; padding:8px }
  .dialog .row{ display:flex; gap:8px; align-items:center }
  .dialog .row input{ flex:1; border:1px solid #e3e6ef; border-radius:8px; padding:8px }
  .dialog .run{ padding:.6rem .9rem; border:none; border-radius:8px; background:#4a66ff; color:#fff; font-weight:700 }
  .dialog .close{ justify-self:end; padding:.5rem .9rem; border:1px solid #d0d6ea; border-radius:8px; background:#fff }
  .dialog .msg{ margin:0; color:#2c3e70; font-weight:600 }
  .dialog .result{ list-style:none; margin:0; padding:0; max-height: 200px; overflow:auto }
  .dialog .result li{ padding:4px 0; border-bottom:1px dashed #eee }
</style>
