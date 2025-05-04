from physics import Scalar, Vector, Point
from PhysicsSimulation.constants import g, G
from typing import Self


class PointBody:
    """
    A point-body in space, with a mass, velocity, position, charge, momentum, ...
    """
    def __init__(self,
                 mass: Scalar,
                 velocity: Vector,
                 coordinates: Point,
                 charge: Scalar = 0.) -> None:
        self.mass: Scalar = mass
        self.velocity: Vector = velocity
        self.coordinates: Point = coordinates
        self.charge: Scalar = charge

    def momentum(self) -> Vector:
        """Calculate the momentum of the body."""
        return Vector.from_point(self.velocity.point * self.mass)

    def kinetic_energy(self) -> Scalar:
        """Calculate the kinetic energy of the body."""
        return 0.5 * self.mass * self.velocity.magnitude() ** 2

    def potential_energy(self,
                         height: Scalar,
                         gravity: Vector = g) -> Scalar:
        """Calculate the potential energy of the body at the given height."""
        return self.mass * height * gravity.magnitude()

    def acceleration(self, force: Vector) -> Vector:
        """Calculate the acceleration of the body if a force is applied to it."""
        return force / self.mass

    def force(self, acceleration: Vector) -> Vector:
        """Calculate the force of the body if it is accelerated."""
        return acceleration * self.mass

    def gravitational_force(self, other: Self) -> Vector:
        """Calculate the gravitational force applied to the body."""
        force: Scalar = G * self.mass * other.mass / self.coordinates.distance(other.coordinates) ** 2
        return Vector.from_magnitude(force, other.coordinates, self.coordinates)

    def advance(self, dt: Scalar) -> None:
        """Advance the body by dt seconds."""
        self.coordinates.x += self.velocity.point.x * dt
        self.coordinates.y += self.velocity.point.y * dt
        self.coordinates.z += self.velocity.point.z * dt