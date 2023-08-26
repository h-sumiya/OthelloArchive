import torch
from tools import *
from Network import OseroNetworks


data = bytes([0, 0, 0, 4, 0, 100, 0, 100])
rsb = RustBuilder()
rsb.add("TEST", data)
rsb.add("TEST2", data)
rsb.add("TEST3", data)
import pyperclip
pyperclip.copy(rsb.build())

