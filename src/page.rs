use axum::response::Html;

pub const AUTH_PAGE: Html<&str> = Html(
  r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <meta name="robots" content="noindex" />
    <title>Authentication</title>
    <style>
      body {
        background-color: #f1f2f3;
      }

      form {
        display: flex;
        margin: 30vh auto 0;
        gap: 0.5rem;
        padding: 1rem;
        max-width: 350px;
        min-width: 280px;
      }

      #passcode {
        display: block;
        border: none;
        outline: none;
        font-size: 2rem;
        width: 100%;
        height: 56px;
        background-color: #fff;
        padding-inline: 1rem;
        transition: all 0.2s;
        border-radius: 0.5rem;
        box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
        box-sizing: border-box;
      }

      #passcode:disabled {
        color: rgba(0, 0, 0, 0.5);
      }

      #submit {
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        aspect-ratio: 1;
        height: 56px;
        color: white;
        background-color: #fbbf24;
        border-radius: 0.5rem;
        box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
        transition: all 0.2s;
      }

      #submit:disabled {
        background-color: #d4d4d4;
      }

      #submit:not(:disabled):hover {
        background-color: #f59e0b;
        cursor: pointer;
      }
    </style>
  </head>
  <body>
    <form>
      <input id="passcode" type="password" required />
      <button id="submit">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          height="24"
          viewBox="0 -960 960 960"
          width="24"
        >
          <path
            fill="currentColor"
            d="M647-440H160v-80h487L423-744l57-56 320 320-320 320-57-56 224-224Z"
          />
        </svg>
      </button>
    </form>

    <script>
      'use strict'

      const formEl = document.querySelector('form')
      const passcodeEl = document.getElementById('passcode')
      const submitEl = document.getElementById('submit')

      formEl.addEventListener('submit', async (e) => {
        e.preventDefault()

        passcodeEl.disabled = true
        submitEl.disabled = true

        const result = await (
          await fetch(window.location.pathname, {
            method: "POST",
            headers: {
              'Forward-Wall-Passcode': passcodeEl.value,
            },
          })
        ).text()

        if (result === 'yes') {
          window.location.reload()
        } else {
          const animation = formEl.animate(
            [
              { transform: 'translateZ(0)' },
              { transform: 'translate3d(-10px, 0, 0)' },
              { transform: 'translate3d(10px, 0, 0)' },
              { transform: 'translate3d(-10px, 0, 0)' },
              { transform: 'translate3d(10px, 0, 0)' },
              { transform: 'translate3d(-10px, 0, 0)' },
              { transform: 'translate3d(10px, 0, 0)' },
              { transform: 'translateZ(0)' },
            ],
            {
              duration: 500,
            }
          )
          animation.addEventListener('finish', () => {
            passcodeEl.disabled = false
            submitEl.disabled = false
          })
        }
      })
    </script>
  </body>
</html>"#,
);
