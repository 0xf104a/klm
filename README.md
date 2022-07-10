# klm
[![klmd](https://github.com/Andrewerr/klm/actions/workflows/klmd.yaml/badge.svg)](https://github.com/Andrewerr/klm/actions/workflows/klmd.yaml)
[![pyklm-errors](https://github.com/Andrewerr/klm/actions/workflows/pylint.yml/badge.svg)](https://github.com/Andrewerr/klm/actions/workflows/pylint.yml)
[![pyklm-test](https://github.com/Andrewerr/klm/actions/workflows/pyklm-test.yml/badge.svg)](https://github.com/Andrewerr/klm/actions/workflows/pyklm-test.yml)


Keyboard Light Management. 
A project to provide a flexible way to work with keyboard lightning on different keyboards models.

## Subprojects
### KLMd
A daemon which communicates with keyboard lightning via HID API. See [documentation](https://github.com/Andrewerr/klm/blob/main/klmd/README.md).
### pyklm
A python interface to klmd.
