#!/usr/bin/env bash

set -euo pipefail

[ "$TRAVIS_PULL_REQUEST" == "false" ] || exit 0
[ "$TRAVIS_RUST_VERSION" == "nightly" ] || exit 0

# Decrypt our deploy key. We use a deploy key instead of a token because
# GitHub tokens provide a scary level of access, whereas deploy keys are
# tied to a single repo.
eval "$(ssh-agent -s)"
openssl aes-256-cbc -K "$encrypted_75416a70f63e_key" -iv "$encrypted_75416a70f63e_iv" -in .travis/deploy_key.enc -out .travis/deploy_key -d
chmod 600 .travis/deploy_key
ssh-add .travis/deploy_key

# There may be a tagged release and a master branch build running at the
# same time, which may potentially cause trouble. Try again if that happens,
# since we want both commits.
for attempt in 1 2 3; do
    cd "$TRAVIS_BUILD_DIR" && rm -rf gh-pages

    git clone --depth 10 --branch gh-pages "git@github.com:${TRAVIS_REPO_SLUG}.git" gh-pages
    cd gh-pages

    git config user.name "$(git --no-pager show -s --format='%an' HEAD) (via Travis CI)"
    git config user.email "$(git --no-pager show -s --format='%ae' HEAD)"

    if [ -z "$TRAVIS_TAG" ]; then
        DOC_TARGET="doc/${TRAVIS_BRANCH}"
        COMMIT_MSG="Rebuild pages at ${TRAVIS_COMMIT}"
    else
        DOC_TARGET="doc/${TRAVIS_TAG}"
        COMMIT_MSG="Release ${TRAVIS_TAG}"
    fi

    rm -rf "${DOC_TARGET}"
    mkdir -p doc
    cp -R ../target/doc "${DOC_TARGET}"

    git add -A .
    git commit -m "${COMMIT_MSG}"

    git push origin gh-pages && break || echo "Push refused on attempt $attempt/3"
done
