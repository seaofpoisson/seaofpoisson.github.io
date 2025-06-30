from gcaops.all import *

GC = UndirectedGraphComplex(QQ, connected=True, biconnected=True, min_degree=3, implementation='vector', sparse=True)
gamma3 = GC.cohomology_basis(4,6)[0]
gamma5 = GC.cohomology_basis(6,10)[0]
gamma3_gamma5 = gamma3.bracket(gamma5)
