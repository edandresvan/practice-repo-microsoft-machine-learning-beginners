# Practice Repository: Microsoft Machine Learning for Beginners

This is a repository for practicing activities from the [Microsoft Machine Learning for Beginners online course](https://github.com/microsoft/ML-For-Beginners).

## Installation Setup on Ubuntu Linux

In this repository, each machine learning project is being developed as a Rust project in order to practice the Python version in Jupyter Notebooks and the Rust version as an executable. The goal is to understand the machine learning concepts by using Python frameworks, and then explore a possible implementation by using Rust tools.

For this reason, it is necessary to install both Python and Rust libraries. The most important libraries are the following:

| Python                                  |Rust                                                       |
|-----------------------------------------|-----------------------------------------------------------|
|[Pandas](https://pandas.pydata.org/)     |[Polars](https://www.pola.rs/)                             |
|[Polars](https://www.pola.rs/)           |[Plotly](https://github.com/igiagkiozis/plotly)            |
|[Matplotlib](https://matplotlib.org/)    |[maud](https://github.com/lambda-fairy/maud)                |
|[scikit-learn](https://scikit-learn.org/)|[mimalloc](https://github.com/purpleprotocol/mimalloc_rust)|
|[Jupyter](https://jupyter.org/)          |                                                           |
|[yapf](https://github.com/google/yapf)   |                                                           |
|[pyenv](https://github.com/pyenv/pyenv)  |                                                           |

The following installation steps are for **Linux Ubuntu 23.04**.

### Python Installation

To keep development simple, [pyenv](https://github.com/pyenv/pyenv) is used to create a shared virtual environment for all projects and [pip-tools](https://github.com/jazzband/pip-tools) to install libraries.

#### Install pyenv:

Install dependencies:

```shell
$ sudo aptitude update; sudo apt install curl build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev curl libncursesw5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev -y;
```

Install `pyenv`:

```shell
$ curl https://pyenv.run | bash;
```

Then add these lines to your `.bashrc` file:

```shell
export PATH="$HOME/.pyenv/bin:$PATH"
eval "$(pyenv init --path)"
eval "$(pyenv virtualenv-init -)"
```

Verify `pyenv` is working:
    
```shell
$ pyenv --help
```

#### Install a Python version

```shell
$ pyenv install 3.11.2;
``` 

#### Create the Python Virtual Environment

Move to your working directory where you will develop the projects:

```shell
$ pyenv local 3.11.2;
$ pyenv virtualenv microsoft-machine-learning-beginners;
$ pyenv local 3.11.2/envs/microsoft-machine-learning-beginners;
$ pyenv activate 3.11.2/envs/microsoft-machine-learning-beginners;
```

#### Install Python Dependencies

Install `pip-tools` and then the project dependencies.

```shell
$ pip install --upgrade pip;
$ python -m pip install pip-tools --upgrade;
$ pip-compile requirements.in --upgrade;
$ pip install -r requirements.txt;
```

## Rust Installation

### Install Rust Toolchain

Install the Rust toolchain as [usual](https://www.rust-lang.org/tools/install):

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs| sh
```

