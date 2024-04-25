// CREAR ARCHIVO ejemplo.cpp
#include <functional>
#include <iostream>

class Laboratorio {
  int num;
};

class Practica {
  int a;
  Laboratorio lab;

public:
  operator Laboratorio() { return lab; }

  operator int() { return a; }
};

void function(int a) { std::cout << "funcion (int) ejecutada"; }

void function(Laboratorio la) {
  std::cout << "Funcion (Laboratorio) ejecutada";
}

int main() {
  Practica p;
  function((Laboratorio)p);
  return 0;
}
