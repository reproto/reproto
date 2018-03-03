# Using reproto

## Installing Reproto

Reproto can be installed through cargo:

```
$ cargo install reproto
```

## Getting Started

To easily get started with reproto, you can initialize a new project using `reproto init`.
This will create a basic [`reproto.toml`](manifest.md), which can be used to customize how your
specifications are built.

Let's start by setting up a simple specification in an isolated directory:

```bash
$ mkdir example
$ cd example
$ reproto init
INFO - Writing Manifest: reproto.toml
INFO - Creating: proto/io/reproto
INFO - Writing: proto/io/reproto/example.reproto
```

Next, let's try to compile this project into Java using a couple of modules:

```bash
$ reproto --debug build --lang java --module jackson --module lombok
```

You should now have a number of files generated in `target/io/reproto/example`, corresponding to
the schema that is defined in `proto/example.reproto`.

Next up, you might be interested to read the following sections:

* Documentation for the [specification language](spec.md).
* Documentation for the [build manifest](manifest.md).

## Setting up a repository

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

[packages."io.reproto.toystore"]
version = "1"
```

```bash
$ mkdir my-project
$ cd my-project
$ # write reproto.toml
$ reproto --debug doc --index git+https://github.com/$repo
$ open output/index.html
```
