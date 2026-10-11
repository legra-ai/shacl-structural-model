"""Verify the published archive includes and passes the repository test suite."""

import pathlib
import subprocess
import tarfile
import tempfile
import tomllib


def main():
    root = pathlib.Path(__file__).resolve().parent.parent
    manifest = tomllib.loads((root / "Cargo.toml").read_text())
    package = manifest["package"]
    stem = f"{package['name']}-{package['version']}"
    tests = subprocess.check_output(
        ["git", "ls-files", "tests/"], cwd=root, text=True
    ).splitlines()
    if not tests:
        raise RuntimeError("No tracked integration tests found")

    with tempfile.TemporaryDirectory(prefix="shacl-model-package-") as directory:
        target = pathlib.Path(directory)
        subprocess.run(
            [
                "cargo", "package", "--allow-dirty", "--no-verify",
                "--target-dir", str(target),
            ],
            cwd=root, check=True,
        )
        with tarfile.open(target / "package" / f"{stem}.crate") as archive:
            members = set(archive.getnames())
            missing = [name for name in tests if f"{stem}/{name}" not in members]
            if missing:
                raise RuntimeError(f"Published archive omits tests: {', '.join(missing)}")
            archive.extractall(target / "unpacked", filter="data")

        # Execute from the extracted archive, with no repository test sources.
        unpacked = target / "unpacked" / stem
        for selection in ["--all-targets", "--doc"]:
            subprocess.run(
                ["cargo", "test", "--locked", selection,
                 "--target-dir", str(target / "test-build")],
                cwd=unpacked, check=True,
            )


if __name__ == "__main__":
    main()
