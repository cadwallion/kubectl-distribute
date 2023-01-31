# kubectl-distribute

A helper program/kubectl plugin designed to handle the distribution of a file to multiple pods based upon a label selector.  If you need to copy a local file to the current pod(s) of a Kubernetes Deployment resource and don't want to look up the pod name for that Deployment, this program can help.  If you 
need to copy a file to multiple pods across multiple Deployments, this program can help.

`kubectl-distribute` queries for all pods matching a specific label (as specified by `-l` flag) with an array of values (as specified by `-v` flag(s)), then copies the file (as specified by `-f` flag) to the destination path (as specified by `-p` flag) within the matching pod containers.

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