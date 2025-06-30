### SVELTE 5 DOCUMENTATION ###

TITLE: Creating Basic Reactive State with $state in Svelte
DESCRIPTION: Demonstrates how to declare a reactive variable `count` using `$state(0)` in a Svelte component. It shows that the variable can be updated directly like a regular number, and the UI automatically reflects changes.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/02-$state.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);
</script>

<button onclick={() => count++}>
	clicks: {count}
</button>
```

----------------------------------------

TITLE: Creating a Clickable Button in Svelte
DESCRIPTION: This Svelte component demonstrates a basic interactive element. It defines a JavaScript function `greet` that displays an alert, an HTML button that triggers this function on click, and inline CSS to style the button. This snippet illustrates Svelte's approach to combining script, markup, and style within a single component file.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/01-introduction/01-overview.md#_snippet_0

LANGUAGE: Svelte
CODE:
```
<!--- file: App.svelte --->
<script>
	function greet() {
		alert('Welcome to Svelte!');
	}
</script>

<button onclick={greet}>click me</button>

<style>
	button {
		font-size: 2em;
	}
</style>
```

----------------------------------------

TITLE: Dynamic Component Rendering without `<svelte:component>` (Svelte)
DESCRIPTION: This Svelte snippet demonstrates the Svelte 5 capability to render dynamic components directly by assigning a component reference to a variable and using that variable as a tag. This change makes the `<svelte:component>` element largely unnecessary for dynamic rendering, simplifying component composition.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_36

LANGUAGE: svelte
CODE:
```
<script>
	import A from './A.svelte';
	import B from './B.svelte';

	let Thing = $state();
</script>

<select bind:value={Thing}>
	<option value={A}>A</option>
	<option value={B}>B</option>
</select>

<!-- these are equivalent -->
<Thing />
<svelte:component this={Thing} />
```

----------------------------------------

TITLE: Initializing Reactive State with $state in Svelte
DESCRIPTION: This snippet demonstrates how to declare a reactive variable in Svelte 5 using the `$state` rune. Unlike Svelte 4's implicit reactivity with `let`, `$state` explicitly marks a variable as reactive, allowing direct reads and writes without wrappers. This change enables consistent reactivity patterns outside the top level of components.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);
</script>
```

----------------------------------------

TITLE: Binding Input Value with Svelte $state
DESCRIPTION: This code demonstrates basic two-way data binding for an `<input>` element's `value` property using Svelte's `$state` reactive primitive. Changes in the input field will update the `message` variable, and vice versa, with its current value displayed in a paragraph.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/12-bind.md#_snippet_3

LANGUAGE: Svelte
CODE:
```
<script>
	let message = $state('hello');
</script>

<input bind:value={message} />
<p>{message}</p>
```

----------------------------------------

TITLE: Creating Derived State with $derived in Svelte
DESCRIPTION: This example illustrates how to define a derived reactive value in Svelte 5 using the `$derived` rune. It replaces the `$:` statement previously used for derivations in Svelte 4. The `$derived` rune ensures that `double` automatically updates when `count` changes, and its value is directly accessible.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_1

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);
	const double = $derived(count * 2);
</script>
```

----------------------------------------

TITLE: Basic Svelte Component Structure
DESCRIPTION: This snippet illustrates the fundamental structure of a Svelte component file. It shows the three optional sections: <script module> for module-level logic, <script> for instance-level logic, and <style> for component-scoped CSS, along with the main markup area.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/01-introduction/03-svelte-files.md#_snippet_0

LANGUAGE: svelte
CODE:
```
/// file: MyComponent.svelte
<script module>
	// module-level logic goes here
	// (you will rarely use this)
</script>

<script>
	// instance-level logic goes here
</script>

<!-- markup (zero or more items) goes here -->

<style>
	/* styles go here */
</style>
```

----------------------------------------

TITLE: Declaring Component Properties with $props in Svelte 5
DESCRIPTION: This snippet illustrates the fundamental change in Svelte 5 for declaring component properties. It shows how `export let` declarations from Svelte 4 are replaced by destructuring from the `$props()` rune, allowing for default values and required properties.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_3

LANGUAGE: svelte
CODE:
```
<script>
	export let optional = 'unset';
	export let required;
</script>
```

LANGUAGE: svelte
CODE:
```
<script>
	let { optional = 'unset', required } = $props();
</script>
```

----------------------------------------

TITLE: Synchronizing State with $derived in Svelte (Recommended)
DESCRIPTION: This example shows the recommended way to handle derived state in Svelte using the `$derived` rune. It efficiently computes `doubled` based on `count` without the need for an `$effect`, promoting a more declarative and performant approach to reactivity.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/04-$effect.md#_snippet_9

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);
	let doubled = $derived(count * 2);
</script>
```

----------------------------------------

TITLE: Svelte $state Rune Basic Declaration
DESCRIPTION: This snippet illustrates the basic declaration of reactive state using Svelte's `$state` rune. When you reference a variable declared with `$state`, you are accessing its current value, similar to how primitive values are accessed.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/02-$state.md#_snippet_13

LANGUAGE: javascript
CODE:
```
let a = $state(1);
let b = $state(2);
```

----------------------------------------

TITLE: Initializing a SvelteKit Project using npm
DESCRIPTION: This snippet provides the command-line steps to create a new SvelteKit application, navigate into its directory, install dependencies, and start the development server. It leverages `npx` for initial project scaffolding and `npm` for package management.
SOURCE: https://github.com/sveltejs/svelte/blob/main/packages/svelte/README.md#_snippet_0

LANGUAGE: bash
CODE:
```
npx sv create my-app
cd my-app
npm install
npm run dev
```

----------------------------------------

TITLE: Declaring a Basic Svelte Snippet
DESCRIPTION: This snippet demonstrates the fundamental syntax for declaring a Svelte snippet without any parameters. Snippets are reusable blocks of markup within components, defined using the `{#snippet ...}{/snippet}` block.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/06-snippet.md#_snippet_0

LANGUAGE: Svelte
CODE:
```
{#snippet name()}...{/snippet}
```

----------------------------------------

TITLE: Declaring Basic Derived State in Svelte
DESCRIPTION: This snippet demonstrates the fundamental use of the `$derived` rune to create a reactive variable `doubled` that automatically updates whenever its dependency `count` changes. It highlights how `$derived` ensures that `doubled` always reflects the current value of `count * 2` within the Svelte component.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/03-$derived.md#_snippet_0

LANGUAGE: Svelte
CODE:
```
<script>
	let count = $state(0);
	let doubled = $derived(count * 2);
</script>

<button onclick={() => count++}>
	{doubled}
</button>

<p>{count} doubled is {doubled}</p>
```

----------------------------------------

TITLE: Example of Svelte {#if} with {:else if} and {:else}
DESCRIPTION: This comprehensive example showcases the full conditional logic capabilities of Svelte's {#if} block, including {:else if} and {:else} clauses. It checks the 'porridge.temperature' and renders different messages based on whether it's too hot, too cold, or just right, demonstrating chained conditions and a default fallback.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/02-if.md#_snippet_4

LANGUAGE: svelte
CODE:
```
{#if porridge.temperature > 100}
	<p>too hot!</p>
{:else if 80 > porridge.temperature}
	<p>too cold!</p>
{:else}
	<p>just right!</p>
{/if}
```

----------------------------------------

TITLE: Creating Complex Derived State with $derived.by in Svelte
DESCRIPTION: This example illustrates `$derived.by`, which accepts a function for more complex derivations that don't fit a single expression. It calculates the `total` sum of numbers in a reactive array, demonstrating how the derived value updates automatically when elements are added to `numbers`.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/03-$derived.md#_snippet_1

LANGUAGE: Svelte
CODE:
```
<script>
	let numbers = $state([1, 2, 3]);
	let total = $derived.by(() => {
		let total = 0;
		for (const n of numbers) {
			total += n;
		}
		return total;
	});
</script>

<button onclick={() => numbers.push(numbers.length + 1)}>
	{numbers.join(' + ')} = {total}
</button>
```

----------------------------------------

TITLE: Handling Invalid Prop Mutations in Svelte Components
DESCRIPTION: This example demonstrates the `ownership_invalid_mutation` warning in Svelte, which occurs when a child component attempts to mutate a prop (`person`) passed from a parent (`App.svelte`) without explicit two-way binding. The `Child.svelte` component uses `bind:value` on properties of the `person` object, which is owned by `App.svelte`, leading to potential issues in reasoning about state changes. The suggested fixes include using callback props or marking the prop as `$bindable`.
SOURCE: https://github.com/sveltejs/svelte/blob/main/packages/svelte/messages/client-warnings/warnings.md#_snippet_3

LANGUAGE: svelte
CODE:
```
<!--- file: App.svelte --->
<script>
	import Child from './Child.svelte';
	let person = $state({ name: 'Florida', surname: 'Man' });
</script>

<Child {person} />
```

LANGUAGE: svelte
CODE:
```
<!--- file: Child.svelte --->
<script>
	let { person } = $props();
</script>

<input bind:value={person.name}>
<input bind:value={person.surname}>
```

----------------------------------------

TITLE: Initializing State with $state Rune in Svelte
DESCRIPTION: This snippet demonstrates the use of the `$state` rune in Svelte 5+ to declare a reactive state variable. The `$state` rune is a built-in Svelte keyword that automatically makes `message` reactive, meaning any changes to it will trigger updates in the UI. Unlike regular JavaScript functions, runes do not need to be imported and cannot be assigned or passed as values.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/01-what-are-runes.md#_snippet_0

LANGUAGE: javascript
CODE:
```
let message = $state('hello');
```

----------------------------------------

TITLE: Scoping CSS in Svelte Components
DESCRIPTION: This snippet illustrates how Svelte automatically scopes CSS rules defined within a <style> block to the component itself. This ensures that styles applied to elements like <p> will only affect paragraphs within this specific component, preventing global style conflicts.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/01-introduction/03-svelte-files.md#_snippet_2

LANGUAGE: svelte
CODE:
```
<style>
	p {
		/* this will only affect <p> elements in this component */
		color: burlywood;
	}
</style>
```

----------------------------------------

TITLE: Managing Component State with `$state` in Svelte 5 (JavaScript)
DESCRIPTION: This snippet illustrates how to manage reactive properties in Svelte 5 components, replacing the `$set` method. It uses the new `$state` rune to create a reactive object for component properties. This reactive `props` object is then passed to the `mount` function, and its properties can be directly manipulated to trigger updates.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_29

LANGUAGE: javascript
CODE:
```
import { mount } from 'svelte';
import App from './App.svelte'

const props = $state({ foo: 'bar' });
const app = mount(App, { target: document.getElementById("app"), props });
props.foo = 'baz';
```

----------------------------------------

TITLE: Overriding Derived Values for Optimistic UI in Svelte
DESCRIPTION: This snippet demonstrates how to temporarily override a `$derived` value for optimistic UI updates. It immediately increments the `likes` count on click, then attempts to update the server, rolling back the local change if the server operation fails, providing instant user feedback.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/03-$derived.md#_snippet_2

LANGUAGE: Svelte
CODE:
```
<script>
	let { post, like } = $props();

	let likes = $derived(post.likes);

	async function onclick() {
		// increment the `likes` count immediately...
		likes += 1;

		// and tell the server, which will eventually update `post`
		try {
			await like();
		} catch {
			// failed! roll back the change
			likes -= 1;
		}
	}
</script>

<button {onclick}>🧡 {likes}</button>
```

----------------------------------------

TITLE: Implementing Deeply Reactive State with $state Proxies
DESCRIPTION: Explains how `$state` creates a deeply reactive proxy when used with arrays or simple objects. This allows Svelte to track changes to nested properties and array methods, triggering granular UI updates.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/02-$state.md#_snippet_1

LANGUAGE: javascript
CODE:
```
let todos = $state([
	{
		done: false,
		text: 'add more todos'
	}
]);
```

----------------------------------------

TITLE: Accessing and Updating Writable Stores in Svelte
DESCRIPTION: This snippet demonstrates how to declare a writable store, access its value using the `$` prefix, and update it using both the `.set` method and direct assignment to the `$`-prefixed variable within a Svelte component. It shows the reactive nature of stores.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/06-runtime/01-stores.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<script>
	import { writable } from 'svelte/store';

	const count = writable(0);
	console.log($count); // logs 0

	count.set(1);
	console.log($count); // logs 1

	$count = 2;
	console.log($count); // logs 2
</script>
```

----------------------------------------

TITLE: Setting Reactive State in Svelte Context
DESCRIPTION: This Svelte snippet demonstrates how to store reactive state in context using Svelte's `$state` rune. A `counter` object is created as reactive state and then made available via `setContext` under the key 'counter', allowing multiple child components to access and react to its changes.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/06-runtime/02-context.md#_snippet_3

LANGUAGE: Svelte
CODE:
```
<script>
	import { setContext } from 'svelte';
	import Child from './Child.svelte';

	let counter = $state({
		count: 0
	});

	setContext('counter', counter);
</script>

<button onclick={() => counter.count += 1}>
	increment
</button>

<Child />
<Child />
<Child />
```

----------------------------------------

TITLE: Implementing Side Effects with $effect in Svelte
DESCRIPTION: This snippet shows how to create side effects in Svelte 5 using the `$effect` rune, which replaces the `$:` statement for side effects in Svelte 4. The provided callback function will execute whenever its dependencies (e.g., `count`) change, triggering an alert if `count` exceeds 5. Note that `$effect`'s execution timing differs from `$:`.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_2

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);

	$effect(() => {
		if (count > 5) {
			alert('Count is too high!');
		}
	});
</script>
```

----------------------------------------

TITLE: Bidirectional State Linking with Function Bindings in Svelte (Recommended)
DESCRIPTION: This snippet demonstrates the preferred method for bidirectional state linking in Svelte, combining `$derived` for one-way derivation and a function binding for updating the source state. This approach avoids the pitfalls of using `$effect` for synchronization, resulting in cleaner and more predictable reactivity.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/04-$effect.md#_snippet_11

LANGUAGE: svelte
CODE:
```
<script>
	const total = 100;
	let spent = $state(0);
	let left = $derived(total - spent);

	function updateLeft(left) {
		spent = total - left;
	}
</script>

<label>
	<input type="range" bind:value={spent} max={total} />
	{spent}/{total} spent
</label>

<label>
	<input type="range" bind:value={() => left, updateLeft} max={total} />
	{left}/{total} left
</label>
```

----------------------------------------

TITLE: Handling DOM Events in Svelte
DESCRIPTION: Shows the basic syntax for listening to DOM events in Svelte using `on:` prefixed attributes (e.g., `onclick`). The attribute's value is a JavaScript function that executes when the event fires.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/01-basic-markup.md#_snippet_10

LANGUAGE: svelte
CODE:
```
<button onclick={() => console.log('clicked')}>click me</button>
```

----------------------------------------

TITLE: Spreading All Props for Event Forwarding in Svelte 5
DESCRIPTION: This example demonstrates how Svelte 5's `$props()` rune, combined with JavaScript spread syntax, allows for concisely forwarding all properties, including event handlers, from a parent to a child component's underlying element. This replaces the need to explicitly list and forward each event or use `$$props` from Svelte 4.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_10

LANGUAGE: svelte
CODE:
```
<script>
</script>

<button {...$$props} on:click on:keydown on:all_the_other_stuff>
	click me
</button>
```

LANGUAGE: svelte
CODE:
```
<script>
	let props = $props();
</script>

<button {...props}>
	click me
</button>
```

----------------------------------------

TITLE: Passing Svelte Snippets as Explicit Component Props
DESCRIPTION: This example demonstrates how to pass Svelte snippets to child components as explicit props. Snippets are treated as values, allowing components to receive and render dynamic content provided by their parent, similar to content slots.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/06-snippet.md#_snippet_7

LANGUAGE: Svelte
CODE:
```
<script>
	import Table from './Table.svelte';

	const fruits = [
		{ name: 'apples', qty: 5, price: 2 },
		{ name: 'bananas', qty: 10, price: 1 },
		{ name: 'cherries', qty: 20, price: 0.5 }
	];
</script>

{#snippet header()}
	<th>fruit</th>
	<th>qty</th>
	<th>price</th>
	<th>total</th>
{/snippet}

{#snippet row(d)}
	<td>{d.name}</td>
	<td>{d.qty}</td>
	<td>{d.price}</td>
	<td>{d.qty * d.price}</td>
{/snippet}

<Table data={fruits} {header} {row} />
```

----------------------------------------

TITLE: Using Component Tags in Svelte
DESCRIPTION: Demonstrates how Svelte components are declared and used within markup, distinguishing them from standard HTML elements by their capitalized tag names. It shows importing a Svelte component and then rendering it.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/01-basic-markup.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<script>
	import Widget from './Widget.svelte';
</script>

<div>
	<Widget />
</div>
```

----------------------------------------

TITLE: Typing Component Properties with $props
DESCRIPTION: This example demonstrates how to define and type component properties using an interface and destructuring `$props()`. It covers various property types, including required, optional, snippet (using `Snippet` type), and event handler functions, ensuring type safety for component inputs.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/03-typescript.md#_snippet_3

LANGUAGE: Svelte
CODE:
```
<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		requiredProperty: number;
		optionalProperty?: boolean;
		snippetWithStringArgument: Snippet<[string]>;
		eventHandler: (arg: string) => void;
		[key: string]: unknown;
	}

	let {
		requiredProperty,
		optionalProperty,
		snippetWithStringArgument,
		eventHandler,
		...everythingElse
	}: Props = $props();
</script>

<button onclick={() => eventHandler('clicked button')}>
	{@render snippetWithStringArgument('hello')}
</button>
```

----------------------------------------

TITLE: Attaching DOM Event Listeners in Svelte 5
DESCRIPTION: This snippet shows the updated syntax for attaching DOM event listeners in Svelte 5. Instead of the `on:` directive used in Svelte 4, event handlers are now treated as regular properties on elements, simplifying the syntax.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_5

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);
</script>

<button on:click={() => count++}>
	clicks: {count}
</button>
```

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);
</script>

<button click={() => count++}>
	clicks: {count}
</button>
```

----------------------------------------

TITLE: Mounting Svelte Components with Reactive Props in Runes Mode JS/TS
DESCRIPTION: This JavaScript example illustrates how to mount a Svelte component (`App.svelte`) and manage its reactive properties using the `$state` rune in `.svelte.js` or `.svelte.ts` files. It demonstrates how to initialize and update reactive props, contrasting with the direct property assignment of Svelte 4.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_40

LANGUAGE: JavaScript
CODE:
```
import { mount } from 'svelte';
import App from './App.svelte'

const props = $state({ foo: 'bar' });
const app = mount(App, { target: document.getElementById("app"), props });
props.foo = 'baz';
```

----------------------------------------

TITLE: Handling Non-Reactive Updates in Svelte
DESCRIPTION: This snippet demonstrates a common issue where a variable declared without `$state` is reassigned, failing to trigger updates in the UI. It shows how to declare a reactive variable using `$state` to ensure correct UI updates. The warning `non_reactive_update` is thrown when a non-$state variable is updated and read in a reactive context.
SOURCE: https://github.com/sveltejs/svelte/blob/main/packages/svelte/messages/compile-warnings/script.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<script>
	let reactive = $state('reactive');
	let stale = 'stale';
</script>

<p>This value updates: {reactive}</p>
<p>This value does not update: {stale}</p>

<button onclick={() => {
	stale = 'updated';
	reactive = 'updated';
}}>update</button>
```

----------------------------------------

TITLE: Passing Reactive State Proxy as Props in Svelte
DESCRIPTION: This Svelte snippet from `App.svelte` demonstrates passing a reactive state proxy object (created with `$state`) as a prop to a child component. This allows the child to receive a reference to the parent's reactive state, enabling shared reactivity.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/05-$props.md#_snippet_10

LANGUAGE: svelte
CODE:
```
<!--- file: App.svelte --->
<script>
	import Child from './Child.svelte';

	let object = $state({count: 0});
</script>

<Child {object} />
```

----------------------------------------

TITLE: Managing Intervals with $effect and Teardown in Svelte
DESCRIPTION: This example illustrates how to use `$effect` to manage a `setInterval` timer that increments a `count` state variable. It showcases the use of a teardown function, which `clearInterval`s the timer, ensuring proper cleanup when the `milliseconds` state changes (causing the effect to re-run) or when the component is destroyed. This prevents memory leaks and ensures correct interval behavior.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/04-$effect.md#_snippet_1

LANGUAGE: Svelte
CODE:
```
<script>
	let count = $state(0);
	let milliseconds = $state(1000);

	$effect(() => {
		// This will be recreated whenever `milliseconds` changes
		const interval = setInterval(() => {
			count += 1;
		}, milliseconds);

		return () => {
			// if a teardown function is provided, it will run
			// a) immediately before the effect re-runs
			// b) when the component is destroyed
			clearInterval(interval);
		};
	});
</script>

<h1>{count}</h1>

<button onclick={() => (milliseconds *= 2)}>slower</button>
<button onclick={() => (milliseconds /= 2)}>faster</button>
```

----------------------------------------

TITLE: Adding Type Safety to Svelte Props with TypeScript Interface
DESCRIPTION: This Svelte snippet demonstrates a more structured approach to adding type safety using a dedicated TypeScript interface. Defining a `Props` interface separately improves readability and reusability, clearly outlining the expected types for component properties.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/05-$props.md#_snippet_15

LANGUAGE: svelte
CODE:
```
<script lang="ts">
	interface Props {
		adjective: string;
	}

	let { adjective }: Props = $props();
</script>
```

----------------------------------------

TITLE: Receiving Props in Svelte with $props()
DESCRIPTION: This Svelte snippet shows the basic method for receiving all passed props within a child component (`MyComponent.svelte`) using the `$props()` rune. The received props are assigned to a `props` variable, which can then be accessed using dot notation (e.g., `props.adjective`).
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/05-$props.md#_snippet_1

LANGUAGE: svelte
CODE:
```
<!--- file: MyComponent.svelte --->
<script>
	let props = $props();
</script>

<p>this component is {props.adjective}</p>
```

----------------------------------------

TITLE: Creating a New SvelteKit Project (Bash)
DESCRIPTION: This snippet demonstrates the command-line steps to initialize a new SvelteKit project. It uses `npx sv create` to scaffold the project, navigates into the new directory, installs necessary npm dependencies, and starts the development server.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/01-introduction/02-getting-started.md#_snippet_0

LANGUAGE: bash
CODE:
```
npx sv create myapp
cd myapp
npm install
npm run dev
```

----------------------------------------

TITLE: Passing Reactive State as Props in Svelte
DESCRIPTION: This Svelte snippet from `App.svelte` shows how a parent component can pass a reactive state variable (`count` declared with `$state`) as a prop to a child component (`Child.svelte`). Changes to `count` in the parent will automatically propagate to the child.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/05-$props.md#_snippet_6

LANGUAGE: svelte
CODE:
```
<!--- file: App.svelte --->
<script>
	import Child from './Child.svelte';

	let count = $state(0);
</script>

<button onclick={() => (count += 1)}>
	clicks (parent): {count}
</button>

<Child {count} />
```

----------------------------------------

TITLE: Parent Component Handling Callback Props in Svelte 5
DESCRIPTION: This snippet from `App.svelte` demonstrates how a parent component interacts with a child component (`Pump.svelte`) using callback props in Svelte 5. Instead of listening for dispatched events, the parent passes functions as properties to the child, which the child then invokes.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_7

LANGUAGE: svelte
CODE:
```
<!--- file: App.svelte --->
<script>
	import Pump from './Pump.svelte';

	let size = $state(15);
	let burst = $state(false);

	function reset() {
		size = 15;
		burst = false;
	}
</script>

<Pump
	on:inflate={(power) => {
		size += power.detail;
		if (size > 75) burst = true;
	}}
	on:deflate={(power) => {
		if (size > 0) size -= power.detail;
	}}
/>

{#if burst}
	<button onclick={reset}>new balloon</button>
	<span class="boom">💥</span>
{:else}
	<span class="balloon" style="scale: {0.01 * size}">
		🎈
	</span>
{/if}
```

LANGUAGE: svelte
CODE:
```
<!--- file: App.svelte --->
<script>
	import Pump from './Pump.svelte';

	let size = $state(15);
	let burst = $state(false);

	function reset() {
		size = 15;
		burst = false;
	}
</script>

<Pump
	inflate={(power) => {
		size += power;
		if (size > 75) burst = true;
	}}
	deflate={(power) => {
		if (size > 0) size -= power;
	}}
/>

{#if burst}
	<button onclick={reset}>new balloon</button>
	<span class="boom">💥</span>
{:else}
	<span class="balloon" style="scale: {0.01 * size}">
		🎈
	</span>
{/if}
```

----------------------------------------

TITLE: Typing Svelte $state Variables
DESCRIPTION: This snippet demonstrates the basic way to explicitly type a `$state` variable in Svelte. It shows how to declare a variable with a specific type (e.g., `number`) and initialize it with a value, ensuring type consistency for reactive state.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/03-typescript.md#_snippet_7

LANGUAGE: TypeScript
CODE:
```
let count: number = $state(0);
```

----------------------------------------

TITLE: Destructuring Props in Svelte
DESCRIPTION: This Svelte snippet illustrates the more common and idiomatic way to receive specific props in a component: using JavaScript destructuring assignment with the `$props()` rune. This directly extracts the `adjective` prop into a local variable, making it more convenient to use.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/05-$props.md#_snippet_2

LANGUAGE: svelte
CODE:
```
<!--- file: MyComponent.svelte --->
<script>
	let { adjective } = $props();
</script>

<p>this component is {adjective}</p>
```

----------------------------------------

TITLE: Forwarding DOM Events to Parent Components in Svelte 5
DESCRIPTION: This snippet illustrates how Svelte 5 simplifies forwarding DOM events from a child component to its parent. Instead of using `on:click` on the child and then re-emitting, the child component simply accepts an `onclick` callback prop and applies it directly to the element, allowing the parent to define the handler.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_9

LANGUAGE: svelte
CODE:
```
<script>
</script>

<button on:click>
	click me
</button>
```

LANGUAGE: svelte
CODE:
```
<script>
	let { onclick } = $props();
</script>

<button {onclick}>
	click me
</button>
```

----------------------------------------

TITLE: Declaring a Svelte Snippet with Parameters
DESCRIPTION: This example illustrates how to declare a Svelte snippet that accepts multiple parameters. Parameters allow snippets to be dynamic and render different content based on the provided values, similar to function arguments.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/06-snippet.md#_snippet_1

LANGUAGE: Svelte
CODE:
```
{#snippet name(param1, param2, paramN)}...{/snippet}
```

----------------------------------------

TITLE: Refactoring Markup with Svelte Snippets and Render Tags
DESCRIPTION: This example demonstrates how to refactor duplicative markup using Svelte snippets. A `figure` snippet is defined to encapsulate the common image rendering logic, which is then reused with the `{@render}` tag, reducing code repetition and improving maintainability.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/06-snippet.md#_snippet_3

LANGUAGE: Svelte
CODE:
```
{#snippet figure(image)}
	<figure>
		<img src={image.src} alt={image.caption} width={image.width} height={image.height} />
		<figcaption>{image.caption}</figcaption>
	</figure>
{/snippet}

{#each images as image}
	{#if image.href}
		<a href={image.href}>
			{@render figure(image)}
		</a>
	{:else}
		{@render figure(image)}
	{/if}
{/each}
```

----------------------------------------

TITLE: Binding to Component Properties with bind:property in Svelte
DESCRIPTION: This snippet illustrates the general syntax for `bind:property`, enabling two-way data binding between a parent's `variable` and a child component's `property`. This allows changes to the property within the child to propagate back to the parent.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/12-bind.md#_snippet_23

LANGUAGE: Svelte
CODE:
```
bind:property={variable}
```

----------------------------------------

TITLE: Passing Props to a Svelte Component
DESCRIPTION: This Svelte snippet demonstrates how to pass properties (props) from a parent component (`App.svelte`) to a child component (`MyComponent.svelte`). Props are passed as attributes on the child component tag, similar to HTML attributes. The `adjective` prop is given the string value 'cool'.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/05-$props.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<!--- file: App.svelte --->
<script>
	import MyComponent from './MyComponent.svelte';
</script>

<MyComponent adjective="cool" />
```

----------------------------------------

TITLE: Basic Svelte Boundary Usage
DESCRIPTION: This snippet shows the basic syntax for using the `<svelte:boundary>` component in Svelte. It demonstrates how to wrap content with the boundary and optionally provide an `onerror` handler to manage errors that occur within its children.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/05-special-elements/01-svelte-boundary.md#_snippet_0

LANGUAGE: Svelte
CODE:
```
<svelte:boundary onerror={handler}>...</svelte:boundary>
```

----------------------------------------

TITLE: Understanding Update Propagation with $derived in Svelte
DESCRIPTION: This snippet demonstrates Svelte's push-pull reactivity model. It shows that a derived value (`large`) is only re-evaluated and causes downstream updates if its new value is referentially different from its previous one, even if its dependencies (`count`) change. This optimizes performance by preventing unnecessary DOM updates.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/03-$derived.md#_snippet_4

LANGUAGE: Svelte
CODE:
```
<script>
	let count = $state(0);
	let large = $derived(count > 10);
</script>

<button onclick={() => count++}>
	{large}
</button>
```

----------------------------------------

TITLE: Lazy Loading Svelte Component with Await Block
DESCRIPTION: This Svelte `{#await}` block demonstrates dynamic component loading using `import()`, rendering the component only after its module is successfully fetched. This pattern is crucial for code splitting and improving application performance by deferring component loading.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/05-await.md#_snippet_8

LANGUAGE: svelte
CODE:
```
{#await import('./Component.svelte') then { default: Component }}
	<Component />
{/await}
```

----------------------------------------

TITLE: Passing Data Up: Svelte 4 Slot `item={entry}` vs. Svelte 5 Snippet in Child
DESCRIPTION: This Svelte component shows the child-side implementation for passing data back up. It contrasts the Svelte 4 `<slot item={entry} />` syntax with the Svelte 5 `{@render item(entry)}` for iterating over items and rendering content. It also shows the migration for the 'empty' slot/snippet.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_23

LANGUAGE: Svelte
CODE:
```
<script>
	let { items, item, empty } = $props();
</script>

{#if items.length}
	<ul>
		{#each items as entry}
			<li>
				<slot item={entry} />
				{@render item(entry)}
			</li>
		{/each}
	</ul>
{:else}
	<slot name="empty" />
	{@render empty?.()}
{/if}
```

----------------------------------------

TITLE: Avoiding $effect for Simple State Synchronization in Svelte (Anti-pattern)
DESCRIPTION: This snippet demonstrates an anti-pattern where `$effect` is used to synchronize a derived state (`doubled`) with a base state (`count`). This approach is discouraged for simple derivations as it's less efficient and less idiomatic than using `$derived`.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/02-runes/04-$effect.md#_snippet_8

LANGUAGE: svelte
CODE:
```
<script>
	let count = $state(0);
	let doubled = $state();

	// don't do this!
	$effect(() => {
		doubled = count * 2;
	});
</script>
```

----------------------------------------

TITLE: Keyed Each Block Syntax in Svelte
DESCRIPTION: This syntax introduces a `key` expression to the `{#each}` block. The key uniquely identifies each list item, enabling Svelte to intelligently update the list by inserting, moving, and deleting items efficiently.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/03-each.md#_snippet_4

LANGUAGE: svelte
CODE:
```
{#each expression as name (key)}...{/each}
```

----------------------------------------

TITLE: Running Svelte 5 Migration Script
DESCRIPTION: This command line snippet shows how to automatically upgrade a Svelte project to Svelte 5 syntax using the provided migration script. It updates dependencies and migrates various syntax patterns like runes, event attributes, slot creations, and slot usages.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/07-misc/07-v5-migration-guide.md#_snippet_24

LANGUAGE: bash
CODE:
```
npx sv migrate svelte-5
```

----------------------------------------

TITLE: Migrating from svelte:component in Svelte 5 Runes Mode
DESCRIPTION: The `<svelte:component>` tag is deprecated in Svelte 5's runes mode because components are now dynamic by default, eliminating the need for this explicit tag to re-render components on change. Instead of `<svelte:component this={X}>`, you can directly use `<X>` or assign the dynamic component to a capitalized variable (e.g., `{@const Component = ...}` or `const Component = $derived(...)`) and then render it as `<Component />`. This simplifies dynamic component rendering logic.
SOURCE: https://github.com/sveltejs/svelte/blob/main/packages/svelte/messages/compile-warnings/template.md#_snippet_3

LANGUAGE: Svelte
CODE:
```
{#each items as item}
	---<svelte:component this={item.condition ? Y : Z} />---
	+++{@const Component = item.condition ? Y : Z}+++
	+++<Component />+++
{/each}
```

LANGUAGE: Svelte
CODE:
```
<script>
	// ...
	let condition = $state(false);
	+++const Component = $derived(condition ? Y : Z);+++
</script>

---<svelte:component this={condition ? Y : Z} />---
+++<Component />+++
```

----------------------------------------

TITLE: Defining Svelte Key Block Syntax
DESCRIPTION: This snippet illustrates the basic syntax of a Svelte key block. It defines a region whose content will be destroyed and recreated whenever the `expression` changes, ensuring a fresh state for its children.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/03-template-syntax/04-key.md#_snippet_0

LANGUAGE: Svelte
CODE:
```
<!- copy: false  ->
{#key expression}...{/key}
```

----------------------------------------

TITLE: Handling Each Block Item Reassignment and Binding in Svelte
DESCRIPTION: This section illustrates the correct and incorrect ways to reassign or bind to items within Svelte's {#each} blocks. In legacy mode, direct reassignment or binding to the entry variable was possible but led to bugs. In runes mode, this is forbidden, and instead, you must use the array and index (array[i]) to modify values, ensuring predictable behavior, especially with derived states.
SOURCE: https://github.com/sveltejs/svelte/blob/main/packages/svelte/messages/compile-errors/script.md#_snippet_0

LANGUAGE: svelte
CODE:
```
<script>
	let array = [1, 2, 3];
</script>

{#each array as entry}
	<!-- reassignment -->
	<button on:click={() => entry = 4}>change</button>

	<!-- binding -->
	<input bind:value={entry}>
{/each}
```

LANGUAGE: svelte
CODE:
```
<script>
	let array = $state([1, 2, 3]);
</script>

{#each array as entry, i}
	<!-- reassignment -->
	<button onclick={() => array[i] = 4}>change</button>

	<!-- binding -->
	<input bind:value={array[i]}>
{/each}
```

----------------------------------------

TITLE: Defining Shared State with Svelte 5 Runes
DESCRIPTION: This TypeScript snippet demonstrates how to define a shared reactive state object using Svelte 5's `$state` rune in a `.svelte.js` file. This approach is recommended for extracting logic and creating shared state with universal reactivity outside of components.
SOURCE: https://github.com/sveltejs/svelte/blob/main/documentation/docs/06-runtime/01-stores.md#_snippet_1

LANGUAGE: typescript
CODE:
```
/// file: state.svelte.js
export const userState = $state({
	name: 'name',
	/* ... */
});
```
