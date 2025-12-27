# Convertidor CSV

Convierte datos de varios formatos (JSON, XML, etc.) en archivos CSV correctamente formateados.

---

> ¡Copia este prompt en nuestra [Consola](/dashboard) de desarrollador para probarlo tú mismo!

|        | Contenido |
| --- | --- |
| Sistema | Como experto en conversión de datos, tu tarea es convertir datos de diferentes formatos (JSON, XML, etc.) en archivos CSV correctamente formateados. El usuario proporcionará los datos de entrada en el formato original, junto con cualquier requisito específico o preferencias para la salida CSV (por ejemplo, orden de columnas, delimitador, codificación). Asegúrate de tener una comprensión clara de la estructura de datos y el formato CSV deseado, haciendo cualquier pregunta aclaratoria según sea necesario. Una vez que tengas la información necesaria, genera la salida CSV siguiendo las reglas de formato apropiadas, como usar comas como delimitadores, encerrar valores entre comillas si es necesario, y manejar caracteres especiales o saltos de línea correctamente. Finalmente, proporciona cualquier instrucción adicional o consejos para guardar o usar el archivo CSV. |
| Usuario   | Por favor convierte los siguientes datos JSON en un archivo CSV: <br/> <br/> [ <br/> \{ <br/> "name": "John Doe", <br/> "age": 30, <br/> "city": "New York", <br/> "email": "john.doe@example.com" <br/> }, <br/> \{ <br/> "name": "Jane Smith", <br/> "age": 25, <br/> "city": "London", <br/> "email": "jane.smith@example.com" <br/> }, <br/> \{ <br/> "name": "Bob Johnson", <br/> "age": 35, <br/> "city": "Paris", <br/> "email": "bob.johnson@example.com" <br/> } <br/> ] <br/> Requisitos: <br/> - Las columnas en el CSV deben estar en el orden: name, age, city, email <br/> - Usar punto y coma (;) como delimitadores <br/> - Encerrar todos los valores entre comillas dobles (") |

## Salida de Ejemplo

Basándose en los datos JSON proporcionados y los requisitos, aquí está el archivo CSV convertido:

```csv csv
"name";"age";"city";"email"
"John Doe";"30";"New York";"john.doe@example.com"
"Jane Smith";"25";"London";"jane.smith@example.com"
"Bob Johnson";"35";"Paris";"bob.johnson@example.com"
```

Explicación:

    - La primera línea del archivo CSV contiene los encabezados de columna: "name", "age", "city", y "email", separados por punto y coma (;) y encerrados entre comillas dobles (").
    - Cada línea subsiguiente representa una fila de datos, con valores separados por punto y coma (;) y encerrados entre comillas dobles (").
    - El orden de los valores en cada fila corresponde al orden de columnas especificado: name, age, city, y email.
    - Los caracteres especiales, si están presentes en los valores, se preservan dentro de las comillas dobles.

Para guardar el archivo CSV:

    1. Copia los datos CSV proporcionados arriba.
    2. Abre un editor de texto (por ejemplo, Notepad, TextEdit, o cualquier otro editor de texto plano).
    3. Pega los datos CSV en el editor de texto.
    4. Guarda el archivo con una extensión .csv (por ejemplo, data.csv).

Nota: Al abrir el archivo CSV en una aplicación de hoja de cálculo (por ejemplo, Microsoft Excel, Google Sheets), asegúrate de especificar el delimitador correcto (punto y coma) y la presencia de comillas dobles alrededor de los valores para asegurar el formato apropiado.

---

## Solicitud API

<Tabs>
<Tab title="Python">

```python
import anthropic

client = anthropic.Anthropic(  # defaults to os.environ.get("ANTHROPIC_API_KEY")
    api_key="my_api_key",
)
message = client.messages.create(
    model="claude-sonnet-4-5",
    max_tokens=1000,
    temperature=0,
    system="Como experto en conversión de datos, tu tarea es convertir datos de diferentes formatos (JSON, XML, etc.) en archivos CSV correctamente formateados. El usuario proporcionará los datos de entrada en el formato original, junto con cualquier requisito específico o preferencias para la salida CSV (por ejemplo, orden de columnas, delimitador, codificación). Asegúrate de tener una comprensión clara de la estructura de datos y el formato CSV deseado, haciendo cualquier pregunta aclaratoria según sea necesario. Una vez que tengas la información necesaria, genera la salida CSV siguiendo las reglas de formato apropiadas, como usar comas como delimitadores, encerrar valores entre comillas si es necesario, y manejar caracteres especiales o saltos de línea correctamente. Finalmente, proporciona cualquier instrucción adicional o consejos para guardar o usar el archivo CSV.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": 'Por favor convierte los siguientes datos JSON en un archivo CSV: \n \n[ \n { \n "name": "John Doe", \n "age": 30, \n "city": "New York", \n "email": "[email protected]" \n }, \n { \n "name": "Jane Smith", \n "age": 25, \n "city": "London", \n "email": "[email protected]" \n }, \n { \n "name": "Bob Johnson", \n "age": 35, \n "city": "Paris", \n "email": "[email protected]" \n } \n] \n \nRequisitos: \n- Las columnas en el CSV deben estar en el orden: name, age, city, email \n- Usar punto y coma (;) como delimitadores \n- Encerrar todos los valores entre comillas dobles (")',
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
  apiKey: "my_api_key", // defaults to process.env["ANTHROPIC_API_KEY"]
});

const msg = await anthropic.messages.create({
  model: "claude-sonnet-4-5",
  max_tokens: 1000,
  temperature: 0,
  system: "Como experto en conversión de datos, tu tarea es convertir datos de diferentes formatos (JSON, XML, etc.) en archivos CSV correctamente formateados. El usuario proporcionará los datos de entrada en el formato original, junto con cualquier requisito específico o preferencias para la salida CSV (por ejemplo, orden de columnas, delimitador, codificación). Asegúrate de tener una comprensión clara de la estructura de datos y el formato CSV deseado, haciendo cualquier pregunta aclaratoria según sea necesario. Una vez que tengas la información necesaria, genera la salida CSV siguiendo las reglas de formato apropiadas, como usar comas como delimitadores, encerrar valores entre comillas si es necesario, y manejar caracteres especiales o saltos de línea correctamente. Finalmente, proporciona cualquier instrucción adicional o consejos para guardar o usar el archivo CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Por favor convierte los siguientes datos JSON en un archivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- Las columnas en el CSV deben estar en el orden: name, age, city, email  \n- Usar punto y coma (;) como delimitadores  \n- Encerrar todos los valores entre comillas dobles (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
<Tab title="AWS Bedrock Python">
```python
from anthropic import AnthropicBedrock

# See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock

# for authentication options

client = AnthropicBedrock()

message = client.messages.create(
model="anthropic.claude-sonnet-4-5-20250929-v1:0",
max_tokens=1000,
temperature=0,
system="Como experto en conversión de datos, tu tarea es convertir datos de diferentes formatos (JSON, XML, etc.) en archivos CSV correctamente formateados. El usuario proporcionará los datos de entrada en el formato original, junto con cualquier requisito específico o preferencias para la salida CSV (por ejemplo, orden de columnas, delimitador, codificación). Asegúrate de tener una comprensión clara de la estructura de datos y el formato CSV deseado, haciendo cualquier pregunta aclaratoria según sea necesario. Una vez que tengas la información necesaria, genera la salida CSV siguiendo las reglas de formato apropiadas, como usar comas como delimitadores, encerrar valores entre comillas si es necesario, y manejar caracteres especiales o saltos de línea correctamente. Finalmente, proporciona cualquier instrucción adicional o consejos para guardar o usar el archivo CSV.",
messages=[
{
"role": "user",
"content": [
{
"type": "text",
"text": "Por favor convierte los siguientes datos JSON en un archivo CSV: \n \n[ \n { \n \"name\": \"John Doe\", \n \"age\": 30, \n \"city\": \"New York\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Jane Smith\", \n \"age\": 25, \n \"city\": \"London\", \n \"email\": \"[email protected]\" \n }, \n { \n \"name\": \"Bob Johnson\", \n \"age\": 35, \n \"city\": \"Paris\", \n \"email\": \"[email protected]\" \n } \n] \n \nRequisitos: \n- Las columnas en el CSV deben estar en el orden: name, age, city, email \n- Usar punto y coma (;) como delimitadores \n- Encerrar todos los valores entre comillas dobles (\")"
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

// See https://docs.claude.com/claude/reference/claude-on-amazon-bedrock
// for authentication options
const client = new AnthropicBedrock();

const msg = await client.messages.create({
  model: "anthropic.claude-sonnet-4-5-20250929-v1:0",
  max_tokens: 1000,
  temperature: 0,
  system: "Como experto en conversión de datos, tu tarea es convertir datos de diferentes formatos (JSON, XML, etc.) en archivos CSV correctamente formateados. El usuario proporcionará los datos de entrada en el formato original, junto con cualquier requisito específico o preferencias para la salida CSV (por ejemplo, orden de columnas, delimitador, codificación). Asegúrate de tener una comprensión clara de la estructura de datos y el formato CSV deseado, haciendo cualquier pregunta aclaratoria según sea necesario. Una vez que tengas la información necesaria, genera la salida CSV siguiendo las reglas de formato apropiadas, como usar comas como delimitadores, encerrar valores entre comillas si es necesario, y manejar caracteres especiales o saltos de línea correctamente. Finalmente, proporciona cualquier instrucción adicional o consejos para guardar o usar el archivo CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Por favor convierte los siguientes datos JSON en un archivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- Las columnas en el CSV deben estar en el orden: name, age, city, email  \n- Usar punto y coma (;) como delimitadores  \n- Encerrar todos los valores entre comillas dobles (\")"
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
    system="Como experto en conversión de datos, tu tarea es convertir datos de diferentes formatos (JSON, XML, etc.) en archivos CSV correctamente formateados. El usuario proporcionará los datos de entrada en el formato original, junto con cualquier requisito específico o preferencias para la salida CSV (por ejemplo, orden de columnas, delimitador, codificación). Asegúrate de tener una comprensión clara de la estructura de datos y el formato CSV deseado, haciendo cualquier pregunta aclaratoria según sea necesario. Una vez que tengas la información necesaria, genera la salida CSV siguiendo las reglas de formato apropiadas, como usar comas como delimitadores, encerrar valores entre comillas si es necesario, y manejar caracteres especiales o saltos de línea correctamente. Finalmente, proporciona cualquier instrucción adicional o consejos para guardar o usar el archivo CSV.",
    messages=[
        {
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Por favor convierte los siguientes datos JSON en un archivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- Las columnas en el CSV deben estar en el orden: name, age, city, email  \n- Usar punto y coma (;) como delimitadores  \n- Encerrar todos los valores entre comillas dobles (\")"
                }
            ]
        }
    ]
)
print(message.content)

```

</Tab>
<Tab title="Vertex AI TypeScript">

```typescript
import { AnthropicVertex } from '@anthropic-ai/vertex-sdk';

// Reads from the `CLOUD_ML_REGION` & `ANTHROPIC_VERTEX_PROJECT_ID` environment variables.
// Additionally goes through the standard `google-auth-library` flow.
const client = new AnthropicVertex();

const msg = await client.messages.create({
  model: "claude-sonnet-4@20250514",
  max_tokens: 1000,
  temperature: 0,
  system: "Como experto en conversión de datos, tu tarea es convertir datos de diferentes formatos (JSON, XML, etc.) en archivos CSV correctamente formateados. El usuario proporcionará los datos de entrada en el formato original, junto con cualquier requisito específico o preferencias para la salida CSV (por ejemplo, orden de columnas, delimitador, codificación). Asegúrate de tener una comprensión clara de la estructura de datos y el formato CSV deseado, haciendo cualquier pregunta aclaratoria según sea necesario. Una vez que tengas la información necesaria, genera la salida CSV siguiendo las reglas de formato apropiadas, como usar comas como delimitadores, encerrar valores entre comillas si es necesario, y manejar caracteres especiales o saltos de línea correctamente. Finalmente, proporciona cualquier instrucción adicional o consejos para guardar o usar el archivo CSV.",
  messages: [
    {
      "role": "user",
      "content": [
        {
          "type": "text",
          "text": "Por favor convierte los siguientes datos JSON en un archivo CSV:  \n  \n[  \n  {  \n    \"name\": \"John Doe\",  \n    \"age\": 30,  \n    \"city\": \"New York\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Jane Smith\",  \n    \"age\": 25,  \n    \"city\": \"London\",  \n    \"email\": \"[email protected]\"  \n  },  \n  {  \n    \"name\": \"Bob Johnson\",  \n    \"age\": 35,  \n    \"city\": \"Paris\",  \n    \"email\": \"[email protected]\"  \n  }  \n]  \n  \nRequisitos:  \n- Las columnas en el CSV deben estar en el orden: name, age, city, email  \n- Usar punto y coma (;) como delimitadores  \n- Encerrar todos los valores entre comillas dobles (\")"
        }
      ]
    }
  ]
});
console.log(msg);

```

</Tab>
</Tabs>