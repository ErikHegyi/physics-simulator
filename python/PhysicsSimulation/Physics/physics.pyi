from enum import Enum


class Degree:
    """A geometrical degree"""
    def __init__(self,
                 degree: int = 0,
                 minute: int = 0,
                 second: int = 0) -> None:
        self.degree: int
        self.minute: int
        self.second: int

    def to_float(self) -> float:
        """
        Convert a degree to a floating point integer.
        For example: 180° 30' 0" -> 180.5°
        """

    @staticmethod
    def from_float(float: float) -> Degree:
        """
        Create a degree from a floating point number.
        For example: 180.5° -> 180° 30' 0"
        """

    def to_radian(self) -> float:
        """Convert the degrees to radians."""

    @staticmethod
    def from_radian(radian: float) -> Degree:
        """Convert radians to degrees."""

    def sin(self) -> float: ...
    def cos(self) -> float: ...
    def tan(self) -> float: ...

    @staticmethod
    def asin(ratio: float) -> Degree: ...
    @staticmethod
    def acos(ratio: float) -> Degree: ...
    @staticmethod
    def atan(ratio: float) -> Degree: ...


class Scalar:
    """A value with only magnitude and no direction"""

    def __init__(self,
                 value: float) -> None:
        self.value: float

    def powi(self) -> Scalar: ...


class Point:
    """A point in 3-dimensional space."""

    def __init__(self,
                 x: Scalar,
                 y: Scalar,
                 z: Scalar) -> None:
        self.x: Scalar
        self.y: Scalar
        self.z: Scalar

    def distance(self, other: Point) -> Scalar:
        """Calculate the distance between this point and the given other point."""


class Vector:
    """A 3-dimensional vector, pointing from [0, 0, 0] to self.point."""

    def __init__(self,
                 x: Scalar,
                 y: Scalar,
                 z: Scalar) -> None:
        self.point: Point

    @staticmethod
    def from_point(point: Point) -> Self:
        """Create a new vector from a point"""

    def to_point(self) -> Point:
        """Convert the Vector to a Point"""

    @staticmethod
    def from_magnitude(magnitude: Scalar, a: Point, b: Point):
        """Create a new vector with the given magnitude, pointing from b towards a."""

    def magnitude(self) -> Scalar:
        """Calculate the magnitude of the vector."""

    def mul(self, other: Scalar) -> Vector:
        """Multiply the vector by the given amount."""


class Radiation:
    """
    **Radiation**

    The radiation emitted by a black body.
    :param wavelength: The wavelength of the radiation.
    :param frequency: The frequency of the radiation.
    :param temperature: The temperature of the body.
    """

    def __init__(self,
                 temperature: Scalar) -> None:
        self.wavelength: Scalar
        self.frequency: Scalar
        self.temperature: Scalar

    @staticmethod
    def from_wavelength(wavelength: Scalar) -> Radiation:
        """Calculate the radiation based on the wavelength of the emitted light."""

    @staticmethod
    def from_frequency(frequency: Scalar) -> Radiation:
        """Calculate the radiation based on the frequency of the emitted light."""

    @staticmethod
    def color_from_wavelength(wavelength: Scalar) -> (float, float, float, float):
        """Calculate the color of the emitted light based on its wavelength"""

    def color(self) -> (float, float, float, float):
        """Calculate the color of the emitted light."""


class PointBody:
    """
    **Point Body**

    A body, with no volume, surface, ...
    :param mass: The mass of the body.
    :param velocity: The velocity of the body.
    :param coordinates: The coordinates of the body.
    :param charge: The charge of the body.
    """

    def __init__(self,
                 mass: Scalar,
                 velocity: Vector,
                 coordinates: Point,
                 charge: Scalar) -> None:
        self.mass: Scalar
        self.velocity: Vector
        self.coordinates: Point
        self.charge: Scalar

    def momentum(self) -> Vector:
        """Calculate the momentum of the body."""

    def kinetic_energy(self) -> Scalar:
        """Calculate the kinetic energy of the body."""

    def potential_energy(self) -> Scalar:
        """Calculate the potential energy of the body."""

    def acceleration(self, force: Vector) -> Vector:
        """
        Calculate the acceleration of the body based on the force applied to it.
        :param force: The force applied on the body.
        """

    def force(self, acceleration: Vector) -> Vector:
        """
        Calculate the force applied to the body based on the body's acceleration.
        :param acceleration: The acceleration of the body.
        """

    def distance(self, point: Point) -> Scalar:
        """
        Calculate the distance between the body and a given point.
        """

    def gravitational_force(self, other: PointBody) -> Vector:
        """
        Calculate the gravitational force between two bodies.
        """

    def advance(self, dt: Scalar) -> None:
        """
        Advance the body by dt seconds.
        """


class PlanetType(Enum):
    Terrestrial = 0
    GasGiant = 1
    Satellite = 2


class Planet:
    """
    **Planet**

    :param name: The name of the planet.
    :param radius: The radius of the planet.
    :param planet_type: The type of the planet (terrestrial, gas giant, satellite).
    :param mass: The mass of the planet.
    :param velocity: The velocity of the planet.
    :param coordinates: The coordinates of the planet.
    """

    def __init__(self,
                 name: str,
                 velocity: Vector,
                 coordinates: Point,
                 mass: Scalar,
                 radius: Scalar,
                 planet_type: PlanetType) -> None:
        self.name: str
        self.radius: Scalar
        self.planet_type: PlanetType
        self.point_body: PointBody

    def mass(self) -> Scalar: ...
    def velocity(self) -> Vector: ...
    def coordinates(self) -> Point: ...

    def set_mass(self, mass: Scalar) -> None: ...
    def set_velocity(self, velocity: Vector) -> None: ...
    def set_coordinates(self, coordinates: Point) -> None: ...

    def momentum(self) -> Vector: ...
    def kinetic_energy(self) -> Scalar: ...
    def acceleration(self, force: Vector) -> Vector: ...
    def force(self, acceleration: Vector) -> Vector: ...
    def distance(self, other: Point) -> Scalar: ...
    def gravitational_force(self, other: PointBody) -> Vector: ...
    def advance(self, dt: Scalar) -> None: ...

    def surface_acceleration(self) -> Vector: ...


class StarType(Enum):
    O = 0
    """Blue stars"""

    B = 1
    """Blue-white stars"""

    A = 2
    """White stars"""

    F = 3
    """Yellow-white stars"""

    G = 4
    """Yellow stars"""

    K = 5
    """Orange stars"""

    M = 6
    """Red stars"""


class Star:
    """
    **Star**

    :param name: The name of the star.
    :param radius: The radius of the star.
    :param mass: The mass of the star.
    :param velocity: The velocity of the star.
    :param coordinates: The coordinates of the star.
    """

    def __init__(self,
                 name: str,
                 velocity: Vector,
                 coordinates: Point,
                 mass: Scalar,
                 radius: Scalar) -> None:
        self.name: str
        self.radius: Scalar
        self.star_type: StarType
        self.point_body: PointBody

    def mass(self) -> Scalar: ...
    def velocity(self) -> Vector: ...
    def coordinates(self) -> Point: ...

    def set_mass(self, mass: Scalar) -> None: ...
    def set_velocity(self, velocity: Vector) -> None: ...
    def set_coordinates(self, coordinates: Point) -> None: ...

    def momentum(self) -> Vector: ...
    def kinetic_energy(self) -> Scalar: ...
    def acceleration(self, force: Vector) -> Vector: ...
    def force(self, acceleration: Vector) -> Vector: ...
    def distance(self, other: Point) -> Scalar: ...
    def gravitational_force(self, other: PointBody) -> Vector: ...
    def advance(self, dt: Scalar) -> None: ...

    def surface_acceleration(self) -> Vector: ...
    def luminosity(self) -> Scalar: ...
    def surface_temperature(self) -> Scalar: ...
    def color(self) -> (float, float, float, float): ...
    def star_type(self) -> StarType: ...

class Constants:
    ZERO: Scalar
    """**Zero**"""

    NULL_VECTOR
    """**Zero (vector)**"""

    ORIGO: Point
    """
    **Origo**
    
    The zero-point for the simulation.
    """

    G: Scalar
    """
    **Gravitational Constant**
    
    Newton's gravitational constant.
    
    Value: 6.6743 * 10^-11 m^3 kg^-1 s^-2
    """

    g: Vector
    """
    **Gravitational Acceleration on Earth**
    
    Value: 9.81 ms^-2
    """

    PI: Scalar
    """**PI**"""

    AU: Scalar
    """
    **Astronomical Unit**
    
    The average distance between the Earth and the Sun.
    
    Value: 149 597 870 700 m
    """

    LIGHTYEAR: Scalar
    """
    **Light Year**
    
    The distance light travels in a year.
    """

    EARTH_MASS: Scalar
    """
    **Earth's Mass**
    
    The mass of Earth.
    
    Value: 5.97219 * 10^24 kg
    """

    EARTH_VELOCITY: Vector
    """
    **Earth's Velocity**
    
    The velocity of Earth.
    
    Value: 29 780 km/s (in the simulation, it is pointing left)
    """

    SOLAR_MASS: Scalar
    """
    **Solar Mass**
    
    The mass of the Sun.
    
    Value: 2 * 10^30 kg
    """

    SOLAR_LUMINOSITY: Scalar
    """
    **Solar Luminosity**
    
    The power emitted by the Sun.
    
    Value: 3.828 * 10^26 W
    """

    STEFAN_BOLTZMANN_CONSTANT: Scalar
    """
    **Stefan-Boltzmann Constant**
    
    The Stefan-Boltzmann constant, used for calculating radiation and luminosity from temperature.
    
    Value: 5.670367 * 10^-8 W m^-2 K^-4
    """

    SECOND: Scalar
    """**One Second**"""

    MINUTE: Scalar
    """**One Minute**"""

    HOUR: Scalar
    """**One Hour**"""

    DAY: Scalar
    """**One Day**"""

    WEEK: Scalar
    """**One Week**"""

    MONTH: Scalar
    """**One Month**"""

    YEAR: Scalar
    """**One Year**"""

    c: Scalar
    """
    **Lightspeed**
    
    The speed of light moving in a vacuum.
    
    Value: 299 792 458 m/s
    """
