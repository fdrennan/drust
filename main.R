# TODO R Execute LR
rm(list = ls())
library(readr)
library(dplyr)
library(purrr)
library(rustr)

mtcars <- map_dfr(1:100, ~ as_tibble(mtcars))
mtcars <- map_dfr(1:100, ~ as_tibble(mtcars))
mtcars <- map_dfr(1:30, ~ as_tibble(mtcars))
print(mtcars)
write_csv(mtcars, 'mtcars.csv')
results <- execute_lr('mtcars.csv', 'am')
