# Example script
# Sets keyboard color to blue
from pyklm.connection import KLMConnection
from pyklm.rgb import RGB

# Create connection
connection = KLMConnection()
# Stage command
connection.set_color(RGB(0, 0, 255))
# Send command to daemon
connection.commit()
