const { createProxyMiddleware } = require('http-proxy-middleware');

module.exports = function(app) {
  app.use(
    '/api/auth',
    createProxyMiddleware({
      target: 'http://localhost:8080/',
      changeOrigin: true,
    })
  );
  app.use(
    '/api/shipments',
    createProxyMiddleware({
      target: 'http://localhost:8081/',
      changeOrigin: true,
    })
  );
};
