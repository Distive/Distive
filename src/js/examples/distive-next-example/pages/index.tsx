import type { NextPage } from 'next'
import React, { useEffect, useState } from 'react'
import { Page, } from '@distive/sdk'
import initDistiveHook, { ThreadState } from '@distive/react'
import { PostStatus, } from '@distive/react'
import { Button, Textarea, VStack, Text, Container, Stack, ButtonGroup, IconButton, HStack, Divider, useToast, Collapse, useEditableControls, Editable, EditablePreview, Input, EditableTextarea, Avatar } from '@chakra-ui/react'
import { ChatIcon, EditIcon, DeleteIcon, CloseIcon, CheckIcon, ArrowDownIcon, ArrowUpIcon } from '@chakra-ui/icons'
import { useRouter } from 'next/router'

const useDistive = initDistiveHook({
  serverId: "rrkah-fqaaa-aaaaa-aaaaq-cai",
  host: 'http://localhost:8000',
})._unsafeUnwrap()

const Home: NextPage = () => {
  const router = useRouter()
  const { channel = 'main' } = router.query as { channel?: string }
  // const {}
  return (
    <Container maxW='container.sm' p='2' bg='gray.50' boxShadow='inner' >
      <Thread channelId={channel} />
    </Container>
  )
}


interface RenderPageProps {
  parentId?: string
  page?: Page
  channelId: string
}

const threadObjToArray = (threadObj: ThreadState): Array<ThreadState['']> => {
  return Object.values(threadObj)
}


const Thread = ({ page, parentId, channelId }: RenderPageProps) => {
  const toast = useToast()


  const {
    thread,
    removePost,
    addPost,
    updatePost,
    loading,
    loadMore,
    toggleMetadata,
    remainingPostCount
  } = useDistive({
    channelID: channelId,
    initialPage: page,
    limit: 8,
    onPostStatusChange: function ({ id, status, type }): void {
      if (status === 'SENDING')
        return

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
        },
        'METADATA': {
          'SUCCESS': {
            title: 'Post metadata updated',
            description: 'The post metadata has been updated',
          },
          'FAILURE': {
            title: 'Failed to update post metadata',
            description: 'The post metadata could not be updated',
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

    <div>
      {
        threadObjToArray(thread)
          .sort((a, b) => b.created_at - a.created_at)
          .map(comment =>
            <React.Fragment key={comment.id}>
              <Divider />
              <Comment
                votePost={(postId, vote) => {
                  toggleMetadata({ label: vote, postId })
                }}
                channelId={channelId}
                parentId={parentId}
                comment={comment}
                removePost={removePost}
                addPost={addPost}
                updatePost={updatePost}
              />
            </React.Fragment>
          )
      }
    </div>

    {
      !(remainingPostCount === 0) &&
      <div style={{ marginTop: 10 }}>
        <Button
          isFullWidth
          isLoading={loading}
          onClick={loadMore}>
          {remainingPostCount < 0 ? 'Load Comments' : `Load More`} {remainingPostCount > 0 && `${remainingPostCount} Remaining`}
        </Button>
      </div>
    }

  </Stack>
}

interface CommentProps {
  parentId?: string
  channelId: string
  //type of value of ThreadState 
  comment: ThreadState['']
  removePost: (id: string) => void
  votePost: (id: string, vote: 'up' | 'down') => void
  addPost: (input: { content: string, parentId?: string }) => void
  updatePost: (input: { content: string, postId: string, parentId?: string }) => void
}

const Comment = ({ comment, removePost, updatePost, addPost, votePost, parentId, channelId }: CommentProps) => {
  const [replyVisible, setReplyVisible] = useState(false)

  return <Stack>
    <VStack
      style={{
        padding: 10,
        opacity: (['SENDING_REMOVE', 'SENDING_ADD', 'SENDING_UPDATE'] as PostStatus[]).includes(comment.status) ? 0.5 : 1,
        display: comment.status === 'SUCCESS_REMOVE' ? 'none' : 'block',
        borderLeft: '2px solid #02C39A',
        borderColor: parentId ? '#30343F' : '#FAFAFF',
        backgroundColor: '#fefefe',
        paddingRight: 0
      }}

    >
      <HStack>
        <Avatar
          size='xs'
        // name={comment.userId}
        />
        <Text fontSize={'xs'}>{getHumanReadableTime((Date.now()) - comment.created_at)}</Text>
      </HStack>

      <Editable
        defaultValue={comment.content}
        fontSize='sm'
        onSubmit={(content) => content !== comment.content && updatePost({ content, postId: comment.id, parentId })}
      >
        <EditablePreview />
        <Input minHeight={20} as={EditableTextarea} />
        <CommentControls />
      </Editable>

      {/* <ReactionBarSelector/> */}
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
        <div style={{ marginLeft: 10 }}>
          <Thread channelId={channelId} page={comment.replies} parentId={comment.id} />
        </div>
      </div>
    </VStack>
  </Stack>

  function CommentControls() {
    const { isEditing, getSubmitButtonProps, getCancelButtonProps, getEditButtonProps } = useEditableControls()

    return <HStack
      justifyContent={'flex-end'}
      paddingRight={4}
    >
      {!isEditing ?
        <HStack width={'100%'} justifyContent='space-between' >
          <Stack opacity={comment.status === 'SENDING_METADATA' ? 0.2 : 1}>
            <CommentVote
              currentVote={(() => {
                const labels = comment.toggledMetadataLabels;
                if (labels.includes('up') && labels.includes('down')) return 'none'
                if (labels.includes('up')) return 'up'
                if (labels.includes('down')) return 'down'
                return 'none'
              })()}
              onVote={(vote) => {
                return votePost(comment.id, vote)
              }}
              loading={comment.status === 'SENDING_METADATA'}
            />
          </Stack>
          <ButtonGroup size='sm' isAttached variant='outline'>
            <IconButton boxShadow='inner' aria-label='Toggle Reply' onClick={() => setReplyVisible(!replyVisible)} size='xs' icon={<ChatIcon />} />
            <IconButton boxShadow='inner' aria-label='Update' isLoading={comment.status === 'SENDING_UPDATE'} icon={<EditIcon />} size='xs' mr='-px'
              {...getEditButtonProps()}
            >Update</IconButton>
            <IconButton boxShadow='inner' onClick={() => removePost(comment.id)} isLoading={comment.status === 'SENDING_REMOVE'} size='xs' aria-label='Delete Comment' icon={<DeleteIcon />} />
          </ButtonGroup>
        </HStack>
        :
        <ButtonGroup size='sm' isAttached variant='outline'>
          <IconButton boxShadow='inner' aria-label='Accept Edit' size='xs' icon={<CheckIcon />}
            {...getSubmitButtonProps()} />
          <IconButton boxShadow='inner' aria-label='Decline Edit' icon={<CloseIcon />} size='xs' mr='-px'
            {...getCancelButtonProps()} />
        </ButtonGroup>}
    </HStack>
  }
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


interface CommentVoteProps {
  currentVote: 'up' | 'down' | 'none'
  onVote: (vote: 'up' | 'down') => void
  loading: boolean
}

const CommentVote = ({ currentVote, onVote,loading }: CommentVoteProps) => {

  return <ButtonGroup isDisabled={loading} size='lg' isAttached variant='outline'>
    <IconButton boxShadow='inner' aria-label='upvote' onClick={() => currentVote !== 'none' && onVote('up')}
      size='xs' icon={<ArrowUpIcon color={currentVote === 'up' ? 'green.300': 'gray'} />}
    />
    <IconButton boxShadow='inner' aria-label='downvote' onClick={() => currentVote !== 'none' && onVote('down')}
      size='xs' icon={<ArrowDownIcon color={currentVote === 'down' ? 'orange.300': 'gray'} />}
    />
  </ButtonGroup>



}


const getHumanReadableTime = (ms: number, dp = 0) => {
  const timeScalars = [1000, 60, 60, 24, 7, 52];
  const timeUnits = ['ms', 's', 'm', 'h', 'd', 'w', 'y'];

  let timeScalarIndex = 0, scaledTime = ms;


  while (scaledTime > timeScalars[timeScalarIndex]) {
    scaledTime /= timeScalars[timeScalarIndex++];
  }
  if (timeScalarIndex < 2) return `now`

  return `${scaledTime.toFixed(dp)}${timeUnits[timeScalarIndex]}`;
}

export default Home
