int max(int a, int b) {
    int x = 12 + 5 - 9 * 5 / (3+2);
    if (a > b) {
        return a;
    }
    return b;
}

int suma_arreglo(int *nums, int n) {
    int total = 0;
    int i = 0;

    while (i < n) {
        total = total + nums[i];
        i++;
    }
    return total;
}

int clasificar_nota(int puntaje) {
    switch (puntaje / 10) {
        case 10:
        case 9:
            return 1;
        case 8:
        case 7:
            return 2;
        default:
            return 0;
    }
}

int main() {
    int numeros[5] = {10, 25, 3, 42, 17};
    int i = 0;
    int mayor = numeros[0];
    int pares = 0;
    float promedio = 0.0;
    const char *titulo = "Estadisticas";

    for (i = 0; i < 5; i++) {
        if (numeros[i] % 2 == 0) {
            pares++;
        } else {
            mayor = max(mayor, numeros[i]);
        }
    }

    promedio = (float) suma_arreglo(numeros, 5) / 5.0;

    do {
        if (promedio >= 20.0) {
            break;
        }
        promedio = promedio + 1.0;
    } while (promedio < 20.0);

    i = clasificar_nota((int) promedio);
    return pares > 2 ? 1 : mayor != 0;
}
