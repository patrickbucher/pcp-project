#!/bin/sh
SESSION=ownership
SHELL=zsh

# change to folder of script
cd $(dirname "$0")

# start session
tmux -2 -f /dev/null new-session -d -s $SESSION

# create window 1
tmux new-window -t $SESSION:1 -n "scalar-types"
tmux send-keys "cd ../code/ownership" Enter
tmux send-keys "clear" Enter
tmux send-keys "make 01" Enter

# create window 2
tmux new-window -t $SESSION:2 -n "string-normal"
tmux send-keys "cd ../code/ownership" Enter
tmux send-keys "clear" Enter
tmux send-keys "make 02" Enter

# create window 3
tmux new-window -t $SESSION:3 -n "string-clone"
tmux send-keys "cd ../code/ownership" Enter
tmux send-keys "clear" Enter
tmux send-keys "make 03" Enter

# create window 4
tmux new-window -t $SESSION:4 -n "function-scopes"
tmux send-keys "cd ../code/ownership" Enter
tmux send-keys "clear" Enter
tmux send-keys "make 04" Enter

# create window 5
tmux new-window -t $SESSION:5 -n "borrowing"
tmux send-keys "cd ../code/ownership" Enter
tmux send-keys "clear" Enter
tmux send-keys "make 05" Enter

# create window 6
tmux new-window -t $SESSION:6 -n "mutable-reference"
tmux send-keys "cd ../code/ownership" Enter
tmux send-keys "clear" Enter
tmux send-keys "make 06" Enter

# create window 7
tmux new-window -t $SESSION:7 -n "single-mutable-ref"
tmux send-keys "cd ../code/ownership" Enter
tmux send-keys "clear" Enter
tmux send-keys "make 07" Enter

# kill window 0
tmux select-window -t $SESSION:0
tmux send-keys "exit" Enter

# select window 1
tmux select-window -t $SESSION:1

# attach to session
tmux -2 attach-session -t $SESSION

# kill session if required
tmux kill-session -t $SESSION
