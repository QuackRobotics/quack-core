# Organization profile (staging)

GitHub renders the [Quack Robotics](https://github.com/QuackRobotics) landing page from a public repository named **`.github`** at:

```text
.github/profile/README.md
.github/profile/assets/...
```

This folder is a **staging copy** of that layout. The Cursor GitHub integration could not create `QuackRobotics/.github` (`Resource not accessible by integration`).

## Publish to GitHub

1. Create a public repo: `https://github.com/organizations/QuackRobotics/repositories/new`  
   - Name: `.github`  
   - Visibility: **Public**
2. Copy the contents of `org-profile/profile/` into the root of that repo so paths match:
   - `profile/README.md`
   - `profile/assets/`
3. Commit and push to `main`.
4. Within a few minutes, open `https://github.com/QuackRobotics` to verify the org profile.

Optional: set the organization description (requires org owner):

```text
Open-source ~42 cm biped Micro Duck — hardware, firmware, sim, ML, API & dashboard.
```

## Preview before publish

On this branch, preview the markdown (paths are relative to `profile/`):

- [profile/README.md](./profile/README.md) on GitHub after merge
- Or browse `org-profile/profile/README.md` in the quack-core repo file viewer