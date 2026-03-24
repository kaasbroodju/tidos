import { useState } from 'react';

export default function HelloWorld() {
    const [name, setName] = useState('');

    return (
        <div>
            <input
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Enter your name"
            />
            {name && <p>Hello {name}!</p>}
        </div>
    );
}
