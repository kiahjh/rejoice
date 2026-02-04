# Rejoice Cloud - Planning Document

> One-click deployment platform for Rejoice applications

---

## Current Implementation Status

**Last Updated:** February 4, 2026

### What's Working

The Rejoice Cloud dashboard is functional with:

1. **Authentication**
   - GitHub OAuth login/logout
   - Session management via cookies

2. **Project Management**
   - Create new projects linked to GitHub repos
   - View project list and details
   - Delete projects

3. **Manual Deployments**
   - Click "Deploy Now" to trigger a deployment
   - Automatic project structure detection (client/, public/, database)
   - Dockerfile generation tailored to project type
   - Deploy to Fly.io with remote builder
   - Real-time log streaming to database
   - View deployment status and logs with auto-refresh

4. **Environment Variables**
   - Add/delete encrypted environment variables
   - Preview-only flag for variables
   - Variables injected as Fly secrets at deploy time

5. **Component System**
   - Full Tailwind-based UI component library
   - Cards, buttons, forms, badges, icons, etc.

### What's Not Yet Implemented

- GitHub App integration (webhooks, auto-deploy on push)
- Preview deployments (auto-create on PR)
- Custom domains
- Database migrations at deploy time
- Email notifications
- Billing/usage tracking
- Usage display in dashboard

---

## Vision

A deployment platform specifically optimized for Rejoice apps, offering the simplicity of Vercel with the performance of Rust. Built for indie developers and small teams who want to deploy without thinking about infrastructure.

**Domain Structure:**
- `rejoice.sh` - Framework homepage
- `cloud.rejoice.sh` - Cloud dashboard
- `*.rejoice.sh` - Deployed applications (e.g., `myapp.rejoice.sh`)
- `pr-{N}.{app}.rejoice.sh` - Preview deployments (e.g., `pr-123.myapp.rejoice.sh`)

---

## Decisions Made

### Infrastructure & Architecture

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Hosting Provider** | Fly.io | Excellent Rust support, fast cold starts (Firecracker), built-in networking/TLS, don't reinvent the wheel |
| **Database** | SQLite + Fly Volumes | Simple, no external service, keeps costs low. Can add Turso later if users need multi-region/horizontal scaling |
| **Dashboard Stack** | Rejoice | Dogfooding the framework, proves it works |
| **Default Region** | US East (Virginia / `iad`) | Good latency for US, decent for Europe |
| **Multi-Region** | Not for MVP | Single region initially, add multi-region when there's demand |

### Git & Deployment

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Git Provider** | GitHub only (MVP) | Simplifies integration, covers majority of users |
| **GitHub Integration** | GitHub App | Better UX than OAuth+webhooks, proper status checks, repo permissions |
| **Build Triggers** | Git push + manual button | Auto-deploy on push to main, preview on PR, plus manual "Deploy Now" |
| **Preview Deployments** | Auto-create on PR | Every PR gets `pr-{N}.{app}.rejoice.sh` |
| **Preview Cleanup** | Auto-delete when PR closes | Clean up resources automatically |
| **Rollbacks** | Redeploy previous build | Simple - re-run a past successful deployment |

### Database for Previews

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Default Strategy** | Copy from production | Copy the .db file to preview's volume for realistic testing |
| **Configurable** | Yes, via `rejoice.toml` | Support `copy`, `empty`, or `seed` strategies |

```toml
[cloud.preview]
database = "copy"  # "copy" | "empty" | "seed"
seed_file = "db/seeds.sql"  # only used if database = "seed"
```

**How preview DBs work:**
- `copy` - Clone production .db file to preview volume (default)
- `empty` - Fresh DB, just run migrations
- `seed` - Fresh DB + run seed file after migrations

### Authentication & Users

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Dashboard Auth** | GitHub OAuth only | Fits the workflow, one less password |
| **Team Support** | Single user per project (MVP) | Simplify auth, add teams post-MVP |

### Domains & SSL

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Custom Domains** | Supported from day 1 | Users expect to bring their own domains |
| **SSL Certificates** | Auto-provisioned via Fly | Fly handles Let's Encrypt automatically |

### Secrets & Configuration

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Environment Variables** | Store in dashboard DB, encrypt at rest | Simple, matches Vercel model |
| **Display** | Masked in UI, click to reveal | Standard security UX |
| **Injection** | At deploy time | Baked into the deployment |

### Observability

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Logs** | Aggregated in dashboard | Pull from Fly, display in our UI |
| **Build Notifications** | Dashboard + GitHub status + email | Full notification coverage for failures |

### Compute & Pricing

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Pricing Model** | Pure usage-based | Pay for compute time + bandwidth + DB |
| **Markup** | 10-20% over Fly's costs | Sustainable margin without being expensive |
| **Idle Behavior** | Auto-sleep after idle | Default to stopped, wake on request (~500ms cold start) |
| **Billing Method** | Card on file, monthly invoice | Standard SaaS model |
| **Spending Limits** | Optional per-user | Prevent bill shock, great for indies |
| **Billing Provider** | Stripe | Handles all complexity |

### CLI

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **CLI Tool** | Dashboard only (MVP) | CLI can come later |

---

## Technical Architecture

### System Components

```
┌─────────────────────────────────────────────────────────────────┐
│                        cloud.rejoice.sh                         │
│                     (Rejoice Dashboard App)                     │
├─────────────────────────────────────────────────────────────────┤
│  - GitHub OAuth login                                           │
│  - Project management UI                                        │
│  - Deployment history & logs                                    │
│  - Environment variables management                             │
│  - Domain configuration                                         │
│  - Usage & billing dashboard                                    │
└─────────────────────────────────────────────────────────────────┘
          │                    │
          ▼                    ▼
┌─────────────────┐  ┌─────────────────┐
│   GitHub App    │  │   Fly.io API    │
│                 │  │                 │
│ - Webhooks      │  │ - Create apps   │
│ - Status checks │  │ - Deploy        │
│ - Repo access   │  │ - Volumes       │
└─────────────────┘  │ - Logs          │
                     │ - Domains       │
                     │ - Certificates  │
                     │ - Metrics       │
                     └─────────────────┘
                              │
                              ▼
                     ┌─────────────────┐
                     │  User's App     │
                     │  (Fly Machine)  │
                     │  + Volume       │
                     │  (SQLite .db)   │
                     │                 │
                     │ myapp.rejoice.sh│
                     └─────────────────┘
```

### Deployment Flow

```
1. User pushes to GitHub (main branch or PR)
                    │
                    ▼
2. GitHub App receives webhook
                    │
                    ▼
3. Dashboard creates deployment record (status: "building")
                    │
                    ▼
4. Build triggered on Fly.io:
   a. Clone repo
   b. Run `bun install` (if client/ exists)
   c. Run `rejoice build --release`
   d. Build Docker image with binary + static assets
                    │
                    ▼
5. If PR: Set up preview database volume
      - "copy": Download prod .db, upload to preview volume
      - "empty": Just create empty volume, migrations will init
      - "seed": Empty volume, migrations + seed file
   If main: Use production volume (persistent)
                    │
                    ▼
6. Deploy to Fly.io:
   a. Create/update Fly app
   b. Set environment variables
   c. Deploy new machine
   d. Run migrations (if configured)
   e. Health check
                    │
                    ▼
7. Update deployment record (status: "success" or "failed")
                    │
                    ▼
8. Post GitHub status check
   Send email notification (if failed)
```

### Database Schema (Dashboard)

```sql
-- Users (from GitHub OAuth)
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    github_id INTEGER UNIQUE NOT NULL,
    github_username TEXT NOT NULL,
    email TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    spending_limit_cents INTEGER  -- NULL = no limit
);

-- Projects
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    github_repo TEXT NOT NULL,  -- "owner/repo"
    github_installation_id INTEGER NOT NULL,
    fly_app_name TEXT UNIQUE,
    fly_volume_id TEXT,  -- Production volume for SQLite
    custom_domain TEXT,
    preview_db_strategy TEXT DEFAULT 'copy',  -- 'copy', 'empty', 'seed'
    seed_file TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, name)
);

-- Environment Variables
CREATE TABLE env_vars (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id),
    key TEXT NOT NULL,
    encrypted_value BLOB NOT NULL,
    is_preview_only BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, key)
);

-- Deployments
CREATE TABLE deployments (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id),
    git_sha TEXT NOT NULL,
    git_branch TEXT NOT NULL,
    git_message TEXT,
    pr_number INTEGER,  -- NULL for production deploys
    status TEXT NOT NULL,  -- 'pending', 'building', 'deploying', 'success', 'failed'
    fly_machine_id TEXT,
    url TEXT,  -- The deployed URL
    build_logs TEXT,
    error_message TEXT,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    finished_at DATETIME
);

-- Usage tracking (for billing)
CREATE TABLE usage_records (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id),
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    compute_seconds INTEGER DEFAULT 0,
    bandwidth_bytes INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, period_start)
);
```

### Environment Variables

Each deployed app needs these env vars injected:

```bash
# Set by Rejoice Cloud automatically
REJOICE_ENV=production|preview
DATABASE_URL=/data/app.db  # Path on the Fly volume

# User-defined (from dashboard)
# ... whatever the user configures
```

### Fly Volume Setup

Each app gets a persistent volume mounted at `/data`:
- Production: One volume, persists across deploys
- Preview: Separate volume per PR, deleted when PR closes
- Volume size: Start small (1GB), expandable

---

## MVP Scope

### Must Have (MVP)

- [x] **Dashboard**
  - [x] GitHub OAuth login/logout
  - [x] Create project from GitHub repo
  - [x] View project list
  - [x] View deployment history
  - [x] View deployment logs (streamed from Fly)
  - [x] Manage environment variables (CRUD, masked display)
  - [x] Manual "Deploy Now" button
  - [ ] Basic usage display

- [ ] **GitHub Integration**
  - [ ] GitHub App creation & installation flow
  - [ ] Webhook handling (push, pull_request)
  - [ ] Post commit status checks

- [x] **Build & Deploy**
  - [x] Detect Rejoice project structure
  - [x] Build Rust binary + client assets
  - [x] Create Docker image
  - [x] Deploy to Fly.io
  - [ ] Run database migrations

- [ ] **Preview Deployments**
  - [ ] Auto-create on PR open/update
  - [ ] Set up preview volume with DB (copy/empty/seed)
  - [ ] Unique URL per PR
  - [ ] Auto-delete on PR close (cleanup volume too)

- [ ] **Custom Domains**
  - [ ] Add custom domain to project
  - [ ] Configure DNS instructions
  - [ ] Auto-provision SSL via Fly

- [ ] **Notifications**
  - [ ] Email on build failure
  - [ ] GitHub status checks

- [ ] **Billing**
  - [ ] Stripe integration
  - [ ] Usage tracking
  - [ ] Monthly invoicing
  - [ ] Spending limit configuration

### Nice to Have (Post-MVP)

- [ ] Team collaboration (invite members)
- [ ] CLI tool (`rejoice deploy`, `rejoice logs`)
- [ ] Multiple git providers (GitLab, Bitbucket)
- [ ] Multi-region deployment
- [ ] Database backups
- [ ] Advanced analytics / metrics
- [ ] Webhooks for deployment events
- [ ] Deploy from branch (not just main)
- [ ] Scheduled deploys
- [ ] A/B deployments / canary releases

---

## Framework Changes Required

The Rejoice framework needs these updates to support Cloud:

### Must Have

1. **`rejoice.toml` Configuration File**
   ```toml
   [app]
   name = "myapp"
   
   [cloud]
   region = "iad"  # optional, defaults to iad
   
   [cloud.preview]
   database = "copy"  # "copy" | "empty" | "seed"
   seed_file = "db/seeds.sql"
   
   [cloud.env]
   # Environment variable requirements
   required = ["STRIPE_SECRET_KEY", "OPENAI_API_KEY"]
   ```

2. **Health Check Endpoint**
   - Auto-generate `/_health` that returns 200
   - Used by Fly for health checks

3. **Graceful Shutdown**
   - Handle SIGTERM properly
   - Drain connections before exit
   - Required for zero-downtime deploys

4. **Environment Detection**
   ```rust
   rejoice::env::is_production()  // true if REJOICE_ENV=production
   rejoice::env::is_preview()     // true if REJOICE_ENV=preview
   ```

### Nice to Have

- Turso/libSQL support (if users need multi-region or horizontal scaling)
- Asset fingerprinting (hash-based filenames)
- Build info injection (git sha, build time)
- Structured logging with `tracing`

---

## Open Questions / Future Considerations

1. **Build Caching Strategy**
   - How aggressively to cache Cargo builds?
   - Use sccache? Fly's built-in caching?
   - Cache invalidation when Cargo.toml changes?

2. **Monorepo Support**
   - What if Rejoice app is in a subdirectory?
   - Need `[cloud] root = "apps/web"` config?

3. **Database Migrations**
   - Auto-run on deploy? Manual trigger?
   - What if migration fails mid-deploy?
   - Rollback strategy for failed migrations?

4. **Secrets Rotation**
   - How to update secrets without redeploying?
   - Or always require redeploy?

5. **Abuse Prevention**
   - Rate limiting on builds
   - Crypto mining detection
   - Terms of service enforcement

6. **Volume Backups**
   - Fly supports volume snapshots
   - Auto-backup before deploys? Scheduled backups?
   - User-triggered backup/restore in dashboard?

7. **Turso Option (Future)**
   - Add as optional upgrade path for users who need:
     - Multi-region read replicas
     - Horizontal scaling
     - Edge reads
   - Would require framework support for libSQL

---

## Timeline & Progress

Starting immediately, working on Cloud and framework updates in parallel.

**Phase 1: Foundation (Weeks 1-3)** ✅ COMPLETE
- [x] Set up Rejoice Cloud project (itself a Rejoice app)
- [x] GitHub OAuth integration
- [x] Basic project CRUD
- [x] Fly.io API integration basics

**Phase 2: Core Deployment (Weeks 4-6)** ✅ COMPLETE
- [x] Build system (clone, compile, dockerize)
- [x] Deploy to Fly.io
- [x] Environment variable injection
- [x] Basic logs display (with streaming updates)

**Phase 3: GitHub Integration (Weeks 7-8)** - NOT STARTED
- [ ] GitHub App setup
- [ ] Webhook handling
- [ ] Auto-deploy on push
- [ ] Status checks

**Phase 4: Preview Deployments (Weeks 9-10)** - NOT STARTED
- [ ] PR webhook handling
- [ ] Preview volume setup (copy/empty/seed DB)
- [ ] Preview URL routing
- [ ] Auto-cleanup (delete volumes on PR close)

**Phase 5: Polish & Billing (Weeks 11-13)** - NOT STARTED
- [ ] Custom domains
- [ ] Email notifications
- [ ] Stripe integration
- [ ] Usage tracking
- [ ] Spending limits

**Phase 6: Framework Updates (Parallel)** - PARTIAL
- [ ] `rejoice.toml` config
- [ ] Health checks
- [ ] Graceful shutdown

---

## Success Metrics

- Deploy a Rejoice app in under 5 minutes from repo connection
- Preview deployments live within 3 minutes of PR creation
- 99.9% uptime for deployed apps
- Build times under 5 minutes for typical apps
- At least 10 external users within 3 months of launch

---

## Known Issues / Bugs Fixed

### Fixed (February 4, 2026)

- **Dockerfile fails when `public/` doesn't exist**: The generated Dockerfile was always trying to `COPY` the `public/` directory even if the project didn't have one. Now the deployer detects whether `public/` exists and only includes the COPY command when needed.

---

*Document created: February 2026*
*Last updated: February 4, 2026*
