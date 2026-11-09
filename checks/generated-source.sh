# The source of a crate made of what the built tool generates: every
# declaration generated into generated/, and the crate root, when one is
# given, as src/lib.rs, which reaches them as ../generated/<stem>.rs. A
# refusal is printed before the build fails.
set -eu
mkdir -p "$out/generated" "$out/src"
for declaration in $declarations; do
  status=0
  reply="$("$package/bin/ethos-zero" "Generate.{ «$fixtures/$declaration» «$out/generated» }" 2>&1)" || status=$?
  case "$status:$reply" in
    0:Generated.*) echo "$reply" ;;
    *) echo "$declaration (exit $status): $reply" >&2; exit 1 ;;
  esac
done
if [ -n "$root" ]; then
  cp "$root" "$out/src/lib.rs"
fi
