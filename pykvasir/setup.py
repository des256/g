import setuptools
import shutil

with open("README.md", "r") as fh:
    long_description = fh.read()

shutil.copyfile('target/release/libpysteel.so','pysteel.so')

setuptools.setup(
    name='pysteel',
    version='0.1',
    scripts=[],
    author="Desmond Germans",
    author_email="desmond@germansmedia.nl",
    description="Game Engine with Tools.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/des256/steel",
    packages=['.'],
    package_dir={'': '.'},
    package_data={'': ['pysteel.so']},
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
