<h1 align="center">mop</h1>
<p align="center"><code>mop</code> is a cross-platform command-line tool designed to clean up those messy CSV column names.</p>



# Features

* Clean CSV column names automatically.
* Convert special characters to ASCII equivalents.
* Replace spaces with underscores.
* Ensure unique column names by appending counters to duplicates.
* Works with both files and piped input.
* Add default names (x_1, x_2, etc.) for missing headers or extra columns.

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






Problems Addressed:

Inconsistent Naming Conventions:

Issue: Datasets often come with column names that are inconsistent in terms of capitalization, spacing, and special characters. For example, you might encounter "First Name", "lastName", "AGE", and "e-mail Address" all in the same dataset.
Solution: The tool standardizes all column names to lowercase and replaces spaces and special characters with underscores. This creates a consistent naming convention throughout the dataset.
Special Characters and Spaces:

Issue: Column names containing spaces, punctuation, or special symbols (e.g., "Total Amount $", "User@ID", "Address (Home)") can cause issues in programming environments and database queries.
Solution: The tool removes or replaces special characters with underscores, ensuring that column names are safe to use in code and comply with most programming language variable naming rules.
Non-ASCII Characters and Accents:

Issue: Names with accented characters or non-ASCII symbols (e.g., "München", "São Paulo", "naïve café") can lead to encoding issues and errors in systems that do not support Unicode.
Solution: The tool transliterates or removes accents, converting characters to their ASCII equivalents (e.g., "Munchen", "Sao_Paulo", "naive_cafe") to enhance compatibility.
Leading and Trailing Underscores:

Issue: After cleaning, some column names might end up with leading or trailing underscores (e.g., "name", "id"), which can be unsightly or problematic in certain contexts.
Solution: The tool trims any leading or trailing underscores, resulting in cleaner and more professional-looking column names.
Empty or Invalid Column Names:

Issue: Datasets may contain columns with empty names or names that become empty after cleaning (e.g., columns named " " or consisting solely of special characters).
Solution: The tool replaces empty or invalid names with a default placeholder (e.g., "x") to ensure that every column has a valid and accessible name.
Duplicate Column Names:

Issue: Datasets can have columns with identical names, especially after cleaning (e.g., multiple columns named "total" after removing special characters).
Solution: The tool appends a numeric suffix to duplicate names to ensure uniqueness (e.g., "total", "total_2", "total_3"), preventing overwriting and confusion during data processing.
Columns Starting with Numbers:

Issue: Column names that start with numbers (e.g., "123data") can be invalid in many programming languages and databases, which often require names to start with a letter or underscore.
Solution: The tool can be extended to prepend an underscore or alter the name to comply with naming conventions (e.g., "_123data" or "data_123").
Inconsistent Spacing and Delimiters:

Issue: Variations in spacing and the use of different delimiters (e.g., hyphens, periods, slashes) can lead to inconsistent naming and difficulty in accessing columns programmatically.
Solution: The tool replaces all types of delimiters and multiple consecutive non-alphanumeric characters with a single underscore, standardizing the separation between words.
Case Sensitivity Issues:

Issue: Case-sensitive environments can treat "Name" and "name" as distinct columns, leading to unexpected behaviors.
Solution: By converting all names to lowercase, the tool eliminates case sensitivity issues, ensuring that columns are recognized uniformly across different systems.

// Goals
1. Empty names replaced with x_1, x_2, x_3, ...
2. resulting names are unique (no duplicate names)
3. resulting names consist only of letters, digits, and underscores
4. accented characters are replaced with their ascii equivalent. o umlaut -> o and enye -> n
5. defaults to lower snake_case, but can be changed to camelCase or PascalCase
6. A parsing option with the inputs of 1,2,3 will be allowed for the following
    1. "MYMop" -> "MY_Mop")(default) snake_case: my_mop, camelCase: myMop, PascalCase: MyMop
    2. "MYMop" -> "MYM_op" snake_case: mym_op, camelCase: mymOp, PascalCase: MymOp
    2. "MYMop" -> "MYMop" snake_case: mymop, camelCase: mymop, PascalCase: Mymop
7. users can input numeral-position. so when there are duplicate names the number would be on the left or right of the name. 
    1. "x" -> "x_1" or "1_x"
    2. "x" -> "x_2" or "2_x"
    3. "x" -> "x_3" or "3_x"
8. user can write csv to file, user can print csv to stdout, printing to stdout is default
