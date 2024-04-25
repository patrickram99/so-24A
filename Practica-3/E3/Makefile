mensaje: mensaje.o salida_alt.o
	gcc -o mensaje mensaje.o salida_alt.o

mensaje.o: mensaje.c salida_alt.h
	gcc -c -g mensaje.c

salida_alt.o: salida_alt.c salida_alt.h
	gcc -c -g salida_alt.c

