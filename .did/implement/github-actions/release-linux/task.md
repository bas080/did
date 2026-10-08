# Feature Specification: GitHub release workflow job for Linux binary

## Overview
Create a GitHub release workflow job to build and publish the Linux version of `did`.

## Decision
- Target: `x86_64-unknown-linux-gnu`.
- Distribution: Standalone binary artifact published to GitHub Releases.
- Build: Use the standard `x86_64-unknown-linux-gnu` runner in GitHub Actions.
