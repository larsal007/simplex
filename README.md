# Simplex <!-- omit from toc -->

![Latest Release](https://img.shields.io/github/v/release/larsal007/simplex)
[![Build Status](https://travis-ci.com/larsal007/simplex.svg?branch=master)](https://travis-ci.com/larsal007/simplex)

:rocket: Fast regex CLI for multiple local and remote sources

## Table of Content <!-- omit from toc -->

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)

## Introduction

Simplex is a fast and powerful command-line tool written in Rust that allows you to perform regular expression searches on multiple local and remote sources. It provides a convenient way to search for patterns in text files, URLs, and source files of URLs. With Simplex, you can easily count the number of matches, list the files that match the pattern, print line numbers of matches, and more.

## Installation

To install Simplex, follow the steps below:

1. Ensure you have Rust installed. If not, you can install it by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Open a terminal or command prompt.

3. Run the following command to install Simplex:

   ```shell
   cargo install simplex
   ```

Once the installation is complete, you can start using Simplex right away!

## Usage

```shell
Regex, a fast command line tool written in Rust

Usage: simplex [OPTIONS] --regex <PATTERN>

Options:
  -h, --help     Print help
  -V, --version  Print version

SEARCH:
  -r, --regex <PATTERN>  Sets the regex pattern to search for

Inputs:
  -f, --file <FILE>     Sets the text file to search in
  -u, --url <URL>       Sets the URL to use
  -s, --urlfile <FILE>  Sets the source file of URLs to use

Flags:
  -i, --inverse     The regex is used as exclusion pattern
  -c, --count       Count the number of matches
  -l, --list        List the files that match the pattern
  -n, --linenumber  Print the line number of the match
  -v, --verbose     Print test information verbosely

Output:
  -D, --delimiter <DELIM>  Sets the delimiter to use [default: ,]
  -S, --separator <SEP>    Sets the separator to use [default: :]
  -N, --newline <NEWLINE>  Sets the newline character to use
  -R, --replace <PATTERN>  Sets the replacement pattern to use
```
