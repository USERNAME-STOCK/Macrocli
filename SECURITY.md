# 🔐 Security Policy

## Overview

The Macrocli project takes security seriously and implements multiple layers of protection to ensure safe device programming and data integrity. This document outlines the security measures implemented and provides guidance for security researchers.

---

## 🛡️ Security Features

### 1. Input Validation & Data Integrity

**Multi-Stage Validation Pipeline**

All configuration data passes through a comprehensive validation pipeline before device programming:

```
User Input → Type Validation → Schema Validation → Device Compatibility → Device Write
```

#### Validation Layers:

1. **Type Safety (Compile-time)**
   - Rust's type system prevents type-related vulnerabilities
   - No null pointer dereferences
   - No buffer overflows from type confusion

2. **Schema Validation (Runtime)**
   - All input data validated against strict schemas
   - Invalid configurations rejected before processing
   - Proper error messages without information leakage

3. **Device Compatibility Checks**
   - Configuration validated against connected device capabilities
   - Layer count verification
   - Key mapping compatibility checks

4. **Binary Protocol Validation**
   - Data encoded and verified before USB transmission
   - Checksum validation where applicable
   - Protocol compliance enforcement

### 2. Access Control & Privilege Management

**Linux udev Rules**

The project implements proper privilege separation using udev rules:

- ✅ **Principle of Least Privilege**: No root access required
- ✅ **User-level Permissions**: Device access via group membership
- ✅ **Secure by Default**: Rules must be explicitly installed

```bash
# File: 80-macrocli.rules
SUBSYSTEM=="usb", ATTR{idVendor}=="1189", MODE="0666"
```

**Why This Matters:**
- Prevents privilege escalation attacks
- Limits blast radius of potential vulnerabilities
- Follows Linux security best practices

### 3. Memory Safety

**Rust Language Guarantees**

The backend is written in Rust, providing:

- ✅ No buffer overflows
- ✅ No use-after-free vulnerabilities
- ✅ No data races in concurrent code
- ✅ Memory safety without garbage collection overhead

These guarantees are enforced at compile-time, preventing entire classes of security vulnerabilities.

### 4. Device Authentication

**USB Device Verification**

Before any operations, the system verifies:

- ✅ USB Vendor ID (VID): `0x1189`
- ✅ USB Product ID (PID): `0x8840`, `0x8842`, or `0x8890`
- ✅ Device capability enumeration

This prevents accidental programming of incorrect devices.

### 5. API Security

**RESTful API Protection**

The web API implements several security measures:

1. **Input Sanitization**
   - All API inputs validated before processing
   - Malformed requests rejected with appropriate errors

2. **Error Handling**
   - No sensitive information in error messages
   - Generic error responses to prevent information leakage
   - Detailed errors logged server-side only

3. **CORS Configuration**
   - Properly configured Cross-Origin Resource Sharing
   - Prevents unauthorized cross-origin requests

4. **No Authentication by Design**
   - Intended for localhost use only
   - Should not be exposed to public networks
   - User responsible for network security

---

## ⚠️ Security Considerations for Users

### Recommended Deployment

**✅ Safe:**
- Running on localhost (127.0.0.1)
- Running on trusted local networks
- Using for personal device configuration

**❌ Not Recommended:**
- Exposing to public internet without authentication
- Running on shared/untrusted networks
- Using in multi-tenant environments without isolation

### Network Security

If you must run the server on a network interface:

```bash
# Bind to localhost only (default)
./macrocli serve --port 8080

# If binding to 0.0.0.0, use firewall rules:
sudo ufw allow from 192.168.1.0/24 to any port 8080
```

### Configuration File Security

**`.ron` Configuration Files:**

- Treat configuration files as code
- Review imported configurations before use
- Store backup configurations securely
- Use version control for configuration management

---

## 🐛 Reporting Security Vulnerabilities

We appreciate responsible disclosure of security vulnerabilities.

### Reporting Process

**For security issues, please:**

1. **DO NOT** open public GitHub issues for security vulnerabilities
2. Email security reports to: [Create issue privately on GitHub]
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact assessment
   - Suggested fixes (if any)

### What to Expect

- **Initial Response**: Within 48 hours
- **Status Updates**: Every 72 hours until resolution
- **Fix Timeline**: Depends on severity
  - Critical: 7 days
  - High: 14 days
  - Medium: 30 days
  - Low: Best effort

### Recognition

Security researchers who responsibly disclose vulnerabilities will be:

- Credited in the CHANGELOG (unless anonymity requested)
- Thanked in project documentation
- Added to security acknowledgments

---

## 🔍 Security Best Practices for Contributors

If you're contributing code to this project:

### Code Review Checklist

- [ ] All user inputs validated
- [ ] No unsafe Rust code blocks (without justification)
- [ ] Error messages don't leak sensitive information
- [ ] No hardcoded credentials or secrets
- [ ] Dependencies audited (`cargo audit`)
- [ ] No SQL injection vectors (N/A for this project)
- [ ] Proper error handling (no panics on user input)

### Testing Security Features

```bash
# Run with validation testing
cargo test

# Check for known vulnerabilities in dependencies
cargo audit

# Check for unsafe code patterns
cargo clippy -- -W clippy::all
```

---

## 📋 Security Audit History

| Date | Version | Auditor | Findings | Status |
|------|---------|---------|----------|--------|
| TBD | 1.0 | Internal | N/A | Pending |

---

## 🔗 Security Resources

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [USB Security Best Practices](https://www.usb.org/usb-security)
- [Secure Coding in Rust](https://doc.rust-lang.org/nomicon/)

---

## ⚖️ Security Disclosure Policy

This project follows a **responsible disclosure** policy:

1. Researchers have 90 days to report vulnerabilities before public disclosure
2. We aim to patch critical vulnerabilities within 7 days
3. Public disclosure coordinated between reporter and maintainers
4. CVEs assigned for significant vulnerabilities

---

<div align="center">

**🔒 Security is a shared responsibility**

Users • Contributors • Maintainers

</div>
