# MRI-extract-surfaces

Set of containerized scripts to extract mesh surfaces from T1/T2-weighted [MRI
scans](https://en.wikipedia.org/wiki/Magnetic_resonance_imaging).  Uses
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
https://github.com/bast/MRI-extract-surfaces/releases - then make the container image executable.

Here is an example which uses the container with the `T1_ernie.nii.gz` example
data file:
```bash
$ ./extract-surfaces.sif ernie /home/user/ernie_data T1_ernie.nii.gz
```

The above example reads `T1_ernie.nii.gz` and creates the directories
`m2m_ernie` and `/home/user/ernie_data`.  On my computer the process takes
30-60 minutes.  The folder `m2m_ernie` is created in the same directory as the
container image and contains many output files from
[SimNIBS](https://simnibs.github.io/simnibs/).

The input-file does not have to be gzipped, you can also use a plain NIfTI file:
```bash
$ ./extract-surfaces.sif ernie /home/user/ernie_data T1_ernie.nii
```

If you have a T2-weighted MRI scan as well, you can use both T1 and T2 data as input:
```bash
$ ./extract-surfaces.sif ernie /home/user/ernie_data T1.nii.gz T2.nii.gz
```

The generated directory `/home/user/ernie_data` contains the following files:
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
├── eeg-positions.csv
├── m2m_data
│   ├── data.msh
│   ├── final_tissues.nii.gz
│   ├── T1.nii.gz
│   └── toMNI
│       ├── Conform2MNI_nonl.nii.gz
│       ├── final_tissues_MNI.nii.gz
│       └── MNI2Conform_nonl.nii.gz
├── outside-only.txt
└── VERSION
```


## Where to get an example input file

You can get the [example dataset](https://simnibs.github.io/simnibs/build/html/dataset.html) like this:
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
