# Create React App

```js
npx create-react-app my-app
cd my-app
npm start

npm run build
```

## 阻止默认行为

```js
<a href="#" onclick="console.log('The link was clicked.'); return false">
  Click me
</a>

function ActionLink() {
  function handleClick(e) {
    e.preventDefault();
    console.log('The link was clicked.');
  }

  return (
    <a href="#" onClick={handleClick}>
      Click me
    </a>
  );
}
```

## workspace

```bash
npm install -g npm@7

├── package.json
└── packages
    ├── package-a
    │   └── package.json
    └── package-b
        └── package.json
```

```json
package.json
{
  "workspaces": ["./packages/*"]
}
```

```bash
in the root of your repository
npm install

# Run "test" script on all packages
npm run test --workspaces
npm run test  -ws

# Runs "test" only on package-a
npm run test --workspace package-a
npm run test -w package-a

# Install `lodash` on `package-a`
npm install lodash --workspace package-a

# Install `tap` on `package-b` as a dev dependency
npm install tap --workspace package-b --save-dev

# Install `package-a` on `package-b`
npm install package-a --workspace package-b

# Install `eslint` in all packages
npm install eslint --workspaces
```

## create library

```bash
npm install -g create-react-library

create-react-library

npm start # at the root of your project to start the library compilation.
npm start # inside the example folder to see the library in action
```

## nginx

```bash
sudo npm run build
sudo mkdir /var/www/
sudo scp -r ./build/* /var/www/build/

server {
    listen 0.0.0.0:80;
    server_name mydomainname.com;
    access_log /var/log/nginx/app.log;
    root /var/www/build;
    index index.html index.htm;
    try_files $uri /index.html;
    location / {
        try_files $uri $uri/ = 404;
    }
}

sudo service nginx stop
sudo service nginx start
```