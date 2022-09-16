import { createEditor } from 'slate'
import { Slate, Editable, withReact } from 'slate-react'
import { BaseEditor, } from 'slate'
import { ReactEditor } from 'slate-react'
import { useState } from 'react'

type CustomElement = { type: 'paragraph'; children: CustomText[] }
type CustomText = { text: string }

declare module 'slate' {
    interface CustomTypes {
        Editor: BaseEditor & ReactEditor
        Element: CustomElement
        Text: CustomText
    }
}

const initialValue = [{
    type: 'paragraph',
    children: [{ text: 'A line of text in a paragraph.' }],
},]

export default () => {

    const [editor] = useState(() => withReact(createEditor()))

    return <Slate
            editor={editor}
            value={initialValue}
        >
            <Editable  className='text-left w-full border-[#E3E3E3] border-[1px]  bg-[#FAFAFA] p-4 rounded-lg' />
        </Slate>
  
}