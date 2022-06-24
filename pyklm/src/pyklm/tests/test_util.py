from pyklm.util import byteargs

@byteargs
def example(number: int):
    pass

def test_byteargs_positive():
    example(16)

def test_byteargs_positive():
    try:
        example(256)
        return False
    except ValueError:
        return True
    return False
