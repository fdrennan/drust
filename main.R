# TODO R Execute LR
rm(list = ls())
library(readr)
library(dplyr)
library(purrr)
library(rustr)
library(cli)
# mtcars <- map_dfr(1:100, ~ as_tibble(mtcars))
# mtcars <- map_dfr(1:30, ~ as_tibble(mtcars))
df <- data.frame(
  x = c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
  y = c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
)
df <- mutate_all(df, as.integer)
results <- execute_lr(df, "x")

# results <- execute_lr(iris, 'am')
