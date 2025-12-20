# Soporte multilingüe

Claude destaca en tareas en múltiples idiomas, manteniendo un fuerte desempeño multilingüe relativo al inglés.

---

## Descripción general

Claude demuestra capacidades multilingües robustas, con un desempeño particularmente fuerte en tareas de cero disparos en múltiples idiomas. El modelo mantiene un desempeño relativo consistente en idiomas tanto ampliamente hablados como de recursos limitados, lo que lo convierte en una opción confiable para aplicaciones multilingües.

Tenga en cuenta que Claude es capaz en muchos idiomas más allá de los evaluados a continuación. Le recomendamos que pruebe con cualquier idioma relevante para sus casos de uso específicos.

## Datos de desempeño

A continuación se muestran las puntuaciones de evaluación de cadena de pensamiento de cero disparos para modelos Claude en diferentes idiomas, mostradas como un porcentaje relativo al desempeño en inglés (100%):

| Idioma | Claude Opus 4.1<sup>1</sup> | Claude Opus 4<sup>1</sup> | Claude Sonnet 4.5<sup>1</sup> | Claude Sonnet 4<sup>1</sup> | Claude Haiku 4.5<sup>1</sup> |
|----------|---------------|---------------|---------------|-----------------|------------------|
| Inglés (línea base, fijo al 100%) | 100% | 100% | 100% | 100% | 100% |
| Español | 98.1% | 98.0% | 98.2% | 97.5% | 96.4% |
| Portugués (Brasil) | 97.8% | 97.3% | 97.8% | 97.2% | 96.1% |
| Italiano | 97.7% | 97.5% | 97.9% | 97.3% | 96.0% |
| Francés | 97.9% | 97.7% | 97.5% | 97.1% | 95.7% |
| Indonesio | 97.3% | 97.2% | 97.3% | 96.2% | 94.2% |
| Alemán | 97.7% | 97.1% | 97.0% | 94.7% | 94.3% |
| Árabe | 97.1% | 96.9% | 97.2% | 96.1% | 92.5% |
| Chino (Simplificado) | 97.1% | 96.7% | 96.9% | 95.9% | 94.2% |
| Coreano | 96.6% | 96.4% | 96.7% | 95.9% | 93.3% |
| Japonés | 96.9% | 96.2% | 96.8% | 95.6% | 93.5% |
| Hindi | 96.8% | 96.7% | 96.7% | 95.8% | 92.4% |
| Bengalí | 95.7% | 95.2% | 95.4% | 94.4% | 90.4% |
| Suajili | 89.8% | 89.5% | 91.1% | 87.1% | 78.3% |
| Yoruba | 80.3% | 78.9% | 79.7% | 76.4% | 52.7% |

<sup>1</sup> Con [pensamiento extendido](/docs/es/build-with-claude/extended-thinking).

<Note>
Estas métricas se basan en conjuntos de pruebas en inglés de [MMLU (Comprensión del Lenguaje Multitarea Masiva)](https://en.wikipedia.org/wiki/MMLU) que fueron traducidos a 14 idiomas adicionales por traductores humanos profesionales, como se documenta en [el repositorio simple-evals de OpenAI](https://github.com/openai/simple-evals/blob/main/multilingual_mmlu_benchmark_results.md). El uso de traductores humanos para esta evaluación garantiza traducciones de alta calidad, particularmente importante para idiomas con menos recursos digitales.
</Note>

***

## Mejores prácticas

Al trabajar con contenido multilingüe:

1. **Proporcione contexto de idioma claro**: Aunque Claude puede detectar el idioma de destino automáticamente, indicar explícitamente el idioma de entrada/salida deseado mejora la confiabilidad. Para una fluidez mejorada, puede indicarle a Claude que use "lenguaje idiomático como si fuera un hablante nativo".
2. **Use escrituras nativas**: Envíe texto en su escritura nativa en lugar de transliteración para obtener resultados óptimos
3. **Considere el contexto cultural**: La comunicación efectiva a menudo requiere conciencia cultural y regional más allá de la pura traducción

También le sugerimos que siga nuestras [directrices generales de ingeniería de indicaciones](/docs/es/build-with-claude/prompt-engineering/overview) para mejorar mejor el desempeño de Claude.

***

## Consideraciones de soporte de idioma

- Claude procesa entrada y genera salida en la mayoría de idiomas mundiales que utilizan caracteres Unicode estándar
- El desempeño varía según el idioma, con capacidades particularmente fuertes en idiomas ampliamente hablados
- Incluso en idiomas con menos recursos digitales, Claude mantiene capacidades significativas

<CardGroup cols={2}>
  <Card title="Guía de Ingeniería de Indicaciones" icon="edit" href="/docs/es/build-with-claude/prompt-engineering/overview">
    Domine el arte de la elaboración de indicaciones para aprovechar al máximo Claude.
  </Card>
  <Card title="Biblioteca de Indicaciones" icon="books" href="/docs/es/resources/prompt-library">
    Encuentre una amplia gama de indicaciones preelaboradas para diversas tareas e industrias. Perfecto para inspiración o inicios rápidos.
  </Card>
</CardGroup>