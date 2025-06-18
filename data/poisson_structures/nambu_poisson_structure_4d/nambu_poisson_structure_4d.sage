S.<xi1,xi2,xi3,xi4> = SuperfunctionAlgebra(SR, var('x,y,z,w'), simplify='expand', is_zero='is_trivial_zero')
P = -(xi1*xi2*xi3*xi4).bracket(S(function('u1')(x,y,z,w))).bracket(S(function('u2')(x,y,z,w)))
