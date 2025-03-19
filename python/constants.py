from physics import Scalar, Vector, Point

# Newton's gravitational constant
G: Scalar = 6.6743e-11

# Gravitational acceleration on Earth
g: Vector = Vector(0., -9.81, 0.)

# PI
PI: Scalar = 3.141592653589793

# Astronomical Unit
AU: Scalar = 149_597_870_700.

# Earth's Mass
M_Earth: Scalar = 5.97219e24

# Earth's Velocity
v_Earth: Vector = Vector(-29780., 0., 0.)

# Sun's Mass
M_Sun: Scalar = 2e30

# Zero (Scalar)
ZERO: Scalar = 0.

# Zero (Vector)
NULL_VECTOR: Vector = Vector(ZERO, ZERO, ZERO)

# Origo
ORIGO: Point = Point(ZERO, ZERO, ZERO)