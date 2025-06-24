from gcaops.all import *

D = DifferentialPolynomialRing(QQ, fibre_names=['u'], base_names=['x','y'], max_differential_orders=[3])
u = D.fibre_variables()[0]
x, y = D.base_variables()

S.<xi1,xi2> = SuperfunctionAlgebra(D, (x, y))
P = u*xi1*xi2

GC = UndirectedGraphComplex(QQ, implementation='vector', sparse=True)
gamma3 = GC.cohomology_basis(4,6)[0]

Q_gamma3_2d = S.zero()
try:
    Q_gamma3_2d = load('Q_gamma3_2d.sobj')
except FileNotFoundError:
    gamma3_op_2d = S.graph_operation(gamma3)
    Q_gamma3_2d = gamma3_op_2d(P,P,P,P)
    save(Q_gamma3_2d, 'Q_gamma3_2d.sobj')

with open('Q_gamma3_2d.txt', 'w') as f:
    print(Q_gamma3_2d, file=f)
