import React, { createContext } from 'react';

interface InternalContext {
    currentUserID: string;
    // Used to open the reply modal
    activateReply: (postId: string) => void;
    reactions: Array<{ className: string, value: string }>;
    UserInfoComponent: ({ userId }: { userId: string }) => JSX.Element
}



export const DefaultInternalContext: InternalContext = {
    currentUserID: '',
    activateReply: () => { },
    reactions: [],
    UserInfoComponent: () => <></>

}

const InternalContext = createContext<InternalContext>(DefaultInternalContext);


export default InternalContext;