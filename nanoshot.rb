class Mytool < Formula
  desc "Simple screenshotting tool for MacOS"
  homepage "https://github.com/antichrome1/nanoshot-rs"
  url "https://github.com/antichrome1/nanoshot-rs/archive/refs/tags/v0.9.0.tar.gz"
  sha256 "PUT_THE_64_CHARACTER_SHA256_HERE"
  license "ISC"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/nanoshot"
  end

end
