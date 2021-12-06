# Rustfolio

Rustfolio is a program for monitoring your investment portfolio. It's main purpose is to show a summary of all your assets and their share in the entire portfolio. The portfolio data is stored in a `.csv` file, so it can be imported into any spreadsheet processor like Libre Office Calc or Microsoft Exel for more complicated processing and analisys. The files are stored in `~/.local/share/rustfolio` directory.

![Showcase](screenshots/showcase.png)

## Example usage

### Show the portfolio "MyPortfolio"

```sh
rustfolio --file MyPortfolio
```

`--file` flag can be omitted. In this case the program will promt to select the portfolio that exists.
To see the entire history of the portfolio instead of the latest entry, add `--table` flag

```sh
rustfolio --file MyPortfolio --table
```

### Create a new portfolio

```sh
rustfolio new MyPortfolio
```

If the portfolio name is not specified, the program will ask you to enter it.

### Modify a portfolio

```sh
rustfolio add --file MyPortfolio
```

The program will promt you to provide data for each asset category in the portfolio and will save it in the specified file. If `--file` is omitted, it will promt to select an existing portfolio. If none exists, it will promt to create a new one.

Every time you `add` to an existing portfolio, this data will be added to the portfolio .csv file with the current date and time. This way, you will have a history of your portfolio

### List available portfolios

```sh
rustfolio list
```

### Export a portfolio as a .csv file

```sh
rustfolio export -o output.csv -p MyPortfolio
```

Flag `-p` can be omitted. In this case you will be promted to select one of the available portfolios
