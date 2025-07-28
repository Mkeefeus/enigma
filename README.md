# enigma

This project is an excuse to try new language(s). The goal is to make the german "Enigma" encryption machine.

## Properties of Enigma

### Enigma I

-   3 out of 5 rotors
-   Single static reflector
-   Plugboard A-Z
-   Up to 10 plugs

### Enigma I Wiring

| Rotor |          Alphabet          | Notch | Turnover |  #  |
| :---: | :------------------------: | :---: | :------: | :-: |
|  ETW  | ABCDEFGHIJKLMNOPQRSTUVWXYZ |       |          |     |
|   I   | EKMFLGDQVZNTOWYHXUSPAIBRCJ |   Y   |    Q     |  1  |
|  II   | AJDKSIRUXBLHWTMCQGZNPYFVOE |   M   |    E     |  1  |
|  III  | BDFHJLCPRTXVZNYEIWGAKMUSQO |   D   |    V     |  1  |
|  IV   | ESOVPZJAYQUIRHXLNFTGKDCMWB |   R   |    J     |  1  |
|   V   | VZBRGITYUPSDNHLXAWMJQOFECK |   H   |    Z     |  1  |
| UKW-A | EJMZALYXVBWFCRQUONTSPIKHGD |       |          |     |
| UKW-B | YRUHQSLDPXNGOKMIEBFZCWVJAT |       |          |     |
| UKW-C | FVPJIAOYEDRZXWGCTKUQSBNMHL |       |          |     |

### Application workflow
1. Prompt user for version of Engima to use
2. Prompt user for selection or rotors to use
3. Prompt user for rotor settings
4. Prompt user for plugboard settings
5. Await keypress.
6. Plugboard mutation
7. Rotor mutations
8. Reflector mutation
9. Return rotor mutations
10. Return plugboard mutation

### TODO
- Structs for rotors and plugboard
- Select rotors in order. Rotor 3 can be before rotor 1 in the machine