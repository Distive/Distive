import React from 'react'
import { ComponentMeta, ComponentStory } from '@storybook/react'
import Distive from '../index'
import { DefaultComponent } from '../lib';
import { Post, PostInputButtons, PostTextBox } from './components'

export default {
    title: 'Distive/Default',
    component: Distive,

} as ComponentMeta<typeof Distive>;

const Template: ComponentStory<typeof Distive> = (args) => <Distive {...args} />;

export const Default = Template.bind({});
Default.args = {
    config: {
        canisterId: '6nfmt-3yaaa-aaaag-aaqla-cai',
        newPageForNestedThread: true,
        reactions: [
            {
                display: 'Like',
                value: 'like'
            },
        ],
        resolveProfileImageUrl: (userId: string) => `https://distive.com/api/v1/users/${userId}/profile-image`,
        mode: 'FLAT'
    },
    Components: {
        RootContainer: ({ children }) => {
            return <div style={{ height: '100%' }} className='absolute top-0 bottom-0 left-0 right-0 h-full w-full'>
                <div
                    style={{height:'100%'}}
                    className=' relative w-full h-full z-50 overflow-hidden'
                >
                    <div style={{height:'100%'}} className='bg-white flex w-full h-full overflow-hidden'>
                        {children}
                    </div>
                </div>
            </div>
        },
        ChannelContainer: ({ children }) => {
            return <div style={{ flexGrow: 1, height: '100%' }} className='bg-[#f6f1f1] rounded-md relative h-full overflow-hidden'>
                <div style={{height: '100%'}} className='flex flex-col h-full'>
                    {children}
                </div>
            </div>
        },
        NavBar: ({ children, activeChannelTab }) => {
            return <div style={{ flexBasis: '30%' }} className='bg-white w-full relative flex-grow-0 flex-shrink-0 basis-1/4 overflow-visible'>
                {children}
            </div>
        },
        NavBarChannelTab: ({ children, title }) => {
            return <div>
                {title}
                {children}
            </div>
        },
        InputContainer: ({ children }) => {
            // return <div/>
            return <footer style={{ order: 3,  minHeight: 62, position:'relative', zIndex: 100 }} className=' border-[#E3E3E3] border-[1px]   bg-white shadow-lg w-full p-4 rounded-md flex flex-col gap-2'>
                <div className='flex items-center gap-2'>

                    <img
                        src='https://lh3.googleusercontent.com/ogw/AOh-ky0X9R3cke61nRpzmJ8DDam82ZyRIlwjvAf_lQOPvQ=s32-c-mo'
                        className='w-8 h-8 rounded-full aspect-square'
                    />
                    <span>Replying to <b>@rosymaplewitch</b></span>
                </div>
                {children}
            </footer>
        },
        ChannelTopBar: () => {
            return <header className='relative z-[1000] w-full h-[59px]'>
                Channel Name
            </header>
        },
        ImageUploadButton: DefaultComponent,
        InputButtonsContainer: () => {
            return <>
                <PostTextBox />
                <div className='flex gap-2 items-center w-full justify-between'>
                    <PostInputButtons.CameraIcon />
                    <PostInputButtons.SendIcon />
                </div>
            </  >
        },
        Post,
        PostsContainer: ({ children, isReply }) => {
            return <div style={{
                gap: 48,
                height: '100%',
                marginTop: 48,
                marginBottom: 48
            }} className={`ml-12 w-full overflow-x-hidden absolute   flex flex-col ${!isReply ? 'max-h-[100%]  overflow-y-scroll ': ' '}`}>
                {children}
            </div>
        },
        ChannelPosts: ({ children }) => {
            return <div style={{ position: 'relative', zIndex: 1, flex: '1 1 0', order: 2, height:'100%' }}>
                {/* <div style={{}}> */}
                {children}
               {/* </div> */}
            </div>
        },
        PostsLoader: () => {
            return <div>

            </div>
        },
        PhotoButton: () => {
            return <div>

            </div>
        },
        SubmitButton: () => {
            return <div>

            </div>
        },
        TextBox: () => {
            return <div>

            </div>
        }
    }
}