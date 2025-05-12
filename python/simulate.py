from PhysicsSimulation.Astronomy import read

sim = read("simulations/sun_and_terrestrials.psa")
sim.initialize_graphics()
sim.run()
