# CertaBlock Setup Guide

This guide provides detailed instructions for setting up CertaBlock on your local machine.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Backend Setup](#backend-setup)
- [Frontend Setup](#frontend-setup)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

Before you begin, ensure you have the following installed:

### Required Software

| Software    | Version | Installation Link                   |
| ----------- | ------- | ----------------------------------- |
| **Rust**    | 1.70+   | [rustup.rs](https://rustup.rs/)     |
| **Node.js** | 18+     | [nodejs.org](https://nodejs.org/)   |
| **Git**     | Latest  | [git-scm.com](https://git-scm.com/) |

### Browser Extension

- **MetaMask**: Install from [metamask.io](https://metamask.io/)

### Verify Installations

```bash
# Check Rust version
rustc --version
cargo --version

# Check Node.js version
node --version
npm --version

# Check Git version
git --version
```

---

## Backend Setup

### 1. Clone the Repository

```bash
git clone https://github.com/hasancoded/CertaBlock.git
cd CertaBlock
```

### 2. Build the Backend

```bash
# Development build
cargo build

# Production build (optimized)
cargo build --release
```

**Expected Output:**

```
   Compiling certablock v0.1.0
    Finished release [optimized] target(s) in X.XXs
```

### 3. Run the Backend

```bash
# Development mode
cargo run

# Production mode
cargo run --release
```

**Expected Output:**

```
CertaBlock REST API running on http://127.0.0.1:3000
CORS enabled for React app on http://localhost:5173
Available endpoints:
  AUTHENTICATION:
      POST /auth/nonce
      POST /auth/verify
  ...
```

The backend will be available at `http://127.0.0.1:3000`

### 4. Test the Backend

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

---

## Frontend Setup

### 1. Navigate to Dashboard Directory

```bash
cd dashboard
```

### 2. Install Dependencies

```bash
npm install
```

**Expected Output:**

```
added XXX packages in XXs
```

### 3. Start Development Server

```bash
npm run dev
```

**Expected Output:**

```
  VITE v5.x.x  ready in XXX ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
```

The dashboard will be available at `http://localhost:5173`

### 4. Build for Production

```bash
npm run build
```

**Expected Output:**

```
vite v5.x.x building for production...
✓ XXX modules transformed.
dist/index.html                  X.XX kB
dist/assets/index-XXXXX.js      XXX.XX kB
✓ built in X.XXs
```

---

## Verification

### 1. Backend Health Check

```bash
curl http://127.0.0.1:3000/stats
```

**Expected Response:**

```json
{
  "total_blocks": 1,
  "pending_transactions": 0,
  "mining_difficulty": 4,
  "is_mining": false
}
```

### 2. Frontend Access

1. Open browser to `http://localhost:5173`
2. You should see the CertaBlock dashboard
3. Click "Connect Wallet" to connect MetaMask

### 3. MetaMask Configuration

1. Open MetaMask
2. Connect to localhost network (if needed)
3. Approve the connection request from CertaBlock

---

## Troubleshooting

### Backend Issues

#### Port Already in Use

**Error:**

```
Error: Address already in use (os error 48)
```

**Solution:**

```bash
# Find process using port 3000
lsof -i :3000  # macOS/Linux
netstat -ano | findstr :3000  # Windows

# Kill the process or change the port in src/main.rs
```

#### Rust Toolchain Missing

**Error:**

```
error: no default toolchain configured
```

**Solution:**

```bash
rustup default stable
```

#### Build Failures

**Solution:**

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

### Frontend Issues

#### Node Modules Issues

**Solution:**

```bash
# Remove node_modules and package-lock.json
rm -rf node_modules package-lock.json

# Reinstall
npm install
```

#### Port 5173 Already in Use

**Solution:**

```bash
# Kill the process or change port in vite.config.js
# Or use a different port
npm run dev -- --port 5174
```

#### CORS Errors

**Ensure:**

- Backend is running on `http://127.0.0.1:3000`
- Frontend is running on `http://localhost:5173`
- CORS is properly configured in backend

### MetaMask Issues

#### Connection Rejected

**Solution:**

1. Open MetaMask
2. Go to Settings → Connected Sites
3. Remove CertaBlock if listed
4. Refresh the page and reconnect

#### Wrong Network

**Solution:**

1. Switch to Ethereum Mainnet or your preferred network
2. Refresh the page

---

## Next Steps

After successful setup:

1. **Explore the Dashboard**: Navigate through different sections
2. **Mine a Block**: Try the mining functionality with different difficulty levels
3. **Issue a Certificate**: Test the certificate management system
4. **Transfer Tokens**: Experiment with the token system
5. **Read the Documentation**: Check out [API.md](./API.md) and [WHITEPAPER.md](./WHITEPAPER.md)

---

## Additional Resources

- **API Documentation**: [docs/API.md](./API.md)
- **Technical Whitepaper**: [docs/WHITEPAPER.md](./WHITEPAPER.md)
- **Contributing Guide**: [CONTRIBUTING.md](../CONTRIBUTING.md)
- **GitHub Repository**: [github.com/hasancoded/CertaBlock](https://github.com/hasancoded/CertaBlock)

---

For additional help, please open an issue on GitHub.
