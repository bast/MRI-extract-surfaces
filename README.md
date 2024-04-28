# MRI-extract-surfaces

Set of containerized scripts to extract mesh surfaces from a T1-weighted [MRI
scan](https://en.wikipedia.org/wiki/Magnetic_resonance_imaging).  Uses
[SimNIBS](https://simnibs.github.io/simnibs/) under the hood, plus some smaller
scripts.  The surfaces are then used in another project:
https://github.com/bast/tms-location


## Requirements

You need an installation of [Apptainer](https://apptainer.org/) (e.g. following
the [quick
installation](https://apptainer.org/docs/user/latest/quick_start.html#quick-installation)).
Alternatively, [SingularityCE](https://sylabs.io/singularity/) should also
work.


## How to use it

First download the container image (ending with *.sif) from here:
https://github.com/bast/MRI-extract-surfaces/releases

Here is an example which uses the container with the `T1_ernie.nii.gz` example
data file:
```bash
$ ./extract-surfaces.sif T1_ernie.nii.gz ernie_data
```

The above example reads `T1_ernie.nii.gz` and creates a directory `ernie_data`.
On my computer the process takes 30-60 minutes.

The input-file does not have to be gzipped, you can also use a plain NIfTI file:
```bash
$ ./extract-surfaces.sif T1_ernie.nii ernie_data
```

The output directory does not have to be in the same location and you can do
this instead:
```bash
$ ./extract-surfaces.sif T1_ernie.nii.gz /home/user/somewhere/ernie_data
```

The generated directory `ernie_data` contains the following files (once you
replace `T1_ernie.nii.gz` with your actual file, the generated file names might
be different):
```
ernie_data/
├── 1001.txt
├── 1002.txt
├── 1003.txt
├── 1005.txt
├── 1006.txt
├── 1007.txt
├── 1008.txt
├── 1009.txt
├── 1010.txt
└── outside-only.txt
```

Running the container also creates another folder `m2m_T1_ernie` (in the same
directory as the container image) containing many output files from
[SimNIBS](https://simnibs.github.io/simnibs/).


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


## About the container image

To build the image, I have used [this wonderful
guide](https://github.com/singularityhub/singularity-deploy) as starting point
and inspiration.

I find it important that everybody can verify how the container image was
built. And you can! You can inspect the definition file and all scripts which
are all part of this repository.
