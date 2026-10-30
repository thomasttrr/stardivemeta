#!/usr/bin/env bash
# StarDiveMeta health checks (plan T5.2). Run by .github/workflows/health.yml
# daily, and runnable by hand:
#
#   SITE_URL=https://staging.stardivemeta.pages.dev scripts/health-check.sh
#
# Inputs (environment):
#   SITE_URL                    Site to check. Default: the staging URL.
#   MAIN_HEAD_SHA               Expected main HEAD commit. In the workflow
#                               this is the checkout's HEAD. Locally, pass
#                               `git rev-parse origin/main`.
#   MAIN_HEAD_AT                ISO-8601 time of that commit (check 2's
#                               30-minute grace). Workflow: `git log -1
#                               --format=%cI`. Locally likewise.
#   CODES_CHECKED_AT_OVERRIDE   Acceptance-test hook (T5.2): pretend
#   SOURCES_CHECKED_AT_OVERRIDE build.json carried this timestamp instead.
#                               Only ever set by a manual workflow run.
#   FAILURES_FILE               Where to write failed check names, one per
#                               line (default: health-failures.txt). The
#                               workflow turns the first into an issue title.
#
# The five checks (plan T5.2):
#   (1) build.json returns 200;
#   (2) build.json.commit == main HEAD, or HEAD is younger than 30 min;
#   (3) codes_checked_at is less than 30 h old;
#   (4) sources_checked_at is less than 30 h old (the sweep heartbeat —
#       if X's workspace dies, the sweeps stop and this notices);
#   (5) /, /codes/ and /c/narae/ return 200 and contain the disclaimer.
#
# Exit 0 = all pass. Exit 1 = at least one failed (names in FAILURES_FILE).
set -u

SITE_URL="${SITE_URL:-https://staging.stardivemeta.pages.dev}"
SITE_URL="${SITE_URL%/}"
FAILURES_FILE="${FAILURES_FILE:-health-failures.txt}"
DISCLAIMER="Fan-made site. Not an official MONGIL: STAR DIVE or Netmarble site."
MAX_STAMP_AGE_SECONDS=$((30 * 3600))

: > "$FAILURES_FILE"
fail() { echo "FAIL: $1 — $2"; echo "$1" >> "$FAILURES_FILE"; }
pass() { echo "PASS: $1"; }

now_epoch="$(date -u +%s)"
to_epoch() { date -u -d "$1" +%s 2>/dev/null || echo ""; }

# --- fetch build.json (checks 1–4 all hang off it) -------------------------
build_body="$(mktemp)"
build_code="$(curl -sS -o "$build_body" -w '%{http_code}' "$SITE_URL/build.json" 2>/dev/null || echo 000)"

if [ "$build_code" != "200" ]; then
  fail "build.json" "GET $SITE_URL/build.json returned $build_code"
  echo "Health: FAILED (build.json unreachable; remaining checks skipped)"
  exit 1
fi
pass "build.json returns 200"

# --- (2) build commit vs main HEAD ----------------------------------------
build_commit="$(jq -r '.commit // ""' "$build_body")"
if [ -z "${MAIN_HEAD_SHA:-}" ]; then
  echo "SKIP: commit freshness (MAIN_HEAD_SHA not provided)"
elif [ "$build_commit" = "$MAIN_HEAD_SHA" ]; then
  pass "build commit == main HEAD"
else
  head_epoch="$(to_epoch "${MAIN_HEAD_AT:-}")"
  if [ -n "$head_epoch" ] && [ $((now_epoch - head_epoch)) -lt 1800 ]; then
    pass "build commit differs but main HEAD is <30 min old (deploy in flight)"
  else
    fail "stale build" "build.json commit ${build_commit:-<none>} != main HEAD $MAIN_HEAD_SHA and HEAD is not <30 min old"
  fi
fi

# --- (3)+(4) stamp freshness ----------------------------------------------
check_stamp() {
  local name="$1" field="$2" override="$3" value epoch age
  value="$override"
  if [ -z "$value" ]; then
    value="$(jq -r ".${field} // \"\"" "$build_body")"
  fi
  epoch="$(to_epoch "$value")"
  if [ -z "$epoch" ]; then
    fail "$name" "$field missing or unparseable in build.json (value: '${value}')"
    return
  fi
  age=$((now_epoch - epoch))
  if [ "$age" -lt "$MAX_STAMP_AGE_SECONDS" ]; then
    pass "$name fresh (${age}s old)"
  else
    fail "$name" "$field is ${age}s old (limit 30 h) — value ${value}"
  fi
}
check_stamp "codes check" "codes_checked_at" "${CODES_CHECKED_AT_OVERRIDE:-}"
check_stamp "sources check" "sources_checked_at" "${SOURCES_CHECKED_AT_OVERRIDE:-}"

# --- (5) key pages live + disclaimer ---------------------------------------
for path in "/" "/codes/" "/c/narae/"; do
  page_body="$(mktemp)"
  page_code="$(curl -sS -o "$page_body" -w '%{http_code}' "$SITE_URL$path" 2>/dev/null || echo 000)"
  if [ "$page_code" != "200" ]; then
    fail "page $path" "GET $SITE_URL$path returned $page_code"
  elif ! grep -qF "$DISCLAIMER" "$page_body"; then
    fail "page $path" "disclaimer not found on $path"
  else
    pass "page $path 200 + disclaimer"
  fi
  rm -f "$page_body"
done

rm -f "$build_body"
if [ -s "$FAILURES_FILE" ]; then
  echo "Health: FAILED"
  exit 1
fi
echo "Health: OK"
exit 0
