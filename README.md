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

## Examples

```
# Copy hello.txt to /data/config in all pods that match the label "app=rest-api" 
# kubectl-distribute -v rest-api -f hello.txt -p /data/config
# Copy hello.txt to /data/config in all pods with a label "app" matching "rest-api" or "soap-api"
# kubectl-distribute -v rest-api -v soap-api -f hello.txt -p /data/config
```