#!/usr/bin/env python3
"""
Publish the ASTROpay markdown backlog to GitHub issues via `gh issue create`.

Usage:
  python3 scripts/publish_issue_backlog.py --dry-run
  python3 scripts/publish_issue_backlog.py --repo dreamgenies/astropay
  python3 scripts/publish_issue_backlog.py --repo dreamgenies/astropay --start AP-101 --end AP-150

This script is intentionally strict about the markdown structure in
docs/issue-backlog/astropay-250-issues.md. If that file changes format,
update this parser instead of silently creating malformed issues.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BACKLOG_PATH = ROOT / "docs" / "issue-backlog" / "astropay-250-issues.md"

LABEL_SPECS = {
    "area:backend-rust": ("0e8a16", "Rust Axum backend work"),
    "area:database": ("1d76db", "Schema, migrations, and query work"),
    "area:docs": ("5319e7", "Documentation and contributor guidance"),
    "area:frontend": ("fbca04", "Next.js app and UI work"),
    "area:infrastructure": ("0052cc", "Deployment, CI, and environment work"),
    "area:observability": ("c2e0c6", "Logging, metrics, tracing, and alerts"),
    "area:performance": ("bfd4f2", "Performance and scalability work"),
    "area:security": ("b60205", "Auth, secret handling, and abuse resistance"),
    "area:stellar": ("0e8a16", "Stellar network and transaction work"),
    "area:testing": ("d4c5f9", "Automated and manual verification work"),
    "difficulty:advanced": ("d93f0b", "High-complexity work item"),
    "difficulty:intermediate": ("fbca04", "Medium-complexity work item"),
    "difficulty:starter": ("0e8a16", "Good entry point for contributors"),
    "type:bug": ("d73a4a", "Behavior is wrong and needs correction"),
    "type:docs": ("0075ca", "Documentation-only change"),
    "type:feature": ("a2eeef", "New capability or behavior"),
    "type:ops": ("5319e7", "Operational or maintenance work"),
    "type:performance": ("c5def5", "Optimization and throughput work"),
    "type:refactor": ("f9d0c4", "Structural cleanup without intended behavior change"),
    "type:test": ("7057ff", "Test coverage or verification work"),
}

HEADING_RE = re.compile(r"^## (.+)$")
ISSUE_RE = re.compile(r"^### (AP-\d{3}) (.+)$")
LABELS_RE = re.compile(r"^- Labels: (.+)$")
DONE_RE = re.compile(r"^- Done when: (.+)$")


@dataclass
class Issue:
    issue_id: str
    title: str
    section: str
    labels: list[str]
    done_when: str
    relevant_code: list[str]

    @property
    def github_title(self) -> str:
        return f"{self.issue_id} {self.title}"

    @property
    def body(self) -> str:
        scope_lines = "\n".join(f"- `{path}`" for path in self.relevant_code) or "- None listed"
        label_lines = "\n".join(f"- `{label}`" for label in self.labels)
        return (
            "## Summary\n"
            f"{self.title}\n\n"
            "## Scope\n"
            f"Backlog section: `{self.section}`\n\n"
            "Relevant code:\n"
            f"{scope_lines}\n\n"
            "## Acceptance Criteria\n"
            f"- [ ] {self.done_when}\n"
            "- [ ] Error and edge-case handling is covered\n"
            "- [ ] Tests or verification steps are included\n"
            "- [ ] Docs are updated if the workflow or contract changed\n\n"
            "## Labels\n"
            f"{label_lines}\n\n"
            "## Notes\n"
            f"Imported from `{BACKLOG_PATH.relative_to(ROOT)}`.\n"
        )


def parse_backlog(path: Path) -> list[Issue]:
    lines = path.read_text(encoding="utf-8").splitlines()
    issues: list[Issue] = []
    section = ""
    relevant_code: list[str] = []
    collecting_relevant = False

    for index, line in enumerate(lines):
        heading_match = HEADING_RE.match(line)
        if heading_match:
            section = heading_match.group(1)
            relevant_code = []
            collecting_relevant = False
            continue

        if line == "Relevant code:":
            collecting_relevant = True
            continue

        if collecting_relevant:
            if line.startswith("- `") and line.endswith("`"):
                relevant_code.append(line[3:-1])
                continue
            if line.strip() == "":
                continue
            collecting_relevant = False

        issue_match = ISSUE_RE.match(line)
        if issue_match:
            issue_id, title = issue_match.groups()
            if index + 2 >= len(lines):
                raise ValueError(f"Incomplete issue block for {issue_id}")
            labels_match = LABELS_RE.match(lines[index + 1])
            done_match = DONE_RE.match(lines[index + 2])
            if not labels_match or not done_match:
                raise ValueError(f"Malformed issue block for {issue_id}")
            labels = [token.strip().strip("`") for token in labels_match.group(1).split(",")]
            issues.append(
                Issue(
                    issue_id=issue_id,
                    title=title,
                    section=section,
                    labels=labels,
                    done_when=done_match.group(1),
                    relevant_code=list(relevant_code),
                )
            )

    return issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Publish ASTROpay backlog issues to GitHub.")
    parser.add_argument("--repo", help="GitHub repo in owner/name format. Required unless --dry-run.")
    parser.add_argument("--start", help="First issue ID to publish, inclusive.")
    parser.add_argument("--end", help="Last issue ID to publish, inclusive.")
    parser.add_argument("--dry-run", action="store_true", help="Print issue payloads instead of calling GitHub.")
    parser.add_argument(
        "--sync-labels",
        action="store_true",
        help="Create or update the custom labels required by the backlog before publishing issues.",
    )
    parser.add_argument(
        "--limit",
        type=int,
        help="Maximum number of issues to publish after range filtering. Useful for batching.",
    )
    return parser.parse_args()


def filter_issues(issues: list[Issue], start: str | None, end: str | None, limit: int | None) -> list[Issue]:
    selected = []
    active = start is None

    for issue in issues:
        if issue.issue_id == start:
            active = True
        if active:
            selected.append(issue)
        if issue.issue_id == end:
            break

    if start and not any(issue.issue_id == start for issue in issues):
        raise ValueError(f"Start issue {start} was not found")
    if end and not any(issue.issue_id == end for issue in issues):
        raise ValueError(f"End issue {end} was not found")
    if start and end:
        start_num = int(start.split("-")[1])
        end_num = int(end.split("-")[1])
        if start_num > end_num:
            raise ValueError("--start must not be after --end")
    if limit is not None:
        selected = selected[:limit]
    return selected


def ensure_gh_auth() -> None:
    result = subprocess.run(
        ["gh", "auth", "status"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError("GitHub CLI is not authenticated. Run `gh auth login -h github.com` first.")


def sync_labels(repo: str, labels: set[str]) -> None:
    missing = sorted(label for label in labels if label not in LABEL_SPECS)
    if missing:
        raise RuntimeError(f"Missing label specifications for: {', '.join(missing)}")
    for label in sorted(labels):
        color, description = LABEL_SPECS[label]
        cmd = [
            "gh",
            "label",
            "create",
            label,
            "--repo",
            repo,
            "--color",
            color,
            "--description",
            description,
            "--force",
        ]
        subprocess.run(cmd, check=True)


def create_issue(repo: str, issue: Issue) -> None:
    cmd = [
        "gh",
        "issue",
        "create",
        "--repo",
        repo,
        "--title",
        issue.github_title,
        "--body",
        issue.body,
    ]
    for label in issue.labels:
        cmd.extend(["--label", label])
    subprocess.run(cmd, check=True)


def main() -> int:
    args = parse_args()
    issues = parse_backlog(BACKLOG_PATH)
    selected = filter_issues(issues, args.start, args.end, args.limit)

    if args.dry_run:
        for issue in selected:
            print(f"{issue.github_title} | {', '.join(issue.labels)}")
        print(f"\nDry run complete: {len(selected)} issues selected from {len(issues)} total.")
        return 0

    if not args.repo:
        raise RuntimeError("--repo is required unless --dry-run is used.")

    ensure_gh_auth()
    if args.sync_labels:
        all_labels = {label for issue in issues for label in issue.labels}
        print(f"Syncing {len(all_labels)} labels in {args.repo}...", flush=True)
        sync_labels(args.repo, all_labels)
    for issue in selected:
        print(f"Creating {issue.github_title}...", flush=True)
        create_issue(args.repo, issue)

    print(f"Created {len(selected)} issues in {args.repo}.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:  # pragma: no cover
        print(f"error: {exc}", file=sys.stderr)
        raise SystemExit(1)
