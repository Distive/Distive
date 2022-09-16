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
            return <div className='absolute top-0 bottom-0 left-0 right-0'>
                <div
                className=' relative overflow-hidden'
                >
                    <div className='bg-white grid grid-flow-col p-6'>
                {children}

                    </div>
            </div>
            </div>
        },
        ChannelContainer: ({ children }) => {
            return <div className='bg-[#f6f1f1] rounded-md relative'>
                {children}
            </div>
        },
        NavBar: ({ children, activeChannelTab }) => {
            return <div className='bg-white w-full'>
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
            return <div className='sticky border-[#E3E3E3]  border-[1px]   bg-white shadow-lg w-full p-4 rounded-md flex flex-col  items-start justify-between gap-4'>
                <div className='flex items-center gap-2'>
                   
                    <img
                        src='https://lh3.googleusercontent.com/ogw/AOh-ky0X9R3cke61nRpzmJ8DDam82ZyRIlwjvAf_lQOPvQ=s32-c-mo'
                        className='w-8 h-8 rounded-full aspect-square'
                    />
                    <span>Replying to <b>@rosymaplewitch</b></span>
                </div>
                {children}
            </div>
        },
        ChannelTopBar: DefaultComponent,
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
            return <div className={`flex flex-col gap-8 ml-12 pt-12 ${!isReply && 'max-h-[100%] overflow-y-scroll '}`}>
                {children}
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