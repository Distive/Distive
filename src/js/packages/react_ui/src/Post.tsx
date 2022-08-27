import { PostThreadState } from './lib';
import { PostControls } from "./PostControls";
import Popup from 'reactjs-popup';
import { useContext } from 'react';
import InternalContext from './context/internalContext';

// sys_author:Allen
export const Post = ({ content, id, metadata, userId, created_at }: PostThreadState) => {
    const internal = useContext(InternalContext)
    const { currentUserID, reactions, activateReply, UserInfoComponent } = internal

    const authorName = (() => {
        // expects a metadata label of the form 'sys_author:name'
        let authorMetadata = metadata?.find(m => m.label.includes('sys_author'));
        return authorMetadata ? authorMetadata.label.split(':')[1] : userId;
    })();


    return <div className='post-container'>
        <div className="post-info">
            <Popup
                 
                modal
                trigger={
                    <img className="post-avatar current-user" src="https://www.redditstatic.com/avatars/defaults/v2/avatar_default_6.png" />
                }
            >
                <div className='user-info_container'>
                    <UserInfoComponent
                        userId={userId}
                    />
                </div>
            </Popup>
            <div className="post-info_name">
                <div className="post-info_name_text">
                    {authorName}
                </div>
                <div className="post-info_name_date">
                    {getHumanReadableTime((Date.now()) - created_at)}
                </div>
            </div>
            <div className="post-menu" >
                <Popup
                    arrow={false}
                    trigger={<div className='post-menu-button' />}
                    position='center center'
                >
                    <ul className='post-menu-list'>
                        <li className='post-menu-item'>Flag</li>
                        <li className='post-menu-item'>Delete</li>
                    </ul>
                </Popup>
            </div>
        </div>
        {/* </div> */}
        <p className="post-text_content">
            {content}
        </p>
        <PostControls postId={id} />
    </div>;
};


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
