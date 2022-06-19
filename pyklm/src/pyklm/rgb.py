from pyklm.util import byteargs

class RGB:
    """
     Stores RGB color compatiable with klmd
    """
    @byteargs
    def __init__(self, r: int, g: int, b: int):
        """
         Initializes RGB color.

         :param r: Red color itensity(0-255)
         :param g: Green color itensity(0-255)
         :param b: Blue color itensit(0-255)
        """
        self.r = r
        self.g = g
        self.b = b

    def to_bytearray(self) -> bytearray:
        """
         Converts RGB color to byte sequence of color itensties

         :return bytearray: byte sequence of color
        """
        return bytearray([self.r, self.g, self.b])

