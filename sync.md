while inotifywait -r -e modify,create,delete,move --exclude '(\.git|node_modules|vendor|target)' .; do
  rsync -a --rsync-path="rsync" \
    --exclude='/target' --exclude='/.git' \
    ./ node1:/home/thedrewen/dev/chip-8-emulator
done