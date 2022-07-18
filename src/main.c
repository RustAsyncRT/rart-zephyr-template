/**
 * @file main.c
 * @author Matheus T. dos Santos (matheus.santos@edge.ufal.br)
 * @brief
 * @version 0.1
 * @date 03/04/2022
 *
 * @copyright Copyright (c) 2022
 *
 */
#include "rart-defines.h"
#include <sys/printk.h>

int main(void) {
  printk("Starting Mindcore loop...\n");
  main_task();
  printk("End of Mindcore loop\n");

  return 0;
}
