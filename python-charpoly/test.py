import sympy as sp
import re

def test(lines):
    matrices = []
    it = iter(lines)
    for line in it:
        match = re.match(r"Matrix\((\d+)\)", line)
        if match:
            size = int(match.group(1))
            matrix = [[int(i) for i in next(it, None).split()] for _ in range(size)]

            next(it, None)
            coeffs_rust = [int(i) for i in next(it, None).strip("[]").split(", ")]
            x = sp.Symbol('x')
            poly_rust = sum(c * x**i for i, c in enumerate(coeffs_rust))

            M = sp.Matrix(matrix)
            poly_sympy = M.charpoly(x).as_expr()

            difference = sp.simplify(poly_rust - poly_sympy)

            print(f"Matrix({size}):")
            print(f"  polynomial_rust: {coeffs_rust}")
            print(f"  polynomial_sympy: {M.charpoly(x).as_expr()}")
            print(f"  the difference between the two above: {difference}")

lines = ""
with open("data.txt", "r", encoding="utf-8") as f:
    lines = [line.strip() for line in f]

test(lines)
