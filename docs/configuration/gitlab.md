# GitLab Authentication Setup

Shortly supports GitLab OAuth authentication, allowing users to log in using their GitLab accounts (gitlab.com or self-hosted GitLab instance). When authentication is enabled, you can restrict URL creation to authenticated users only.

## Prerequisites

- GitLab account (gitlab.com or self-hosted instance)
- Admin access to create OAuth applications in GitLab
- Shortly instance with publicly accessible URL

## Step 1: Create GitLab OAuth Application

### For GitLab.com

1. Log in to [gitlab.com](https://gitlab.com)
2. Navigate to **User Settings** → **Applications**
3. Click **Add new application**
4. Fill in the application details:

   | Field | Value |
   |-------|-------|
   | **Name** | Shortly URL Shortener |
   | **Redirect URI** | `https://your-domain.com/api/auth/callback` |
   | **Confidential** | Yes (checked) |
   | **Scopes** | `read_user` (only this scope is required) |

5. Click **Save application**
6. **Important**: Copy the **Application ID** and **Secret** - you'll need these for configuration

### For Self-Hosted GitLab

1. Log in to your GitLab instance
2. Navigate to **User Settings** → **Applications** (or **Admin Area** → **Applications** for instance-wide apps)
3. Follow the same steps as above, but use your GitLab instance URL:
   - **Redirect URI**: `https://your-shortly-domain.com/api/auth/callback`

**Note**: The redirect URI must exactly match the URL where Shortly will handle OAuth callbacks.

## Step 2: Configure Shortly

### Configuration File Method

Edit your `config.yml`:

```yaml
# Base URL must match your public Shortly URL
base-url: "https://your-domain.com"

# Enable authentication
auth:
  enabled: true
  type: gitlab
  
  providers:
    gitlab:
      # For gitlab.com use: https://gitlab.com
      # For self-hosted use your GitLab URL: https://gitlab.example.com
      base-url: https://gitlab.com
      
      # Application ID from Step 1
      application-id: YOUR_APPLICATION_ID_HERE
      
      # Secret from Step 1
      secret: YOUR_SECRET_HERE

# Optional: Require authentication for URL creation
features:
  create-url:
    enabled: true
    auth-only: true  # Set to true to require login for creating URLs
```

### Environment Variables Method (Recommended for Production)

For better security, use environment variables instead of storing credentials in config files:

```bash
# Enable authentication
export AUTH_ENABLED=true
export AUTH_TYPE=gitlab

# GitLab OAuth configuration
export AUTH_PROVIDERS_GITLAB_BASE_URL="https://gitlab.com"
export AUTH_PROVIDERS_GITLAB_APPLICATION_ID="your-application-id"
export AUTH_PROVIDERS_GITLAB_SECRET="your-secret"

# Base URL (must match your public URL)
export BASE_URL="https://your-domain.com"

# Optional: Require authentication for URL creation
export FEATURES_CREATE_URL_AUTH_ONLY=true
```

### Docker Compose Example

Create a `.env` file:

```bash
AUTH_ENABLED=true
AUTH_TYPE=gitlab
AUTH_PROVIDERS_GITLAB_BASE_URL=https://gitlab.com
AUTH_PROVIDERS_GITLAB_APPLICATION_ID=your-application-id
AUTH_PROVIDERS_GITLAB_SECRET=your-secret
BASE_URL=https://your-domain.com
FEATURES_CREATE_URL_AUTH_ONLY=true
```

Your `docker-compose.yml` should pass these variables:

```yaml
services:
  shortly:
    image: tinyops/shortly:latest
    environment:
      - AUTH_ENABLED=${AUTH_ENABLED}
      - AUTH_TYPE=${AUTH_TYPE}
      - AUTH_PROVIDERS_GITLAB_BASE_URL=${AUTH_PROVIDERS_GITLAB_BASE_URL}
      - AUTH_PROVIDERS_GITLAB_APPLICATION_ID=${AUTH_PROVIDERS_GITLAB_APPLICATION_ID}
      - AUTH_PROVIDERS_GITLAB_SECRET=${AUTH_PROVIDERS_GITLAB_SECRET}
      - BASE_URL=${BASE_URL}
      - FEATURES_CREATE_URL_AUTH_ONLY=${FEATURES_CREATE_URL_AUTH_ONLY}
```

### Kubernetes/Helm Example

Store credentials in a Kubernetes Secret:

```bash
kubectl create secret generic shortly-gitlab-oauth \
  --from-literal=application-id='your-application-id' \
  --from-literal=secret='your-secret'
```

Reference in your values.yaml or deployment:

```yaml
env:
  - name: AUTH_ENABLED
    value: "true"
  - name: AUTH_TYPE
    value: "gitlab"
  - name: AUTH_PROVIDERS_GITLAB_BASE_URL
    value: "https://gitlab.com"
  - name: AUTH_PROVIDERS_GITLAB_APPLICATION_ID
    valueFrom:
      secretKeyRef:
        name: shortly-gitlab-oauth
        key: application-id
  - name: AUTH_PROVIDERS_GITLAB_SECRET
    valueFrom:
      secretKeyRef:
        name: shortly-gitlab-oauth
        key: secret
```

## Step 3: Restart Shortly

After configuration, restart the Shortly application:

```bash
# Docker
docker-compose restart

# Kubernetes
kubectl rollout restart -n shortly deployment/shortly
```

## Step 4: Verify Authentication

1. Navigate to your Shortly instance: `https://your-domain.com`
2. Click the **Login** button
3. You should be redirected to GitLab OAuth authorization page
4. After authorizing, you'll be redirected back to Shortly and logged in
5. Your GitLab username and avatar should appear in the UI

## Authentication Flow

Here's how the OAuth flow works:

```
User → Shortly Login → GitLab OAuth → User Authorizes → 
GitLab Callback → Shortly Creates Session → User Logged In
```

**Detailed steps:**

1. User clicks "Login" → `GET /api/auth/login`
2. Shortly redirects to GitLab OAuth URL
3. User authorizes the application on GitLab
4. GitLab redirects back to `GET /api/auth/callback?code=...`
5. Shortly exchanges the code for an access token
6. Shortly fetches user info from GitLab API (`/api/v4/user`)
7. Shortly creates/updates user record in database
8. Shortly creates session and sets session cookie
9. User is redirected to homepage, now authenticated

## Configuration Options

### Authentication Required for URL Creation

```yaml
features:
  create-url:
    enabled: true
    auth-only: true  # true = login required, false = anyone can create URLs
```

### Disable Authentication Entirely

```yaml
auth:
  enabled: false  # Completely disable authentication

features:
  create-url:
    enabled: true
    auth-only: false  # Anyone can create URLs
```

## Database Schema

When authentication is enabled, Shortly uses three tables:

### Users Table
Stores GitLab user information:
- `id`: Internal user ID
- `gitlab_id`: GitLab user ID (unique)
- `username`: GitLab username
- `email`: User email (optional)
- `avatar_url`: Profile picture URL
- `created_at`, `updated_at`: Timestamps

### Sessions Table
Stores active user sessions:
- `token`: Session token (stored in cookie)
- `user_id`: Reference to users table
- `created_at`, `last_used_at`, `expires_at`: Session lifecycle

### URLs Table
Enhanced with user tracking:
- `user_id`: Reference to users table (which user created the URL)

## API Endpoints

When authentication is enabled, the following endpoints are available:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/auth/login` | GET | Initiates OAuth flow, redirects to GitLab |
| `/api/auth/callback` | GET | OAuth callback handler (GitLab redirects here) |
| `/api/auth/session` | GET | Returns current session info (user data) |
| `/api/auth/logout` | POST | Destroys session, logs user out |

## Related Documentation

- [Configuration Guide](configuration.md) - All configuration options
- [Security Guide](security.md) - Security best practices
- [Installation Guide](install/) - Deployment instructions
