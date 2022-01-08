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

mtcars$am <- as.integer(mtcars$am)
results <- execute_lr(mtcars, "am")
return_df(mtcars)
