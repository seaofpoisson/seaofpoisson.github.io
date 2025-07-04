from gcaops.all import *

D = DifferentialPolynomialRing(QQ, fibre_names=['u'], base_names=['x','y'], max_differential_orders=[7])
u = D.fibre_variables()[0]
x, y = D.base_variables()

S.<xi1,xi2> = SuperfunctionAlgebra(D, (x, y))
P = u*xi1*xi2

GC = UndirectedGraphComplex(QQ, connected=True, biconnected=True, min_degree=3, implementation='vector', sparse=True)
gamma3 = GC.cohomology_basis(4,6)[0]
gamma5 = GC.cohomology_basis(6,10)[0]
gamma3_gamma5 = gamma3.bracket(gamma5)

Q_gamma3_gamma5_2d = S.zero()
try:
    Q_gamma3_gamma5_2d = load('Q_gamma3_gamma5_2d.sobj')
except FileNotFoundError:
    gamma3_gamma5_op_2d = S.graph_operation(gamma3_gamma5)
    Q_gamma3_gamma5_2d = gamma3_gamma5_op_2d(P,P,P,P,P,P,P,P,P)
    save(Q_gamma3_gamma5_2d, 'Q_gamma3_gamma5_2d.sobj')

with open('Q_gamma3_gamma5_2d.txt', 'w') as f:
    print(Q_gamma3_gamma5_2d, file=f)
