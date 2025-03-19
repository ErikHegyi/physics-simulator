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

# One second
SECOND: Scalar = 1.

# One minute
MINUTE: Scalar = 60.

# One hour
HOUR: Scalar = 3600.

# One Day
DAY: Scalar = 86_400.

# One Week
WEEK: Scalar = 604_800.

# One Month (28 days)
MONTH: Scalar = 2_419_200.

# One Year
YEAR: Scalar = 31_556_926.