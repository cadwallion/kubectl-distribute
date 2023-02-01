# kubectl-distribute

A helper program/kubectl plugin designed to handle the distribution of a file to multiple pods based upon a label selector. `kubectl-distribute` queries for all pods matching a specific label (as specified by `-l` flag) with an array of values (as specified by `-v` flag(s)), then copies the file (as specified by `-f` flag) to the destination path (as specified by `-p` flag) within the matching pod containers.

```
Usage: kubectl-distribute [OPTIONS] --file <FILE> --path <PATH>

Options:
  -v, --value <VALUE>  The pod label value(s) used to select pods for distribution
  -f, --file <FILE>    The file to distribute to the selected pods
  -p, --path <PATH>    The path in each of the pods to distribute the file
  -l, --label <LABEL>  The label key, used to select pods for distribution [default: app]
  -h, --help           Print help
  -V, --version        Print version

```

## Installation

`kubectl-distribute` can be installed via Cargo by running `cargo install kubectl-distribute`.  Alternatively, download the latest release from the [Releases page](https://github.com/cadwallion/kubectl-distribute/releases).

```
curl -OsL https://github.com/cadwallion/kubectl-distribute/releases/download/v0.1.0/kubectl-distribute-0.1.0-amd64
chmod +x kubectl-distribute-0.1.0-amd64
mv kubectl-distribute-0.1.0-amd64 /usr/local/bin/
```

## Examples

```
# Copy hello.txt to /data/config in all pods that match the label "app=rest-api" 
# kubectl-distribute -v rest-api -f hello.txt -p /data/config
# Copy hello.txt to /data/config in all pods with a label "app" matching "rest-api" or "soap-api"
# kubectl-distribute -v rest-api -v soap-api -f hello.txt -p /data/config
# The real reason this thing exists; copy a Minecraft plugin to all Minecraft deployments in the cluster :D
# kubectl-distribute -f minecraft-plugin.jar -p /data/plugins -v mc-survival -v mc-creative -v mc-lobby
```