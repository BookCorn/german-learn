<script lang="ts">
  // Clean Flashcard component with accessible flip and POS-specific UI
  type FlashcardMetadata =
    | { type: 'noun'; gender?: string | null; plural?: string | null }
    | { type: 'verb'; present_form?: string | null; preterite_form?: string | null; perfect_form?: string | null }
    | { type: 'adjective_adverb'; attribute?: string | null; comparison_forms?: string[] | null };

  export type Flashcard = {
    entry_id: number;
    word: string;
    part_of_speech: string;
    meaning?: string | null;
    english?: string | null;
    examples?: string | null;
    status?: string | null;
    times_seen: number;
    times_mastered: number;
    last_seen_at?: string | null;
    metadata?: FlashcardMetadata | null;
  };

  let { card }: { card: Flashcard } = $props();

  let showBack = $state(false);

  $effect(() => {
    // reset flip when card id changes
    showBack = false;
  });

  const statusText = (s?: string | null) => {
    const v = (s ?? 'new').toLowerCase();
    if (v === 'mastered') return '已掌握';
    if (v === 'learning') return '学习中';
    return '未学习';
  };

  const noun = $derived(card?.metadata && (card.metadata as any).type === 'noun' ? (card.metadata as any) : null);
  const verb = $derived(card?.metadata && (card.metadata as any).type === 'verb' ? (card.metadata as any) : null);
  const adj  = $derived(card?.metadata && (card.metadata as any).type === 'adjective_adverb' ? (card.metadata as any) : null);

  function formatGender(g?: string | null) {
    if (!g) return '';
    const n = g.trim().toLowerCase();
    return n.charAt(0).toUpperCase() + n.slice(1);
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      showBack = !showBack;
    }
  }

  function parseExamples(value?: string | null) {
    if (!value) return [] as string[];
    return value.split(/[\r\n\u{FF1B};]+/u).map((x) => x.trim()).filter(Boolean);
  }

  const examples = $derived(parseExamples(card?.examples));
</script>

<div class="scene">
  <article class={`card ${showBack ? 'is-flipped' : ''}`}
    role="button"
    tabindex="0"
    aria-pressed={showBack}
    onclick={() => (showBack = !showBack)}
    onkeydown={onKey}
  >
    <!-- Front -->
    <div class="face front">
      <header class="top">
        {#if card.part_of_speech === 'noun'}
          <span class="pos">名词</span>
        {/if}
        <span class="status" data-status={(card.status ?? 'new').toLowerCase()}>{statusText(card.status)}</span>
      </header>
      <div class="main">
        <h2 class="word">{card.word}</h2>
        {#if card.part_of_speech === 'noun' && noun?.plural}
          <p class="sub">复数：{noun.plural}</p>
        {:else if card.english}
          <p class="sub">{card.english}</p>
        {:else if card.meaning}
          <p class="sub">{card.meaning}</p>
        {/if}
      </div>
    </div>

    <!-- Back -->
    <div class="face back">
      <header class="back-head">
        {#if card.part_of_speech === 'noun'}
          <div class="articles">
            {#if noun?.gender}
              <span class={`chip ${noun.gender?.trim().toLowerCase()}`}>{formatGender(noun.gender)}</span>
            {/if}
            {#if noun?.plural}
              <span class="chip plural">die 复数</span>
            {/if}
          </div>
        {/if}
        <h3 class="title">{card.word}</h3>
      </header>

      <section class="meaning">
        {#if card.meaning}
          <p class="zh">{card.meaning}</p>
        {/if}
        {#if card.english}
          <p class="en">{card.english}</p>
        {/if}
      </section>

      {#if verb && (verb.present_form || verb.preterite_form || verb.perfect_form)}
        <ul class="morph">
          {#if verb.present_form}<li><span>现在时</span><em>{verb.present_form}</em></li>{/if}
          {#if verb.preterite_form}<li><span>过去时</span><em>{verb.preterite_form}</em></li>{/if}
          {#if verb.perfect_form}<li><span>完成时</span><em>{verb.perfect_form}</em></li>{/if}
        </ul>
      {:else if adj && (adj.attribute || (adj.comparison_forms?.length ?? 0) > 0)}
        <ul class="morph">
          {#if adj.attribute}<li><span>属性</span><em>{adj.attribute}</em></li>{/if}
          {#if adj.comparison_forms?.[0]}<li><span>比较级</span><em>{adj.comparison_forms?.[0]}</em></li>{/if}
          {#if adj.comparison_forms?.[1]}<li><span>最高级</span><em>{adj.comparison_forms?.[1]}</em></li>{/if}
        </ul>
      {/if}

      <footer class="meta">
        <small>复习 {card.times_seen} 次 · 掌握 {card.times_mastered} 次</small>
      </footer>
    </div>
  </article>
</div>

<style>
.scene { perspective: 1200px; }
.card {
  position: relative;
  width: 100%;
  min-height: 260px;
  transform-style: preserve-3d;
  transition: transform .55s ease;
}
.card.is-flipped { transform: rotateY(180deg); }

.face {
  position: absolute;
  inset: 0;
  background: #fff;
  border: 1px solid #e5eef8;
  border-radius: 24px;
  box-shadow: 0 12px 28px rgba(18, 46, 84, .10);
  padding: 1.25rem 1rem;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  backface-visibility: hidden;
}
.back { transform: rotateY(180deg); }

.top, .back-head { display:flex; align-items:center; justify-content: space-between; gap:.8rem; }
.pos { background:#fff3e6; color:#8b5600; border:1px solid #ffe2bd; border-radius: 999px; padding:.2rem .6rem; font-weight:700; font-size:.78rem; }
.status { border:1px solid #d7e0f0; border-radius:999px; padding:.22rem .7rem; font-weight:700; font-size:.78rem; }
.status[data-status='learning']{ background:#f7fff0; color:#476d1f; border-color:#cfe6b7; }
.status[data-status='mastered']{ background:#f2f8ff; color:#0f6ad2; border-color:#b8d6ff; }

.main { text-align:center; margin: .3rem 0 .4rem; }
.word { margin:0; font-size:2.2rem; font-weight:800; color:#1b2536; }
.sub  { margin:.5rem 0 0; color:#4b5d79; font-weight:600; }

.back-head { justify-content:flex-start; gap: .75rem; }
.title { margin:0; font-size:1.5rem; font-weight:800; color:#1b2536; }
.articles { display:flex; flex-direction:column; gap:.35rem; }
.chip { border-radius:999px; padding:.22rem .6rem; font-weight:800; font-size:.78rem; border:1px solid transparent; }
.chip.der { background:rgba(28,122,255,.12); color:#1c7aff; border-color:#bcd6ff; }
.chip.das { background:rgba(16,181,120,.12); color:#0fb578; border-color:#bfeeda; }
.chip.die { background:rgba(233,66,77,.12); color:#e9424d; border-color:#ffd0d3; }
.chip.plural { background:rgba(147,107,255,.12); color:#8a5cff; border-color:#dec8ff; }

.meaning { display:flex; flex-direction:column; gap:.3rem; }
.meaning .zh { margin:0; font-weight:700; color:#1b2536; }
.meaning .en { margin:0; color:#4b5d79; }

.morph { list-style:none; margin:.6rem 0 0; padding:0; display:flex; gap:.6rem; flex-direction:column; }
.morph li { display:flex; align-items:baseline; gap:.6rem; }
.morph li span { color:#6a7a96; font-size:.82rem; }
.morph li em { font-style:normal; color:#1b2536; font-weight:700; }

.meta { color:#6a7a96; font-size:.8rem; margin-top:.6rem; }

@media (min-width: 520px){ .word{font-size:2.4rem;} }
</style>
