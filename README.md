Yet another tool for computing sha256sum of files in TAR archive.
Build with cargo:
```
cargo build --release
```

Usage:
```
tarsum /path/to/archive.tar
```

If no file is specified, `tarsum` reads archive from standard input.
You may also specify used compression (one of gzip, bzip2, xz):
```
cat /path/to/archive.tar.gz |tarsum --compression gzip
```

