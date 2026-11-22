Файл с основными командами, доступными для вызова.

На данный момент имеются **четыре** основные команды:

1. `styles` -- операции со стилями.
2. `tags` -- операции с тэгами.
3. `tasks` -- операции с задачами.
4. `events` -- операции с событиями.

У всех этих команд общий набор операций:

1. `add` -- добавить объект.
2. `get_all` -- получить все объекты.
3. `get_by_id` -- получить конкретный объект по id.
4. `update` -- изменить существующий объект, передав id существующего, а также его новые данные.
5. `delete_all` -- удалить все объекты.
6. `delete_by_id` удалить существующий объект по id.

Все они имеют схожий интерфейс вызова и в данный момент используются для CRUD-операций с back-end-частью.
Общий вызов `tauri`-команды из `JS` выглядит так:

```js
const result = await invoke("command_name", args);
```

Определённые back-end-частью команды все принимают обязательные параметры:

1. `request` -- название переменной в `rust`-части кода.
2. `op` -- тип операции. Располагается в `request`.

Минимальный вызов команды в нашем случае выглядит так:

```js
const result = await invoke("command_name", {
  request: {
    op: "operation_type",
  },
});
```

Некоторые команды принимают дополнительные необходимые аргументы. В нашем случае, на данный момент,
чаще всего это `id` и/или `data`.

```js
const result = await invoke("command_name", {
  request: {
    op: "operation_type",
    id: "object_id",
    data: {
      name: "Name",
      age: "Age",
    },
  },
});
```

Ниже перечисленные все примеры операций для команды `styles`.
В поле `data` тут передаются данные, необходимые для создания объекта,
а в поле `id` -- id уже существующего.

```js
const default_uuid = "878b25ae-f556-49cf-91c2-cd896bafae18";

const add_result = await invoke("styles", {
  request: {
    op: "add",
    data: {
      id: default_uuid,
      color: "#000000",
      background_color: "#ffffff",
    },
  },
}).catch((error) => {
  console.error(error);
});
console.log(add_result);

const get_all_result = invoke("styles", {
  request: {
    op: "get_all",
  },
});
console.log(get_all_result);

const get_by_id_result = invoke("styles", {
  request: {
    op: "get_by_id",
    id: default_uuid,
  },
});
console.log(get_by_id_result);

invoke("styles", {
  request: {
    op: "update",
    id: default_uuid,
    data: {
      id: default_uuid,
      color: "#ffdd00",
      background_color: "#223344",
    },
  },
});

invoke("styles", {
  request: {
    op: "delete_all",
  },
});

invoke("styles", {
  request: {
    op: "delete_by_id",
    id: default_uuid,
  },
});
```
