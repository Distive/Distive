import React, { useContext } from 'react';
import { PostControlsProps } from './lib';
import { ReplyButton } from "./ReplyButton";
import Popup from 'reactjs-popup';
import InternalContext from './context/internalContext';

export const PostControls = ({ postId }: PostControlsProps) => {
    const internal = useContext(InternalContext)
    
    return <div className="post-controls">
        <ReplyButton postId={postId} />
        <div className="reactions-container">
            <div className="reaction">
                <div className="reaction-icon icon reaction-icon_annoyed" />
                <div className="reaction-count">5</div>
            </div>
            <div className="reaction">
                <div className="reaction-icon icon reaction-icon_happy" />
                <div className="reaction-count">50</div>
            </div>
        </div>

        <Popup
            arrow={false}
            trigger={<div className="reactions-select">
                <div className='icon icon-react' />
            </div>}
            position='center center'
        >
            <ul className='reactions-list'>
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_happy'/>
                </li> 
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_annoyed'/>
                </li> 
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_happy'/>
                </li> 
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_happy'/>
                </li> 
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_happy'/>
                </li> 
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_happy'/>
                </li> 
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_happy'/>
                </li> 
                <li className='reactions-item' >
                    <div className='icon reaction-icon reaction-icon_happy'/>
                </li> 
                
             
            </ul>
        </Popup>

    </div>;
};
