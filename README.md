# Forward Wall

适用于 Traefik `ForwardAuth` Middleware 的一个非常简易的认证服务器，提供可配置的单 Passcode 认证。旨在阻止直接访问，无用户管理、权限控制等功能。

例如，我用来防止网站被 GFW 特征匹配，导致 DNS 污染 233333

## Usage

1. 构建 Docker 镜像
2. 配置需要认证的 Services
   ```yaml
   labels:
     - traefik.http.routers.something-need-auth.middlewares=fw-auth
     - traefik.http.middlewares.fw-auth.forwardauth.address=http://<Address of Forward Wall>/<Passcode>
   ```

## Hint

当 App 需要进行 `fetch` 请求时，请注意发送 Cookie。

也可以为 API 端点设置排除：

```yaml
labels:
  - traefik.http.routers.example-app.rule=Host(`example.com`)
  - traefik.http.routers.example-app-bypass.rule=Host(`example.com`) && (PathPrefix(`/api`) || Path(`/manifest.webmanifest`))
  - traefik.http.routers.example-app.middlewares=fw-auth
  - traefik.http.middlewares.fw-auth.forwardauth.address=http://<Address of Forward Wall>/<Passcode>
```
