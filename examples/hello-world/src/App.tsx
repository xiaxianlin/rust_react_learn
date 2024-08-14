import { useState } from 'react'

function App() {
    const [name, setName] = useState(() => 'ayou')

    return (
        <div onClick={() => setName('ayouayou')}>
            <Comp>{name}</Comp>
        </div>
    )
}

function Comp({ children }) {
    return (
        <span>
            <i>{`Hello world, ${children}`}</i>
        </span>
    )
}

export default App
