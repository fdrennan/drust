# TODO R Execute LR
rm(list = ls())
library(readr)
library(dplyr)
library(purrr)
library(rustr)
library(cli)

df <- data.frame(
  x = round(rnorm(1000, 0, 1), 4) * 1000
) %>% mutate(y = x * 2)

df <- mutate_all(df, as.integer)

results <- execute_lr(df, "x")
