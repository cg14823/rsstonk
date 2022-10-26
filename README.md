# RSSTONK

:warning: This is made as a tool to learn rust. As such the code quality is LOW
bordering a crime against all rustaceans. 

![Example screenshot](/img/example_screenshot.png)

This is command line tool to print the stock value of a given symbol.

## Building

Build the tool using cargo

```
> cargo build
```

## Using

The tool uses [finnhub.io](https://finnhub.io/) to retrieve stock data. Go there and get an API
and store it in an environment variable `FINNHUB_API_KEY`. The tool will read it to do the requests.

Run the tool with

```
rrstonk -- <SYMBOL>
```

