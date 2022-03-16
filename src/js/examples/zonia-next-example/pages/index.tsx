import type { NextPage } from 'next'
import { Page } from 'zomia'
import initZoniaHook, { ThreadState } from 'zomia-react'
const useZonia = initZoniaHook({
  serverId: "rrkah-fqaaa-aaaaa-aaaaq-cai"
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
      <Component />
    </div>
  )
}




const Component = () => {
  return <RenderThread />
}


interface RenderPageProps {
  page?: Page
}

const RenderThread = ({ page }: RenderPageProps) => {
  const {
    thread,
    removePost,
    addPost,
    updatePost,
    loading,
    loadMore,
    remainingPostCount
  } = useZonia({ channelID: "channel_1", initialPage: page, limit: 20 })

  return <div>
    {
      loading ?
        <div>Loading...</div> :
        <div style={{ marginTop: 10 }}>
          {Object.entries(thread).map(([commentId, comment]) => {
            return <div style={{ marginTop: 10 }}>
              <div>{comment.content}</div>
              {/* <div>{comment.userId}</div> */}
              <div key={comment.id} style={{ marginLeft: 20 }}>
                <RenderThread page={comment.replies} />
              </div>
            </div>
          })}
        </div>
    }
    {

      <div style={{ marginTop: 10 }}>
        <button disabled={!((remainingPostCount > 0 ||
          remainingPostCount === -1)
          && !loading)} onClick={loadMore}>Load more ({remainingPostCount})</button>
      </div>
    }
  </div>
}



export default Home
