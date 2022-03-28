import type { NextPage } from 'next'
import React, { useState } from 'react'
import { Page } from 'zomia'
import initZoniaHook, { ThreadState } from 'zomia-react'
import { PostStatus, } from 'zomia-react'
import { Button, Textarea, VStack, Text, Container, Stack, ButtonGroup, IconButton, HStack, Divider } from '@chakra-ui/react'
import { AddIcon, ChatIcon, EditIcon, DeleteIcon } from '@chakra-ui/icons'
const useZonia = initZoniaHook({
  // serverId: "rrkah-fqaaa-aaaaa-aaaaq-cai"
  serverId: "rofub-iaaaa-aaaai-ab7da-cai"

})._unsafeUnwrap()

const Home: NextPage = () => {

  // const {}
  return (
    <Container p='10' bg='gray.50' boxShadow='base'>
      <Thread />
    </Container>
  )
}


interface RenderPageProps {
  parentId?: string
  page?: Page
}

const threadObjToArray = (threadObj: ThreadState): Array<ThreadState['']> => {
  return Object.values(threadObj)
}


const Thread = ({ page, parentId }: RenderPageProps) => {
  const {
    thread,
    removePost,
    addPost,
    updatePost,
    loading,
    loadMore,
    remainingPostCount
  } = useZonia({ channelID: "channel_1", initialPage: page, limit: 8 })




  return <Stack  >

    {!parentId && <CommentInput
      onSubmit={(content) => addPost({ content, parentId })}
      loading={false}
      buttonText='Comment'
    />}

    <div>
    {threadObjToArray(thread)
      // .filter(comment => comment.status !== 'SENDING_ADD')
      .sort((a, b) => b.created_at - a.created_at)
      .map(comment =>
        <React.Fragment key={comment.id}>
          <Divider />
          <Comment
            parentId={parentId}
            comment={comment}
            removePost={removePost}
            addPost={addPost}
          />

        </React.Fragment>
      )
    }
   </div>

    {
      !(remainingPostCount === 0) &&
      <div style={{ marginTop: 10 }}>
        <Button isFullWidth isLoading={loading} onClick={loadMore}>Load more {remainingPostCount > 0 && `${remainingPostCount} Remaining`}</Button>
      </div>
    }

  </Stack>
}

interface CommentProps {
  parentId?: string
  //type of value of ThreadState 
  comment: ThreadState['']
  removePost: (id: string) => void
  addPost: (input: { content: string, parentId?: string }) => void

}

const Comment = ({ comment, removePost, addPost, parentId }: CommentProps) => {
  const [replyVisible, setReplyVisible] = useState(false)

  return <Stack>

    <VStack
      style={{
        padding: 10,
        opacity: (['SENDING_REMOVE', 'SENDING_ADD', 'SENDING_UPDATE'] as PostStatus[]).includes(comment.status) ? 0.5 : 1,
        display: comment.status === 'SUCCESS_REMOVE' ? 'none' : 'block',
        borderLeft: '2px solid #02C39A',
        borderColor: parentId ? '#02C39A' : '#E8F0FF',
        backgroundColor: '#fefefe',
        paddingRight: 0
      }}

    >

      <Text fontSize='sm'>{comment.content}</Text>

      <HStack
        justifyContent={'space-between'}
        paddingRight={4}
      >

        <Button onClick={() => setReplyVisible(!replyVisible)} size='xs' leftIcon={<ChatIcon />} >Reply</Button>
        <ButtonGroup size='sm' isAttached variant='outline'>
          <Button size='xs' mr='-px'>Update</Button>
          <IconButton onClick={() => removePost(comment.id)} isLoading={comment.status === 'SENDING_REMOVE'} size='xs' aria-label='Delete Comment' icon={<DeleteIcon />} />
        </ButtonGroup>
      </HStack>
      {
        replyVisible && <CommentInput
          onSubmit={(content) => addPost({ content, parentId })}
          loading={(['SENDING_REPLY'] as PostStatus[]).includes(comment.status)}
          buttonText='Reply'
        />
      }

      {/* <div>{comment.userId}</div> */}
      <div style={{ marginLeft: 20 }}>
        <Thread page={comment.replies} parentId={comment.id} />
      </div>
      <div style={{ background: 'red', display: (['FAILURE_ADD', 'FAILURE_REMOVE', 'FAILURE_UPDATE', 'FAILURE_REPLY'] as PostStatus[]).includes(comment.status) ? 'block' : 'none' }}>Unable to send</div>
    </VStack>
  </Stack>
}


interface CommentInputProps {
  onSubmit: (content: string) => void
  loading?: boolean,
  buttonText?: string
}
const defaultCommentInputProps: CommentInputProps = {
  onSubmit: () => { },
  loading: false,
  buttonText: 'Send'
}

const CommentInput = ({ onSubmit, loading, buttonText }: CommentInputProps = defaultCommentInputProps) => {
  const [comment, setComment] = useState('')

  return <VStack
    alignItems={'flex-end'}
  >
    <Textarea
      value={comment}
      onChange={e => setComment(e.target.value)}
      placeholder='Comment'

    />

    <Button size={'sm'} loadingText='Sending' isLoading={loading} onClick={() => onSubmit(comment)}>
      {buttonText}
    </Button>
  </VStack>
}

export default Home
