# Git Workflow

## Strategy: GitHub Flow

Simple workflow with main branch + feature branches.

## Branches

- **master** - Production code (protected)
- **feature/name** - New features
- **fix/name** - Bug fixes
- **docs/name** - Documentation

## Workflow

### 1. Create Branch
```bash
git checkout -b feature/my-feature
```

### 2. Commit (Conventional Commits)
```bash
git commit -m "feat: add feature"
git commit -m "fix: resolve bug"
git commit -m "docs: update readme"
```

### 3. Push & PR
```bash
git push origin feature/my-feature
gh pr create
```

### 4. Merge
- CI must pass
- Squash and merge
- Delete branch

### 5. Release
```bash
git tag v0.3.1
git push origin v0.3.1
```

## Commit Types

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation
- **refactor**: Code restructuring
- **perf**: Performance
- **test**: Tests
- **chore**: Maintenance

## Versioning

- **Major** (v1.0.0): Breaking changes
- **Minor** (v0.3.0): New features
- **Patch** (v0.3.1): Bug fixes

## Best Practices

✅ Keep branches short-lived
✅ Commit often
✅ Write clear messages
✅ Delete merged branches
✅ Tag all releases

❌ No direct commits to master
❌ No force push to master

---

**Simple, fast, effective!** 🚀
