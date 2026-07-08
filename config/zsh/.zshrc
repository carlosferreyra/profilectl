# ==============================================
# ZSH Configuration
# Modular setup - loads configuration from ~/.zsh/
#
# Sourced files:
# - ~/.zsh/00_environment.zsh
# - ~/.zsh/01_plugins.zsh
# - ~/.zsh/02_options.zsh
# - ~/.zsh/03_aliases.zsh
# - ~/.zsh/04_functions.zsh
# - ~/.zsh/05_completions.zsh
# - ~/.zsh/06_init.zsh
# ==============================================

# Source all .zsh files from the ~/.zsh directory in numerical order
if [ -d ~/.zsh ]; then
  for file in ~/.zsh/*.zsh; do
    source "$file"
  done
  unset file
fi
