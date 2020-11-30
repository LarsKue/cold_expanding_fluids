
import numpy as np
from matplotlib import pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from grid_solver import GridSolver
from copy import deepcopy
from progress_timer import timer

import os
import pickle

"""
def gaussian(x, y, z, x0, y0, z0, sigx, sigy, sigz, amplitude=1.0):
    return amplitude * np.exp(-0.5 * (((x - x0) / sigx) ** 2 + ((y - y0) / sigy) ** 2 + ((z - z0) / sigz) ** 2))
"""

def gross_pitaevskii(mass, lambd):
    np.seterr(all='raise')
    
    foldername = "Test11"
    pathmod = foldername + "/Data/"
    imgpathmod = foldername + "/Figures/"
    
    if not os.path.exists(foldername):
        os.makedirs(foldername)
    
    if not os.path.exists(foldername + "/Data"):
        os.makedirs(foldername + "/Data")
    
    if not os.path.exists(foldername + "/Figures"):
        os.makedirs(foldername + "/Figures")
    
    N = 100
    
    maxRange = 6
    
    x = y = z = np.linspace(-maxRange, maxRange, N)

    x, y, z = np.meshgrid(x, y, z)
    
    
    mu = 5.0
    
    """
    kx = 1. / 100
    ky = 4. / 100
    kz = 1. / 100
    """
    hbar = 1.05 * 10**(-34) #reduced planck constant
    m = 1.44 * 10**(-25) #particle mass (typical size for 87Rb: 1.44 * 10**(-25) kg)
    omega_x = 20 * np.pi #external potential strength in x-direction (typical size: 20pi rad/s)
    gamma_y = 5 #ratio omega_y / omega_x
    gamma_z = 1 #ratio omega_z / omega_x
    N_particle = 100 #particle number (typical size: 10**2 ~ 10**7)
    a_swave = 5.1 * 10**(-9) #s-wave scattering length (related to interaction strength) (typical size: 5.1 * 10**(-9) m)
    
    a_zero = np.sqrt(hbar / (omega_x * m)) #oscillator length scale (x-direction) (typical size with above params: 3.407 * 10**(-6) m)
    x_s = a_zero #chosen length scale
    #the chosen time scale is t_s = 1 / omega_x
    
    #epsilon = (a_zero / x_s)**2 #indirect ratio between time scale (omega_x -> a_zero) and spatial scale (x_s)
    epsilon = 1.
    
    #delta = 4 * np.pi * a_swave * N_particle / a_zero #dimensionless particle interaction strength (typical size with above params: N_particle * 1.881 * 10**(-2))
    delta = N_particle * 1.881 * 10**(-2)
    
    kappa = delta * np.sqrt(epsilon)**5 #full dimensionless prefactor of self-interaction
    
    """
    # external potential applied to the particles
    def V():
        return ((kx * x ** 2 + ky * y ** 2 + kz * z ** 2) / 2.).astype(np.complex128)
    
    
    def thomas_fermi_approx(pot, chem_pot):
        non_normed = 1./lambd * (np.full((N, N, N), chem_pot) - pot())
        return ((np.maximum(non_normed, np.zeros((N, N, N))))** (0.5)).astype(np.complex128)
    
    def update_func(_, d):
        gs, ggs = GridSolver.gradients(d)
        delta_phi = ggs[0][0] + ggs[1][1] + ggs[2][2]
        #print(type(1j * delta_phi / (2 * m) - 1j * l * np.abs(d) ** 2 * d / 6))
        #print(- 1j * V() * d)
        
        #print(d)
        
        return 1j * delta_phi / (2 * mass) - 1j * lambd * np.abs(d) ** 2 * d / 6 - 1j * V() * d# - 1j * mu * d
        
    def update_func_free(_, d):
        gs, ggs = GridSolver.gradients(d)
        delta_phi = ggs[0][0] + ggs[1][1] + ggs[2][2]
        #print(type(1j * delta_phi / (2 * m) - 1j * l * np.abs(d) ** 2 * d / 6))
        #print(- 1j * V() * d)
        
        #print(d)
        
        return 1j * delta_phi / (2 * mass) - 1j * lambd * np.abs(d) ** 2 * d / 6# - 1j * mu * d
    """
    
    V = ((x ** 2 + gamma_y * y ** 2 + gamma_z * z ** 2) / 2.).astype(np.complex128)
    
    def update_func(_, d):
        gs, ggs = GridSolver.gradients(d)
        delta_phi = ggs[0][0] + ggs[1][1] + ggs[2][2]
        
        partone = 1j * epsilon * delta_phi / 2
        parttwo = -1j * np.where((kappa / epsilon) * np.abs(d) < 10**(1), (kappa / epsilon) * (np.abs(d) ** 2).astype(np.complex128) * (d).astype(np.complex128), 0)
        partthree = -1j * (V / epsilon) * d
        
        return (partone + parttwo + partthree).astype(np.complex128)
        #return (1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * np.abs(d) ** 2 * d - 1j * (V / epsilon) * d).astype(np.complex128)
        #return (1j * epsilon * delta_phi / 2 - 1j * np.where((kappa / epsilon) * np.abs(d) < 10**(5), (kappa / epsilon) * np.abs(d) ** 2 * d, 0) - 1j * (V / epsilon) * d).astype(np.complex128)
        #return (1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * np.float_power(np.fmin(np.around(np.abs(d), 10), 10.**10), 2) * d - 1j * (V / epsilon) * d).astype(np.complex128)
        #return np.around(1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * np.abs(d) ** 2 * d - 1j * (V / epsilon) * d, 12)
        #return np.around(1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * (d.real**2 + d.imag**2) * d - 1j * (V / epsilon) * d, 20)
        
    def update_func_free(_, d):
        gs, ggs = GridSolver.gradients(d)
        delta_phi = ggs[0][0] + ggs[1][1] + ggs[2][2]
        
        #return (1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * np.abs(d) ** 2 * d).astype(np.complex128)
        return (1j * epsilon * delta_phi / 2 - 1j * np.where((kappa / epsilon) * np.abs(d) < 10**(5), (kappa / epsilon) * np.abs(d) ** 2 * d, 0)).astype(np.complex128)
        #return (1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * np.float_power(np.fmin(np.around(np.abs(d), 10), 10.**10), 2) * d).astype(np.complex128)
        #return np.around(1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * np.abs(d) ** 2 * d, 12)
        #return np.around(1j * epsilon * delta_phi / 2 - 1j * (kappa / epsilon) * (d.real**2 + d.imag**2) * d, 20)
    
    def thomas_fermi_approx():
        non_normed = 1./(kappa / epsilon) * (np.full((N, N, N), mu) - V)
        return ((np.maximum(non_normed, np.zeros((N, N, N))))**(0.5)).astype(np.complex128)
    
    def initial_gaussian():
        raw_gaussian = (((gamma_y * gamma_z)**(0.25)) / ((np.pi * epsilon)**(0.75))) * np.exp(-((x ** 2 + gamma_y * (y ** 2) + gamma_z * (z ** 2))) / (2. * epsilon))
        
        #def cutoff_at(_nn, _c):
        #    if _nn < _c:
        #        return 0
        #    else:
        #        return _nn
        #
        #lower_cutoff = (((gamma_y * gamma_z)**(0.25)) / ((np.pi * epsilon)**(0.75))) * 10**(-12)
        #print((lambda _n: cutoff_at(_n, lower_cutoff))(1))
        
        cutoff_gaussian = np.around(raw_gaussian, 12)
        #print(cutoff_gaussian[N // 2][N // 2][N // 2])
        
        return (cutoff_gaussian).astype(np.complex128)
    
    h = 0.1
    eqtime = 10
    expand_time = 10
    neq = round(eqtime / h)
    n = round(expand_time / h)
    
    timestep = 1
    subsections = round(expand_time / timestep)
    nmod = round(n / subsections)
    
    
    #data = thomas_fermi_approx()
    data = initial_gaussian()
    
    data_dict = {"N": N, "maxRange": maxRange, "mu": mu, "m": m, "N_particle": N_particle, "a_swave": a_swave, "epsilon": epsilon, "omega_x": omega_x, "gamma_y": gamma_y, "gamma_z": gamma_z, "t": -eqtime, "data": data}
    
    def save_data(ddict):
        fname = "t=" + str(ddict["t"]) + ".dat"
        
        
        #with open(pathmod + fname, "wb") as f:
            #np.save(f, data)
        
        with open(pathmod + fname, "wb") as f:
            pickle.dump(ddict, f)
            
    
    def load_data(fname):
        with open(pathmod + fname, "rb") as f:
            return pickle.load(f)
    
    save_data(data_dict)
    
    #testname = "t=" + str(data_dict["t"]) + ".dat"
    
    #testdict = load_data(testname)
    #testdata = testdict["data"]
    
    def make_figure(ddict):
        fig = plt.figure(figsize=(10, 10))
        ax = fig.add_subplot(111, projection="3d")
        ax.plot_surface(x[:, :, ddict["N"] // 2], y[:, :, ddict["N"] // 2], np.abs(ddict["data"][:, :, ddict["N"] // 2]) ** 2, cmap="hot")
        ax.set_xlabel("x")
        ax.set_ylabel("y")
        ax.set_zlabel("z")
        ax.set_zlim(0, 0.4)
        plt.title(f"Absolute Squared Distribution at z = {z[0,0,N // 2]}")
        
        plt.savefig(imgpathmod + "t=" + str(ddict["t"]) + ".png")
        
        plt.close()
        #plt.show()
        
    make_figure(data_dict)
    
    
    
    
    
    g = GridSolver(deepcopy(data))

    
    
    @timer(neq)
    def steps_trap():
       g.step_rk4(update_func, 0, h)
    
    @timer(nmod)
    def steps_free():
        g.step_rk4(update_func_free, 0, h)
    
    def regularize(_n):
        if np.isnan(_n):
            return 0
        else:
            return _n
    
    
    steps_trap()
    
    data_dict["t"] = 0
    data_dict["data"] = g.data
    save_data(data_dict)
    
    make_figure(data_dict)
    
    #fig = plt.figure(figsize=(10, 10))
    #ax = fig.add_subplot(111, projection="3d")
    #ax.plot_surface(x[:, :, N // 2], y[:, :, N // 2], np.abs(g.data[:, :, N // 2]) ** 2, cmap="hot")
    #ax.set_xlabel("x")
    #ax.set_ylabel("y")
    #ax.set_zlabel("z")
    #ax.set_zlim(0, 1)
    #plt.savefig(pathmod + "t=0.png")
    #plt.title(f"Absolute Squared Distribution at z = {z[0,0,N // 2]}")
    
    #plt.close()
    
    for i in range(0, subsections):
        steps_free()
        
        data_dict["t"] += timestep
        data_dict["data"] = g.data
        save_data(data_dict)
        
        make_figure(data_dict)
        #fig = plt.figure(figsize=(10, 10))
        #ax = fig.add_subplot(111, projection="3d")
        #ax.plot_surface(x[:, :, N // 2], y[:, :, N // 2], np.abs(g.data[:, :, N // 2]) ** 2, cmap="hot")
        #ax.set_xlabel("x")
        #ax.set_ylabel("y")
        #ax.set_zlabel("z")
        #ax.set_zlim(0, 1)
        #plt.savefig(pathmod + f"t={nmod*(i+1)*h:.4f}.png")
        
        #plt.close()
    
    
        
    #g.data = np.vectorize(regularize)(g.data)
    



def main(argv: list) -> int:
    gross_pitaevskii(1, 1)
    return 0


if __name__ == "__main__":
    import sys
    main(sys.argv)
