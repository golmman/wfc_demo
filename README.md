# wfc_demo

## Description of the Algorithm

### General Setup

#### Input

- path for the image to extract patterns from
- pattern width
- pattern height
- target image width
- target image height

#### Output

- generated image

#### Description

```
extract_patterns()
build_propagator()
initialize_wave()

loop {
  observe()
  propagate()
}

combine observations()
```

### Extract Patterns

#### Input

- path for the image to extract patterns from
- pattern width
- pattern height

#### Output

- `PatternData`: list of unique, weighted patterns

#### Description

- treat input image as a torus
- for each image pixel store all possible patterns
- count duplicated patterns as pattern weights
- remove duplicated patterns

### Build Propagator

#### Input

- `PatternData`

#### Output

- `PatternPropagator`: table of unique, weighted pattern pixels and their relationships to other pattern pixels
- here the weights are converted to probabilites (floats in the range of [0, 1])
  - proper probabilities will be important in the observe step

#### Description

Goal: prepare a lookup table for the propagate step

- calculate relationships
- add up weights of duplicated pattern pixels
- remove duplicated pattern pixels
- convert weights to probabilities

### Initialize Wave

#### Input

* pattern propagator
* target image width
* target image height

#### Output

* `Wave`: matrix with target image size, initialized with a list of all `PatternPropagator` pixels for each entry

#### Description

* create empty matrix
* for each entry add the list of `PatternPropagator` pixels

### Observe

#### Input

* `Wave`
* `PatternPropagator`

#### Output

* updated wave
* index of changed wave entry

#### Description

* search for the wave entry with more than 1 list elements left and the lowest shannon entropy
  * loop over all entries
  * skip iteration if entry is a list of 1 element
  * calculate the shannon entropy and update best entry variable if lower
    * let p_i be the probability of a propagator pixel, then the shannon entropy is calculated as -sum_i(p_i * log(p_i))
  * if all entries have only one list element left exit the outer loop, i.e. jump to `combine_observations`
* for the found wave entry pick a random pattern pixel, according to their probabilites
* "collapse" the wave list at this entry to the one picked pixel

### Propagate

#### Input

#### Output

#### Description

* if an entry consists of an empty list a contradiction was found:
  * backtrack, or
  * restart algorithm from the `initialize_wave` step (recommended by Gumin)

### Combine Observations

#### Input

#### Output

#### Description

## Links and other resources

- the original idea by Maxim Gumin: https://github.com/mxgmn/WaveFunctionCollapse
- reasonably good paper by Isaac Karth and Adam Smith describing more details: https://adamsmith.as/papers/wfc_is_constraint_solving_in_the_wild.pdf

