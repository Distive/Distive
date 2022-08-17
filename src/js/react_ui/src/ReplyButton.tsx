import React, { useContext } from 'react';
import InternalContext from './context/internalContext';


export const ReplyButton = ({ postId }: { postId: string; }) => {
    const internal = useContext(InternalContext);

    return <div
        onClick={() => {
            internal.activateReply(postId);
        }}
        className="reply-button">
        Reply
    </div>;
};
