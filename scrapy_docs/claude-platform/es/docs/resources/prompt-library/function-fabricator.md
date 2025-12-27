# Fabricador de funciones

Crear funciones de Python basadas en especificaciones detalladas.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Tu tarea es crear funciones de Python basadas en las solicitudes en lenguaje natural proporcionadas. Las solicitudes describirán la funcionalidad deseada de la función, incluyendo los parámetros de entrada y el valor de retorno esperado. Implementa las funciones según las especificaciones dadas, asegurándote de que manejen casos límite, realicen las validaciones necesarias y sigan las mejores prácticas para la programación en Python. Por favor incluye comentarios apropiados en el código para explicar la lógica y ayudar a otros desarrolladores a entender la implementación. |
| Usuario   | Quiero una función que pueda resolver un rompecabezas de Sudoku. La función debe tomar una cuadrícula de Sudoku de 9x9 como entrada, donde las celdas vacías están representadas por el valor 0. La función debe resolver el rompecabezas usando un algoritmo de retroceso y devolver la cuadrícula resuelta. Si el rompecabezas no se puede resolver, debe devolver None. La función también debe validar la cuadrícula de entrada para asegurar que sea un rompecabezas de Sudoku válido. |

## Ejemplo de Salida

```python Python
def solve_sudoku(grid):
   def is_valid(row, col, num):
       # Verificar si el número es válido en la fila y columna actual
       for i in range(9):
           if grid[row][i] == num or grid[i][col] == num:
               return False

       # Verificar si el número es válido en la subcuadrícula 3x3 actual
       start_row, start_col = (row // 3) * 3, (col // 3) * 3
       for i in range(3):
           for j in range(3):
               if grid[start_row + i][start_col + j] == num:
                   return False

       return True

   def backtrack(row, col):
       # Si hemos llenado todas las celdas, el rompecabezas está resuelto
       if row == 9:
           return True

       # Moverse a la siguiente fila si hemos llegado al final de la fila actual
       if col == 9:
           return backtrack(row + 1, 0)

       # Si la celda actual ya está llena, moverse a la siguiente celda
       if grid[row][col] != 0:
           return backtrack(row, col + 1)

       # Intentar llenar la celda actual con números del 1 al 9
       for num in range(1, 10):
           if is_valid(row, col, num):
               grid[row][col] = num
               if backtrack(row, col + 1):
                   return True
               grid[row][col] = 0

       # Si no se encuentra un número válido, retroceder
       return False

   # Validar la cuadrícula de entrada
   if not all(len(row) == 9 and all(0 <= cell <= 9 for cell in row) for row in grid):
       return None

   # Iniciar el algoritmo de retroceso
   if backtrack(0, 0):
       return grid
   else:
       return None

```

---

## Solicitud de API

<Tabs>
<Tab title="Python">
```python
import anthropic

client = anthropic.Anthropic(  # por defecto os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Tu tarea es crear funciones de Python basadas en las solicitudes en lenguaje natural proporcionadas. Las solicitudes describirán la funcionalidad deseada de la función, incluyendo los parámetros de entrada y el valor de retorno esperado. Implementa las funciones según las especificaciones dadas, asegurándote de que manejen casos límite, realicen las validaciones necesarias y sigan las mejores prácticas para la programación en Python. Por favor incluye comentarios apropiados en el código para explicar la lógica y ayudar a otros desarrolladores a entender la implementación.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Quiero una función que pueda resolver un rompecabezas de Sudoku. La función debe tomar una cuadrícula de Sudoku de 9x9 como entrada, donde las celdas vacías están representadas por el valor 0. La función debe resolver el rompecabezas usando un algoritmo de retroceso y devolver la cuadrícula resuelta. Si el rompecabezas no se puede resolver, debe devolver None. La función también debe validar la cuadrícula de entrada para asegurar que sea un rompecabezas de Sudoku válido.",
                }
            ],
        }
    ],
)
print(message.content)


```
</Tab>
<Tab title="TypeScript">

```typescript
import Anthropic from "@anthropic-ai/sdk";

const anthropic = new Anthropic({
  apiKey: "my_api_key", // por defecto process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Tu tarea es crear funciones de Python basadas en las solicitudes en lenguaje natural proporcionadas. Las solicitudes describirán la funcionalidad deseada de la función, incluyendo los parámetros de entrada y el valor de retorno esperado. Implementa las funciones según las especificaciones dadas, asegurándote de que manejen casos límite, realicen las validaciones necesarias y sigan las mejores prácticas para la programación en Python. Por favor incluye comentarios apropiados en el código para explicar la lógica y ayudar a otros desarrolladores a entender la implementación.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Quiero una función que pueda resolver un rompecabezas de Sudoku. La función debe tomar una cuadrícula de Sudoku de 9x9 como entrada, donde las celdas vacías están representadas por el valor 0. La función debe resolver el rompecabezas usando un algoritmo de retroceso y devolver la cuadrícula resuelta. Si el rompecabezas no se puede resolver, debe devolver None. La función también debe validar la cuadrícula de entrada para asegurar que sea un rompecabezas de Sudoku válido."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# para opciones de autenticación

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Tu tarea es crear funciones de Python basadas en las solicitudes en lenguaje natural proporcionadas. Las solicitudes describirán la funcionalidad deseada de la función, incluyendo los parámetros de entrada y el valor de retorno esperado. Implementa las funciones según las especificaciones dadas, asegurándote de que manejen casos límite, realicen las validaciones necesarias y sigan las mejores prácticas para la programación en Python. Por favor incluye comentarios apropiados en el código para explicar la lógica y ayudar a otros desarrolladores a entender la implementación.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Quiero una función que pueda resolver un rompecabezas de Sudoku. La función debe tomar una cuadrícula de Sudoku de 9x9 como entrada, donde las celdas vacías están representadas por el valor 0. La función debe resolver el rompecabezas usando un algoritmo de retroceso y devolver la cuadrícula resuelta. Si el rompecabezas no se puede resolver, debe devolver None. La función también debe validar la cuadrícula de entrada para asegurar que sea un rompecabezas de Sudoku válido."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="AWS Bedrock TypeScript">

```typescript
import AnthropicBedrock from "@anthropic-ai/bedrock-sdk";

// Ver https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// para opciones de autenticación
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Tu tarea es crear funciones de Python basadas en las solicitudes en lenguaje natural proporcionadas. Las solicitudes describirán la funcionalidad deseada de la función, incluyendo los parámetros de entrada y el valor de retorno esperado. Implementa las funciones según las especificaciones dadas, asegurándote de que manejen casos límite, realicen las validaciones necesarias y sigan las mejores prácticas para la programación en Python. Por favor incluye comentarios apropiados en el código para explicar la lógica y ayudar a otros desarrolladores a entender la implementación.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Quiero una función que pueda resolver un rompecabezas de Sudoku. La función debe tomar una cuadrícula de Sudoku de 9x9 como entrada, donde las celdas vacías están representadas por el valor 0. La función debe resolver el rompecabezas usando un algoritmo de retroceso y devolver la cuadrícula resuelta. Si el rompecabezas no se puede resolver, debe devolver None. La función también debe validar la cuadrícula de entrada para asegurar que sea un rompecabezas de Sudoku válido."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
<Tab title="Vertex AI Python">
```python
from anthropic import AnthropicVertex

client = AnthropicVertex()

message = client.messages.create(
model="claude-sonnet-4@20250514",
max_tokens=1000,
temperature=0,
system="Tu tarea es crear funciones de Python basadas en las solicitudes en lenguaje natural proporcionadas. Las solicitudes describirán la funcionalidad deseada de la función, incluyendo los parámetros de entrada y el valor de retorno esperado. Implementa las funciones según las especificaciones dadas, asegurándote de que manejen casos límite, realicen las validaciones necesarias y sigan las mejores prácticas para la programación en Python. Por favor incluye comentarios apropiados en el código para explicar la lógica y ayudar a otros desarrolladores a entender la implementación.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Quiero una función que pueda resolver un rompecabezas de Sudoku. La función debe tomar una cuadrícula de Sudoku de 9x9 como entrada, donde las celdas vacías están representadas por el valor 0. La función debe resolver el rompecabezas usando un algoritmo de retroceso y devolver la cuadrícula resuelta. Si el rompecabezas no se puede resolver, debe devolver None. La función también debe validar la cuadrícula de entrada para asegurar que sea un rompecabezas de Sudoku válido."
}
]
}
]
)
print(message.content)

````
</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Lee de las variables de entorno `CLOUD_ML_REGION` y `ANTHROPIC_VERTEX_PROJECT_ID`.
// Además pasa por el flujo estándar de `google-auth-library`.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Tu tarea es crear funciones de Python basadas en las solicitudes en lenguaje natural proporcionadas. Las solicitudes describirán la funcionalidad deseada de la función, incluyendo los parámetros de entrada y el valor de retorno esperado. Implementa las funciones según las especificaciones dadas, asegurándote de que manejen casos límite, realicen las validaciones necesarias y sigan las mejores prácticas para la programación en Python. Por favor incluye comentarios apropiados en el código para explicar la lógica y ayudar a otros desarrolladores a entender la implementación.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Quiero una función que pueda resolver un rompecabezas de Sudoku. La función debe tomar una cuadrícula de Sudoku de 9x9 como entrada, donde las celdas vacías están representadas por el valor 0. La función debe resolver el rompecabezas usando un algoritmo de retroceso y devolver la cuadrícula resuelta. Si el rompecabezas no se puede resolver, debe devolver None. La función también debe validar la cuadrícula de entrada para asegurar que sea un rompecabezas de Sudoku válido."
        }
      ]
    }
  ]
});
console.log(msg);

````

</Tab>
</Tabs>