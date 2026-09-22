# 🐙 Git & GitHub Practical Cheatsheet & Handbook

> A beginner-friendly, practical guide to Git: understanding the internal model, undoing mistakes, rewinding commits, rewriting history, cherry-picking, resolving conflicts, reflog disaster recovery, bisect debugging, daily commands, and writing GitHub Actions CI workflows.

---

## 📑 Table of Contents
1. [Core Mental Model: The 3 Git Trees & What `HEAD` Is](#1-core-mental-model-the-3-git-trees--what-head-is)
2. [Undoing Mistakes & Rollback Scenarios](#2-undoing-mistakes--rollback-scenarios)
   - [Scenario A: Just committed, need to fix message or add a forgotten file](#scenario-a-just-committed-need-to-fix-message-or-add-a-forgotten-file)
   - [Scenario B: Committed wrong, keep changes in code](#scenario-b-committed-wrong-keep-changes-in-code)
   - [Scenario C: Committed wrong, completely throw away the changes](#scenario-c-committed-wrong-completely-throw-away-the-changes)
   - [Scenario D: Rollback where some changes stay and some are removed](#scenario-d-rollback-where-some-changes-stay-and-some-are-removed)
   - [Scenario E: Rollback 3 to 4 commits back](#scenario-e-rollback-3-to-4-commits-back)
   - [Scenario F: If you already pushed to GitHub (Public commits)](#scenario-f-if-you-already-pushed-to-github-public-commits)
3. [Modifying an Older Local Commit (Commit 1 out of 3)](#3-modifying-an-older-local-commit-commit-1-out-of-3)
4. [Cherry-Pick: What, Why, and How to Use It](#4-cherry-pick-what-why-and-how-to-use-it)
5. [Merge Conflicts Demystified](#5-merge-conflicts-demystified)
6. [Git Merge vs. Git Rebase & Squashing Commits](#6-git-merge-vs-git-rebase--squashing-commits)
7. [The Safety Net: `git reflog` (Recovering "Lost" Commits & Deleted Branches)](#7-the-safety-net-git-reflog-recovering-lost-commits--deleted-branches)
8. [Git Bisect: Finding Bugs Fast via Binary Search](#8-git-bisect-finding-bugs-fast-via-binary-search)
9. [Git Tags & Releases](#9-git-tags--releases)
10. [Git Worktrees: Multitasking Across Branches Without Switching](#10-git-worktrees-multitasking-across-branches-without-switching)
11. [Detective Git: Blame & Pickaxe Search](#11-detective-git-blame--pickaxe-search)
12. [Daily Git Commands Cheatsheet](#12-daily-git-commands-cheatsheet)
13. [GitHub Actions & Workflow YAML Guide](#13-github-actions--workflow-yaml-guide)
14. [GitHub Collaboration & Authentication (SSH, Remotes & PRs)](#14-github-collaboration--authentication-ssh-remotes--prs)
15. [Real-World Emergency Scenarios & High-Impact Fixes](#15-real-world-emergency-scenarios--high-impact-fixes)
   - [15.1 Leaked Secrets & Private Keys Pushed to GitHub](#151-leaked-secrets--private-keys-pushed-to-github)
   - [15.2 The 100MB Giant File Rejection (Push Blocked) & Git LFS](#152-the-100mb-giant-file-rejection-push-blocked--git-lfs)
   - [15.3 Accidentally Committed Directly to `main` Instead of a Feature Branch](#153-accidentally-committed-directly-to-main-instead-of-a-feature-branch)
   - [15.4 Escaping the "Detached HEAD" State](#154-escaping-the-detached-head-state)
   - [15.5 Windows vs. Linux Line Endings Hell (CRLF vs. LF)](#155-windows-vs-linux-line-endings-hell-crlf-vs-lf)
   - [15.6 Keeping an Out-of-Date Feature Branch in Sync with `main`](#156-keeping-an-out-of-date-feature-branch-in-sync-with-main)
   - [15.7 File Renaming and Case-Sensitivity Glitches on Windows](#157-file-renaming-and-case-sensitivity-glitches-on-windows)
   - [15.8 Advanced Stashing: Untracked Files, Stash Inspection & Stash-to-Branch](#158-advanced-stashing-untracked-files-stash-inspection--stash-to-branch)

---

## 1. Core Mental Model: The 3 Git Trees & What `HEAD` Is

### The 3 Areas of Git
Git manages your code across 3 distinct zones:

```text
[ Working Directory ]  --( git add )-->  [ Staging Area (Index) ]  --( git commit )-->  [ Git History (Repository) ]
(files you edit)                         (prepared snapshot)                            (permanent saved snapshots)
```

1. **Working Directory (Working Tree):** The actual files on your hard drive you can open and edit.
2. **Staging Area (`Index`):** The waiting room. When you type `git add file.rs`, you tell Git: *"Include this file in the next snapshot."*
3. **Repository (`Commit History`):** The database where snapshots are permanently stored with a unique SHA hash.

### What is `HEAD`?
- **`HEAD` is a pointer (bookmark) that says: "You are currently here."**
- Usually, `HEAD` points to the latest commit on the branch you are actively working on (e.g. `main` or `feature`).
- **`HEAD~1`** means: 1 commit before current `HEAD` (the parent commit).
- **`HEAD~2`** means: 2 commits before current `HEAD`.
- **`HEAD~3`** means: 3 commits before current `HEAD`.

```text
Commit A  <--  Commit B  <--  Commit C (HEAD)
 HEAD~2          HEAD~1          HEAD
```

---

## 2. Undoing Mistakes & Rollback Scenarios

### Scenario A: Just committed, need to fix message or add a forgotten file
You just ran `git commit -m "add indexer skeleton"`, but forgot to include `config.rs` or had a typo.

```bash
# 1. Edit the file or add the forgotten file
git add config.rs

# 2. Amend the last commit without creating a second commit
git commit --amend -m "add indexer skeleton and config"

# Or amend keeping the same message:
git commit --amend --no-edit
```

---

### Scenario B: Committed wrong, keep changes in code
You committed, realized you weren't ready, but you do **not** want to lose your code.

#### Option 1: Keep changes STAGED (`--soft`)
```bash
git reset --soft HEAD~1
```
- **What happens:** The last commit is undone. The files remain in your **Staging Area** as green changes (`git status`). You can add more files and re-commit.

#### Option 2: Keep changes UNSTAGED in Working Directory (`--mixed`, default)
```bash
git reset HEAD~1
# (or git reset --mixed HEAD~1)
```
- **What happens:** The last commit is undone. Your files stay exactly as you wrote them on disk, but they appear as unstaged (red changes).

---

### Scenario C: Committed wrong, completely throw away the changes
You made a disaster commit and want to wipe it off the face of the earth.

```bash
# CAUTION: This deletes all code changes in that commit permanently!
git reset --hard HEAD~1
```
- **What happens:** `HEAD`, the staging area, and your local working files are rolled back to the previous commit. All changes in that commit are destroyed.

---

### Scenario D: Rollback where some changes stay and some are removed
**Requirement:** You committed 5 files (or multiple changes). You want to rollback the commit, keep 3 files, and throw away the other 2.

#### Step-by-Step:
```bash
# Step 1: Undo the commit, putting all changes back into working directory
git reset HEAD~1

# Step 2: Check status to see all modified files
git status
# Example:
#   file_to_keep_1.rs
#   file_to_keep_2.rs
#   file_to_discard.rs

# Step 3: Discard the files you DO NOT want
git restore file_to_discard.rs
# (Older git syntax: git checkout -- file_to_discard.rs)

# If it's a new untracked file you want to delete:
# rm unwanted_file.rs   (or on PowerShell: Remove-Item unwanted_file.rs)

# Step 4: Stage the files you WANT to keep
git add file_to_keep_1.rs file_to_keep_2.rs

# Step 5: Make your clean commit
git commit -m "clean commit with only desired files"
```

#### What if you only want part of a single file?
Use interactive restore:
```bash
git restore -p file.rs
```
Git will show each chunk ("hunk") and ask: `Discard this hunk from worktree [y,n,q,a,d,e,?]?`. Press `y` to discard, `n` to keep!

---

### Scenario E: Rollback 3 to 4 commits back
You made commits `C1 -> C2 -> C3 -> C4`. You want to go back to `C1` (jump back 3 commits).

#### 1. If you want to keep all code from C2, C3, C4 as uncommitted changes:
```bash
# Rewind 3 commits, code remains in working directory
git reset HEAD~3

# Or to keep them already staged:
git reset --soft HEAD~3
```

#### 2. If you want to completely erase the last 3 commits and code:
```bash
# WARNING: Wipes all changes from those 3 commits
git reset --hard HEAD~3
```

#### 3. Rewind to a specific known commit hash:
```bash
# 1. Look up commit history
git log --oneline -n 10

# Output:
# a1b2c3d (HEAD) commit 4
# f4e5d6c commit 3
# 7g8h9i0 commit 2
# 1a2b3c4 commit 1

# 2. Reset directly to commit 1's hash:
git reset --soft 1a2b3c4
```

---

### Scenario F: If you already pushed to GitHub (Public commits)
> **Rule of thumb:** Never use `git reset --hard` on commits that teammates have already pulled, because it alters history and causes sync chaos.

Instead, use **`git revert`**:
```bash
# Revert the latest commit by creating an opposite "undo" commit:
git revert HEAD

# Revert a specific commit:
git revert <commit-hash>

# Revert the last 3 commits:
git revert HEAD~2..HEAD
```
- **Why this is safe:** It does not erase past history. It appends a brand new commit that undoes the previous code. Push it safely: `git push origin main`.

*(If it's your personal private branch and nobody else is working on it, you can `git reset` and push with `git push --force-with-lease origin branch-name`).*

---

## 3. Modifying an Older Local Commit (Commit 1 out of 3)

### The Problem
You have 3 local commits:
- `Commit 1` (Oldest local commit): Added database connection setup
- `Commit 2`: Added schema migration
- `Commit 3` (Latest / `HEAD`): Added user endpoints

Now you realize: **"I forgot a config field in Commit 1!"**

### The Solution: Interactive Rebase (`git rebase -i`)

#### Step 1: Start interactive rebase for the last 3 commits
```bash
git rebase -i HEAD~3
```

#### Step 2: An editor will open showing the commits in chronological order (oldest at top)
```text
pick a1b2c3d Commit 1: database connection
pick f4e5d6c Commit 2: schema migration
pick 7g8h9i0 Commit 3: user endpoints
```

Change the word `pick` to `edit` (or just `e`) on Commit 1:
```text
edit a1b2c3d Commit 1: database connection
pick f4e5d6c Commit 2: schema migration
pick 7g8h9i0 Commit 3: user endpoints
```
Save and close the editor (In VS Code / Notepad: Ctrl+S and close; in Vim: `:wq`).

#### Step 3: Git pauses at Commit 1
Git pauses time right after Commit 1 was made! Now:
1. Open your files and make the fix (e.g. edit `db.rs`).
2. Stage the modified file:
   ```bash
   git add db.rs
   ```
3. Amend the commit:
   ```bash
   git commit --amend
   ```
   *(Update the message if you want, or add `--no-edit` to keep the same message).*

#### Step 4: Continue the rebase
Tell Git to replay Commit 2 and Commit 3 on top of your modified Commit 1:
```bash
git rebase --continue
```
Git will fast-forward and re-apply Commits 2 and 3 automatically.

> **Panic Button:** If you get confused or run into messy conflicts:
> ```bash
> git rebase --abort
> ```
> This immediately returns your repository to the exact state before you started the rebase.

---

## 4. Cherry-Pick: What, Why, and How to Use It

### What is Cherry-Pick?
**Analogy:** Imagine a shopping cart with 10 items. You don't want the whole cart; you just want the single apple from it.
**Technical:** `git cherry-pick <commit-hash>` takes a specific commit from *any* branch and applies that exact commit's changes onto your *current* branch as a new commit.

### Why do we use it?
1. **Critical Bugfix / Hotfix:** A bug was fixed in an unreleased experimental branch (`feature-experimental`), and production (`main`) needs that fix immediately without merging the unfinished feature code.
2. **Accidental Commit on Wrong Branch:** You committed work to `main` by mistake instead of `feature-auth`. You can switch to `feature-auth`, cherry-pick that commit, and reset `main`.
3. **Pulling one utility from a colleague's branch:** A teammate wrote a helper function you need, but their branch isn't ready for a full PR merge yet.

### How to Cherry-Pick:
```bash
# 1. Find the hash of the commit you want (from git log or GitHub)
git log --oneline feature-branch -n 5
# Example output: e9b1c2f fix: prevent integer overflow in parser

# 2. Switch to the branch where you want to apply the fix
git switch main

# 3. Cherry-pick that commit
git cherry-pick e9b1c2f
```

#### If there is a merge conflict during cherry-pick:
1. Git will pause and flag conflict files.
2. Open the file, resolve the conflict markings (`<<<<<<<`, `=======`, `>>>>>>>`).
3. Stage the resolved file:
   ```bash
   git add resolved_file.rs
   ```
4. Continue:
   ```bash
   git cherry-pick --continue
   ```
*(Or abort: `git cherry-pick --abort`)*

---

## 5. Merge Conflicts Demystified

### Why do merge conflicts occur?
Git can automatically merge files 95% of the time. A **conflict** only happens when:
- You and another developer changed the **exact same lines of code** in two different ways.
- Git cannot guess whose code is correct, so it halts and asks a human to decide.

### How to Read Conflict Markers
When a conflict happens, Git writes special markers directly into the file:

```rust
<<<<<<< HEAD (Current change: What is currently in your active branch)
let rpc_timeout_ms = 5000;
=======
let rpc_timeout_ms = 10000;
>>>>>>> feature/long-timeout (Incoming change: What is in the branch you are merging)
```

- Everything between `<<<<<<<` and `=======` is YOUR branch's code.
- Everything between `=======` and `>>>>>>>` is the INCOMING code.

### Step-by-Step Resolution:
1. Open the file in your editor (VS Code will highlight "Accept Current Change", "Accept Incoming Change", or "Accept Both").
2. Delete the conflict lines you don't want, or combine them so the code compiles and tests pass.
3. Remove all marker lines (`<<<<<<<`, `=======`, `>>>>>>>`).
4. Stage the resolved file:
   ```bash
   git add src/main.rs
   ```
5. Complete the merge:
   ```bash
   git commit -m "Merge branch 'feature/long-timeout' and resolve timeout conflict"
   ```

> **Abort Emergency:** If conflicts feel overwhelming and you want to start over:
> ```bash
> git merge --abort
> ```

---

## 6. Git Merge vs. Git Rebase & Squashing Commits

### The Core Difference

| Aspect | `git merge` | `git rebase` |
|:---|:---|:---|
| **What it does** | Combines two branch tips by creating a new "Merge commit" | Re-writes history by moving your commits onto the tip of `main` |
| **History Look** | Preserves branch history graphs (spider web) | Clean, flat, straight linear line |
| **Traceability** | Perfect for seeing when features were merged | Cleaner to read and easier to `git bisect` |

```text
MERGE:
A---B---C (main)
 \       \
  D---E---M (merge commit on main)

REBASE:
A---B---C (main)
         \
          D'---E' (commits replayed on top of C)
```

### The Golden Rule of Rebase
> **Never rebase a public/shared branch that other developers have pulled.**
> Only rebase your own local feature branch before merging into `main`.

---

### How to Squash 5 Messy Commits into 1 Clean Commit
When working on a feature, you might make messy commits:
- `wip`
- `fix typo`
- `oops forgot semicolon`
- `fix test`
- `done`

Before submitting your Pull Request, clean this up into a single professional commit:

```bash
git rebase -i HEAD~5
```

In the editor:
```text
pick 1a2b3c4 feat: implement token decoder
squash 2b3c4d5 wip
squash 3c4d5e6 fix typo
squash 4d5e6f7 oops forgot semicolon
squash 5e6f7a8 done
```
*(Tip: `squash` keeps commit messages; `fixup` or `f` discards the messy messages and keeps only the top commit's message).*

Save and close. Git merges all 5 commits into 1 single tidy commit!

---

## 7. The Safety Net: `git reflog` (Recovering "Lost" Commits & Deleted Branches)

### What is Reflog?
Git almost **never** deletes your commits immediately. Even if you ran `git reset --hard` or deleted a branch by accident, Git keeps a hidden log of every single move `HEAD` has made for ~30 days.

This is the **Reflog** (Reference Log).

### How to Rescue a Lost Commit:
1. Run `git reflog`:
   ```bash
   git reflog
   ```
   Output:
   ```text
   a1b2c3d HEAD@{0}: reset: moving to HEAD~2
   f4e5d6c HEAD@{1}: commit: Important indexer parsing logic  <-- THE LOST WORK!
   7g8h9i0 HEAD@{2}: commit: Added database client
   ```
2. You see `f4e5d6c` was the commit before your accidental reset.
3. Bring it back to life!
   ```bash
   # Option 1: Create a new branch pointing directly to the lost commit
   git switch -c recovered-work f4e5d6c

   # Option 2: Or reset your current branch back to it
   git reset --hard f4e5d6c
   ```
*Your "deleted" work is 100% restored.*

---

## 8. Git Bisect: Finding Bugs Fast via Binary Search

Imagine you have 100 commits between `v1.0.0` (working) and today's commit (broken). How do you find which exact commit broke the build without manually testing 100 times?

**`git bisect`** uses binary search to find the culprit in ~7 checks ($2^7 = 128$).

### Step-by-Step:
```bash
# 1. Start bisect mode
git bisect start

# 2. Tell Git the current commit has the bug
git bisect bad

# 3. Tell Git an older commit/tag that was definitely working
git bisect good v1.0.0

# 4. Git checks out the middle commit automatically. Run your test:
cargo test

# 5. If test fails:
git bisect bad

# 6. If test passes:
git bisect good

# Git repeats until it prints:
# "c3d4e5f is the first bad commit"
# Author: Yash <yash@...>
# Commit message: changed slot deserializer

# 7. End bisect and return to your normal branch:
git bisect reset
```

---

## 9. Git Tags & Releases

Tags are permanent bookmarks pointing to a specific commit, typically used for software releases (e.g. `v1.0.0`).

### Types of Tags:
1. **Lightweight Tag:** Just a named pointer to a commit.
   ```bash
   git tag v1.0.0-beta
   ```
2. **Annotated Tag (Recommended):** Stores the author, date, and a release message with GPG signature capability.
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0 with SPL token support"
   ```

### Managing Tags:
```bash
# List all tags
git tag -l

# Show tag details & commit info
git show v1.0.0

# Push a specific tag to GitHub
git push origin v1.0.0

# Push all local tags to GitHub
git push origin --tags

# Delete a tag locally
git tag -d v1.0.0

# Delete a tag from GitHub
git push origin --delete v1.0.0
```

---

## 10. Git Worktrees: Multitasking Across Branches Without Switching

### The Problem
You are working on a huge feature on branch `feature-decoder`. Files are uncommitted, dependencies are building. Suddenly, production has an urgent bug that needs a quick fix on `main`.
Normally, you'd have to stash your changes, switch branches, build, fix, switch back, and pop the stash.

### The Solution: `git worktree`
A **worktree** lets you have multiple branches checked out in **separate folders simultaneously**, sharing the exact same `.git` database!

```bash
# Create a new directory alongside your project checked out to 'main'
git worktree add ../rust-indexer-hotfix main

# Open that new directory in another terminal/editor, make your quick fix, commit, and push!
# Your main project folder remains completely untouched and dirty files are undisturbed.

# When done, delete the worktree folder:
git worktree remove ../rust-indexer-hotfix
```

---

## 11. Detective Git: Blame & Pickaxe Search

### Who wrote this code? (`git blame`)
```bash
# See line-by-line who last edited src/main.rs, with commit hash and timestamp:
git blame src/main.rs

# Blame only lines 20 through 45:
git blame -L 20,45 src/main.rs
```

### When was a specific variable or function introduced? (`Pickaxe`)
If you want to know when `MAX_SLOT_LAG` was added or removed across the whole history of the codebase:
```bash
git log -S "MAX_SLOT_LAG" --oneline
```

### View commit history with line changes for a specific file:
```bash
git log -p src/main.rs
```

---

## 12. Daily Git Commands Cheatsheet

### 🔍 Checking Status & History
| Command | What it does |
|:---|:---|
| `git status` | Shows modified, untracked, and staged files |
| `git status -s` | Short/compact status format |
| `git log --oneline -n 10` | Shows last 10 commits as single lines |
| `git log --oneline --graph --all` | ASCII visualization of branches and merges |
| `git diff` | Shows unstaged changes in your working tree |
| `git diff --staged` (or `--cached`) | Shows changes that are staged for the next commit |

### 🌿 Branches
| Command | What it does |
|:---|:---|
| `git branch` | Lists local branches |
| `git branch -a` | Lists local and remote branches |
| `git switch <branch>` | Switches to an existing branch (modern syntax) |
| `git switch -c <new-branch>` | Creates and switches to a new branch |
| `git checkout -b <new-branch>` | Traditional equivalent of `switch -c` |
| `git branch -m <new-name>` | Renames the current branch |
| `git branch -d <branch>` | Safely deletes a merged branch |
| `git branch -D <branch>` | Force deletes a branch |

### 💾 Staging & Committing
| Command | What it does |
|:---|:---|
| `git add <file>` | Stages a specific file |
| `git add .` | Stages all changes in the current directory |
| `git add -p` | Interactive staging (pick individual code hunks) |
| `git commit -m "message"` | Commits staged changes |
| `git commit -am "message"` | Automatically stages tracked files and commits |
| `git commit --amend` | Modifies the latest commit |

### 📦 Stashing (Saving work temporarily without committing)
| Command | What it does |
|:---|:---|
| `git stash` | Shelves dirty working changes away |
| `git stash save "work in progress"` | Stashes with a descriptive label |
| `git stash pop` | Re-applies the most recent stash and removes it from stash list |
| `git stash apply` | Re-applies stash but keeps it in the stash list |
| `git stash list` | Lists all saved stashes |
| `git stash drop` | Discards the most recent stash |

### 🚀 Syncing with GitHub
| Command | What it does |
|:---|:---|
| `git remote -v` | Shows configured remote URLs |
| `git fetch origin` | Downloads changes from remote without merging |
| `git fetch --prune` | Cleans up local references to deleted remote branches |
| `git pull origin main` | Fetches and merges remote changes into current branch |
| `git push -u origin main` | Pushes current branch to remote and sets upstream tracking |
| `git push` | Pushes to tracked remote branch |

### 🧹 Cleaning Untracked Junk
| Command | What it does |
|:---|:---|
| `git clean -n` | Dry run: preview untracked files that would be deleted |
| `git clean -fd` | Force delete untracked files and directories |

---

## 13. GitHub Actions & Workflow YAML Guide

### What is GitHub Actions?
GitHub Actions is an automated runner in the cloud. Every time you push code or open a Pull Request, GitHub boots a temporary virtual machine (Ubuntu, Windows, or macOS) to automatically build your project, run unit tests, and check code style.

### Where does it live?
All workflow files **must** be stored in:
```text
.github/workflows/<workflow-name>.yml
```

### Anatomy of a Workflow File
Here are the core concepts explained simply:
- **`name`**: The friendly title shown in the GitHub Actions tab.
- **`on`**: The triggers (e.g. `push`, `pull_request`, `schedule`).
- **`jobs`**: Group of tasks. Each job runs on its own virtual machine.
- **`runs-on`**: The operating system (`ubuntu-latest`, `windows-latest`, etc.).
- **`steps`**: Sequence of actions executed in order inside the job.
- **`uses`**: Pre-built community actions (like `actions/checkout@v4` to download your repo code).
- **`run`**: Terminal shell commands to run (like `cargo test` or `npm test`).

---

### Ready-to-Use Example: Rust CI Workflow
Create a file at `.github/workflows/ci.yml`:

```yaml
name: Rust Continuous Integration

# When should this workflow run?
on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

# Jobs to execute
jobs:
  check-and-test:
    name: Build, Lint & Test
    runs-on: ubuntu-latest

    steps:
      # Step 1: Clone this repository into the runner VM
      - name: Checkout repository code
        uses: actions/checkout@v4

      # Step 2: Install the Rust toolchain
      - name: Set up Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      # Step 3: Cache dependencies so subsequent runs take seconds instead of minutes
      - name: Cache cargo registry & build target
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      # Step 4: Check code formatting
      - name: Check code formatting (rustfmt)
        run: cargo fmt --check

      # Step 5: Run linter for code smells and warnings
      - name: Run Clippy lints
        run: cargo clippy -- -D warnings

      # Step 6: Build the project
      - name: Build project
        run: cargo build --verbose

      # Step 7: Run test suite
      - name: Run unit & integration tests
        run: cargo test --verbose
```

### When to Trigger Workflows:
1. **On Pull Request (`on: [pull_request]`):** Runs CI automatically when someone creates a PR so you know if tests pass before clicking Merge.
2. **On Push to Main (`on: push: branches: [main]`):** Ensures the main branch is never broken.
3. **On Tag Release (`on: push: tags: ['v*']`):** Automatically compiles release binaries and creates a GitHub Release.
4. **Scheduled (`on: schedule: - cron: '0 0 * * *'`):** Runs nightly tests or maintenance checks.

---

## 14. GitHub Collaboration & Authentication (SSH, Remotes & PRs)

### Authentication: SSH Keys vs Personal Access Tokens (PAT)
Password authentication to GitHub was deprecated in 2021. You have two options:

#### Option A: SSH Key (Recommended — set and forget)
1. Generate an SSH key:
   ```bash
   ssh-keygen -t ed25519 -C "your_email@example.com"
   ```
2. Copy the public key (`~/.ssh/id_ed25519.pub`).
3. Add it to GitHub: **Settings -> SSH and GPG keys -> New SSH Key**.
4. Clone via SSH:
   ```bash
   git clone git@github.com:yash-0025/solana-indexer.git
   ```

#### Option B: Personal Access Token (PAT)
- GitHub Settings -> Developer Settings -> Personal access tokens (Tokens classic) -> Generate token with `repo` scope.
- Use this token instead of your account password when prompted by Git.

---

### Contributing to Open Source (Fork & Upstream Workflow)
When contributing to a repo you don't own:
1. **Fork** the repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/solana-indexer.git
   ```
3. Add the original author's repository as an `upstream` remote:
   ```bash
   git remote add upstream https://github.com/original-owner/solana-indexer.git
   ```
4. Keep your fork in sync with upstream changes:
   ```bash
   git fetch upstream
   git switch main
   git merge upstream/main
   ```

---

### Pull Request (PR) Best Practices
1. **Never commit directly to `main` for new features:**
   Create a descriptive branch:
   - `feat/token-decoder`
   - `fix/rpc-reconnect`
   - `docs/update-readme`
2. **Push your branch:**
   ```bash
   git push -u origin feat/token-decoder
   ```
3. Open a Pull Request on GitHub.
4. Verify that all GitHub Actions CI checks pass green before merging!

---

## 15. Real-World Emergency Scenarios & High-Impact Fixes

### 15.1 Leaked Secrets & Private Keys Pushed to GitHub

#### 🚨 The Emergency Rule: ROTATE FIRST, CLEAN SECOND
> If you pushed an RPC API key, Solana wallet keypair (`id.json`), AWS token, or database password to a public GitHub repository, **assume it was compromised within 3 seconds**. Web scraper bots monitor the public GitHub firehose continuously.
> 
> **Step 0:** Go to your provider (Solana devnet/mainnet, QuickNode, Helius, AWS) and **revoke/rotate the key immediately**.
> Deleting the file from Git afterwards protects you from future leaks, but it does NOT secure a compromised credential.

#### Why `git rm secret.json` and a new commit is NOT enough:
Git is an append-only timeline. If you delete `secret.json` in Commit 2, **anyone can still browse Commit 1 in GitHub's web history and view your plaintext secret.**

#### How to Permanently Scrub a Secret from Entire History:

##### Modern Method: Using `git-filter-repo` (Fastest & Official)
1. Install `git-filter-repo` (requires Python):
   ```bash
   pip install git-filter-repo
   ```
2. Purge the sensitive file completely from every commit in repository history:
   ```bash
   git filter-repo --path .env --invert-paths
   ```
   *(Or for a Solana keypair file: `git filter-repo --path id.json --invert-paths`)*

3. Re-add your remote (git-filter-repo removes remotes as a safety feature):
   ```bash
   git remote add origin https://github.com/yash-0025/solana-indexer.git
   ```

4. Force-push the cleansed history:
   ```bash
   git push origin --force --all
   git push origin --force --tags
   ```

##### Alternative: Using BFG Repo-Cleaner
If you have Java installed, BFG is very fast:
```bash
# Delete a file from entire history:
bfg --delete-files id.json

# Or replace passwords inside any file with ***REMOVED***:
echo "my_super_secret_rpc_password" > passwords.txt
bfg --replace-text passwords.txt

# Expel dangling refs and force push:
git reflog expire --expire=now --all && git gc --prune=now --aggressive
git push origin --force --all
```

> **GitHub Web Cache Warning:** Even after force pushing, GitHub may retain cached views of commit URLs for a short while. If a sensitive key was indexed, submit a ticket to [GitHub Support](https://support.github.com) asking to clear the commit cache, or make the repository private and recreate it.

---

### 15.2 The 100MB Giant File Rejection (Push Blocked) & Git LFS

#### The Rookie Trap
You accidentally committed a 150MB database file `data.sqlite` or a large binary. When you run `git push`, GitHub rejects it with:
```text
remote: error: File data.sqlite is 150.00 MB; this exceeds GitHub's file size limit of 100.00 MB
```
You run `rm data.sqlite`, make a new commit `git commit -m "removed big file"`, and try to `git push` again.
**GitHub STILL REJECTS IT!**
**Why?** Because the giant file is still permanently recorded inside the *first* commit you made!

#### How to Fix It:

##### Case 1: The giant file was in your most recent local commit
```bash
# 1. Undo the commit without deleting files from disk:
git reset HEAD~1

# 2. Add the file to .gitignore so it never gets committed again:
echo "data.sqlite" >> .gitignore

# 3. Stage the rest of your files and recommit:
git add .
git commit -m "commit without large file"

# 4. Push safely:
git push origin main
```

##### Case 2: The giant file is 3 or more commits deep in your local history
Use `git-filter-repo` to strip all blobs larger than 50MB:
```bash
git filter-repo --strip-blobs-bigger-than 50M
git push origin main --force
```

#### What if you ACTUALLY need large files in your repository?
Use **Git LFS (Large File Storage)**:
```bash
# 1. Install Git LFS once
git lfs install

# 2. Tell Git LFS to track specific file types
git lfs track "*.sqlite"
git lfs track "*.tar.gz"

# 3. Commit the .gitattributes configuration
git add .gitattributes
git commit -m "track sqlite databases with git lfs"

# 4. Git will now upload pointers to GitHub and stream the large binaries to LFS storage!
```

---

### 15.3 Accidentally Committed Directly to `main` Instead of a Feature Branch

#### The Problem
You worked for two hours and made 3 commits. You run `git status` and see:
```text
On branch main
Your branch is ahead of 'origin/main' by 3 commits.
```
You realize: *"I was supposed to do this on `feature/rpc-client`, not `main`!"*

#### The 3-Command Fix:
```bash
# Step 1: Create the feature branch right where you are.
# This copies all 3 commits to the new branch!
git branch feature/rpc-client

# Step 2: Reset your local 'main' branch back to match GitHub's clean main
git reset --hard origin/main

# Step 3: Switch to your new feature branch!
git switch feature/rpc-client
```
- Your local `main` is clean again.
- All 3 commits are safely living on `feature/rpc-client`.
- Zero code was lost.

---

### 15.4 Escaping the "Detached HEAD" State

#### What is it?
You checked out an older commit to inspect it:
```bash
git checkout a1b2c3d
```
Git prints a scary message:
`You are in 'detached HEAD' state. You can look around, make experimental changes and commit them...`

#### What it actually means:
Normally, `HEAD` points to a branch name (e.g. `HEAD -> main -> commit C`).
In "Detached HEAD", `HEAD` points directly to a raw commit hash (e.g. `HEAD -> commit A`).

#### The Danger:
If you make new commits while detached, and then switch back to `main`, those new commits have no branch pointing to them. They will eventually be deleted by Git's garbage collector.

#### How to handle it:
- **If you made experimental commits that you WANT to keep:**
  Create a brand new branch right now to anchor them:
  ```bash
  git switch -c feature/experimental-saved
  ```
  *(Now your commits belong to `feature/experimental-saved`!)*

- **If you were just looking around and want to go back home:**
  ```bash
  git switch main
  ```
  *(Any uncommitted changes you made in detached state will be discarded or warned).*

---

### 15.5 Windows vs. Linux Line Endings Hell (CRLF vs. LF)

#### The Problem
You clone a repo on Windows, touch one line in `src/main.rs`, and run `git diff`.
Suddenly, Git claims **every single line in the entire file has changed!**
Or Git warns: `warning: in the working copy of 'Cargo.toml', LF will be replaced by CRLF the next time Git touches it`.

#### Why this happens:
- **Windows** writes line breaks as Carriage Return + Line Feed (`\r\n` / CRLF).
- **Linux & macOS** write line breaks as Line Feed only (`\n` / LF).

#### The Permanent Fix: `.gitattributes`
Create a `.gitattributes` file in your repository root with:

```gitattributes
# Force LF line endings for all text files in Git
* text=auto eol=lf

# Explicitly ensure code and markdown stay LF
*.rs text eol=lf
*.toml text eol=lf
*.md text eol=lf
*.json text eol=lf
*.yml text eol=lf

# Keep Windows batch files CRLF if needed
*.bat text eol=crlf
*.cmd text eol=crlf
```

Then tell Git to re-normalize your existing files according to the new rule:
```bash
git add --renormalize .
git commit -m "chore: enforce LF line endings via .gitattributes"
```

---

### 15.6 Keeping an Out-of-Date Feature Branch in Sync with `main`

#### The Problem
You opened a Pull Request for `feature/indexer-metrics`.
While you waited for review, three other teammates merged their PRs into `main`.
GitHub shows: *"This branch is out of date with the base branch"*.

#### Solution 1: Merge `main` into your feature branch (Safest & Simplest)
```bash
# 1. Switch to your feature branch
git switch feature/indexer-metrics

# 2. Fetch the latest changes from GitHub
git fetch origin

# 3. Merge origin/main into your branch
git merge origin/main

# 4. If there are conflicts, resolve them, then:
git add .
git commit -m "Merge origin/main into feature/indexer-metrics"
git push origin feature/indexer-metrics
```

#### Solution 2: Rebase your feature branch on `main` (Linear, Clean History)
```bash
# 1. Switch to your feature branch
git switch feature/indexer-metrics

# 2. Fetch and rebase onto latest main
git fetch origin
git rebase origin/main

# 3. If conflicts arise, resolve them and run git rebase --continue

# 4. Push with force-with-lease (since rebase rewrites your branch commit hashes):
git push --force-with-lease origin feature/indexer-metrics
```

> **Why `--force-with-lease`?**
> Standard `git push -f` blindly overwrites the remote branch even if someone else pushed a commit to it. `--force-with-lease` checks if anyone else pushed changes first; if they did, it refuses to overwrite them, protecting you from accidentally erasing a coworker's work.

---

### 15.7 File Renaming and Case-Sensitivity Glitches on Windows

#### The Problem
You rename a file from `solanaclient.rs` to `SolanaClient.rs`.
On Windows (which has a case-insensitive filesystem), `git status` shows **no changes at all!** Git fails to register the capitalization change.

#### The Fix:
Use `git mv` with a temporary intermediate name:
```bash
git mv solanaclient.rs temp_client.rs
git mv temp_client.rs SolanaClient.rs
git commit -m "fix: rename solanaclient.rs to SolanaClient.rs"
```

---

### 15.8 Advanced Stashing: Untracked Files, Stash Inspection & Stash-to-Branch

#### 1. Stashing Untracked (New) Files
Standard `git stash` only saves files that are **already tracked** by Git. If you created a new file `new_decoder.rs`, `git stash` leaves it behind!
```bash
# Stash tracked AND untracked files:
git stash -u
# (or git stash --include-untracked)

# Stash everything, including files ignored by .gitignore:
git stash -a
```

#### 2. Peek Inside a Stash Without Popping It
```bash
# View list of stashes:
git stash list

# View summary of files changed in latest stash:
git stash show stash@{0}

# View full code diff inside the stash:
git stash show -p stash@{0}
```

#### 3. Turn a Stash Directly into a New Branch
If you stashed complicated work days ago, and your current branch has evolved so much that popping the stash causes ugly conflicts:
```bash
git stash branch my-stash-branch stash@{0}
```
Git automatically checks out the exact commit where you originally created the stash, applies your stashed code cleanly, and puts you onto a brand new branch `my-stash-branch`!

