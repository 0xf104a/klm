def byteargs(func):
    """
     Wrapper that ensures that all int arguments of
     a function are in byte range(0-255)
    """
    def wrapper(*args):
        for arg in args:
            if isinstance(arg, int):
                if arg < 0 or arg > 255:
                    raise ValueError("Argument value %d is out of range" % arg)
        return func(*args)
    return wrapper



