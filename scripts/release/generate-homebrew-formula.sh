#!/usr/bin/env bash
set -euo pipefail

raw_version="${1:?version required}"
repo="${2:?github repo required (owner/name)}"
sha_linux="${3:?sha256 for linux required}"
sha_macos_x86="${4:?sha256 for macos x86_64 required}"
sha_macos_arm="${5:?sha256 for macos arm64 required}"
output="${6:-packaging/homebrew/coco-local.rb}"

version="${raw_version#v}"
tag="v${version}"

mkdir -p "$(dirname "${output}")"

cat > "${output}" <<EOF
class CocoLocal < Formula
  desc "Context Core local service"
  homepage "https://github.com/${repo}"
  version "${version}"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/${repo}/releases/download/${tag}/coco-local-${raw_version}-aarch64-apple-darwin.tar.gz"
      sha256 "${sha_macos_arm}"
    else
      url "https://github.com/${repo}/releases/download/${tag}/coco-local-${raw_version}-x86_64-apple-darwin.tar.gz"
      sha256 "${sha_macos_x86}"
    end
  end

  on_linux do
    url "https://github.com/${repo}/releases/download/${tag}/coco-local-${raw_version}-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "${sha_linux}"
  end

  def install
    bin.install "coco-local"
  end

  test do
    assert_match "coco-local", shell_output("\#{bin}/coco-local --help")
  end
end
EOF

echo "Wrote ${output}"
