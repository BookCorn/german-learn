# 德语豆豆宝 - 技术文档

## 目录

1. [技术栈简介](#技术栈简介)
2. [已完成的功能] (#已完成的功能)
3. [复现指导](#复现指导)

## 技术栈简介

- **前端**: 使用 Svelte 框架，结合 Tailwind CSS 进行样式设计，确保界面简洁且响应迅速。
- **后端**: 采用 Rust的开发框架Axum并假设 FastAPI 框架，利用其高性能和安全性处理 API 请求。
- **数据库**: 选择 PostgreSQL 作为数据库，适合处理复杂查询和大规模数据存储。

## 已完成的功能

### 基本的前端页面

- 首页设计
- 用户个人资料页面
- 课程学习页面
- 课程进度跟踪页面
- 单词闪卡的基本框架
- “开发中”页面

### 后端 API

- 学习进度保存与查询
- 单词闪卡词库
- 基本的单词数据统计与分析

### 单词闪卡

- 支持动词、名词、形容词/副词不同词性的内容展示
- 支持词性、例句、复数、不规则变位等信息的展示
- 支持基本的分类查看功能

## 复现指导

### 环境准备

1. **Rust / Cargo**
   - 建议使用 `rustup` 安装稳定版：`curl https://sh.rustup.rs -sSf | sh`
   - 安装完成后执行 `rustc -V && cargo -V` 验证版本。
2. **Node.js（前端调试用）**
   - 安装 LTS 版本 (≥20.x)，核对 `node -v`、`npm -v`。
3. **PostgreSQL**
   - 安装 14+ 版本，保持本地服务运行并确保 `psql` 可用。
4. **SeaORM CLI（可选，用于重新生成 entity）**
   - `cargo install sea-orm-cli`

### 克隆项目

```bash
git clone https://github.com/BookCorn/german-learn
cd german-learn
```

### 数据库初始化

1. **创建数据库与账号（示例凭据）**

```bash
psql postgres <<'SQL'
CREATE ROLE app WITH LOGIN PASSWORD 'ReQTA35ZIQZHEVQ';
ALTER ROLE app CREATEDB;
CREATE DATABASE deutsch OWNER app;
GRANT ALL PRIVILEGES ON DATABASE deutsch TO app;
SQL
```

2. **初始化表结构**

> 若直接复用线上库，可跳过；以下用于本地空库初始化。

```bash
psql postgresql://app:ReQTA35ZIQZHEVQ@127.0.0.1:5432/deutsch <<'SQL'
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS vocabulary_entries (
    entry_id SERIAL PRIMARY KEY,
    word TEXT NOT NULL,
    part_of_speech TEXT NOT NULL,
    english TEXT,
    meaning TEXT,
    examples TEXT,
    themes TEXT,
    source_table TEXT NOT NULL,
    source_created_time TIMESTAMPTZ,
    extra JSONB
);

CREATE TABLE IF NOT EXISTS flashcard_progress (
    progress_id SERIAL PRIMARY KEY,
    entry_id INTEGER NOT NULL UNIQUE REFERENCES vocabulary_entries(entry_id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'new',
    times_seen INTEGER NOT NULL DEFAULT 0,
    times_mastered INTEGER NOT NULL DEFAULT 0,
    last_seen_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_flashcard_progress_status ON flashcard_progress(status);

CREATE TABLE IF NOT EXISTS flashcard_reviews (
    review_id BIGSERIAL PRIMARY KEY,
    entry_id INTEGER NOT NULL REFERENCES vocabulary_entries(entry_id) ON DELETE CASCADE,
    result TEXT NOT NULL,
    notes TEXT,
    reviewed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_flashcard_reviews_entry_id ON flashcard_reviews(entry_id);
SQL
```

> 📁 **migration-docs 现成脚本**
>
> - `migration-docs/schema.sql`：简化版结构脚本，可直接执行 `psql -f migration-docs/schema.sql`。
> - `migration-docs/schema_export.sql`：`pg_dump --schema-only` 导出的权威结构，包含序列、唯一约束、索引等元信息，适合在全新环境一次性恢复。
> - `migration-docs/data_export.sql`：样例数据导出（含 `vocabulary_entries`、进度与复习记录），在结构建立后执行 `psql -f migration-docs/data_export.sql` 即可导入。
>
> 推荐顺序：先执行 `schema_export.sql` 或 `schema.sql` 初始化结构，再视需要导入 `data_export.sql`。如只需词库，可编辑 `data_export.sql` 保留 `INSERT INTO vocabulary_entries` 段落。

3. **导入词汇数据**

- 可以使用生产环境导出的 `pg_dump`。

### 后端服务启动

1. **配置环境变量**

创建 `.env`（或在 shell 中导出）：

```bash
DATABASE_URL=postgresql://<user>:<password>@127.0.0.1:5432/<database>
SERVER_ADDR=127.0.0.1:8080   # 可选，默认即此端口
```

2. **启动服务**

```bash
cargo run
```

启动后可验证：

- `GET http://127.0.0.1:8080/health`
- `GET http://127.0.0.1:8080/api/v1/flashcards/next`
- `POST http://127.0.0.1:8080/api/v1/flashcards/{entry_id}/review`
- `GET http://127.0.0.1:8080/api/v1/flashcards/stats`

### 前端调试（可选）

参考 `references/svelte-flashcard-frontend`：

```bash
cd references/svelte-flashcard-frontend
npm install
npm run dev -- --open
```

将前端请求的 API 根路径指向 `http://127.0.0.1:8080/api/v1`。

### 常用辅助命令

- **重新生成 SeaORM entity**

```bash
sea-orm-cli generate entity \
  --database-url postgresql://app:ReQTA35ZIQZHEVQ@127.0.0.1:5432/deutsch \
  --output-dir src/entity
```

- **调整日志级别**
  - 通过 `RUST_LOG=german_learn=debug` 控制输出，方便调试。
