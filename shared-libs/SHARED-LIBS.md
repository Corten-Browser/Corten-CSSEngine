# Shared Libraries

This directory contains shared libraries used by CSS engine components.

## Libraries

### browser-types
Shared type definitions for browser integration (DomNode, Url, ViewportInfo, etc.)

**Status**: Mock implementation for standalone development
**Version**: 0.1.0

### browser-interfaces
Shared interfaces and traits for browser components (BrowserComponent trait)

**Status**: Mock implementation for standalone development
**Version**: 0.1.0

## Usage in Components

Components can depend on these shared libraries by adding to their Cargo.toml:

```toml
[dependencies]
browser-types = { path = "../../shared-libs/browser-types" }
browser-interfaces = { path = "../../shared-libs/browser-interfaces" }
```

## Integration with CortenBrowser

When the CSS Engine is integrated into the full CortenBrowser project, these mock implementations will be replaced with the actual browser-wide shared libraries.

For standalone development and testing, these mocks provide the necessary types and interfaces.
