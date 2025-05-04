from pathlib import Path
from PhysicsSimulation.Astronomy.celestial_body import Celestial, CelestialType
from re import fullmatch, search, Match
from PhysicsSimulation.constants import NULL_VECTOR, ORIGO, AU, LIGHTYEAR, M_Sun, c
from PhysicsSimulation import Scalar, Vector, Point
from PhysicsSimulation.Astronomy.celestial_simulation import CelestialSimulation


def convert_exponents(string: str) -> float:
    if '^' in string and '*' in string:  # Convert the exponents to numbers
        a, e = string.split('*')
        x, y = e.split('^')
        return float(a) * pow(float(x), float(y))
    elif '^' in string:
        a, b = string.split('^')
        return pow(float(a), float(b))
    elif '*' in string:
        a, b = string.split('*')
        return float(a) * float(b)
    else:
        return float(string)


def velocity_component(component_str: str) -> Scalar:
    if component_str.isdigit():
        return Scalar(int(component_str))
    else:
        num: str = float(search(r"([-?\d\.]+)([\w/]+)", component_str).group(1))
        unit: str = search(r"([-?\d\.]+)([\w/]+)", component_str).group(2)

        match unit.lower():  # Convert to meters per second
            case "m/s": pass
            case "km/s": num *= 1000
            case "km/h": num *= 3.6
            case "c": num *= c.value

        return Scalar(num)


def read_in_velocity(velocity_str: str) -> Vector:
    parsed: Match = search(r"\((-?\d+[\w\./]*),(-?\d+[\w\./]*),(-?\d+[\w\./]*)\)", velocity_str)
    x: Scalar = velocity_component(parsed.group(1))
    y: Scalar = velocity_component(parsed.group(2))
    z: Scalar = velocity_component(parsed.group(3))

    return Vector(x, y, z)


def coordinate_component(component_str: str) -> Scalar:
    if component_str.isdigit():
        return Scalar(int(component_str))
    else:
        num: str = float(search(r"([-?\d\.]+)([\w/]+)", component_str).group(1))
        unit: str = search(r"([-?\d\.]+)([\w/]+)", component_str).group(2)

        match unit.lower():
            case "mm": num /= 1000
            case "cm": num /= 100
            case "dm": num /= 10
            case "m": pass
            case "km": num *= 1000
            case "au": num *= AU.value
            case "ly": num *= LIGHTYEAR.value

        return Scalar(num)


def read_in_coordinates(coordinates_str: str) -> Point:
    parsed: Match = search(r"\((-?\d+[\w\.]*),(-?\d+[\w\.]*),(-?\d+[\w\.]*)\)", coordinates_str)
    x: Scalar = coordinate_component(parsed.group(1))
    y: Scalar = coordinate_component(parsed.group(2))
    z: Scalar = coordinate_component(parsed.group(3))

    return Point(x, y, z)


def read(path: Path) -> CelestialSimulation:
    values = {"celestials": list()}

    last_planet = None
    for line in open(file=path, mode='r').readlines():
        line = line.rstrip()

        # Time change
        if fullmatch(r"dt: \d+ (ms|s|min|h|d|mon|y)\s*", line):
            num: float = float(search(r"dt: (\d+) (ms|s|min|h|hour|hours|d|day|days|mon|month|months|y|year|years)\s*", line).group(1))
            unit: str = search(r"dt: (\d+) (ms|s|min|h|d|mon|y)\s*", line).group(2)

            match unit:
                case "ms": num /= 1000
                case "s": pass
                case "min": num *= 60
                case "h" | "hour" | "hours": num *= 3600
                case "mon" | "month" | "months": num *= 3600 * 28
                case "y" | "year" | "years": num *= 3600 * 365
            values.update({"dt": Scalar(num)})

        # Name
        elif fullmatch(r"^name: [\w\s]+\s*$", line):
            values.update({
                "name": search(r"^name: ([\w\s]+)\s*$", line).group(1).strip()
            })

        # New planet
        elif fullmatch(r"^\w+:\s*$", line):
            if last_planet:
                values["celestials"].append(last_planet)  # Add the planet

            # Reset the planet
            last_planet = Celestial(
                name="",
                celestial_type=CelestialType.RockPlanet,
                velocity=NULL_VECTOR,
                coordinates=ORIGO,
                mass=Scalar(1),
                radius=Scalar(1)
            )
            last_planet.name = search(r"(\w+):\s*", line).group(1)  # Set the name

        # Planet properties
        elif fullmatch(r"^\s+(\w+):\s*([-()\w ,^/\.*]+)$", line):
            key: str = search(r"\s+(\w+):\s*([-()\w ,^/\.*]+)", line).group(1)
            value: str = search(r"\s+(\w+):\s*([-()\w ,^/\.*]+)", line).group(2)

            if key == "type":
                match value.strip().lower().replace(' ', ''):
                    case "blackhole": last_planet.type = CelestialType.BlackHole
                    case "gasplanet" | "gasgiant": last_planet.type = CelestialType.GasPlanet
                    case "star": last_planet.type = CelestialType.Star
                    case _: last_planet.type = CelestialType.RockPlanet
            elif key == "radius":
                value = value.strip().replace(' ', '')
                if not fullmatch(r"\d+\w+", value):
                    continue

                num: int = int(search(r"(\d+)(\w+)", value).group(1))
                unit: str = search(r"(\d+)(\w+)", value).group(2)

                match unit.lower():  # Convert to meters
                    case "mm": num /= 1000
                    case "cm": num /= 100
                    case "dm": num /= 10
                    case "m": pass
                    case "km": num *= 1000
                    case "au": num *= AU.value
                    case "ly": num *= LIGHTYEAR.value
                    case _: continue

                last_planet.radius = Scalar(num)
            elif key == "mass":
                value = value.strip().replace(' ', '')
                if not fullmatch(r"([\d\.*^]+)(\w+)", value):
                    continue

                num: str = search(r"([\d\.*^]+)(\w+)", value).group(1)
                unit: str = search(r"([\d\.*^]+)(\w+)", value).group(2)

                num: float = convert_exponents(num)

                match unit.lower():   # Convert to kilograms
                    case "g": num /= 1000
                    case "kg": pass
                    case "t": num *= 1000
                    case "sm": num *= M_Sun.value
                    case _: continue

                last_planet.mass = Scalar(num)
            elif key == "velocity":
                value = value.strip().replace(' ', '')
                if not fullmatch(r"\((-?\d+[\w\./]*),(-?\d+[\w\./]*),(-?\d+[\w\./]*)\)", value):
                    continue

                last_planet.velocity = read_in_velocity(value)
            elif key == "coordinates":
                value = value.strip().replace(' ', '')
                if not fullmatch(r"\((-?\d+[\w\.]*),(-?\d+[\w\.]*),(-?\d+[\w\.]*)\)", value):
                    continue

                last_planet.coordinates = read_in_coordinates(value)
    if last_planet:
        values["celestials"].append(last_planet)
    return CelestialSimulation(**values)