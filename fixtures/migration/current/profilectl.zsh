#!/bin/zsh
# profilectl - Simple shell profile manager

function profilectl() {
    local action=$1
    shift

    case $action in
        load)
            source ~/Development/carlosferreyra/profilectl/profiles/$2.zsh
            ;;
        list)
            ls ~/Development/carlosferreyra/profilectl/profiles/
            ;;
        *)
            echo "Usage: profilectl {load|list} <profile_name>"
            ;;
    esac
}

# Create profiles directory if not exists
mkdir -p ~/Development/carlosferreyra/profilectl/profiles
