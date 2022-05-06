
const express = require('express')
const app = express()
const port = process.argv[2]

app.get('/', (req, res) => {
    res.send("This is root, port: " + port)
})

app.get('/hello', (req, res) => {
  res.send("This is hello, port: " + port)
})

app.get('/name/:name', (req, res) => {
  res.send(`Your name is: ${req.params.name}, port: ` + port)
})

app.listen(parseInt(port), () => {
    console.log(`Example app listening on port ${port}`)
})
