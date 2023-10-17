import pickle
import sumtree



st = sumtree.SumTree(10)
st.add(42)
print(st)
x = pickle.dumps(st)
st2 = pickle.loads(x)
print(st2)