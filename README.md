<h1 align="center">mop</h1>
<p align="center"><code>mop</code> is a cross-platform command-line tool designed to clean up those messy CSV column names.</p>


<p align="center">
<a href="https://fontmeme.com/pixel-fonts/"><img src="https://fontmeme.com/permalink/241017/596862d73d541b821f564550155bfc68.png" alt="pixel-fonts" border="0"></a>
</p>

# Features

* Clean CSV column names automatically.
* Convert special characters to ASCII equivalents.
* Replace spaces with underscores.
* Ensure unique column names by appending counters to duplicates.
* Works with both files and piped input.
* Add default names (x, x_2, etc.) for missing headers or extra columns.

# Installation

You can install mop using cargo:
```sh
cargo install mop
```

Alternatively, you can clone the repository and build it yourself:
```sh
git clone https://github.com/alexhallam/mop.git
cd mop
cargo build --release
```


# Usage

`mop` can be used by providing a file or by piping a CSV file to it.


# Basic Usage
Clean the column names of a CSV file and output to another file:

```sh
mop data.csv > cleaned_data.csv
```

Piping Input
You can also pipe input to mop:

```sh
cat data.csv | mop > cleaned_data.csv
```

# Examples
Consider a CSV file data.csv with the following content:

```csv
Name, Age,   City!, Amount $$
John, 23,   New York, 100
Mary, 30,   Los Angeles, 200
```

```sh
> mop data.csv
name,age,city,amount
John, 23,   New York, 100
Mary, 30,   Los Angeles, 200
```

If there are more columns of data than headers:

```csv
a,b
1,2,3
4,5,6
```

The cleaned output would be:

```sh
> mop data.csv
a,b,x_1
1,2,3
4,5,6
```

If there are duplicate column names:

```csv
beetlejuice, beetlejuice, beetlejuice
1,2,3
```

```sh
> mop data.csv
beetlejuice,beetlejuice_2,beetlejuice_3
1,2,3
```

# Pair with tidy-viewer (tv) 

given the following data.csv file:

```csv
,First Name,Total Amount $,Füße,,Name,Name
1,Alice,100,Data0,Data1,Alice1,Alice2
2,Bob,200,Data0,Data2,Bob1,Bob2
```

run `mop` and `tv` together:

```sh
> cat data.csv | mop | tidy-viewer

        tv dim: 2 x 7
        x_1 first_name total_amount m_nchen x_2   name   name_2 
     1  1   Alice      100          Data0   Data1 Alice1 Alice2
     2  2   Bob        200          Data0   Data2 Bob1   Bob2
```


# Command-Line Arguments

Get help with `map --help`

```

✨🧹✨ mop is a command-line tool that reads a CSV file, cleans and standardizes the column names, and outputs the modified CSV to stdout.

It removes special characters, transliterates Unicode characters to ASCII, replaces spaces with underscores, and ensures that all column names are unique.

Usage: mop.exe [FILE]

Arguments:
  [FILE]
          CSV file to process (reads from stdin if not provided)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

EXAMPLES:

  mop data.csv > cleaned_data.csv

  mop data.csv | some_other_command

  cat data.csv | mop - > cleaned_data.csv
```

# Inspiration

[clean_names()](https://github.com/sfirke/janitor/blob/main/R/clean_names.R) from janitor by Sam Firke.
