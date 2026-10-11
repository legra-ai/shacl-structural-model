"""Local hooks must reject the same invalid headers as release automation."""

from pathlib import Path
import re
import subprocess
import tempfile
import unittest


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
        self.assertRegex((root / "rustfmt-toolchain").read_text().strip(), r"^nightly-\d{4}-\d{2}-\d{2}$")
        self.assertIn('channel = "1.89.0"', (root / "rust-toolchain.toml").read_text())
        workflow = (root / ".github/workflows/ci.yml").read_text()
        self.assertIn("./scripts/fmt.sh --check", workflow)
        self.assertNotIn("cargo +nightly", workflow)
        self.assertIn("./scripts/fmt.sh --check", (root / ".githooks/pre-commit").read_text())

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


if __name__ == "__main__":
    unittest.main()
