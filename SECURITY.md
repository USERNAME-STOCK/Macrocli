# Security Policy

## Overview

Macrocli implements multiple security layers to ensure safe device programming and data integrity. This document outlines implemented security measures and vulnerability reporting procedures.

---

## Security Features

### Input Validation & Data Integrity

**Multi-Stage Validation Pipeline**

All configuration data undergoes validation before device programming:

```
User Input → Type Validation → Schema Validation → Device Compatibility → Device Write
```

**Validation Layers:**

1. **Type Safety (Compile-time)**
   - Rust's type system prevents type-related vulnerabilities
   - Eliminates null pointer dereferences and buffer overflows

2. **Schema Validation (Runtime)**
   - Input data validated against strict schemas
   - Invalid configurations rejected before processing
   - Error messages designed to prevent information leakage

3. **Device Compatibility Checks**
   - Configuration validated against device capabilities
   - Layer count and key mapping verification

4. **Binary Protocol Validation**
   - Data verification before USB transmission
   - Protocol compliance enforcement

### Access Control & Privilege Management

**Linux udev Rules**

Proper privilege separation implementation:

- Principle of least privilege (no root access required)
- User-level device access via group membership
- Explicit installation required (secure by default)

```bash
# File: 80-macrocli.rules
SUBSYSTEM=="usb", ATTR{idVendor}=="1189", MODE="0666"
```

Benefits:
- Prevents privilege escalation attacks
- Limits vulnerability impact radius
- Follows Linux security best practices

### Memory Safety

**Rust Language Guarantees**

Backend written in Rust provides:

- No buffer overflows
- No use-after-free vulnerabilities
- No data races in concurrent code
- Memory safety without garbage collection overhead

Guarantees enforced at compile-time, preventing entire vulnerability classes.

### Device Authentication

**USB Device Verification**

System verifies before operations:

- USB Vendor ID (VID): `0x1189`
- USB Product ID (PID): `0x8840`, `0x8842`, or `0x8890`
- Device capability enumeration

Prevents accidental programming of incorrect devices.

### API Security

**RESTful API Protection**

Security measures implemented:

1. **Input Sanitization**
   - All API inputs validated before processing
   - Malformed requests rejected with appropriate errors

2. **Error Handling**
   - No sensitive information in error messages
   - Generic error responses prevent information leakage
   - Detailed errors logged server-side only

3. **CORS Configuration**
   - Properly configured Cross-Origin Resource Sharing
   - Prevents unauthorized cross-origin requests

4. **No Authentication by Design**
   - Intended for localhost use only
   - Should not be exposed to public networks
   - User responsible for network security

---

## Security Considerations

### Recommended Deployment

**Safe:**
- Localhost (127.0.0.1) deployment
- Trusted local networks
- Personal device configuration

**Not Recommended:**
- Public internet exposure without authentication
- Shared/untrusted networks
- Multi-tenant environments without isolation

### Network Security

If network binding required:

```bash
# Bind to localhost only (default)
./macrocli serve --port 8080

# If binding to 0.0.0.0, use firewall rules:
sudo ufw allow from 192.168.1.0/24 to any port 8080
```

### Configuration File Security

- Treat configuration files as code
- Review imported configurations before use
- Store backup configurations securely
- Use version control for configuration management

---

## Reporting Security Vulnerabilities

We appreciate responsible disclosure of security vulnerabilities.

### Reporting Process

**For security issues:**

1. **DO NOT** open public GitHub issues for security vulnerabilities
2. Create private security advisory on GitHub or contact maintainers directly
3. Include:
   - Vulnerability description
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fixes (if any)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Updates**: Every 72 hours until resolution
- **Fix Timeline** (severity-dependent):
  - Critical: 7 days
  - High: 14 days
  - Medium: 30 days
  - Low: Best effort

### Recognition

Security researchers who responsibly disclose vulnerabilities will be:

- Credited in CHANGELOG (unless anonymity requested)
- Thanked in project documentation
- Added to security acknowledgments

---

## Security Best Practices for Contributors

### Code Review Checklist

- [ ] All user inputs validated
- [ ] No unsafe Rust code blocks (without justification)
- [ ] Error messages don't leak sensitive information
- [ ] No hardcoded credentials or secrets
- [ ] Dependencies audited (`cargo audit`)
- [ ] Proper error handling (no panics on user input)

### Testing Security Features

```bash
# Run tests with validation
cargo test

# Check for known vulnerabilities
cargo audit

# Check for unsafe code patterns
cargo clippy -- -W clippy::all
```

---

## Security Audit History

| Date | Version | Auditor | Findings | Status |
|------|---------|---------|----------|--------|
| TBD | 1.0 | Internal | N/A | Pending |

---

## Security Resources

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [USB Security Best Practices](https://www.usb.org/usb-security)
- [Secure Coding in Rust](https://doc.rust-lang.org/nomicon/)

---

## Disclosure Policy

Responsible disclosure policy:

1. Researchers have 90 days to report before public disclosure
2. Critical vulnerabilities patched within 7 days
3. Public disclosure coordinated between reporter and maintainers
4. CVEs assigned for significant vulnerabilities

---

Security is a shared responsibility among users, contributors, and maintainers.
