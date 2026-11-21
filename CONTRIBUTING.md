# Contributing

Retro7 Linux is being built in small, reviewable steps. The immediate goal is a reliable live ISO; desktop components will follow after the base image is reproducible.

## Before opening a change

1. Check the roadmap and existing issues to avoid duplicating work.
2. Keep each change focused and explain how it affects the live image.
3. Preserve upstream license notices for packages and copied configuration.
4. Do not add telemetry, automatically run theme scripts, or add privileged behavior without an explicit design and review.

## Pull requests

Include:

- What changed and why
- How the change was checked
- Any hardware, boot-mode, or package assumptions
- Relevant screenshots or logs when they help explain a desktop change

Avoid claiming a boot mode or hardware target works until it has been exercised. For ISO changes, include the build result and the VM boot mode used.

## Project decisions

Several long-term choices are still open. See Architecture. Proposals that affect the distribution's licensing, security model, package sources, or release policy should explain their trade-offs before implementation.
