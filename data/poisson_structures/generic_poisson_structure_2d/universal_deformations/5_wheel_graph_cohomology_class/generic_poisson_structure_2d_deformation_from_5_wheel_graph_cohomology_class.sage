from gcaops.all import *

D = DifferentialPolynomialRing(QQ, fibre_names=['u'], base_names=['x','y'], max_differential_orders=[5])
u = D.fibre_variables()[0]
x, y = D.base_variables()

S.<xi1,xi2> = SuperfunctionAlgebra(D, (x, y))
P = u*xi1*xi2

GC = UndirectedGraphComplex(QQ, implementation='vector', sparse=True)
gamma5 = GC.cohomology_basis(6,10)[0]

Q_gamma5_2d = S.zero()
try:
    Q_gamma5_2d = load('Q_gamma5_2d.sobj')
except FileNotFoundError:
    gamma5_op_2d = S.graph_operation(gamma5)
    Q_gamma5_2d = gamma5_op_2d(P,P,P,P,P,P)
    save(Q_gamma5_2d, 'Q_gamma5_2d.sobj')

with open('Q_gamma5_2d.txt', 'w') as f:
    print(Q_gamma5_2d, file=f)
