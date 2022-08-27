import React from 'react'
import { ComponentMeta, ComponentStory } from '@storybook/react'
import Distive from '../index'
import './default.css'

export default {
    title: 'Distive/Default',
    component: Distive,
    // argTypes: {
    //     config: {

    //     }
    // }
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
    }
}