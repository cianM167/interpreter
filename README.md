## Intstructions:
To use simply download the binary run the install script and type:
`
interpreter {filename}
`
In your terminal.

## Examples:
The interpreter uses a python like syntax
Below are a couple of examples.
### Sample program:
This program:
```
print("Hello World!")
let a = 400
let b = 30.12
let c = a + b
print("C = ", c)
```
Will display this:
```
Hello World!
C = 430.12
```
### Parsing:
The interpreter can parse full equation respecting order of operations:
```
print(10 + 10 * 20)
```
Will display:
```
210
```
And:
```
print((10 + 10) * 20)
```
Will display:
```
400
```
### Boolean eval:
Boolean evaluation:
```
print(10 > 5)
print(1 == 1)
print(10 < 5)
```
Will display:
```
true
true
false
```

