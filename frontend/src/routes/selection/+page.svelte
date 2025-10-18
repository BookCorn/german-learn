<script lang="ts">
  // 数据与状态
  type Choice = { id: number; text: string };
  let dayLabel = '第1天';
  let progress = 0.22; // 进度 0~1
  let title = '学习';
  let question = '如何用德语说“你好”？';
  let choices: Choice[] = [
    { id: 1, text: 'Hallo' },
    { id: 2, text: 'Guten Tag' },
    { id: 3, text: 'Auf Wiedersehen' },
    { id: 4, text: 'Tschüss' }
  ];
  const correct = 1;
  let picked = $state<number | null>(1); // 默认勾选第一项，便于演示
  let submitted = $state(false);

  function goBack() {
    history.length ? history.back() : (location.href = '/');
  }

  async function submit() {
    submitted = true;
    // 可在此对接后端提交记录
    // await fetch('/api/xxx', { method:'POST', body: JSON.stringify({ picked }) })
  }
  const disabled = $derived(picked === null);
  const isCorrect = $derived(submitted && picked === correct);
  const isWrong = $derived(submitted && picked !== correct);
</script>

<main class="page">
  <!-- 顶部栏 -->
  <header class="topbar">
    <button class="back" on:click={goBack} aria-label="返回">
      <span class="arrow">←</span>
    </button>
    <h1 class="title">{title}</h1>
    <div class="spacer"></div>
  </header>

  <!-- 天数与进度条 -->
  <section class="progress-wrap">
    <div class="day">{dayLabel}</div>
    <div class="progress">
      <div class="bar"><div class="fill" style={`width:${Math.max(0, Math.min(1, progress)) * 100}%`}></div></div>
    </div>
  </section>

  <!-- 题目 -->
  <section class="question">
    <h2>{question}</h2>
  </section>

  <!-- 选项列表 -->
  <section class="options">
    {#each choices as c (c.id)}
      <label class={`option ${picked === c.id ? 'active' : ''}`}>
        <input type="radio" name="opt" value={c.id} checked={picked===c.id} on:change={() => (picked = c.id)} />
        <span class="radio">
          <span class="dot"></span>
        </span>
        <span class="label">{c.text}</span>
      </label>
    {/each}
  </section>

  <!-- 结果提示（可选） -->
  {#if isCorrect}
    <p class="feedback ok">回答正确</p>
  {:else if isWrong}
    <p class="feedback wrong">再想想</p>
  {/if}

  <!-- 底部提交按钮 -->
  <footer class="footer">
    <button class="submit" on:click={submit} disabled={disabled}>提交</button>
  </footer>
</main>

<style>
  :global(html, body) { background: #faf8f4; }
  .page { min-height: 100svh; display: grid; grid-template-rows: auto auto auto 1fr auto; }

  /* 顶部栏 */
  .topbar { position: sticky; top: 0; z-index: 10; background: #faf8f4; display: grid; grid-template-columns: 56px 1fr 56px; align-items: center; padding: 10px 12px; }
  .back { width: 44px; height: 44px; border-radius: 999px; border: none; background: #f1eee8; display: grid; place-items: center; font-size: 20px; cursor: pointer; }
  .title { margin: 0; text-align: center; font-size: 22px; font-weight: 800; color: #2a2419; letter-spacing: 0.02em; }
  .spacer { width: 44px; }

  /* 进度 */
  .progress-wrap { padding: 0 20px 8px; display: grid; gap: 8px; }
  .day { font-size: 18px; font-weight: 700; color: #2a2419; }
  .progress .bar { height: 12px; background: #e7e4de; border-radius: 999px; overflow: hidden; }
  .progress .fill { height: 100%; background: #f6a400; border-radius: 999px; }

  /* 题目 */
  .question { padding: 20px; }
  .question h2 { margin: 0; font-size: 34px; line-height: 1.25; font-weight: 900; color: #1d160a; letter-spacing: 0.01em; }

  /* 选项卡片 */
  .options { display: grid; gap: 18px; padding: 8px 20px 24px; }
  .option { position: relative; display: grid; grid-template-columns: 34px 1fr; align-items: center; gap: 12px; padding: 20px 22px; border-radius: 28px; border: 3px solid #e6e3de; background: #ffffff; cursor: pointer; user-select: none; }
  .option.active { border-color: #f6a400; background: #fff6e9; box-shadow: 0 2px 0 0 rgba(246,164,0,0.2) inset; }
  .option input { position: absolute; inset: 0; opacity: 0; pointer-events: none; }
  .option .radio { width: 26px; height: 26px; border-radius: 999px; border: 3px solid #d8d5cf; display: grid; place-items: center; background: #fff; }
  .option.active .radio { border-color: #f6a400; background: #fff; }
  .option .dot { width: 10px; height: 10px; border-radius: 999px; background: transparent; }
  .option.active .dot { background: #1a73e8; box-shadow: 0 0 0 3px #fff inset; }
  .option .label { font-size: 24px; font-weight: 800; color: #20180e; }

  /* 反馈 */
  .feedback { margin: 0 20px; font-weight: 700; }
  .feedback.ok { color: #1a7f37; }
  .feedback.wrong { color: #c03530; }

  /* 底部按钮 */
  .footer { position: sticky; bottom: 0; background: linear-gradient(180deg, rgba(250,248,244,0), #faf8f4 60%); padding: 16px 20px calc(16px + env(safe-area-inset-bottom)); }
  .submit { width: 100%; height: 64px; border-radius: 999px; border: none; background: #f6a400; color: #22180a; font-weight: 900; font-size: 22px; letter-spacing: 0.04em; box-shadow: 0 10px 30px rgba(246,164,0,.35); cursor: pointer; }
  .submit:disabled { opacity: .6; cursor: not-allowed; }

  @media (min-width: 820px) {
    .page { max-width: 480px; margin: 0 auto; border-radius: 20px; box-shadow: 0 20px 60px rgba(0,0,0,.06); background: #faf8f4; }
  }
</style>
