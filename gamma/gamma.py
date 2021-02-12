from yfinance import Ticker
from enum import Enum
import pandas as pd


pd.set_option('display.max_columns', None)

symbol = "GOOG"

class OptionParts(Enum):
    contractSymbol = 0
    lastTradeDate = 1
    strike = 2
    lastPrice = 3
    bid = 4
    ask = 5
    change = 6
    percentChange = 7
    volume = 8
    openInterest = 9
    impliedVolatility = 10
    inTheMoney = 11
    contractSize = 12
    currency = 13


stock = Ticker(symbol)

print(goog.option_chain())
