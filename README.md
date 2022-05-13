# detchar

A simple CLI to detect character encodings in files; similar to `chardet`.

Implemented as a very, very thin wrapper over [chardetng](https://github.com/hsivonen/chardetng).

The example text files in [`./data`](./data) are from [this kaggle dataset](https://www.kaggle.com/datasets/rtatman/character-encoding-examples?resource=download).

## Multithreading

`chardetng` has a feature which parallelises elimination of possible encodings for each text file.
This can be enabled by compiling `detchar` with the `multithreading` feature.

However, this is disabled by default, because for large numbers of files it is generally more effective to just parallelise over files, using e.g. GNU `parallel`:

    cat my_file_list.txt | parallel detchar
