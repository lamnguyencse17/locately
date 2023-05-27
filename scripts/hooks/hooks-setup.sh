DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
HOOKS_DIR="$DIR/../../.git/hooks"

for file in $DIR/*; do
    if [[ $file != *"hooks-setup.sh"* ]]; then
        cp $file $HOOKS_DIR
    fi
done

chmod +x $HOOKS_DIR/*

exit 0