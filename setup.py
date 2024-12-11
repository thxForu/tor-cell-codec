from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="tor-cell-codec",
    version="0.1.0",
    rust_extensions=[
        RustExtension(
            "tor_cell_codec",
            binding=Binding.PyO3,
            debug=False
        )
    ],
    zip_safe=False,
)
