from PhysicsSimulation import Scalar, Vector, Point

# Zero (Scalar)
ZERO: Scalar = Scalar(0.)

# Zero (Vector)
NULL_VECTOR: Vector = Vector(ZERO, ZERO, ZERO)

# Newton's gravitational constant
G: Scalar = Scalar(6.6743e-11)

# Gravitational acceleration on Earth
g: Vector = Vector(ZERO, Scalar(-9.81), ZERO)

# PI
PI: Scalar = Scalar(3.141592653589793)

# Astronomical Unit
AU: Scalar = Scalar(149_597_870_700.)

# Lightyear
LIGHTYEAR: Scalar = Scalar(9.4605284e15)

# Earth's Mass
M_Earth: Scalar = Scalar(5.97219e24)

# Earth's Velocity
v_Earth: Vector = Vector(Scalar(-29780.), ZERO, ZERO)

# Sun's Mass
M_Sun: Scalar = Scalar(2e30)

# Origo
ORIGO: Point = Point(ZERO, ZERO, ZERO)

# One second
SECOND: Scalar = Scalar(1.)

# One minute
MINUTE: Scalar = Scalar(60.)

# One hour
HOUR: Scalar = Scalar(3600.)

# One Day
DAY: Scalar = Scalar(86_400.)

# One Week
WEEK: Scalar = Scalar(604_800.)

# One Month (28 days)
MONTH: Scalar = Scalar(2_419_200.)

# One Year
YEAR: Scalar = Scalar(31_556_926.)

# Lightspeed
c: Scalar = Scalar(299_792_458.)