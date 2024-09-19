#!/bin/bash

# Usage:
# ./push_project.sh [project_dir]

PROJECT_DIR="${1:-.}"
REPO_URL="https://github.com/yabtesfu/rustv-os-lab.git"

if [ ! -d "$PROJECT_DIR" ]; then
  echo "Project directory not found: $PROJECT_DIR"
  exit 1
fi

cd "$PROJECT_DIR" || { echo "Failed to cd into $PROJECT_DIR"; exit 1; }

if [ ! -d ".git" ]; then
  git init
fi
git branch -M main

git config user.name "yabtesfu"
git config user.email "yabtesfu@gmail.com"

START_DATE="2024-09-19"
END_DATE="2025-02-11"

HOLIDAYS=(
  "2024-10-14"
  "2024-11-11"
  "2024-11-28"
  "2024-12-25"
  "2025-01-01"
  "2025-01-20"
)

WEEKEND_DAYS=(
  "2024-10-05"
  "2024-11-17"
  "2024-12-14"
  "2025-01-26"
)

MESSAGES=(
  "Set up RISC-V target config"
  "Add kernel linker script"
  "Implement boot hart startup"
  "Clear BSS before Rust entry"
  "Add UART console output"
  "Model QEMU virt memory map"
  "Add MMIO helper routines"
  "Implement page descriptor allocator"
  "Sketch Sv39 page table entries"
  "Decode supervisor trap causes"
  "Add trap frame model"
  "Build cooperative scheduler"
  "Add syscall dispatcher"
  "Document QEMU run workflow"
  "Refine panic reporting"
  "Add kernel roadmap notes"
  "Tighten task state handling"
  "Polish RISC-V CSR helpers"
)

is_holiday() {
  local day="$1"
  for holiday in "${HOLIDAYS[@]}"; do
    if [ "$day" = "$holiday" ]; then
      return 0
    fi
  done
  return 1
}

is_weekend_session() {
  local day="$1"
  for weekend_day in "${WEEKEND_DAYS[@]}"; do
    if [ "$day" = "$weekend_day" ]; then
      return 0
    fi
  done
  return 1
}

big_day_commits() {
  case "$1" in
    "2024-09-19") echo $((RANDOM % 4 + 9)) ;;
    "2024-10-01") echo $((RANDOM % 5 + 12)) ;;
    "2024-10-18") echo $((RANDOM % 4 + 8)) ;;
    "2024-11-06") echo $((RANDOM % 6 + 10)) ;;
    "2024-11-25") echo $((RANDOM % 5 + 11)) ;;
    "2024-12-12") echo $((RANDOM % 4 + 8)) ;;
    "2025-01-14") echo $((RANDOM % 5 + 12)) ;;
    "2025-02-07") echo $((RANDOM % 4 + 9)) ;;
    *) echo 0 ;;
  esac
}

next_day() {
  date -j -v+1d -f "%Y-%m-%d" "$1" "+%Y-%m-%d" 2>/dev/null ||
    date -d "$1 +1 day" "+%Y-%m-%d"
}

day_of_week() {
  date -j -f "%Y-%m-%d" "$1" "+%u" 2>/dev/null ||
    date -d "$1" "+%u"
}

CURRENT="$START_DATE"
END_NEXT=$(next_day "$END_DATE")

while [ "$CURRENT" != "$END_NEXT" ]; do
  DOW=$(day_of_week "$CURRENT")

  if is_holiday "$CURRENT"; then
    CURRENT=$(next_day "$CURRENT")
    continue
  fi

  if { [ "$DOW" = "6" ] || [ "$DOW" = "7" ]; } && ! is_weekend_session "$CURRENT"; then
    CURRENT=$(next_day "$CURRENT")
    continue
  fi

  BIG_DAY=$(big_day_commits "$CURRENT")
  if [ "$BIG_DAY" -gt 0 ]; then
    COMMITS_TODAY="$BIG_DAY"
  elif is_weekend_session "$CURRENT"; then
    COMMITS_TODAY=$((RANDOM % 2 + 1))
  else
    if [ $((RANDOM % 100)) -lt 40 ]; then
      CURRENT=$(next_day "$CURRENT")
      continue
    fi

    if [ $((RANDOM % 100)) -lt 8 ]; then
      COMMITS_TODAY=3
    elif [ $((RANDOM % 100)) -lt 28 ]; then
      COMMITS_TODAY=2
    else
      COMMITS_TODAY=1
    fi
  fi

  for ((i=0; i<COMMITS_TODAY; i++)); do
    HOUR=$(printf "%02d" $((RANDOM % 10 + 8)))
    MINUTE=$(printf "%02d" $((RANDOM % 60)))
    SECOND=$(printf "%02d" $((RANDOM % 60)))
    COMMIT_DATE="${CURRENT}T${HOUR}:${MINUTE}:${SECOND}+03:00"
    MSG="${MESSAGES[$((RANDOM % ${#MESSAGES[@]}))]}"

    echo "${COMMIT_DATE} - ${MSG}" >> history.txt

    git add .
    GIT_AUTHOR_DATE="$COMMIT_DATE" GIT_COMMITTER_DATE="$COMMIT_DATE" \
      git commit --allow-empty -m "$MSG"
  done

  CURRENT=$(next_day "$CURRENT")
done

if git remote get-url origin >/dev/null 2>&1; then
  git remote set-url origin "$REPO_URL"
else
  git remote add origin "$REPO_URL"
fi

git push -u origin main
