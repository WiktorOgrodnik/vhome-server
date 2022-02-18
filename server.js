const express = require('express');

const app = express();
const port = 3000;

app.get('/api/v1', (req, res) => {

    const welcome = {
        message: 'Welcome to our API!'
    };

    res.setHeader('Content-type', 'application/json; charset=utf8;');
    res.end(JSON.stringify(welcome));
});

app.listen(port);