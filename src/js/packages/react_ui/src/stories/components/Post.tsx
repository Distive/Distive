import React from 'react';
import { Components } from '../../lib';

const AddIcon = () => <svg className='w-4 h-4' fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 15"><path d="M6.531 1.875h.938c.083 0 .125.042.125.125v11c0 .083-.042.125-.125.125H6.53c-.083 0-.125-.042-.125-.125V2c0-.083.042-.125.125-.125Z" fill="#000" /><path d="M1.75 6.906h10.5c.083 0 .125.042.125.125v.938c0 .083-.042.125-.125.125H1.75c-.083 0-.125-.042-.125-.125V7.03c0-.083.042-.125.125-.125Z" fill="#000" /></svg>

const VisibilityIcon = () => <svg className='w-4 h-4' fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 13"><g clip-path="url(#a)" fill="#000"><path d="M6.938 8.25A1.75 1.75 0 0 0 8.68 6.348L6.785 8.243c.05.005.101.007.153.007Zm5.792-7.163L12.063.42a.125.125 0 0 0-.177 0l-1.708 1.709C9.235 1.647 8.176 1.406 7 1.406c-3.003 0-5.244 1.564-6.722 4.692a.942.942 0 0 0 0 .805c.59 1.244 1.302 2.241 2.134 2.992L.759 11.547a.125.125 0 0 0 0 .177l.667.667a.125.125 0 0 0 .177 0L12.73 1.264a.125.125 0 0 0 0-.177ZM4.188 6.5a2.75 2.75 0 0 1 4.045-2.426l-.76.76A1.751 1.751 0 0 0 5.27 7.034l-.76.76A2.736 2.736 0 0 1 4.188 6.5Z" /><path d="M13.722 6.097c-.55-1.158-1.205-2.103-1.964-2.834L9.506 5.516a2.751 2.751 0 0 1-3.553 3.552l-1.91 1.91c.886.41 1.872.616 2.957.616 3.003 0 5.244-1.564 6.722-4.692a.941.941 0 0 0 0-.805Z" /></g><defs><clipPath id="a"><path fill="#fff" d="M0 0h14v13H0z" /></clipPath></defs></svg>

const ReplyIcon = () => <svg className='w-4 h-4' fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 15"><g clip-path="url(#a)"><path d="M6.25 7.5a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Zm3.125 0a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Zm-6.25 0a.75.75 0 1 0 1.5 0 .75.75 0 0 0-1.5 0Zm10.331-2.713A7.002 7.002 0 0 0 7 .5h-.03A6.984 6.984 0 0 0 0 7.533a7.022 7.022 0 0 0 .75 3.123v2.375a.719.719 0 0 0 .719.719h2.376c.97.488 2.039.745 3.124.75h.033c.936 0 1.843-.181 2.698-.536a6.946 6.946 0 0 0 2.231-1.487A6.985 6.985 0 0 0 14 7.532a6.973 6.973 0 0 0-.544-2.745Zm-2.36 6.844A5.782 5.782 0 0 1 7 13.312h-.026a5.834 5.834 0 0 1-2.705-.68l-.131-.07h-2.2v-2.2l-.07-.13a5.833 5.833 0 0 1-.68-2.705 5.777 5.777 0 0 1 1.68-4.122 5.769 5.769 0 0 1 4.107-1.717h.027a5.784 5.784 0 0 1 4.112 1.701 5.784 5.784 0 0 1 1.702 4.138 5.79 5.79 0 0 1-1.72 4.104Z" fill="#000"/></g><defs><clipPath id="a"><path fill="#fff" transform="translate(0 .5)" d="M0 0h14v14H0z"/></clipPath></defs></svg>

const postButtonStyles = 'border-[#E3E3E3] border-[1.5px] rounded-full bg-[#FAFAFA] p-1 px-2'
const Post: Components['Post'] = ({ children, state }) => {
    return <div>
        <div className='mb-2'>
            <span className='mr-2 text-sm'>{state.userId}</span>
            <span className='text-xs'>{state.created_at}</span>
        </div>
        <div className='flex gap-4'>
            <div className='w-12 h-12 aspect-square bg-purple-700 rounded-full' />
            <div className='bg-[#FAFAFA] border-[#E3E3E3] border-[1px] p-4 w-fit rounded-lg relative'>
                <p className='w-fit pb-4 max-w-prose'>
                    {state.content}
                </p>
                <div className='absolute w-11/12 flex justify-between'>
                    <div className='flex flex-row gap-2'>
                        {state.metadata.map(({ label, ...rest }) => {
                            if (label.includes('reaction:')) {
                                return {
                                    ...rest,
                                    label: label.split(':')[1]
                                };
                            } else {
                                return null;
                            }
                        }).filter(Boolean)
                            .map(metadata => {
                                return <div className={`${metadata?.is_toggled[0] ? 'border-[#5200FF80]' : ''} ${postButtonStyles}`}>
                                    <span>{metadata?.label}</span>
                                    {(metadata?.count ?? 0) > 1 && <span className={`pl-2 `}>{metadata?.count}</span>}
                                </div>;
                            })
                        }

                        <div className={`${postButtonStyles} flex items-center`}>
                            <AddIcon />
                        </div>

                    </div>
                    <div className={`bg-white flex rounded-full shadow-sm ${postButtonStyles} gap-2`}>
                        <div className={` bg-transparent border-none flex items-center`}>
                            <ReplyIcon />
                        </div>
                        <div className={`bg-transparent border-none flex items-center`}>
                            <VisibilityIcon />
                        </div>
                    </div>
                </div>
            </div>
        </div>
                {children}
    </div>;
};


export default Post