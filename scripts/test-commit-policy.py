"""Check commit policy and credential boundaries in release automation."""

from pathlib import Path
import re
import subprocess
import tempfile
import unittest


class ReleaseCredentialTests(unittest.TestCase):
    def setUp(self):
        root = Path(__file__).resolve().parents[1]
        workflow = (root / ".github/workflows/ci.yml").read_text()
        prepare = workflow.split("\n  prepare:\n", 1)[1].split("\n  test:\n", 1)[0]
        self.steps = re.split(r"^      - ", prepare, flags=re.MULTILINE)[1:]

    def test_release_credentials_are_only_requested_on_main(self):
        writers = [step for step in self.steps if "id: writer\n" in step]
        self.assertEqual(len(writers), 1)
        # Dependabot and fork PR runs have no Actions secrets. A read-only
        # dispatch on another branch must not require the writer either.
        self.assertIn("        if: github.ref == 'refs/heads/main'\n", writers[0])

    def test_pr_preparation_uses_read_only_token(self):
        releases = [step for step in self.steps if "id: release\n" in step]
        self.assertEqual(len(releases), 1)
        # Branch on the non-main case first: a missing main writer token must
        # never silently select the read-only token and conceal bad secrets.
        self.assertIn(
            "token: ${{ github.ref != 'refs/heads/main' && github.token || steps.writer.outputs.token }}",
            releases[0],
        )


class CommitPolicyTests(unittest.TestCase):
    def test_ci_keeps_warnings_fatal_with_pinned_rust(self):
        root = Path(__file__).resolve().parents[1]
        workflow = (root / ".github/workflows/ci.yml").read_text()
        steps = re.split(r"^      - ", workflow, flags=re.MULTILINE)
        setups = [step for step in steps if step.startswith("uses: actions-rust-lang/setup-rust-toolchain@")]
        self.assertEqual(len(setups), 2, "Both test and publish must configure Rust")
        for step in setups:
            with self.subTest(step=step):
                # v2's CARGO_BUILD_WARNINGS needs Cargo 1.97; this repository
                # pins 1.89, so warning failures must use rustc flags directly.
                self.assertIn("          rustflags: -D warnings\n", step)

    def test_ci_and_hooks_share_pinned_toolchains(self):
        root = Path(__file__).resolve().parents[1]
        self.assertEqual((root / "rustfmt-toolchain").read_text().strip(), "nightly-2026-10-16")
        self.assertIn('channel = "1.99.1"', (root / "rust-toolchain.toml").read_text())
        workflow = (root / ".github/workflows/ci.yml").read_text()
        compiler = re.search(
            r'channel = "([^"]+)"',
            (root / "rust-toolchain.toml").read_text(),
        ).group(1)
        steps = re.split(r"^      - ", workflow, flags=re.MULTILINE)
        compiler_setups = [
            step for step in steps
            if step.startswith("uses: actions-rust-lang/setup-rust-toolchain@")
        ]
        self.assertEqual(len(compiler_setups), 2)
        for step in compiler_setups:
            self.assertIn(f"          toolchain: {compiler}\n", step)
        self.assertIn("./scripts/fmt.sh --check", workflow)
        self.assertNotIn("cargo +nightly ", workflow)
        self.assertIn("./scripts/fmt.sh --check", (root / ".githooks/pre-commit").read_text())

    def test_formatter_setup_uses_a_pinned_input_capable_action(self):
        root = Path(__file__).resolve().parents[1]
        workflow = (root / ".github/workflows/ci.yml").read_text()
        steps = re.split(r"^      - ", workflow, flags=re.MULTILINE)
        setups = [
            step for step in steps
            if "toolchain: ${{ steps.formatter.outputs.toolchain }}" in step
        ]
        self.assertEqual(len(setups), 1)
        action = setups[0].splitlines()[0]
        self.assertRegex(action, r"^uses: dtolnay/rust-toolchain@[0-9a-f]{40}$")
        self.assertIn("          components: rustfmt\n", setups[0])

    def test_commit_header_contract(self):
        hook = Path(__file__).resolve().parents[1] / ".githooks/commit-msg"
        cases = [
            ("build(deps): update serde", True),
            ("ci(deps-dev): update action", True),
            ("feat(api)!: change contract", True),
            ("fix(api): repair\n\nBREAKING CHANGE: new wire shape", True),
            ("fix: missing scope", False),
            ("oops(api): invalid type", False),
            ("bad headline\n\nfix(api): not the headline", False),
            ("fix(api): ", False),
        ]
        for message, valid in cases:
            with self.subTest(message=message), tempfile.NamedTemporaryFile(mode="w") as source:
                source.write(message + "\n")
                source.flush()
                result = subprocess.run([str(hook), source.name], capture_output=True, text=True)
                self.assertEqual(result.returncode == 0, valid, result.stdout + result.stderr)



class MajorReleasePolicyTests(unittest.TestCase):
    def test_major_plan_is_reviewed_before_release_writer_runs(self):
        workflow = (Path(__file__).resolve().parents[1] / ".github/workflows/ci.yml").read_text()
        plan = workflow.split("\n  plan:\n", 1)[1].split("\n  approve-major:\n", 1)[0]
        self.assertIn("plan-only: 'true'", plan)
        self.assertIn("token: ${{ github.token }}", plan)
        self.assertNotIn("RELEASE_WRITER", plan)
        approval = workflow.split("\n  approve-major:\n", 1)[1].split("\n  prepare:\n", 1)[0]
        self.assertIn("environment: major-release", approval)
        self.assertIn("needs.plan.outputs.major == 'true'", approval)
        self.assertIn("Approve major release", plan)
        self.assertIn("WORKFLOW_SOURCE", plan)
        prepare = workflow.split("\n  prepare:\n", 1)[1].split("\n  test:\n", 1)[0]
        self.assertIn("needs: [plan, approve-major]", prepare)
        self.assertIn("needs['approve-major'].result == 'success'", prepare)
        self.assertIn("needs['approve-major'].result == 'skipped'", prepare)
        self.assertIn("actions: read", workflow)
        self.assertNotIn("pending_deployments", workflow)

if __name__ == "__main__":
    unittest.main()
