
import unittest


class TestVec3(unittest.TestCase):
    pass


class TestParticles(unittest.TestCase):

    instance = None

    def setUp(self) -> None:
        import particles
        self.instance = particles.Particles()

    def test_add_particle(self):
        from particles import Vec3, Particles
        self.instance.add_particle(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0), 1.0)

    def test_run(self):
        from particles import Vec3

        for i in range(1000):
            self.instance.add_particle(Vec3(i, i, 0.0), Vec3(0.0, 0.0, 0.0), 1.0)

        self.instance.run(n=1000, h=0.01)


if __name__ == "__main__":
    unittest.main()
