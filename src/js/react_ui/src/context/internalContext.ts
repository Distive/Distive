import { createContext } from 'react';

interface InternalContext {
    currentUserID: string;
    // Used to open the reply modal
    activateReply: (postId: string) => void;
    reactions: Array<{ className: string, value: string }>;
}



export const DefaultInternalContext: InternalContext = {
    currentUserID: '',
    activateReply: () => { },
    reactions: [],
}

const InternalContext = createContext<InternalContext>(DefaultInternalContext);


export default InternalContext;