class Figgit < Formula
  desc "Manage git configurations using workspace names"
  homepage "https://github.com/USERNAME/figgit"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/USERNAME/figgit/releases/download/v#{version}/figgit-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/USERNAME/figgit/releases/download/v#{version}/figgit-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    url "https://github.com/USERNAME/figgit/releases/download/v#{version}/figgit-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256"
  end

  def install
    bin.install "figgit"
  end

  test do
    assert_match "figgit", shell_output("#{bin}/figgit --help")
  end
end
