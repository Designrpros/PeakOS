
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { SystemLinkProvider } from './context/SystemLinkContext'
import './index.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <SystemLinkProvider>
            <App />
        </SystemLinkProvider>
    </React.StrictMode>,
)
