node install

Start-Process -FilePath 'node' -ArgumentList 'index.js 3000'
Start-Process -FilePath 'node' -ArgumentList 'index.js 3001'
Start-Process -FilePath 'node' -ArgumentList 'index.js 3002'