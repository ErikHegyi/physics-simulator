from OpenGL.raw.GL.VERSION.GL_1_0 import glMatrixMode, GL_MODELVIEW
from OpenGL.raw.GLU import gluPerspective

from PhysicsSimulation import Scalar, Vector, Constants
from PhysicsSimulation import Astronomy
from typing import Optional
from PhysicsSimulation.Astronomy.celestial_graphics import CelestialWindow
from glfw import *
from OpenGL.GL import glClear, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, glRotatef, glTranslatef, glLoadIdentity, GL_PROJECTION


class CelestialSimulation:
    def __init__(self,
                 celestials: list[Astronomy.Star | Astronomy.Planet],
                 dt: Scalar = Constants.HOUR,
                 name: str = "Astronomical Simulation") -> None:
        self.dt: Scalar = dt
        self.time: int = 0
        self.celestials: list[Astronomy.Star | Astronomy.Planet] = celestials
        self.name: str = name

        self._time_stopped: bool = False

        self._button_state: bool = False  # Is the left click being held down
        self._drag: tuple[float, float] = (0.0, 0.0)  # How much was the mouse dragged
        self._mouse_pos: tuple[float, float] = None

        self.graphics: Optional[CelestialWindow] = None

    def initialize_graphics(self) -> None:
        self.graphics = CelestialWindow(
            objects=self.celestials,
            width=800,
            height=600,
            background_color=(0.1, 0.1, 0.1, 1.0),
            title=self.name
        )

        set_key_callback(self.graphics.window, self._movement)
        set_mouse_button_callback(self.graphics.window, self._rotation)
        set_cursor_pos_callback(self.graphics.window, self._drag_cursor)
        set_scroll_callback(self.graphics.window, self._zoom)

    def calculate(self) -> None:
        # Calculate the forces applied to each object
        forces: list[Vector] = [Constants.NULL_VECTOR] * len(self.celestials)
        for a in range(len(self.celestials)):
            for b in range(len(self.celestials)):
                if a != b:
                    forces[a] += self.celestials[a].gravitational_force(self.celestials[b].point_body)
        # Move the object based on the force applied to it and its initial velocity
        for i, planet in enumerate(self.celestials):
            planet.advance(self.dt)
            planet.set_velocity(planet.velocity() + planet.acceleration(forces[i]).mul(self.dt))

    def _movement(self, _, key, __, action, ___) -> None:
        speed: float = 0.1

        if key == KEY_W and (action == PRESS or action == REPEAT):  # Forward
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0],
                self.graphics.camera_pos[1],
                self.graphics.camera_pos[2] + speed
            )
        if key == KEY_S and (action == PRESS or action == REPEAT):  # Backward
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0],
                self.graphics.camera_pos[1],
                self.graphics.camera_pos[2] - speed
            )
        if key == KEY_D and (action == PRESS or action == REPEAT):  # Left
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0] - speed,
                self.graphics.camera_pos[1],
                self.graphics.camera_pos[2]
            )
        if key == KEY_A and (action == PRESS or action == REPEAT):  # Right
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0] + speed,
                self.graphics.camera_pos[1],
                self.graphics.camera_pos[2]
            )
        if key == KEY_SPACE and (action == PRESS or action == REPEAT):  # Up
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0],
                self.graphics.camera_pos[1] - speed,
                self.graphics.camera_pos[2]
            )
        if key == KEY_LEFT_SHIFT and (action == PRESS or action == REPEAT):  # Down
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0],
                self.graphics.camera_pos[1] + speed,
                self.graphics.camera_pos[2]
            )
        if key == KEY_TAB and action == PRESS:  # Toggle time
            self._time_stopped = not self._time_stopped

    def _rotation(self, _, button, action, __) -> None:
        if button == MOUSE_BUTTON_1 and action == PRESS:  # Save the mouse pos for the _drag() method to calculate with
            self._button_state = True
            self._mouse_pos = get_cursor_pos(self.graphics.window)
            self._drag = (0.0, 0.0)
        elif button == MOUSE_BUTTON_1 and action == RELEASE:  # Reset the mouse pos
            self._button_state = False
            self._mouse_pos = None
            self._drag = (0.0, 0.0)

    def _zoom(self, _, __, y) -> None:
        c: float = 0.2
        if y < 0:  # Zoom in
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0],
                self.graphics.camera_pos[1],
                self.graphics.camera_pos[2] - c
            )
        elif y > 0:  # Zoom out
            self.graphics.camera_pos = (
                self.graphics.camera_pos[0],
                self.graphics.camera_pos[1],
                self.graphics.camera_pos[2] + c
            )

    def _drag_cursor(self, _, x, y) -> None:
        if self._button_state and self._mouse_pos:  # Calculate how much the mouse was dragged
            dx: float = x - self._mouse_pos[0]
            dy: float = y - self._mouse_pos[1]
            self._drag = (dx, dy)

    def run(self) -> None:
        while not window_should_close(self.graphics.window):
            if not self._time_stopped:
                self.time += self.dt.value
                self.calculate()

            glLoadIdentity()  # Reset the screen
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)  # Clear buffers

            if self._button_state:  # Change camera rotation
                self.graphics.camera_rot = (
                    self.graphics.camera_rot[0] + self._drag[1] / 500,
                    self.graphics.camera_rot[1] + self._drag[0] / 500, 0.0
                )

            # Rotate and move the camera
            glRotatef(self.graphics.camera_rot[0], 1.0, 0.0, 0.0)
            glRotatef(self.graphics.camera_rot[1], 0.0, 1.0, 0.0)
            glRotatef(self.graphics.camera_rot[2], 0.0, 0.0, 1.0)
            glTranslatef(*self.graphics.camera_pos)

            # TODO:
            self.graphics.load_background_image()
            self.graphics.draw_objects()  # Draw the object

            swap_buffers(self.graphics.window)  # Swap front and back buffers
            poll_events()  # React to events (like button presses)
        terminate()  # Terminate GLFW