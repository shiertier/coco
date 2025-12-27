# Subagentes en el SDK

Trabajando con subagentes en el SDK de Claude Agent

---

Los subagentes en el SDK de Claude Agent son IAs especializadas que son orquestadas por el agente principal.
Usa subagentes para la gestión de contexto y paralelización.

Esta guía explica cómo definir y usar subagentes en el SDK usando el parámetro `agents`.

## Descripción General

Los subagentes pueden definirse de dos maneras al usar el SDK:

1. **Programáticamente** - Usando el parámetro `agents` en las opciones de tu `query()` (recomendado para aplicaciones SDK)
2. **Basado en sistema de archivos** - Colocando archivos markdown con frontmatter YAML en directorios designados (`.claude/agents/`)

Esta guía se enfoca principalmente en el enfoque programático usando el parámetro `agents`, que proporciona una experiencia de desarrollo más integrada para aplicaciones SDK.

## Beneficios de Usar Subagentes

### Gestión de Contexto
Los subagentes mantienen contexto separado del agente principal, previniendo la sobrecarga de información y manteniendo las interacciones enfocadas. Este aislamiento asegura que las tareas especializadas no contaminen el contexto de conversación principal con detalles irrelevantes.

**Ejemplo**: Un subagente `research-assistant` puede explorar docenas de archivos y páginas de documentación sin saturar la conversación principal con todos los resultados de búsqueda intermedios - solo devolviendo los hallazgos relevantes.

### Paralelización
Múltiples subagentes pueden ejecutarse concurrentemente, acelerando dramáticamente los flujos de trabajo complejos.

**Ejemplo**: Durante una revisión de código, puedes ejecutar subagentes `style-checker`, `security-scanner`, y `test-coverage` simultáneamente, reduciendo el tiempo de revisión de minutos a segundos.

### Instrucciones y Conocimiento Especializados
Cada subagente puede tener prompts de sistema personalizados con experiencia específica, mejores prácticas y restricciones.

**Ejemplo**: Un subagente `database-migration` puede tener conocimiento detallado sobre mejores prácticas de SQL, estrategias de rollback y verificaciones de integridad de datos que serían ruido innecesario en las instrucciones del agente principal.

### Restricciones de Herramientas
Los subagentes pueden limitarse a herramientas específicas, reduciendo el riesgo de acciones no intencionadas.

**Ejemplo**: Un subagente `doc-reviewer` podría tener acceso solo a herramientas Read y Grep, asegurando que pueda analizar pero nunca modificar accidentalmente tus archivos de documentación.

## Creando Subagentes

### Definición Programática (Recomendada)

Define subagentes directamente en tu código usando el parámetro `agents`:

```typescript
import { query } from '@anthropic-ai/claude-agent-sdk';

const result = query({
  prompt: "Revisa el módulo de autenticación para problemas de seguridad",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Especialista experto en revisión de código. Usar para revisiones de calidad, seguridad y mantenibilidad.',
        prompt: `Eres un especialista en revisión de código con experiencia en seguridad, rendimiento y mejores prácticas.

Al revisar código:
- Identifica vulnerabilidades de seguridad
- Verifica problemas de rendimiento
- Verifica adherencia a estándares de codificación
- Sugiere mejoras específicas

Sé exhaustivo pero conciso en tu retroalimentación.`,
        tools: ['Read', 'Grep', 'Glob'],
        model: 'sonnet'
      },
      'test-runner': {
        description: 'Ejecuta y analiza suites de pruebas. Usar para ejecución de pruebas y análisis de cobertura.',
        prompt: `Eres un especialista en ejecución de pruebas. Ejecuta pruebas y proporciona análisis claro de resultados.

Enfócate en:
- Ejecutar comandos de prueba
- Analizar salida de pruebas
- Identificar pruebas fallidas
- Sugerir correcciones para fallas`,
        tools: ['Bash', 'Read', 'Grep'],
      }
    }
  }
});

for await (const message of result) {
  console.log(message);
}
```

### Configuración de AgentDefinition

| Campo | Tipo | Requerido | Descripción |
|:---|:---|:---|:---|
| `description` | `string` | Sí | Descripción en lenguaje natural de cuándo usar este agente |
| `prompt` | `string` | Sí | El prompt del sistema del agente definiendo su rol y comportamiento |
| `tools` | `string[]` | No | Array de nombres de herramientas permitidas. Si se omite, hereda todas las herramientas |
| `model` | `'sonnet' \| 'opus' \| 'haiku' \| 'inherit'` | No | Anulación de modelo para este agente. Por defecto usa el modelo principal si se omite |

### Definición Basada en Sistema de Archivos (Alternativa)

También puedes definir subagentes como archivos markdown en directorios específicos:

- **Nivel de proyecto**: `.claude/agents/*.md` - Disponible solo en el proyecto actual
- **Nivel de usuario**: `~/.claude/agents/*.md` - Disponible en todos los proyectos

Cada subagente es un archivo markdown con frontmatter YAML:

```markdown
---
name: code-reviewer
description: Especialista experto en revisión de código. Usar para revisiones de calidad, seguridad y mantenibilidad.
tools: Read, Grep, Glob, Bash
---

El prompt del sistema de tu subagente va aquí. Esto define el rol del subagente,
capacidades y enfoque para resolver problemas.
```

**Nota:** Los agentes definidos programáticamente (vía el parámetro `agents`) tienen precedencia sobre los agentes basados en sistema de archivos con el mismo nombre.

## Cómo el SDK Usa Subagentes

Al usar el SDK de Claude Agent, los subagentes pueden definirse programáticamente o cargarse desde el sistema de archivos. Claude:

1. **Carga agentes programáticos** del parámetro `agents` en tus opciones
2. **Auto-detecta agentes del sistema de archivos** de directorios `.claude/agents/` (si no se anula)
3. **Los invoca automáticamente** basado en coincidencia de tareas y la `description` del agente
4. **Usa sus prompts especializados** y restricciones de herramientas
5. **Mantiene contexto separado** para cada invocación de subagente

Los agentes definidos programáticamente (vía parámetro `agents`) tienen precedencia sobre los agentes basados en sistema de archivos con el mismo nombre.

## Subagentes de Ejemplo

Para ejemplos comprensivos de subagentes incluyendo revisores de código, ejecutores de pruebas, depuradores y auditores de seguridad, consulta la [guía principal de Subagentes](https://code.claude.com/docs/sub-agents#example-subagents). La guía incluye configuraciones detalladas y mejores prácticas para crear subagentes efectivos.

## Patrones de Integración del SDK

### Invocación Automática

El SDK invocará automáticamente subagentes apropiados basado en el contexto de la tarea. Asegúrate de que el campo `description` de tu agente indique claramente cuándo debe usarse:

```typescript
const result = query({
  prompt: "Optimiza las consultas de base de datos en la capa API",
  options: {
    agents: {
      'performance-optimizer': {
        description: 'Usar PROACTIVAMENTE cuando los cambios de código puedan impactar el rendimiento. DEBE USARSE para tareas de optimización.',
        prompt: 'Eres un especialista en optimización de rendimiento...',
        tools: ['Read', 'Edit', 'Bash', 'Grep'],
        model: 'sonnet'
      }
    }
  }
});
```

### Invocación Explícita

Los usuarios pueden solicitar subagentes específicos en sus prompts:

```typescript
const result = query({
  prompt: "Usa el agente code-reviewer para verificar el módulo de autenticación",
  options: {
    agents: {
      'code-reviewer': {
        description: 'Especialista experto en revisión de código',
        prompt: 'Eres un revisor de código enfocado en seguridad...',
        tools: ['Read', 'Grep', 'Glob']
      }
    }
  }
});
```

### Configuración Dinámica de Agentes

Puedes configurar dinámicamente agentes basado en las necesidades de tu aplicación:

```typescript
import { query, type AgentDefinition } from '@anthropic-ai/claude-agent-sdk';

function createSecurityAgent(securityLevel: 'basic' | 'strict'): AgentDefinition {
  return {
    description: 'Revisor de código de seguridad',
    prompt: `Eres un revisor de seguridad ${securityLevel === 'strict' ? 'estricto' : 'equilibrado'}...`,
    tools: ['Read', 'Grep', 'Glob'],
    model: securityLevel === 'strict' ? 'opus' : 'sonnet'
  };
}

const result = query({
  prompt: "Revisa este PR para problemas de seguridad",
  options: {
    agents: {
      'security-reviewer': createSecurityAgent('strict')
    }
  }
});
```

## Restricciones de Herramientas

Los subagentes pueden tener acceso restringido a herramientas vía el campo `tools`:

- **Omitir el campo** - El agente hereda todas las herramientas disponibles (por defecto)
- **Especificar herramientas** - El agente solo puede usar las herramientas listadas

Ejemplo de un agente de análisis de solo lectura:

```typescript
const result = query({
  prompt: "Analiza la arquitectura de esta base de código",
  options: {
    agents: {
      'code-analyzer': {
        description: 'Análisis estático de código y revisión de arquitectura',
        prompt: `Eres un analista de arquitectura de código. Analiza la estructura del código,
identifica patrones y sugiere mejoras sin hacer cambios.`,
        tools: ['Read', 'Grep', 'Glob']  // Sin permisos de escritura o ejecución
      }
    }
  }
});
```

### Combinaciones Comunes de Herramientas

**Agentes de solo lectura** (análisis, revisión):
```typescript
tools: ['Read', 'Grep', 'Glob']
```

**Agentes de ejecución de pruebas**:
```typescript
tools: ['Bash', 'Read', 'Grep']
```

**Agentes de modificación de código**:
```typescript
tools: ['Read', 'Edit', 'Write', 'Grep', 'Glob']
```

## Documentación Relacionada

- [Guía Principal de Subagentes](https://code.claude.com/docs/sub-agents) - Documentación comprensiva de subagentes
- [Descripción General del SDK](/docs/es/agent-sdk/overview) - Descripción general del SDK de Claude Agent
- [Configuraciones](https://code.claude.com/docs/settings) - Referencia de archivo de configuración
- [Comandos Slash](https://code.claude.com/docs/slash-commands) - Creación de comandos personalizados