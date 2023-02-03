# SumTree
This module contains a SumTree implementation in Rust with Python wrappers.
Speedup is around 10x in comparison to an equivalent full Python implementation.

Check the project on github: [https://github.com/yamoling/sumtree](https://github.com/yamoling/sumtree)

## Usage
### Initialisation
```python
from sumtree import SumTree
st = SumTree(1024)
st.add(10)
print(st.total)     # 10
print(len(st))      # 1
print(st.capacity)  # 1024
```

### Sampling data
```python
from sumtree import SumTree
st = SumTree(1024)
for i in range(1024):
    st.add(i)
index, value = st.get(500)
print(index, value)  # 32, 32.0

indices, values = st.sample(10)
```

### Updating data
```python
from sumtree import SumTree
st = SumTree(1024)
for i in range(1024):
    st.add(i)
# Set leaf 0 to value 40.
tree.update(0, 40.)
```