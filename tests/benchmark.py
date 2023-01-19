import sumtree
import sumtree_python
from timeit import timeit


CAPACITY = 50_000
N_ITER = 100_000


def bench_python():
    st = sumtree_python.SumTree(CAPACITY)
    for i in range(N_ITER):
        st.add(i)
        index, value = st.get(25_000)


def bench_rust():
    st = sumtree.SumTree(CAPACITY)
    for i in range(N_ITER):
        st.add(i)
        index, value = st.get(25_000)


if __name__ == "__main__":
    duration = timeit(bench_python, number=10)
    print(f"Python duration {duration}s")
    duration = timeit(bench_rust, number=10)
    print(f"Rust duration {duration}s")
