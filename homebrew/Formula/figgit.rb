class Figgit < Formula
  desc "Manage git configurations using workspace names"
  homepage "https://github.com/velarno/experiments"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
<<<<<<< Updated upstream
      url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-aarch64-apple-darwin.tar.gz"
<<<<<<< HEAD
      sha256 "sha256:1760c49f71ecabb94a1fce4977bfa84b1b05f41380fb0b71df51c26b96068d6f"
||||||| parent of e211975 (update sha in formula for brew)
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
=======
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
||||||| Stash base
      url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-aarch64-apple-darwin.tar.gz"
      sha256 "sha256:1760c49f71ecabb94a1fce4977bfa84b1b05f41380fb0b71df51c26b96068d6f"
=======
      url "https://github.com/velarno/experiments/releases/download/v#{version}/figgit-aarch64-apple-darwin.tar.gz"
      sha256 "sha256:1760c49f71ecabb94a1fce4977bfa84b1b05f41380fb0b71df51c26b96068d6f"
>>>>>>> Stashed changes
>>>>>>> e211975 (update sha in formula for brew)
    else
<<<<<<< Updated upstream
      url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-x86_64-apple-darwin.tar.gz"
<<<<<<< HEAD
      sha256 "sha256:d21de75cda8c32c0aba64ccc48fc1a2fe3536cafeefffd7362c40d9db451eecd"
||||||| parent of e211975 (update sha in formula for brew)
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
=======
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
||||||| Stash base
      url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-x86_64-apple-darwin.tar.gz"
      sha256 "sha256:d21de75cda8c32c0aba64ccc48fc1a2fe3536cafeefffd7362c40d9db451eecd"
=======
      url "https://github.com/velarno/experiments/releases/download/v#{version}/figgit-x86_64-apple-darwin.tar.gz"
      sha256 "sha256:d21de75cda8c32c0aba64ccc48fc1a2fe3536cafeefffd7362c40d9db451eecd"
>>>>>>> Stashed changes
>>>>>>> e211975 (update sha in formula for brew)
    end
  end

  on_linux do
<<<<<<< Updated upstream
    url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-x86_64-unknown-linux-gnu.tar.gz"
<<<<<<< HEAD
    sha256 "sha256:87d621d562c72af573e7e1f41ef3f02515fb31d322d652acafc4ee7e460a1c8b"
||||||| parent of e211975 (update sha in formula for brew)
    sha256 "REPLACE_WITH_ACTUAL_SHA256"
=======
    sha256 "REPLACE_WITH_ACTUAL_SHA256"
||||||| Stash base
    url "https://github.com/velarno/figgit/releases/download/v#{version}/figgit-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "sha256:87d621d562c72af573e7e1f41ef3f02515fb31d322d652acafc4ee7e460a1c8b"
=======
    url "https://github.com/velarno/experiments/releases/download/v#{version}/figgit-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "sha256:87d621d562c72af573e7e1f41ef3f02515fb31d322d652acafc4ee7e460a1c8b"
>>>>>>> Stashed changes
>>>>>>> e211975 (update sha in formula for brew)
  end

  def install
    bin.install "figgit"
  end

  test do
    assert_match "figgit", shell_output("#{bin}/figgit --help")
  end
end
