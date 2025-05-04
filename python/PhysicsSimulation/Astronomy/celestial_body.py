from physics import Scalar, Vector, Point
from PhysicsSimulation.constants import G, c
from PhysicsSimulation.point_body import PointBody
from typing import Optional
from enum import Enum


class CelestialType(Enum):
    RockPlanet = 1,
    GasPlanet = 2,
    Star = 3,
    BlackHole = 4

    def __repr__(self) -> str:
        match self:
            case CelestialType.RockPlanet: return "Rock Planet"
            case CelestialType.GasPlanet: return "Gas Planet"
            case CelestialType.Star: return "Star"
            case CelestialType.BlackHole: return "Black Hole"


class Celestial(PointBody):
    """A celestial body."""
    def __init__(self,
                 name: str,
                 celestial_type: CelestialType,
                 velocity: Vector,
                 coordinates: Point,
                 mass: Optional[Scalar] = None,
                 radius: Optional[Scalar] = None,
                 density: Optional[Scalar] = None) -> None:
        self.name: str = name
        self.type: CelestialType = celestial_type
        self.velocity: Vector = velocity
        self.coordinates: Point = coordinates

        if self.type == CelestialType.BlackHole and mass:
            self.mass: Scalar = mass
            self.radius: Scalar = self.schwarzschild_radius()
            self.density: Scalar = float("inf")
        elif self.type == CelestialType.BlackHole and radius:
            self.radius: Scalar = radius
            self.mass: Scalar = self.schwarzschild_mass()
            self.density: Scalar = float("inf")
        elif radius and mass:
            self.mass: Scalar = mass
            self.radius: Scalar = radius
            self.density: Scalar = self.mass / self.radius
        elif density and mass:
            self.mass: Scalar = mass
            self.density: Scalar = density
            self.radius: Scalar = self.mass / self.density
        elif radius and density:
            self.radius: Scalar = radius
            self.density: Scalar = density
            self.mass: Scalar = self.radius * self.density
        else:
            raise ValueError("2 of the following 3 variables needs to be given: mass, radius, density."
                             "The only exception is if the celestial body is a black hole, in which case either the "
                             "radius or the mass needs to be given.")

        super().__init__(self.mass, self.velocity, self.coordinates)

    def schwarzschild_radius(self) -> Scalar:
        return Scalar(2) * G * self.mass / c ** 2

    def schwarzschild_mass(self) -> Scalar:
        return (self.radius * c ** 2) / (Scalar(2) * G)

    def __str__(self) -> str:
        return f"{self.name}" + "{" + (f" mass: {self.mass}, "
                                       f"velocity: {self.velocity.magnitude()}, "
                                       f"coordinates: {self.coordinates} ") + "}"