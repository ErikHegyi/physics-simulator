from PhysicsSimulation.GL.window import *
from PhysicsSimulation import Astronomy
from PhysicsSimulation import Constants


class CelestialWindow(Window):
    def __init__(self,
                 objects: list[Astronomy.Star | Astronomy.Planet],
                 width: int,
                 height: int,
                 background_color: tuple[float, float, float, float],
                 title: Optional[str] = "OpenGL Window",
                 fov: int = 45,
                 camera_location: tuple[float, float, float] = (0.0, 0.0, -5.0)) -> None:
        self._objects: list[Astronomy.Star | Astronomy.Planet] = objects
        super().__init__(
            width,
            height,
            background_color,
            title,
            fov,
            camera_location
        )

        # Enable lighting
        glEnable(GL_LIGHTING)
        glEnable(GL_COLOR_MATERIAL)
        glEnable(GL_LIGHT0)
        glColorMaterial(GL_FRONT_AND_BACK, GL_DIFFUSE)


    def draw_objects(self) -> None:
        i: int = 0  # Keep track of the number of stars (light sources)

        # Calculate the size of the map
        map_size: int = max(
            self._objects,
            key=lambda x: x.coordinates().distance(Constants.ORIGO).value
        ).coordinates().distance(Constants.ORIGO)
        for celestial in self._objects:
            radius_multiplier: int = 100  # The system is so large, that the planets could not be seen without this

            # Get the color based on the celestial type
            if isinstance(celestial, Astronomy.Star):  # Star
                color = celestial.color()
            elif isinstance(celestial, Astronomy.Planet):  # Planet
                if celestial.planet_type == Astronomy.PlanetType.Terrestrial:
                    color = (0.2, 0.2, 0.2, 1.0)
                elif celestial.planet_type == Astronomy.PlanetType.Satellite:
                    color = (0.2, 0.2, 0.2, 1.0)
                else:
                    color = (0.5, 0.5, 0.8, 1.0)
            else:  # Black hole
                color = (0.0, 0.0, 0.0, 1.0)

            # Calculate the coordinates on the screen
            coordinates: tuple[float, float, float] = (
                (celestial.coordinates().x / map_size).value,
                (celestial.coordinates().y / map_size).value,
                (celestial.coordinates().z / map_size).value
            )

            if isinstance(celestial, Astronomy.Star):
                light = GL_LIGHT0 + i  # Handle multiple light sources
                glMaterialfv(GL_FRONT, GL_EMISSION, color)

                glEnable(light)
                glLight(light, GL_POSITION, (coordinates[0], coordinates[1], coordinates[2], 1.0))
                glLightfv(light, GL_DIFFUSE, color)

                i += 1
            else:
                glMaterialfv(GL_FRONT, GL_EMISSION, (0.0, 0.0, 0.0, 0.0))

                # Set the material properties to react to light
                glMaterialfv(GL_FRONT, GL_AMBIENT, (color[0] / 10, color[1] / 10, color[2] / 10, 1.0))
                glMaterialfv(GL_FRONT, GL_DIFFUSE, color)  # Use the planet's color for diffuse reflection


            # Calculate the radius on the screen
            radius: float = (celestial.radius / map_size).value * radius_multiplier

            glDisable(GL_LIGHTING)
            self.draw_text(
                (coordinates[0], coordinates[1] + radius, coordinates[2] + 0.1),
                (1.0, 1.0, 1.0, 1.0),
                celestial.name
            )
            glEnable(GL_LIGHTING)
            self.draw_sphere(coordinates, radius, color, 64)