# The source of a crate made of what the built tool generates: every
# declaration, and the roots of a Nexus together, generated into generated/, and the crate root, when one is
# given, as src/lib.rs, which reaches them as ../generated/<stem>.rs. A
# refusal is printed before the build fails.
set -eu
mkdir -p "$out/generated" "$out/src"
# A Nexus's roots are generated together, whatever order they are listed in.
if [ -n "${nexus:-}" ]; then
  roots=""
  for file in "$fixtures/$nexus"/*.ethos; do roots="$roots «$file»"; done
  status=0
  reply="$("$package/bin/ethos-zero" "Generate.{ [$roots ] «$out/generated/$nexus» }" 2>&1)" || status=$?
  case "$status:$reply" in
    0:Generated.*) echo "$reply" ;;
    *) echo "$nexus (exit $status): $reply" >&2; exit 1 ;;
  esac
fi
for declaration in $declarations; do
  status=0
  reply="$("$package/bin/ethos-zero" "Generate.{ [ «$fixtures/$declaration» ] «$out/generated» }" 2>&1)" || status=$?
  case "$status:$reply" in
    0:Generated.*) echo "$reply" ;;
    *) echo "$declaration (exit $status): $reply" >&2; exit 1 ;;
  esac
done
if [ -n "$root" ]; then
  cp "$root" "$out/src/lib.rs"
fi
