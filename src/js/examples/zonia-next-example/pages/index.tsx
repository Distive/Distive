import type { NextPage } from 'next'
import { useState } from 'react'
import { Page } from 'zomia'
import initZoniaHook from 'zomia-react'
import  { PostStatus } from 'zomia-react'
const useZonia = initZoniaHook({
  // serverId: "rrkah-fqaaa-aaaaa-aaaaq-cai"
  serverId: "rofub-iaaaa-aaaai-ab7da-cai"

})._unsafeUnwrap()

const Home: NextPage = () => {

  // const {}
  return (
    <div>
      {/* {
        loading ? <div>Loading...</div> :
          thread.map(post => (<div>{post.content}</div>))
      }
      {<div style={{ color: 'red' }}>{error}</div>} */}
      <RenderThread />

    </div>
  )
}


interface RenderPageProps {
  parentId?: string
  page?: Page
}

const RenderThread = ({ page, parentId }: RenderPageProps) => {
  const {
    thread,
    removePost,

    addPost,
    updatePost,
    loading,
    loadMore,
    remainingPostCount
  } = useZonia({ channelID: "channel_1", initialPage: page, limit: 20 })
  const [comment, setComment] = useState('')

  return <div>
    {
      loading ?
        <div>Loading...</div> :
        <div style={{ marginTop: 10 }}>
          {Object.entries(thread).map(([_, comment]) => {
            return <div key={comment.id} style={{
              marginTop: 10,
              opacity: [PostStatus.SENDING_REMOVE, PostStatus.SUCCESS_REMOVE].includes(comment.status) ? 0.5 : 1,
            }}>
              <div>{comment.content}</div>
              {/* <div>{comment.userId}</div> */}
              <div key={comment.id} style={{ marginLeft: 20 }}>
                <RenderThread page={comment.replies} parentId={comment.id} />
              </div>
              <button onClick={() => removePost(comment.id)}>Delete</button>
            </div>
          })}
        </div>
    }
    <div>
      <input
        value={comment}
        onChange={e => setComment(e.target.value)}
      />
      <button
        onClick={() => {
          addPost({
            content: comment,
            parentId,
          })
        }}
      >add</button>
    </div>
    {
      ((remainingPostCount > 0 ||
        remainingPostCount === -1)
        && !loading) &&
      <div style={{ marginTop: 10 }}>
        <button onClick={loadMore}>Load more ({remainingPostCount})</button>
      </div>
    }

  </div>
}



export default Home
