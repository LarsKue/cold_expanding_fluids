
import unittest


class TestVec3(unittest.TestCase):
    # TODO
    pass


class TestParticles(unittest.TestCase):

    instance = None

    def setUp(self) -> None:
        import particles
        self.instance = particles.Particles()

        # need at least one particle to perform other tests
        self.instance.add_particle(particles.Vec3(0.0, 0.0, 0.0), particles.Vec3(0.0, 0.0, 0.0), 1.0)

    def test_run(self):
        from particles import Vec3

        for i in range(10):
            self.instance.add_particle(Vec3(i, i, 0.0), Vec3(0.0, 0.0, 0.0), 1.0)

        self.instance.run(n=100, h=0.01)

    def test_potential(self):
        def potential(v):
            return v

        self.instance.set_potential(potential)



        self.instance.run(n=100, h=0.01)

        self.instance.unset_potential()

    def test_error(self):
        def potential_err(_v):
            raise ValueError("I can't compute that")

        self.instance.set_potential(potential_err)

        with self.assertRaises(ValueError):
            self.instance.run(n=100, h=0.01)

        self.instance.unset_potential()



if __name__ == "__main__":
    unittest.main()
