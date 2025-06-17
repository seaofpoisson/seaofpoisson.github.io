S.<xi1,xi2> = SuperfunctionAlgebra(SR, var('x,y'), simplify='expand', is_zero='is_trivial_zero')
P = function('u')(x,y)*xi1*xi2
