function sanitizeHTML(html) {
    if (!DOMPurify.isSupported) {
        return `<div align="center" style="background-color:#1e1f22;color:#f00"><h1>YOUR INTERNET BROWSER IS NOT SUPPORTED</h1></div>`;
    }

    // todo: check if it removes html that could be malicious
    return DOMPurify.sanitize(html, {USE_PROFILES: {html: true}});
}

async function getPost(postCode=null, session=null) {
    let headers = {};
    if (postCode && !session) {
        headers['post-code'] = postCode;
    }

    if (session) {
        headers['session'] = session;
    }

    let response = await fetch('/api/post', {
        method: 'GET',
        headers: headers,
    });

    if (!response.ok) {
        throw new Error('Failed to get the post!');
    }

    return JSON.parse(await response.text());
}
