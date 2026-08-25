from setuptools import setup, find_packages

setup(
    name="acersense-linux",
    version="1.0.0",
    description="Open Source Hardware Control Suite for Acer Nitro, Predator, and Aspire Laptops on Linux",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    author="Rodrigo",
    url="https://github.com/rodrigo47363/acersense-linux",
    license="GPL-3.0",
    packages=find_packages(),
    scripts=[
        "bin/acersense",
        "bin/acersense-gui",
        "bin/acersense-daemon"
    ],
    install_requires=[
        "psutil>=5.9.0",
        "customtkinter>=5.2.0"
    ],
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: End Users/Desktop",
        "License :: OSI Approved :: GNU General Public License v3 (GPLv3)",
        "Operating System :: POSIX :: Linux",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Topic :: System :: Hardware",
    ],
    python_requires=">=3.8",
)
