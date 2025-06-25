from gcaops.all import *

D = DifferentialPolynomialRing(QQ, fibre_names=['u','v','w'], base_names=['x','y','z'], max_differential_orders=[4,4,4])
u,v,w = D.fibre_variables()
x,y,z = D.base_variables()

S.<xi1,xi2,xi3> = SuperfunctionAlgebra(D, (x, y, z))
P = S(u)*xi2*xi3 + S(v)*xi3*xi1 + S(w)*xi1*xi2

GC = UndirectedGraphComplex(QQ, implementation='vector', sparse=True)
gamma3 = GC.cohomology_basis(4,6)[0]
gamma3_op_3d = S.graph_operation(gamma3)

Q_gamma3_3d = S.zero()
try:
    Q_gamma3_3d = load('Q_gamma3_3d.sobj')
except FileNotFoundError:
    Q_gamma3_3d = gamma3_op_3d(P,P,P,P)
    save(Q_gamma3_3d, 'Q_gamma3_3d.sobj')
    with open('Q_gamma3_3d.txt', 'w') as f:
        print(Q_gamma3_3d, file=f)

try:
    L_gamma3_3d = load('L_gamma3_3d.sobj')
except FileNotFoundError:
    L_gamma3_3d = gamma3_op_3d(P,P,P,P.bracket(P))
    P_Q_gamma3_3d = P.bracket(Q_gamma3_3d)
    assert P_Q_gamma3_3d == 2*L_gamma3_3d
    save(L_gamma3_3d, 'L_gamma3_3d.sobj')
    with open('L_gamma3_3d.txt', 'w') as f:
        print(L_gamma3_3d, file=f)
