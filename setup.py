from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="bmr",
    version="1.0",
    rust_extensions=[RustExtension("bmr", binding=Binding.PyO3)],
    packages=["bmr"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)