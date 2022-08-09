# @distive/react

![npm](https://img.shields.io/npm/v/@distive/react)

## Getting set up

To use Distive, go to the [Distive Dashboard](https://dashboard.distive.com), create a new service(canister), and copy your `Canister ID`.


## Installing @distive/react

```sh
npm install @distive/react
```

If you're planning to add authentication, you'll need to install:
```sh
npm install @dfinity/auth-client
```

The package exports a `initDistiveHook` function that initializes the Distive Hook.

```tsx
const useDistive = initDistiveHook({
  serverId: '<your-canister-id>',
})._unsafeUnwrap() //read more about neverthrow at https://www.npmjs.com/package/neverthrow for proper error handling

```

## Using useDistive hook

`useDistive` returns an object with the following properties:

### DistiveHook Properties
Property | Type | Description
--- | --- | ---
loading | `boolean` | True when a top level page is being requested (when the loadMore function is called).
loadMore | `() => void` | Call this function to load more data.
addPost | `(input: AddPostInput) => void` | Add a post
updatePost | `(input: UpdatePostInput) => void` | Update a post
removePost | `(postID: string) => void` | Remove a post
thread | `ThreadState` | Contains object of all posts keyed by their ID. [See the ThreadState type](#thread-state) for more information.
toggleMetadata | `(input: ToggleMetadataInput) => void` | Only works for Authenticated users. Used to implement voting, reactions and flagging. [See Metadata](#metadata) for more information.
remainingPostCount | `number` | The number of posts not yet loaded in the thread.
error | `String` | If an error occurs, it will be stored here. Empty otherwise.


### DistiveHook Parameters
Parameter | Type | Description
--- | --- | ---
channelID | `string` | The ID of the channel to retrieve threads from, a new channel with this ID will be created if it doesn't exist.
initialPage? | `Page` | (Optional) The initial page to load. 
limit? | `number` | Defaults to 10.
onPostStatusChange? | `(payload: PostStatusCallback)=>void` | (Optional) Callback when the status of a post changes. [See PostStatusCallback](#post-status-callback) for more information.


### Example usage in a React component

```tsx
import { useComments, CommentStatus } from '@distive/react';


  const {
    thread,
    removePost,
    addPost,
    updatePost,
    loading,
    loadMore,
    toggleMetadata,
    remainingPostCount,
  } = useDistive({
    channelID: channelId,
    initialPage: page,
    limit: 8,
    onPostStatusChange: function ({ id, status, type, message }): void {
      console.log(`Post ${id} changed status to ${status}`);
      }
 })
  


```

## Examples

- <a href="https://codesandbox.io/s/distive-flat-example-chakra-ui-t2lv45?file=/src/index.tsx">ReactJS + Chakra UI Demo Flat (none-nested threads)</a>

## API Reference

