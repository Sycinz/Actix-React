import './scss/App.scss'
import user_icon from './assets/user-icon.svg'
import key_icon from './assets/key-icon.svg'

function App() {
  return (
    <>
      <form method="POST" action="login">
        <div className="input-container">
          <img src={user_icon} alt="User icon" type="image/svg+xml" className="user-icon" />
          <input type="text" name="username" placeholder="Username" />
        </div>
        <div className="input-container">
          <img src={key_icon} alt="Password icon" className="password-icon" />
          <input type="password" name="password" placeholder="Password" />
        </div>
        <input type="submit" name="submit" value="LOGIN" />
      </form>
    </>
  )
}

export default App