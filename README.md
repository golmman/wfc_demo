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

The propagator holds an 8-dimensional array/tensor which is unfolded into the `pattern_pixels` vector which stores 3 dimensions, then each element has an `relationships` vector which stores the remaining 5 dimensions.

##### `pattern_pixels`

- p1 = 1st pattern index
- x1 = 1st pattern pixel x coordinate
- y1 = 1st pattern pixel y coordinate

The 3-dimensions are only used during the initialization phase, later `pattern_pixels` are accessed as a 1-dimensional array where (p1, x1, y1) is collapsed to the pixel index.

##### `relationships`

- p2 = 2nd pattern index
- x2 = 2nd pattern pixel x coordinate
- y2 = 2nd pattern pixel y coordinate
- u = 2nd pattern pixel x coordinate from the 1st patterns perspective
- v = 2nd pattern pixel y coordinate from the 1st patterns perspective

The 5-dimensions are only used during the initialization phase, later `relationships` are accessed as a 3-dimensional array where (p2, x2, y2) is collapsed to the pixel index.

##### Summary

- The pattern propagator stores 8-dimensional data
- as a 2-dimensional array (3d-`pattern_pixels`, each storing 5d-`relationships`)
- which are accessed like a 4-dimensional array (1d-`pattern_pixels`, 3d-`relationships`)
- the 8-dimensionality is only relevant during initialization

##### Algorithm

- calculate total weight
- initialze pattern pixels with empty relationships
- calculate relationships
  - the number of total relationships per pixel equals P _ (W _ H)^2, where
    - P = total number of patterns (after deduplication)
    - W = pattern width
    - H = pattern height
  - the relationships vector can be thought of as a 5-dimensional array with indices as follows
    - 1st index: pattern index
    - 2nd and 3rd index: x and y coordinate of the pixel inside this pattern
    - 4th and 5th index: x and y coordinate of the compared pixel inside this pattern
    - see below for a graphic example
- remove duplicated pattern pixels
  - TODO: unclear if necessary or how to do this

##### Example for the usage of the relationships vector

```
Pixels not adjacent:
 -------------------
|   |   |   |   |   |
 ---------------------------
|   |   | ~ | B | ~ |   |   |
 ---------------------------
|   |   | ~ | ~ | ~ | A |   |
 ---------------------------
        |   |   |   |   |   |
         -------------------

Pixels adjacent
 -------------------
|   |   |   |   |   |
 -----------------------
|   | ~ | B | ~ | ~ |   |
 -----------------------
|   | ~ | ~ | ~ | A |   |
 -----------------------
    |   |   |   |   |   |
     -------------------
```

- general explanation
  - pictured are to examples of overlapping patterns
  - the patterns have width 5 and height 3
  - the top left coordinate in a pattern is x=0, y=0
  - `~` indicates the overlapping area of the patterns
  - in the 1st example: the top left pattern contains pixel B at x=3, y=1
  - in the 2nd example: the top left pattern contains pixel B at x=2, y=1
  - in both examples: the bottom right pattern contains pixel A at x=3, y=1
  - pixels are _adjacent_ iff both are contained in the overlapping area
- check for the relationship of the pattern pixels in the 2nd example
  - the wave stores a list of `pattern_pixels` indices at each position
  - at some position inside the wave we want to check if the pixel there (pixel A) is compatible / in relationship with another adjacent pixel (pixel B)
    - if the pixels were not adjacent the relationship vector should answer with a `false` value
    - let u = pixel A's index (in the propagators `pattern_pixels` vector)
    - let v = pixel B's index
    - let s = pattern_width \* pattern_height
    - let w = pattern width
    - from pixel A's perspective B is located at x=1, y=0 inside its pattern
    - then the relationship index is calculated as
      - `let r = v * s + y * w + x`
    - then check A's relationship vector for the answer
      - `let a = pattern_propagator.pattern_pixels[u].relationships[r]`
    - a is a boolean with the precalculated (during the build_propagator step) answer

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
