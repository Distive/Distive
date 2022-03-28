import type { NextPage } from 'next'
import React, { useEffect, useState } from 'react'
import { Page, Thread } from 'zomia'
import initZoniaHook, { ThreadState } from 'zomia-react'
import { PostStatus, } from 'zomia-react'
import { Button, Textarea, VStack, Text, Container, Stack, ButtonGroup, IconButton, HStack, Divider, useToast, Menu, MenuButton, MenuItem, MenuList, Collapse } from '@chakra-ui/react'
import { AddIcon, ChatIcon, EditIcon, DeleteIcon, ChevronDownIcon } from '@chakra-ui/icons'

const useZonia = initZoniaHook({
  serverId: "rofub-iaaaa-aaaai-ab7da-cai"
})._unsafeUnwrap()

const Home: NextPage = () => {

  // const {}
  return (
    <Container maxW='container.sm' p='10' bg='gray.50' boxShadow='inner' >
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
  const toast = useToast()

  const {
    thread,
    removePost,
    addPost,
    updatePost,
    loading,
    loadMore,
    remainingPostCount
  } = useZonia({
    channelID: "channel_1",
    initialPage: page,
    limit: 8,
    onPostStatusChange: ({ id, status, type }) => {
      if (status === 'SENDING') return
      
      const map = {
        'REMOVE': {
          'SUCCESS': {
            title: 'Post removed',
            description: 'The post has been removed',
          },
          'FAILURE': {
            title: 'Failed to remove post',
            description: 'The post could not be removed',
          },
        },
        'ADD': {
          'SUCCESS': {
            title: 'Post added',
            description: 'The post has been added',
          },
          'FAILURE': {
            title: 'Failed to add post',
            description: 'The post could not be added',
          },

        },
        'UPDATE': {
          'SUCCESS': {
            title: 'Post updated',
            description: 'The post has been updated',

          },
          'FAILURE': {
            title: 'Failed to update post',
            description: 'The post could not be updated',
          },
        },
        'REPLY': {
          'SUCCESS': {
            title: 'Post added',
            description: 'The post has been added',
          },
          'FAILURE': {
            title: 'Failed to add post',
            description: 'The post could not be added'
          },
        }
      }



      const mapStatus = {
        'SUCCESS': 'success',
        'FAILURE': 'error',
      }



      toast({
        duration: 3000,
        variant: 'subtle',
        position: 'top',
        isClosable: true,
        status: (mapStatus as any)?.[status] ?? 'info',
        ...map?.[type]?.[status] ?? {}
      })


    }
  })

  return <Stack>

    {!parentId && <CommentInput
      onSubmit={(content) => addPost({ content, parentId })}
      loading={false}
      buttonText='Comment'
    />}

    <Stack>
      {
        threadObjToArray(thread)
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
    </Stack>

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
        borderColor: parentId ? '#02C39A' : '#023047',
        backgroundColor: '#fefefe',
        paddingRight: 0
      }}

    >

      <Text fontSize='sm'>{comment.content}</Text>


      <HStack
        justifyContent={'flex-end'}
        paddingRight={4}

      >
        <ButtonGroup size='sm' isAttached variant='outline'>
          <IconButton boxShadow='inner' aria-label='Toggle Reply' onClick={() => setReplyVisible(!replyVisible)} size='xs' icon={<ChatIcon />} />
          <IconButton boxShadow='inner' aria-label='Update' icon={<EditIcon />} size='xs' mr='-px'>Update</IconButton>
          <IconButton boxShadow='inner' onClick={() => removePost(comment.id)} isLoading={comment.status === 'SENDING_REMOVE'} size='xs' aria-label='Delete Comment' icon={<DeleteIcon />} />
        </ButtonGroup>
      </HStack>
      <div  >
        <Stack paddingRight={4}>
          <Collapse unmountOnExit in={replyVisible} animateOpacity>
            <CommentInput
              onSubmit={(content) => addPost({ content, parentId: comment.id })}
              loading={(['SENDING_REPLY'] as PostStatus[]).includes(comment.status)}
              buttonText='Reply'

            />

          </Collapse>
        </Stack>
        <div style={{ marginLeft: 20 }}>
          <Thread page={comment.replies} parentId={comment.id} />
        </div>

      </div>
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
      variant={'filled'}

    />

    <Button size={'xs'} loadingText='Sending' isLoading={loading} onClick={() => onSubmit(comment)}>
      {buttonText}
    </Button>
  </VStack>
}


export default Home
