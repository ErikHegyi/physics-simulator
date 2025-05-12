from typing import Optional
import math
import glfw
from OpenGL.GL import *
from OpenGL.GLU import *
from OpenGL.GLUT import *
from pathlib import Path
from PIL import Image
import numpy


class Window:
    def __init__(self,
                 width: int,
                 height: int,
                 background_color: tuple[float, float, float, float],
                 title: Optional[str] = "OpenGL Window",
                 fov: int = 45,
                 camera_location: tuple[float, float, float] = (0.0, 0.0, -5.0),
                 background_image: Optional[Path] = None) -> None:
        self._width: int = width
        self._height: int = height
        self._fov: int = fov
        self._title: str = title
        self._bg_color: tuple[float, float, float, float] = background_color
        self._camera_pos: tuple[float, float, float] = camera_location
        self._camera_rot: tuple[float, float, float] = (0.0, 0.0, 0.0)
        self._background_image: Optional[Path] = background_image

        # Create the window
        if not glfw.init():
            raise Error("Something went wrong while creating the window.")

        self._window = glfw.create_window(self._width, self._height, self._title, None, None)
        if not self._window:
            glfw.terminate()
            raise Error("Something went wrong while creating the window.")

        glfw.set_window_size_callback(self._window, self.window_size_callback)

        glfw.make_context_current(self._window)

        if self._background_image:
            self._bg_texture_id = self.load_texture(self._background_image)
        else:
            self._bg_texture_id = None

        # Initialize OpenGL
        glClearColor(*self._bg_color)
        glEnable(GL_DEPTH_TEST)
        glMatrixMode(GL_PROJECTION)
        glLoadIdentity()
        gluPerspective(self._fov, (self._width / self._height), 0.1, 50.0)
        glMatrixMode(GL_MODELVIEW)
        glLoadIdentity()
        glTranslatef(*self._camera_pos)  # original: 0.0, 0.0, -5.0

        # Initialize GLUT
        glutInit()


    # TODO: Load a background image properly
    def load_background_image(self) -> None:
        if not self._bg_texture_id:
            return

        glDisable(GL_DEPTH_TEST)
        glDepthMask(GL_FALSE)

        glMatrixMode(GL_PROJECTION)  # Ensure you're in projection mode for ortho
        glPushMatrix()
        glLoadIdentity()
        glOrtho(-1, 1, -1, 1, -1, 1)

        glMatrixMode(GL_MODELVIEW)  # Switch back to modelview for drawing
        glPushMatrix()
        glLoadIdentity()

        glEnable(GL_TEXTURE_2D)
        glBindTexture(GL_TEXTURE_2D, self._bg_texture_id)

        glBegin(GL_QUADS)
        glTexCoord2f(0, 0)
        glVertex2f(-1, -1)
        glTexCoord2f(1, 0)
        glVertex2f(1, -1)
        glTexCoord2f(1, 1)
        glVertex2f(1, 1)
        glTexCoord2f(0, 1)
        glVertex2f(-1, 1)
        glEnd()

        glDisable(GL_TEXTURE_2D)

        glPopMatrix()  # Restore Modelview Matrix
        glMatrixMode(GL_PROJECTION)  # Switch back to Projection Matrix
        glPopMatrix()  # Restore Projection Matrix

        glLoadIdentity()
        glViewport(0, 0, self._width, self._height)
        gluPerspective(self._fov, self._width / self._height, 0.1, 50.0)

        glMatrixMode(GL_MODELVIEW)

        glEnable(GL_DEPTH_TEST)
        glDepthMask(GL_TRUE)


    @property
    def width(self) -> int:
        return self._width

    @width.setter
    def width(self, width: int) -> None:
        self._width = width
        glfw.set_window_size(self._window, self._width, self._height)

    def window_size_callback(self, _, width: int, height: int) -> None:
        glViewport(0, 0, width, height)
        glMatrixMode(GL_PROJECTION)
        glLoadIdentity()
        gluPerspective(self._fov, (width / height), 0.1, 50.0)
        glMatrixMode(GL_MODELVIEW)
        self._width = width
        self._height = height

    @staticmethod
    def draw_circle(coordinates: tuple[float, float],
                    radius: float,
                    color: tuple[float, float, float, float],
                    fill: bool = True,
                    subdivisions: int = 32) -> None:
        alpha: float = 2 * math.pi / subdivisions  # Calculate the angle
        glColor4f(*color)  # Set the color
        match fill:
            case True: glBegin(GL_TRIANGLE_FAN)
            case False: glBegin(GL_LINE_LOOP)

        # Draw the circle
        for i in range(subdivisions + 1):
            angle: float = alpha * i
            x: float = coordinates[0] + radius * math.cos(angle)
            y: float = coordinates[1] + radius * math.sin(angle)
            z: float = 0.0
            glVertex3f(x, y, z)
        glEnd()

    @staticmethod
    def draw_sphere(coordinates: tuple[float, float, float],
                    radius: float,
                    color: tuple[float, float, float, float],
                    subdivisions: int = 32) -> None:
        glPushMatrix()
        glTranslatef(*coordinates)
        glColor4f(*color)  # Set the color
        quad = gluNewQuadric()
        gluQuadricTexture(quad, GL_TRUE)
        gluSphere(quad, radius, subdivisions, subdivisions)  # Draw the sphere
        glPopMatrix()

    @staticmethod
    def draw_text(coordinates: tuple[float, float, float],
                  color: tuple[float, float, float, float],
                  text: str):
        glColor4f(*color)
        glRasterPos3f(*coordinates)
        for char in text:
            glutBitmapCharacter(GLUT_BITMAP_9_BY_15, ord(char))

    @staticmethod
    def load_shader(shader_type: GL_SHADER_TYPE,
                    path: Path) -> GL_SHADER:
        shader: GL_SHADER = glCreateShader(shader_type)
        glShaderSource(shader, open(path, 'r').read())
        glCompileShader(shader)

        if not glGetShaderiv(shader, GL_COMPILE_STATUS):
            info: str = glGetShaderInfoLog(shader).decode()
            raise GLerror(f"Unable to read in shader: {info}")
        return shader

    def set_program(self,
                    vertex_shader: Optional[Path] = None,
                    fragment_shader: Optional[Path] = None) -> None:
        program = glCreateProgram()  # Create the shader program

        # Attach the shaders
        if vertex_shader:
            glAttachShader(program, self.load_shader(GL_VERTEX_SHADER, vertex_shader))
        if fragment_shader:
            glAttachShader(program, self.load_shader(GL_FRAGMENT_SHADER, fragment_shader))
        glLinkProgram(program)

        # Error handling
        if not glGetProgramiv(program, GL_LINK_STATUS):
            info: str = glGetProgramInfoLog(program).decode()
            raise GLerror(f"Unable to create shader program: {info}")
        self._shader_program = program

    @staticmethod
    def load_texture(file: Path) -> GLuint:
        img: Image = Image.open(file)
        image_data = img.tobytes("raw", "RGB", 0, -1)
        width, height = img.size

        texture_id = glGenTextures(1)
        image_format = GL_RGB if img.mode == "RGB" else GL_RGBA

        glBindTexture(GL_TEXTURE_2D, texture_id)

        glPixelStorei(GL_UNPACK_ALIGNMENT, 1)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST)
        glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST)
        glTexEnvf(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE)
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, image_format, GL_UNSIGNED_BYTE, image_data)

        return texture_id


    @property
    def window(self):
        return self._window

    @property
    def camera_pos(self) -> tuple[float, float, float]:
        return self._camera_pos

    @camera_pos.setter
    def camera_pos(self,
                   value: tuple[float, float, float]) -> None:
        self._camera_pos = value

    @property
    def camera_rot(self) -> tuple[float, float, float]:
        return self._camera_rot

    @camera_rot.setter
    def camera_rot(self,
                   value: tuple[float, float, float]) -> None:
        self._camera_rot = value