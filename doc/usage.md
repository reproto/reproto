# Using reproto

## Setting up and using a repository

New repositories can be setup using the `reproto repo init <dir>` command:

```bash
$ reproto repo init my-repo
$ (cd my-repo && git init)
```

This can then be used as a target to publish manifest towards:

```bash
$ local_repo=$PWD/my-repo
$ cd examples
$ reproto publish --index $local_repo
$ cd -
```

This will publish all the example manifests to that repository.

You can now commit and push the changes to the git repository:

```
$ cd $local_repo
$ repo=$USER/reproto-index # change to some repository you own
$ git add .
$ git commit -m "published some changes"
$ git remote add origin git@github.com:$repo
$ git push origin master
$ cd -
```

You can now try to build the following manifest using the new repo that you just set up:

```toml
# File: reproto.toml

output = "output"

[packages."io.reproto.petstore"]
version = "1"
```

```bash
$ mkdir my-project
$ cd my-project
$ # write reproto.toml
$ reproto --debug doc --index git+https://github.com/$repo
$ open output/index.html
```
