class Degree:
    ...


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

    @staticmethod
    def from_magnitude(magnitude: Scalar, a: Point, b: Point):
        """Create a new vector with the given magnitude, pointing from b towards a."""

    def magnitude(self) -> Scalar:
        """Calculate the magnitude of the vector."""

    def mul(self, other: Scalar) -> Vector:
        """Multiply the vector by the given amount."""