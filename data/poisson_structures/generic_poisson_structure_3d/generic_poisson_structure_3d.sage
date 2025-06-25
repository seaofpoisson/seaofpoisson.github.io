D = DifferentialPolynomialRing(QQ, fibre_names=['u','v','w'], base_names=['x','y','z'], max_differential_orders=[1,1,1])
x,y,z = D.base_variables()
u,v,w = D.fibre_variables()

S.<xi1,xi2,xi3> = SuperfunctionAlgebra(D, [x,y,z])
P = S(u)*xi2*xi3 + S(v)*xi3*xi1 + S(w)*xi1*xi2
