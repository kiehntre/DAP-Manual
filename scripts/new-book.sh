#!/usr/bin/env bash

set -e

BOOK="$1"

if [ -z "$BOOK" ]; then
    echo "Usage: ./new-book.sh 05-Docker"
    exit 1
fi

mkdir -p "docs/$BOOK"

cat > "docs/$BOOK/index.md" <<EOT
# ${BOOK#*-}

## Overview

This section contains all documentation relating to ${BOOK#*-}.

EOT

for chapter in \
01-Fundamentals \
02-DAP-Architecture \
03-Networking \
04-Compose \
05-Traefik \
06-Volumes \
07-Backup-Recovery \
08-Debugging \
09-Playbooks \
10-Cheat-Sheet
do
cat > "docs/$BOOK/$chapter.md" <<EOT
# ${chapter#*-}

> Draft

EOT
done

echo
echo "Created documentation book: $BOOK"
