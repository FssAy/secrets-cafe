# API Docs

---

### Ping - `/api/ping`
- **Method:**
- **Description:** Used to test API _(not compiled on the release mode)_.
- **Response:**
  ```json
  {
    "method": <METHOD USED>,
    "msg": "pong"
  }
  ```

---

### Post - `/api/post`
- **Method:** `POST`
- **Description:** Upload the post. Can be called twice from the same IP every 30 minutes. Body size limit is specified in the config. 
- **Body:** Post content as a HTML.
- **Response:**
  ```json
  {
    "code": <UNIQUE POST CODE>,
  }
  ```

- **Method:** `GET`
- **Description:** Get post's data.
- **Parameters:**
  - `session` - Mod's session token. When present will return a post for verification (other params will be ignored).
  - `post_code` - Unique post code. When present will return data related to a specific post, when not present will return the same type of data, but for a random post.
- **Response:**
  ```json
  {
    "code": <UNIQUE POST CODE>,
    "content": <POST CONTENT AS HTML (dirty)>,
    "at": <DATE WHEN POST WAS CREATED>,
    "state": <POST STATE>,
  }
  ```

- **Method:** `PATCH`
- **Description:** Modify post's data.
- **Parameters:**
  - `session` - Mod's session token.
  - `post_code` - Unique post code.
  - `action` - Name of the operation to perform (`approve`, `reject`).
  - `reason` - Short description of why the post has been rejected. Optional, used only on `action: reject`.
- **Response:**
  ```json
  "Ok"
  ```

---
