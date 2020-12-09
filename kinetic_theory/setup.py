import sys
from setuptools import setup
import os
import shutil
import pathlib

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call(
        [sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import Binding, RustExtension

setup_requires = ["setuptools-rust>=0.10.6"]
install_requires = []

package_name = "particles"

python_version_str = ".".join(map(str, sys.version_info[0:2]))

build_path = "build/lib.win-amd64-" + python_version_str + "/" + package_name
module_path = package_name + "/"

if not os.path.exists(module_path):
    os.makedirs(module_path)

setup(name=package_name,
      version="0.1.0",
      author="Lars Kuehmichel",
      description="A Particle Simulation Package",
      rust_extensions=[RustExtension(f"{package_name}.{package_name}",
                                     debug=True,
                                     native=False,
                                     args=["--verbose"])],
      packages=[package_name],
      zip_safe=False)


# copy and overwrite the .pyd into the interpreter directory for testing
shutil.rmtree(module_path)
shutil.copytree(build_path, module_path)
with open(module_path + "__init__.py", "w+") as f:
    f.write(f"from .{package_name} import *\n")

