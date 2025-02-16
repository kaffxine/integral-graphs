import matplotlib.pyplot as plt
import sympy as sp
import re
import time

def test(lines):
    matrices = []
    data_rust_time = []
    data_python_time = []
    data_n_nodes = []
    data_rust_time_parametric = []
    data_python_time_parametric = []
    data_n_nodes_parametric = []
    it = iter(lines)
    for line in it:
        match = re.match(r"Matrix\((\d+)\)", line)
        if match:
            size = int(match.group(1))
            matrix = [[int(i) for i in next(it, None).split()] for _ in range(size)]

            next(it, None)
            duration_rust = int(next(it, None))
            coeffs_rust = [int(i) for i in next(it, None).strip("[]").split(", ")]
            x = sp.Symbol('x')
            poly_rust = sum(c * x**i for i, c in enumerate(coeffs_rust))

            M = sp.Matrix(matrix)
            time_start = time.time_ns()
            poly_sympy = M.charpoly(x).as_expr()
            time_end = time.time_ns()
            duration = time_end - time_start 
    
            difference = sp.simplify(poly_rust - poly_sympy)

            print(f"Matrix({size}):")
            print(f"  polynomial_rust: {coeffs_rust}")
            print(f"  polynomial_sympy: {M.charpoly(x).as_expr()}")
            print(f"  the difference between the two above: {difference}")
            print(f"  duration: {duration}")
            print(f"  duration_rust: {duration_rust}")
            print(f"  duration_rust / duration: {duration_rust / duration}")

            data_n_nodes.append(size)
            data_python_time.append(duration)
            data_rust_time.append(duration_rust)
            if size % 2 == 0 and size / 2 in data_n_nodes:
                i_whole = data_n_nodes.index(size)
                i_halved = data_n_nodes.index(size / 2)
                ratio_python = data_python_time[i_whole] / data_python_time[i_halved]
                ratio_rust = data_rust_time[i_whole] / data_rust_time[i_halved]

                data_n_nodes_parametric.append(size)
                data_python_time_parametric.append(ratio_python)
                data_rust_time_parametric.append(ratio_rust)

    file_names = ("execution_time.png", "execution_time_parametric.png")
    y_labels = (
        (
            "czas wykonywania funkcji spectral::characteristic_polynomial()",
            "czas wykonywania funkcji charpoly z biblioteki sympy"
        ),
        (
            "stosunek czasów r(n) funkcji spectral::characteristic_polynomial()",
            "stosunek czasów r(n) funkcji charpoly z biblioteki sympy"
        )
    )
    x_values = (data_n_nodes, data_n_nodes_parametric)
    y_values = (
        (data_rust_time, data_python_time),
        (data_rust_time_parametric, data_python_time_parametric)
    )

    for i in range(2):
        fix, ax = plt.subplots()
        ax.plot(x_values[i], y_values[i][0], marker='o', color='r', label=y_labels[i][0])
        ax.plot(x_values[i], y_values[i][1], marker='o', color='b', label=y_labels[i][1])
        ax.set_xticks([x for x in x_values[i] if x % 10 == 0])
        ax.legend()
        plt.savefig(file_names[i], dpi=300, bbox_inches='tight')


lines = ""
with open("complete.txt", "r", encoding="utf-8") as f:
    lines = [line.strip() for line in f]

test(lines)
