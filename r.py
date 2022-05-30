import cvxpy as cp
delta = cp.Variable(6)
lam = cp.Variable(6)
z = z_curr - delta + lam
R_new = R + gamma*delta - lam
objective = cp.Maximize(z.T @ mu - kappa*cp.quad_form(z, sigma))
constraints = [
cp.geo_mean(R_new) >= cp.geo_mean(R),
delta >= 0,
lam >= 0
]
problem = cp.Problem(objective, constraints)
problem.solve()