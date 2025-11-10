class Figgit < Formula
  desc "Manage git configurations using workspace names"
  homepage "https://github.com/velarno/experiments"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/velarno/experiments/releases/download/v#{version}/figgit-aarch64-apple-darwin.tar.gz"
      sha256 "sha256:1760c49f71ecabb94a1fce4977bfa84b1b05f41380fb0b71df51c26b96068d6f"
    else
      url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-x86_64-apple-darwin.tar.gz"
      sha256 "sha256:d21de75cda8c32c0aba64ccc48fc1a2fe3536cafeefffd7362c40d9db451eecd"
    end
  end

  on_linux do
    url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "sha256:87d621d562c72af573e7e1f41ef3f02515fb31d322d652acafc4ee7e460a1c8b"
  end

  def install
    bin.install "figgit"
  end

  test do
    assert_match "figgit", shell_output("#{bin}/figgit --help")
  end
end
