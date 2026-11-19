import argparse, os, shutil, tempfile, zipfile
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--install", action="store_true", help="also copy to the default Horizon instance")
args = parser.parse_args()

project = Path(__file__).resolve().parent
output = project / "release" / "fullbright.hmod"
output.parent.mkdir(exist_ok=True)
files = [project / "horizon-mod.toml", project / "Cargo.toml", project / "icon.png", *sorted((project / "src").rglob("*.rs"))]
files.extend(sorted(p for p in (project / "assets").rglob("*") if p.is_file()))
with tempfile.TemporaryDirectory(dir=output.parent) as work:
    archive = Path(work) / output.name
    with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as package:
        for source in files:
            package.write(source, source.relative_to(project).as_posix())
    archive.replace(output)
print(output)

if args.install:
    directory = Path(os.environ["APPDATA"]) / "horizon" / "horizon-native-mods"
    directory.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=directory) as work:
        staged = Path(work) / output.name
        shutil.copyfile(output, staged)
        staged.replace(directory / output.name)
    print(directory / output.name)
