import setuptools
import shutil

with open("README.md", "r") as fh:
    long_description = fh.read()

shutil.copyfile('target/release/libpyengine.so','pyengine.so')

setuptools.setup(
    name='pyengine',
    version='0.1',
    scripts=[],
    author="Desmond Germans",
    author_email="desmond@germansmedia.nl",
    description="Retrofuturistic Devpunk Game Engine with Tools.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/des256/e",
    packages=['.'],
    package_dir={'': '.'},
    package_data={'': ['pyengine.so']},
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
)
