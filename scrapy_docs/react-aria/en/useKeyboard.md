# useKeyboard



```tsx
"use client"
import React from 'react';
import {useKeyboard} from 'react-aria';

function Example() {
  let [events, setEvents] = React.useState<string[]>([]);
  let {keyboardProps} = useKeyboard({
    onKeyDown: e => setEvents(
      events => [`key down: ${e.key}`, ...events]
    ),
    onKeyUp: e => setEvents(
      events => [`key up: ${e.key}`, ...events]
    )
  });

  return (
    <>
      <label htmlFor="example">Example</label>
      <input
        {...keyboardProps}
        id="example" />
      <ul style={{
        height: 100,
        overflow: 'auto',
        border: '1px solid gray',
        width: 200
      }}>
        {events.map((e, i) => <li key={i}>{e}</li>)}
      </ul>
    </>
  );
}
```

## Features

`useKeyboard` handles keyboard interactions. The only difference from DOM events is that propagation
is stopped by default if there is an event handler, unless `event.continuePropagation()` is called.
This provides better modularity by default, so that a parent component doesn't respond to an event
that a child already handled. If the child doesn't handle the event (e.g. it was for an unknown key),
it can call `event.continuePropagation()` to allow parents to handle the event.

## API

<FunctionAPI
  function={docs.exports.useKeyboard}
  links={docs.links}
/>

### KeyboardProps

| Name | Type | Description |
|------|------|-------------|
| `isDisabled` | `boolean | undefined` | Whether the keyboard events should be disabled. |
| `onKeyDown` | `((e: KeyboardEvent) => void) | undefined` | Handler that is called when a key is pressed. |
| `onKeyUp` | `((e: KeyboardEvent) => void) | undefined` | Handler that is called when a key is released. |

### KeyboardResult

| Name | Type | Description |
|------|------|-------------|
| `keyboardProps` \* | `DOMAttributes<FocusableElement>` | Props to spread onto the target element. |
