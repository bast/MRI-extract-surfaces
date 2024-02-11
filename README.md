# MRI-extract-surfaces

Set of containerized scripts to extract mesh surfaces from a T1-weighted MRI
scan. Uses [SimNIBS](https://simnibs.github.io/simnibs/) under the hood, plus
some smaller scripts.  The surfaces are then used in another project:
https://github.com/bast/tms-location


## Requirements

You need an installation of [Apptainer](https://apptainer.org/) (e.g. following
the [quick
installation](https://apptainer.org/docs/user/latest/quick_start.html#quick-installation)).
Alternatively, [SingularityCE](https://sylabs.io/singularity/) should also
work.


## How to use it

Here is an example:
```bash
$ ./extract-surfaces.sif T1_ernie.nii.gz ernie_data
```

This reads `T1_ernie.nii.gz` and creates a directory `ernie_data`. On my
computer the process takes ca. 1 hour.

The generated directory contains the following files (the actual file names can be different):
```

```
(document me ...)


It also creates another folder (document me ...)



## Where to get an example input file

You can get the `T1_ernie.nii.gz` file by downloading and then extracting the
[example dataset](https://simnibs.github.io/simnibs/build/html/dataset.html):
```bash
$ wget https://github.com/simnibs/example-dataset/releases/latest/download/simnibs4_examples.zip
```


## Please cite [SimNIBS](https://simnibs.github.io/simnibs/) if you use this container

**I am not affiliated with SimNIBS** but this
container uses SimNIBS under the hood.

When you publish results based on SimNIBS, please cite [Thielscher, A.,
Antunes, A. and Saturnino, G.B. (2015), Field modeling for transcranial
magnetic stimulation: a useful tool to understand the physiological effects of
TMS? IEEE EMBS 2015, Milano,
Italy](http://dx.doi.org/10.1109/EMBC.2015.7318340).

> [!WARNING]
> SimNIBS is a research tool. Clinical usage is not supported or advised. In
> particular, SimNIBS was not tested to give accurate results in the presence
> of pathological condition. See also https://simnibs.github.io/simnibs/
