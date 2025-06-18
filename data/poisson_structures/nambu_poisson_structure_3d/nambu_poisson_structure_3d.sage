S.<xi1,xi2,xi3> = SuperfunctionAlgebra(SR, var('x,y,z'), simplify='expand', is_zero='is_trivial_zero')
P = (xi1*xi2*xi3).bracket(S(function('u')(x,y,z)))
