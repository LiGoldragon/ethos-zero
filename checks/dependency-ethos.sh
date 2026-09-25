# Every dependency's ethos declaration reads and generates with the built tool.
# A refusal is printed before the check fails, whether the tool exits non-zero
# or answers with anything but Generated.
set -eu
out_dir="$TMPDIR/generated"
for declaration in $declarations; do
  status=0
  reply="$("$package/bin/ethos-zero" "Generate.{ «$declaration» «$out_dir» }" 2>&1)" || status=$?
  case "$status:$reply" in
    0:Generated.*) echo "$reply" ;;
    *) echo "$declaration (exit $status): $reply" >&2; exit 1 ;;
  esac
done
touch "$out"
