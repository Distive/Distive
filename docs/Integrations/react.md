# @distive/react

![npm](https://img.shields.io/npm/v/@distive/react)

## Getting set up

To use [Commont](https://www.commont.app/), you need to create a new account via
our [signup page](https://www.commont.app/signup). You can sign up using an
email and password or by using GitHub or Google. Once you have an account, you
can access the Commont dashboard. Initially, you'll see one iju9default project that
you can configure as you need.

👀 Read the [docs](https://www.commont.app/docs) for more information.

## Installing @commont/react

```sh
yarn add @commont/react commont # npm install @commont/react
```

The package exports a `useComments` hook that you can use to fetch the comments
for your project.

## Using useComments hook

`useComments` fetches comments from the backend on mount and whenever take or
skip change.

### Parameters

`useComments` takes an object with the following parameters:

- **projectId** — Your project ID.
- **topic** — Comments will be fetched for a particular topic, e.g.
  _my-post-about-cats_.
- **take** — Number of comments to fetch.
- **skip** — Number of comments to skip (offset).

### Example usage in a React component

```tsx
import { useComments, CommentStatus } from '@commont/react';

const Post = ({ projectId }) => {
  const { comments, count, loading, refetch, error } = useComments({
    projectId,
    topic: 'post-id'
    take: 10, skip: 0
  });

  return (
    <section>
      <h3>{count} comments</h3>
      {loading ? (
        <p>Loading...</p>
      ) : (
        <div>
          {comments.map(({ author, content, createdAt, status }) => (
            <article key={createdAt} className="bg-gray-100 rounded my-6 p-4">
              <div className="font-bold mb-2">
                {author} ・ {new Date(createdAt).toLocaleDateString()}
              </div>
              <p className="text-gray-700">{content}</p>
            </article>
          ))}
        </div>
      )}
    </section>
  )
}
```

## Examples

- <a href="https://codesandbox.io/s/commont-react-theme-ui-demo-osx9o">Demo with
  Theme UI</a>
- <a href="https://codesandbox.io/s/commont-react-demo-tailwind-pvhgw">Demo with
  Tailwind</a>
- <a href="https://codesandbox.io/s/commont-react-theme-ui-pagination-o4tg8">Demo
  with Theme UI — an example with pagination</a>

## API Reference

### UseCommentsComment

```ts
interface UseCommentsComment {
  topic: string;
  author: string;
  content: string;
  createdAt: string;
  status?: UseCommentsStatus;
  details?: Record<string, any>;
}
```

### UseCommentsStatus

When a user adds a new comment, it will be in one of four states:

- **sending** — add comment request is still pending.
- **added** — the comment was successfully added and is visible for other
  people.
- **delivered-awaiting-approval** — the comment was successfully added, but it's
  not yet visible for other people.
- **failed** — adding a comment was unsuccessful.

```ts
type UseCommentsStatus =
  | 'sending'
  | 'added'
  | 'delivered-awaiting-approval'
  | 'failed';
```

### UseCommentsParameters

```ts
interface UseCommentsParameters {
  projectId: string;
  topic: string;
  take?: number;
  skip?: number;
}
```

### UseCommentsResult

```ts
interface UseComentsResult {
  comments: UseCommentsComment[];
  addComment: ({
    content,
    author,
  }: Pick<UseCommentsComment, 'content' | 'author' | 'details'>) => void;
  refetch: () => void;
  count: number;
  loading: boolean;
  error: string | null;
}
```
