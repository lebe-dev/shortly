TITLE: Create New SvelteKit Project
DESCRIPTION: Uses the SvelteKit CLI to scaffold a new project, providing a clean starting point for development.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/installation/sveltekit.md#_snippet_0

LANGUAGE: bash
CODE:
```
sv create my-app
```

----------------------------------------

TITLE: Create Svelte Form Component with Superforms and shadcn-svelte
DESCRIPTION: This Svelte component defines a reusable form using `sveltekit-superforms` for client-side validation and `shadcn-svelte` UI components. It expects a `SuperValidated` form object as a prop, ensuring type safety with a Zod schema. The component handles input binding and form submission via SvelteKit's `enhance` action.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/form.md#_snippet_4

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import * as Form from "$lib/components/ui/form/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { formSchema, type FormSchema } from "./schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";

  let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
    $props();

  const form = superForm(data.form, {
    validators: zodClient(formSchema),
  });

  const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance>
  <Form.Field {form} name="username">
    <Form.Control>
      {#snippet children({ props })}
        <Form.Label>Username</Form.Label>
        <Input {...props} bind:value={$formData.username} />
      {/snippet}
    </Form.Control>
    <Form.Description>This is your public display name.</Form.Description>
    <Form.FieldErrors />
  </Form.Field>
  <Form.Button>Submit</Form.Button>
</form>
```

----------------------------------------

TITLE: Add and Use shadcn-svelte Button Component
DESCRIPTION: Demonstrates the process of adding a specific component, such as the Button, using the shadcn-svelte CLI. It then shows how to import and integrate this component into a Svelte component for immediate use.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/installation/sveltekit.md#_snippet_5

LANGUAGE: bash
CODE:
```
npx shadcn-svelte add button
```

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
</script>

<Button>Click me</Button>
```

----------------------------------------

TITLE: Add Components with shadcn-svelte CLI
DESCRIPTION: The `add` command allows users to select and add specific components and their dependencies to their project. It presents a list of available components for selection.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/cli.md#_snippet_1

LANGUAGE: bash
CODE:
```
shadcn-svelte@latest add [component]
```

LANGUAGE: txt
CODE:
```
Which components would you like to add? › Space to select. Return to submit.

◯  accordion
◯  alert
◯  alert-dialog
◯  aspect-ratio
◯  avatar
◯  badge
◯  button
◯  card
◯  checkbox
◯  collapsible
```

LANGUAGE: txt
CODE:
```
Usage: shadcn-svelte add [options] [components...]

add components to your project

Arguments:
  components         the components to add or a url to the component

Options:
  -c, --cwd <path>   the working directory (default: the current directory)
  --no-deps          skips adding & installing package dependencies
  -a, --all          install all components to your project (default: false)
  -y, --yes          skip confirmation prompt (default: false)
  -o, --overwrite    overwrite existing files (default: false)
  --proxy <proxy>    fetch components from registry using a proxy
  -h, --help         display help for command
```

----------------------------------------

TITLE: Create Tailwind CSS Class Utility Function (cn)
DESCRIPTION: This TypeScript utility function, `cn`, simplifies the process of conditionally applying and merging Tailwind CSS classes. It leverages `clsx` for conditional class joining and `tailwind-merge` to resolve conflicting classes, ensuring a clean and effective final class string.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/installation/manual.md#_snippet_7

LANGUAGE: typescript
CODE:
```
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
```

----------------------------------------

TITLE: Integrate Row Selection State into Svelte DataTable (Svelte/TypeScript)
DESCRIPTION: This Svelte component snippet shows how to integrate row selection state (`rowSelection`) into a `DataTable` component using `@tanstack/table-core`. It initializes and manages various table states like pagination, sorting, filtering, and column visibility with Svelte's `$state` and `$props` runes, ensuring proper state updates via `on...Change` handlers for a fully interactive data table.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_19

LANGUAGE: Svelte
CODE:
```
<script lang="ts" generics="TData, TValue">
  import {
    type ColumnDef,
    type PaginationState,
    type SortingState,
    type ColumnFiltersState,
    type VisibilityState,
    type RowSelectionState,
    getCoreRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    getFilteredRowModel,
  } from "@tanstack/table-core";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";

  let { columns, data }: DataTableProps<TData, TValue> = $props();

  let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
  let sorting = $state<SortingState>([]);
  let columnFilters = $state<ColumnFiltersState>([]);
  let columnVisibility = $state<VisibilityState>({});
  let rowSelection = $state<RowSelectionState>({});

  const table = createSvelteTable({
    get data() {
      return data;
    },
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onPaginationChange: (updater) => {
      if (typeof updater === "function") {
        pagination = updater(pagination);
      } else {
        pagination = updater;
      }
    },
    onSortingChange: (updater) => {
      if (typeof updater === "function") {
        sorting = updater(sorting);
      } else {
        sorting = updater;
      }
    },
    onColumnFiltersChange: (updater) => {
      if (typeof updater === "function") {
        columnFilters = updater(columnFilters);
      } else {
        columnFilters = updater;
      }
    },
    onColumnVisibilityChange: (updater) => {
      if (typeof updater === "function") {
        columnVisibility = updater(columnVisibility);
      } else {
        columnVisibility = updater;
      }
    },
    onRowSelectionChange: (updater) => {
      if (typeof updater === "function") {
        rowSelection = updater(rowSelection);
      } else {
        rowSelection = updater;
      }
    },
    state: {
      get pagination() {
        return pagination;
      },
      get sorting() {
        return sorting;
      },
      get columnFilters() {
        return columnFilters;
      },
      get columnVisibility() {
        return columnVisibility;
      },
      get rowSelection() {
        return rowSelection;
      },
    },
  });
</script>
```

----------------------------------------

TITLE: Add shadcn-svelte component via CLI
DESCRIPTION: Use the `shadcn-svelte` CLI to add a specific component (e.g., Button) to the project, automatically generating necessary files.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/installation/astro.md#_snippet_10

LANGUAGE: bash
CODE:
```
npx shadcn-svelte add button
```

----------------------------------------

TITLE: Install Carousel Component via CLI
DESCRIPTION: Installs the Carousel component using the shadcn-svelte CLI, which automates the setup process by adding the necessary files to your project.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/carousel.md#_snippet_0

LANGUAGE: shell
CODE:
```
npx shadcn-svelte add carousel
```

----------------------------------------

TITLE: Implement Filtering, Sorting, and Pagination in Svelte DataTable
DESCRIPTION: This Svelte component demonstrates how to integrate `@tanstack/table-core` functionalities like pagination, sorting, and column filtering into a `<DataTable />` component. It shows the setup of `createSvelteTable` with state management for `pagination`, `sorting`, and `columnFilters`, and includes an `Input` component for filtering the 'email' column.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_15

LANGUAGE: svelte
CODE:
```
<script lang="ts" generics="TData, TValue">
  import {
    type ColumnDef,
    type PaginationState,
    type SortingState,
    type ColumnFiltersState,
    getCoreRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    getFilteredRowModel,
  } from "@tanstack/table-core";
  import { Input } from "$lib/components/ui/input/index.js";

  let { columns, data }: DataTableProps<TData, TValue> = $props();

  let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
  let sorting = $state<SortingState>([]);
  let columnFilters = $state<ColumnFiltersState>([]);

  const table = createSvelteTable({
    get data() {
      return data;
    },
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onPaginationChange: (updater) => {
      if (typeof updater === "function") {
        pagination = updater(pagination);
      } else {
        pagination = updater;
      }
    },
    onSortingChange: (updater) => {
      if (typeof updater === "function") {
        sorting = updater(sorting);
      } else {
        sorting = updater;
      }
    },
    onColumnFiltersChange: (updater) => {
      if (typeof updater === "function") {
        columnFilters = updater(columnFilters);
      } else {
        columnFilters = updater;
      }
    },
    state: {
      get pagination() {
        return pagination;
      },
      get sorting() {
        return sorting;
      },
      get columnFilters() {
        return columnFilters;
      },
    },
  });
</script>

<div>
  <div class="flex items-center py-4">
    <Input
      placeholder="Filter emails..."
      value={(table.getColumn("email")?.getFilterValue() as string) ?? ""}
      onchange={(e) => {
        table.getColumn("email")?.setFilterValue(e.currentTarget.value);
      }}
      oninput={(e) => {
        table.getColumn("email")?.setFilterValue(e.currentTarget.value);
      }}
      class="max-w-sm"
    />
  </div>
  <div class="rounded-md border">
    <Table.Root><!-- ... --></Table.Root>
  </div>
</div>
```

----------------------------------------

TITLE: Install Radio Group Component via CLI
DESCRIPTION: Use the shadcn-svelte CLI to automatically add the Radio Group component and its necessary dependencies to your project, streamlining the setup process.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/radio-group.md#_snippet_0

LANGUAGE: cli
CODE:
```
npx shadcn-svelte add radio-group
```

----------------------------------------

TITLE: Install Progress Component
DESCRIPTION: Instructions for installing the Progress component using the shadcn-svelte CLI or manually by installing `bits-ui` and copying source files.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/progress.md#_snippet_0

LANGUAGE: bash
CODE:
```
npx shadcn-svelte add progress
```

LANGUAGE: bash
CODE:
```
npm install bits-ui -D
```

----------------------------------------

TITLE: Initialize shadcn-svelte Project
DESCRIPTION: Initializes a new shadcn-svelte project by installing dependencies, adding the `cn` utility, configuring the project, and setting up CSS variables.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/packages/cli/README.md#_snippet_0

LANGUAGE: bash
CODE:
```
npx shadcn-svelte init
```

----------------------------------------

TITLE: Define CSS Variables for Light and Dark Themes
DESCRIPTION: This CSS snippet defines a set of custom properties (CSS variables) for a UI theme, supporting both light and dark modes. It specifies colors for various components like background, foreground, cards, popovers, primary, secondary, muted, accent, destructive elements, borders, inputs, rings, and chart colors, all using the OKLCH color space. The variables are applied to the `:root` pseudo-class for the light theme and to the `.dark` class for the dark theme.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/theming.md#_snippet_8

LANGUAGE: css
CODE:
```
:root {
  --radius: 0.625rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.13 0.028 261.692);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.13 0.028 261.692);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.13 0.028 261.692);
  --primary: oklch(0.21 0.034 264.665);
  --primary-foreground: oklch(0.985 0.002 247.839);
  --secondary: oklch(0.967 0.003 264.542);
  --secondary-foreground: oklch(0.21 0.034 264.665);
  --muted: oklch(0.967 0.003 264.542);
  --muted-foreground: oklch(0.551 0.027 264.364);
  --accent: oklch(0.967 0.003 264.542);
  --accent-foreground: oklch(0.21 0.034 264.665);
  --destructive: oklch(0.577 0.245 27.325);
  --border: oklch(0.928 0.006 264.531);
  --input: oklch(0.928 0.006 264.531);
  --ring: oklch(0.707 0.022 261.325);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0.002 247.839);
  --sidebar-foreground: oklch(0.13 0.028 261.692);
  --sidebar-primary: oklch(0.21 0.034 264.665);
  --sidebar-primary-foreground: oklch(0.985 0.002 247.839);
  --sidebar-accent: oklch(0.967 0.003 264.542);
  --sidebar-accent-foreground: oklch(0.21 0.034 264.665);
  --sidebar-border: oklch(0.928 0.006 264.531);
  --sidebar-ring: oklch(0.707 0.022 261.325);
}

.dark {
  --background: oklch(0.13 0.028 261.692);
  --foreground: oklch(0.985 0.002 247.839);
  --card: oklch(0.21 0.034 264.665);
  --card-foreground: oklch(0.985 0.002 247.839);
  --popover: oklch(0.21 0.034 264.665);
  --popover-foreground: oklch(0.985 0.002 247.839);
  --primary: oklch(0.928 0.006 264.531);
  --primary-foreground: oklch(0.21 0.034 264.665);
  --secondary: oklch(0.278 0.033 256.848);
  --secondary-foreground: oklch(0.985 0.002 247.839);
  --muted: oklch(0.278 0.033 256.848);
  --muted-foreground: oklch(0.707 0.022 261.325);
  --accent: oklch(0.278 0.033 256.848);
  --accent-foreground: oklch(0.985 0.002 247.839);
  --destructive: oklch(0.704 0.191 22.216);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.551 0.027 264.364);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.034 264.665);
  --sidebar-foreground: oklch(0.985 0.002 247.839);
  --sidebar-primary: oklch(0.488 0.243 264.376);
  --sidebar-primary-foreground: oklch(0.985 0.002 247.839);
  --sidebar-accent: oklch(0.278 0.033 256.848);
  --sidebar-accent-foreground: oklch(0.985 0.002 247.839);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.551 0.027 264.364);
}
```

----------------------------------------

TITLE: Importing UI Components in Svelte
DESCRIPTION: Demonstrates how to import UI components from the shadcn-svelte registry and other local Svelte components, preparing them for use in a Svelte application.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/index.md#_snippet_0

LANGUAGE: Svelte
CODE:
```
import * as Accordion from '$lib/registry/ui/accordion/index.js';
import Callout from '$lib/components/callout.svelte';
```

----------------------------------------

TITLE: Implement Pagination in Svelte Data Table Component
DESCRIPTION: Updates the main `DataTable.svelte` component to include pagination functionality using `@tanstack/table-core`. It initializes `pagination` state and configures the table instance with `getPaginationRowModel` for automatic row pagination.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_10

LANGUAGE: svelte
CODE:
```
<script lang="ts" generics="TData, TValue">
  import {
    type ColumnDef,
    type PaginationState,
    getCoreRowModel,
    getPaginationRowModel,
  } from "@tanstack/table-core";

  type DataTableProps<TData, TValue> = {
    data: TData[];
    columns: ColumnDef<TData, TValue>[];
  };

  let { data, columns }: DataTableProps<TData, TValue> = $props();

  let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });

  const table = createSvelteTable({
    get data() {
      return data;
    },
    columns,
    state: {
      get pagination() {
        return pagination;
      },
    },
    onPaginationChange: (updater) => {
      if (typeof updater === "function") {
        pagination = updater(pagination);
      } else {
        pagination = updater;
      }
    },
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
  });
</script>
```

----------------------------------------

TITLE: Configure components.json via CLI Prompts
DESCRIPTION: Review the interactive prompts from the `shadcn-svelte` CLI to configure `components.json`, defining crucial settings like base color, global CSS file path, and import aliases for various project directories.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/installation/vite.md#_snippet_5

LANGUAGE: cli-output
CODE:
```
Which base color would you like to use? › Slate
Where is your global CSS file? (this file will be overwritten) › src/app.css
Configure the import alias for lib: › $lib
Configure the import alias for components: › $lib/components
Configure the import alias for utils: › $lib/utils
Configure the import alias for hooks: › $lib/hooks
Configure the import alias for ui: › $lib/components/ui
```

----------------------------------------

TITLE: Update utils.ts for Svelte 5 Utility Functions
DESCRIPTION: This snippet shows the updated `src/lib/utils.ts` file, which now primarily exports the `cn` function for class merging and several utility types (`WithoutChild`, `WithoutChildren`, `WithoutChildrenOrChild`, `WithElementRef`) for Svelte 5 component development. It notes the removal of `flyAndScale` and advises caution before updating.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/migration/svelte-5.md#_snippet_2

LANGUAGE: TypeScript
CODE:
```
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
  ? Omit<T, "children">
  : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
  ref?: U | null;
};

```

----------------------------------------

TITLE: Update Svelte Data Table for Sorting Functionality
DESCRIPTION: This snippet shows the necessary updates to the main Svelte data table component to enable sorting. It introduces `SortingState` and `onSortingChange` handlers, along with `getSortedRowModel` from `@tanstack/table-core` to manage and apply sorting logic to the table data.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_13

LANGUAGE: svelte
CODE:
```
<script lang="ts" generics="TData, TValue">
  import {
    type ColumnDef,
    type PaginationState,
    type SortingState,
    getCoreRowModel,
    getPaginationRowModel,
    getSortedRowModel,
  } from "@tanstack/table-core";

  let { columns, data }: DataTableProps<TData, TValue> = $props();

  let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
  let sorting = $state<SortingState>([]);

  const table = createSvelteTable({
    get data() {
      return data;
    },
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onSortingChange: (updater) => {
      if (typeof updater === "function") {
        sorting = updater(sorting);
      } else {
        sorting = updater;
      }
    },
    onPaginationChange: (updater) => {
      if (typeof updater === "function") {
        pagination = updater(pagination);
      } else {
        pagination = updater;
      }
    },
    state: {
      get pagination() {
        return pagination;
      },
      get sorting() {
        return sorting;
      },
    },
  });
</script>
```

----------------------------------------

TITLE: Basic Carousel Usage in Svelte
DESCRIPTION: Demonstrates the fundamental structure of the Carousel component, including importing the components and arranging the `Carousel.Root`, `Carousel.Content`, `Carousel.Item`, `Carousel.Previous`, and `Carousel.Next` elements to create a functional carousel.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/carousel.md#_snippet_2

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import * as Carousel from "$lib/components/ui/carousel/index.js";
</script>

<Carousel.Root>
  <Carousel.Content>
    <Carousel.Item>...</Carousel.Item>
    <Carousel.Item>...</Carousel.Item>
    <Carousel.Item>...</Carousel.Item>
  </Carousel.Content>
  <Carousel.Previous />
  <Carousel.Next />
</Carousel.Root>
```

----------------------------------------

TITLE: Basic Alert Dialog Usage in Svelte
DESCRIPTION: Demonstrates how to import and use the Alert Dialog component in a Svelte application, showcasing its structure with root, trigger, content, header, footer, title, description, cancel, and action elements.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/alert-dialog.md#_snippet_2

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";
</script>

<AlertDialog.Root>
  <AlertDialog.Trigger>Open</AlertDialog.Trigger>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
      <AlertDialog.Description>
        This action cannot be undone. This will permanently delete your account
        and remove your data from our servers.
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action>Continue</AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
```

----------------------------------------

TITLE: Loading Data and Rendering DataTable in SvelteKit Page
DESCRIPTION: This section demonstrates how to load data in a SvelteKit `+page.server.ts` file and then pass it to the `<DataTable />` component in the `+page.svelte` file for rendering. The `load` function fetches payment data, which is then made available to the page component.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_6

LANGUAGE: ts
CODE:
```
export async function load() {
  // logic to fetch payments data here
  const payments = await getPayments();
  return {
    payments,
  };
}
```

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import DataTable from "./data-table.svelte";
  import { columns } from "./columns.js";

  let { data } = $props();
</script>

<DataTable {data} {columns} />
```

----------------------------------------

TITLE: Define Payment Data Type and Sample Data
DESCRIPTION: Defines the 'Payment' type structure, including 'id', 'amount', 'status', and 'email' fields, and provides an example array of 'Payment' objects for demonstration purposes.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_0

LANGUAGE: TypeScript
CODE:
```
type Payment = {
  id: string;
  amount: number;
  status: "pending" | "processing" | "success" | "failed";
  email: string;
};

export const data: Payment[] = [
  {
    id: "728ed52f",
    amount: 100,
    status: "pending",
    email: "m@example.com",
  },
  {
    id: "489e1d42",
    amount: 125,
    status: "processing",
    email: "example@gmail.com",
  },
  // ...
];
```

----------------------------------------

TITLE: Configure Data Table Column for Sorting in TypeScript
DESCRIPTION: This TypeScript snippet demonstrates how to define a column in the data table schema to be sortable. It uses `accessorKey` for the 'email' field and sets the `header` property to render a custom `DataTableEmailButton` component, passing `column.getToggleSortingHandler()` to enable sorting on click.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_14

LANGUAGE: ts
CODE:
```
import type { ColumnDef } from "@tanstack/table-core";
import { renderComponent } from "$lib/components/ui/data-table/index.js";
import DataTableEmailButton from "./data-table-email-button.svelte";

export const columns: ColumnDef<Payment>[] = [
  // ...
  {
    accessorKey: "email",
    header: ({ column }) =>
      renderComponent(DataTableEmailButton, {
        onclick: column.getToggleSortingHandler(),
      }),
  },
];
```

----------------------------------------

TITLE: Initialize shadcn-svelte CLI
DESCRIPTION: Executes the shadcn-svelte initialization command, which begins the interactive setup process for configuring the library within your SvelteKit project.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/installation/sveltekit.md#_snippet_3

LANGUAGE: bash
CODE:
```
npx shadcn-svelte@latest init
```

----------------------------------------

TITLE: Integrate Svelte Form Component into a Page
DESCRIPTION: This Svelte page component demonstrates how to integrate the previously defined `SettingsForm` component. It retrieves page data, including the form object, from SvelteKit's `load` function and passes it as a prop to the form component for rendering and interaction.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/form.md#_snippet_5

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import type { PageData } from "./$types.js";
  import SettingsForm from "./settings-form.svelte";
  let { data }: { data: PageData } = $props();
</script>

<SettingsForm {data} />
```

----------------------------------------

TITLE: Creating a Reusable Svelte DataTable Component
DESCRIPTION: This Svelte component defines a generic DataTable that can render any data based on provided column definitions. It utilizes `@tanstack/table-core` for table logic and custom UI components for rendering. It accepts `columns` and `data` as props.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/data-table.md#_snippet_5

LANGUAGE: svelte
CODE:
```
<script lang="ts" generics="TData, TValue">
  import { type ColumnDef, getCoreRowModel } from "@tanstack/table-core";
  import {
    createSvelteTable,
    FlexRender,
  } from "$lib/components/ui/data-table/index.js";
  import * as Table from "$lib/components/ui/table/index.js";

  type DataTableProps<TData, TValue> = {
    columns: ColumnDef<TData, TValue>[];
    data: TData[];
  };

  let { data, columns }: DataTableProps<TData, TValue> = $props();

  const table = createSvelteTable({
    get data() {
      return data;
    },
    columns,
    getCoreRowModel: getCoreRowModel(),
  });
</script>

<div class="rounded-md border">
  <Table.Root>
    <Table.Header>
      {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
        <Table.Row>
          {#each headerGroup.headers as header (header.id)}
            <Table.Head>
              {#if !header.isPlaceholder}
                <FlexRender
                  content={header.column.columnDef.header}
                  context={header.getContext()}
                />
              {/if}
            </Table.Head>
          {/each}
        </Table.Row>
      {/each}
    </Table.Header>
    <Table.Body>
      {#each table.getRowModel().rows as row (row.id)}
        <Table.Row data-state={row.getIsSelected() && "selected"}>
          {#each row.getVisibleCells() as cell (cell.id)}
            <Table.Cell>
              <FlexRender
                content={cell.column.columnDef.cell}
                context={cell.getContext()}
              />
            </Table.Cell>
          {/each}
        </Table.Row>
      {:else}
        <Table.Row>
          <Table.Cell colspan={columns.length} class="h-24 text-center">
            No results.
          </Table.Cell>
        </Table.Row>
      {/each}
    </Table.Body>
  </Table.Root>
</div>
```

----------------------------------------

TITLE: Implement Svelte Combobox Component Usage
DESCRIPTION: This Svelte code snippet demonstrates the usage of the Combobox component, which combines Popover and Command components for an autocomplete input. It allows users to select a framework from a predefined list, supports searching, and handles UI interactions like opening/closing the dropdown and focusing the trigger button for improved accessibility.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/combobox.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import CheckIcon from "@lucide/svelte/icons/check";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { cn } from "$lib/utils.js";

  const frameworks = [
    {
      value: "sveltekit",
      label: "SvelteKit",
    },
    {
      value: "next.js",
      label: "Next.js",
    },
    {
      value: "nuxt.js",
      label: "Nuxt.js",
    },
    {
      value: "remix",
      label: "Remix",
    },
    {
      value: "astro",
      label: "Astro",
    },
  ];

  let open = $state(false);
  let value = $state("");
  let triggerRef = $state<HTMLButtonElement>(null!);

  const selectedValue = $derived(
    frameworks.find((f) => f.value === value)?.label
  );

  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger() {
    open = false;
    tick().then(() => {
      triggerRef.focus();
    });
  }
</script>

<Popover.Root bind:open>
  <Popover.Trigger bind:ref={triggerRef}>
    {#snippet child({ props })}
      <Button
        variant="outline"
        class="w-[200px] justify-between"
        {...props}
        role="combobox"
        aria-expanded={open}
      >
        {selectedValue || "Select a framework..."}
        <ChevronsUpDownIcon class="ml-2 size-4 shrink-0 opacity-50" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-[200px] p-0">
    <Command.Root>
      <Command.Input placeholder="Search framework..." />
      <Command.List>
        <Command.Empty>No framework found.</Command.Empty>
        <Command.Group>
          {#each frameworks as framework}
            <Command.Item
              value={framework.value}
              onSelect={() => {
                value = framework.value;
                closeAndFocusTrigger();
              }}
            >
              <CheckIcon
                class={cn(
                  "mr-2 size-4",
                  value !== framework.value && "text-transparent"
                )}
              />
              {framework.label}
            </Command.Item>
          {/each}
        </Command.Group>
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
```

----------------------------------------

TITLE: Configure CSS Variables and Tailwind Theme in app.css
DESCRIPTION: This CSS snippet defines custom properties for light and dark themes using HSL color values, imports Tailwind CSS and `tw-animate-css`, and sets up an inline `@theme` directive to map CSS variables to Tailwind's design tokens. It also includes base layer styles for global application.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/migration/tailwind-v4.md#_snippet_9

LANGUAGE: css
CODE:
```
@import "tailwindcss";
@import "tw-animate-css";

@custom-variant dark (&:is(.dark *));

:root {
  --background: hsl(0 0% 100%) /* <- Wrap in HSL */;
  --foreground: hsl(240 10% 3.9%);
  --muted: hsl(240 4.8% 95.9%);
  --muted-foreground: hsl(240 3.8% 46.1%);
  --popover: hsl(0 0% 100%);
  --popover-foreground: hsl(240 10% 3.9%);
  --card: hsl(0 0% 100%);
  --card-foreground: hsl(240 10% 3.9%);
  --border: hsl(240 5.9% 90%);
  --input: hsl(240 5.9% 90%);
  --primary: hsl(240 5.9% 10%);
  --primary-foreground: hsl(0 0% 98%);
  --secondary: hsl(240 4.8% 95.9%);
  --secondary-foreground: hsl(240 5.9% 10%);
  --accent: hsl(240 4.8% 95.9%);
  --accent-foreground: hsl(240 5.9% 10%);
  --destructive: hsl(0 72.2% 50.6%);
  --destructive-foreground: hsl(0 0% 98%);
  --ring: hsl(240 10% 3.9%);
  --sidebar: hsl(0 0% 98%);
  --sidebar-foreground: hsl(240 5.3% 26.1%);
  --sidebar-primary: hsl(240 5.9% 10%);
  --sidebar-primary-foreground: hsl(0 0% 98%);
  --sidebar-accent: hsl(240 4.8% 95.9%);
  --sidebar-accent-foreground: hsl(240 5.9% 10%);
  --sidebar-border: hsl(220 13% 91%);
  --sidebar-ring: hsl(217.2 91.2% 59.8%);

  --radius: 0.5rem;
}

.dark {
  --background: hsl(240 10% 3.9%);
  --foreground: hsl(0 0% 98%);
  --muted: hsl(240 3.7% 15.9%);
  --muted-foreground: hsl(240 5% 64.9%);
  --popover: hsl(240 10% 3.9%);
  --popover-foreground: hsl(0 0% 98%);
  --card: hsl(240 10% 3.9%);
  --card-foreground: hsl(0 0% 98%);
  --border: hsl(240 3.7% 15.9%);
  --input: hsl(240 3.7% 15.9%);
  --primary: hsl(0 0% 98%);
  --primary-foreground: hsl(240 5.9% 10%);
  --secondary: hsl(240 3.7% 15.9%);
  --secondary-foreground: hsl(0 0% 98%);
  --accent: hsl(240 3.7% 15.9%);
  --accent-foreground: hsl(0 0% 98%);
  --destructive: hsl(0 62.8% 30.6%);
  --destructive-foreground: hsl(0 0% 98%);
  --ring: hsl(240 4.9% 83.9%);
  --sidebar: hsl(240 5.9% 10%);
  --sidebar-foreground: hsl(240 4.8% 95.9%);
  --sidebar-primary: hsl(224.3 76.3% 48%);
  --sidebar-primary-foreground: hsl(0 0% 100%);
  --sidebar-accent: hsl(240 3.7% 15.9%);
  --sidebar-accent-foreground: hsl(240 4.8% 95.9%);
  --sidebar-border: hsl(240 3.7% 15.9%);
  --sidebar-ring: hsl(217.2 91.2% 59.8%);
}

@theme inline {
  /* Radius (for rounded-*) */
  --radius-sm: calc(var(--radius) - 4px);
  --radius-md: calc(var(--radius) - 2px);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) + 4px);

  /* Colors */
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-popover: var(--popover);
  --color-popover-foreground: var(--popover-foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-ring: var(--ring);
  --color-radius: var(--radius);
  --color-sidebar: var(--sidebar);
  --color-sidebar-foreground: var(--sidebar-foreground);
  --color-sidebar-primary: var(--sidebar-primary);
  --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
  --color-sidebar-accent: var(--sidebar-accent);
  --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
  --color-sidebar-border: var(--sidebar-border);
  --color-sidebar-ring: var(--sidebar-ring);
}

@layer base {
  * {
    @apply border-border;
  }

  body {
    @apply bg-background text-foreground;
  }
}
```

----------------------------------------

TITLE: Basic Svelte Breadcrumb Usage
DESCRIPTION: Demonstrates the fundamental structure and usage of the Breadcrumb component in a Svelte application, including Root, List, Item, Link, and Separator elements to create a hierarchical navigation path.
SOURCE: https://github.com/huntabyte/shadcn-svelte/blob/main/docs/content/components/breadcrumb.md#_snippet_1

LANGUAGE: svelte
CODE:
```
<script lang="ts">
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
</script>

<Breadcrumb.Root>
  <Breadcrumb.List>
    <Breadcrumb.Item>
      <Breadcrumb.Link href="/">Home</Breadcrumb.Link>
    </Breadcrumb.Item>
    <Breadcrumb.Separator />
    <Breadcrumb.Item>
      <Breadcrumb.Link href="/components">Components</Breadcrumb.Link>
    </Breadcrumb.Item>
    <Breadcrumb.Separator />
    <Breadcrumb.Item>
      <Breadcrumb.Page>Breadcrumb</Breadcrumb.Page>
    </Breadcrumb.Item>
  </Breadcrumb.List>
</Breadcrumb.Root>
```
