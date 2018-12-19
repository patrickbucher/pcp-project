#!/bin/sh
SESSION=patternmatching
SHELL=zsh

# change to folder of script
cd $(dirname "$0")

# start session
tmux -2 -f /dev/null new-session -d -s $SESSION -n "patternmatching"

# split window 0
tmux split-window -v
tmux resize-pane -D 30

# setup top pane of window 0
tmux select-pane -t 1
tmux send-keys "cd ../code/pattern-matching" Enter
tmux send-keys "clear" Enter
tmux send-keys "cargo run" Enter

# setup bottom pane of window 0
tmux select-pane -t 0
tmux send-keys "cd ../code/pattern-matching" Enter
tmux send-keys "clear" Enter
tmux send-keys "bat src/main.rs" Enter

# attach to session
tmux -2 attach-session -t $SESSION

# kill session if required
tmux kill-session -t $SESSION
