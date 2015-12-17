#!/usr/bin/env bash

set -euo pipefail

[ "$TRAVIS_PULL_REQUEST" == "false" ] || exit 0
[ "$TRAVIS_RUST_VERSION" == "nightly" ] || exit 0

eval "$(ssh-agent -s)"
openssl aes-256-cbc -K "$encrypted_75416a70f63e_key" -iv "$encrypted_75416a70f63e_iv" -in .travis/deploy_key.enc -out .travis/deploy_key -d
chmod 600 .travis/deploy_key
ssh-add .travis/deploy_key

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
mkdir -p "${DOC_TARGET}"
mv ../target/doc "${DOC_TARGET}"

git add -A .
git commit -m "${COMMIT_MSG}"
git push
