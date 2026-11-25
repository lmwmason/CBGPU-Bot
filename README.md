# CBGPU-Bot

Discord bot for the CBGPU ambassadors server ([cbgpu.com](https://cbgpu.com)), written in Rust with [poise](https://github.com/serenity-rs/poise).

## Features
- Ambassador registration, approval, profiles and leaderboards
- Missions with participation, submissions and point rewards
- AI assistant (`/cbgpu_ask`, `/cbgpu_explain`, `/cbgpu_summarize`, `/cbgpu_translate`) with a CBGPU knowledge base
- CBGPU info commands (`/cbgpu_about`, `/cbgpu_features`, `/cbgpu_participating_schools`, `/cbgpu_links`)

- Registration tracking: admins mark who registered their school (`/cbgpu_mark_registered`); everyone else gets a DM reminder daily at 23:00 KST

All commands are prefixed with `cbgpu_`.

## Setup
1. Copy `.env.example` to `.env` and fill in `DISCORD_TOKEN` and the `AI_*` values (any OpenAI-compatible API).
2. `cargo run`

Slash commands are registered to a single guild (the ID is set in `src/main.rs`). The bot only needs non-privileged intents.
