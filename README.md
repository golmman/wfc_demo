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
pattern_data = extract_patterns(path, pattern_width, pattern_height)
pattern_propagator = build_propagator(pattern_data)
wave = initialize_wave(pattern_propagator, target_image_width, target_image_height)

loop:
  (wave, updated_index) = observe(wave)
  wave = propagate(wave, updated_index, pattern_propagator)

combine_observations(wave)
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

- `PatternPropagator`: table of (unique?) weighted pattern pixels and their relationships to other pattern pixels
- a total weight is calculated based on the total number of pixels and the weights collected during the pattern extraction

#### Description

Goal: prepare a lookup table for the propagate step

- calculate total weight
- initialze pattern pixels with empty relationships
- calculate relationships
- remove duplicated pattern pixels
  - TODO: unclear if necessary or how to do this

### Initialize Wave

#### Input

- pattern propagator
- target image width
- target image height

#### Output

- `Wave`: matrix with target image size, initialized with a list of all `PatternPropagator` pixels for each entry

#### Description

- create empty matrix
- for each entry add the list of `PatternPropagator` pixels

### Observe

#### Input

- `Wave`
- `PatternPropagator`

#### Output

- updated `Wave`
- index of changed wave entry

#### Description

- search for the wave entry with more than 1 list elements left and the lowest shannon entropy
  - loop over all entries
  - skip iteration if entry is a list of 1 element
  - calculate the shannon entropy and update best entry variable if lower
    - let p_i be the probability of a propagator pixel, then the shannon entropy is calculated as -sum_i(p_i \* log(p_i))
  - if all entries have only one list element left exit the outer loop, i.e. jump to `combine_observations`
- for the found wave entry pick a random pattern pixel, according to their probabilites
- "collapse" the wave list at this entry to the one picked pixel

### Propagate

#### Input

- `PatternPropagator`
- `Wave`
- index of changed wave entry

#### Output

- updated `Wave`

#### Description

- let a be the index of the changed wave entry
- initialize an empty stack of index pairs
- put (a, a) onto the stack
- while the stack is not empty:
  - pop (i, j) from the stack
  - if the wave entry j consists of an empty list a contradiction was found:
    - backtrack, or
    - restart algorithm from the `initialize_wave` step (recommended by Gumin)
  - for each pattern pixel u of wave entry i
    - for each pattern pixel v of wave entry j
      - if the pattern propagator relationship between u and v is false
        - remove v from wave entry j
  - if any remove took place
    - for all 8 neighbors k of index j
      - put (j, k) onto the stack

Note that it is not obvious why this algorithm is correct in the sense that it catches all ways to update the wave while only checking in the 8-neighborhood instead of all possibly affected pixels. Also it seems that there are possibly lots of unecessary/duplicated checks making this not very efficient.

### Combine Observations

#### Input

- collapsed `Wave` (all entries are lists of one element)

#### Output

- image

#### Description

- map one-element-lists to there respective pixels and interprete this a an image

## Links and other resources

- the original idea by Maxim Gumin: https://github.com/mxgmn/WaveFunctionCollapse
- reasonably good paper by Isaac Karth and Adam Smith describing more details: https://adamsmith.as/papers/wfc_is_constraint_solving_in_the_wild.pdf
