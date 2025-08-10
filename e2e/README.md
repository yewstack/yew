# E2E Tests

This directory contains end-to-end tests for Yew using Playwright.

## SVG Rendering Test

The test (`tests/svg-bug.spec.ts`) verifies proper rendering of SVG elements with camelCased tag names. 

### The Bug/Feature

Yew currently has an issue where camelCase SVG filter primitive elements (like `feDropShadow`) are converted to lowercase, causing them to fail to render in some browsers. This test:

1. Renders an SVG with a `feDropShadow` filter effect (should create a red glow)
2. Takes a screenshot and analyzes the pixels
3. **Currently expected to fail** because all pixels are white - the drop shadow doesn't render

## Running Tests Locally

at the root of the project:
```bash
# Install dependencies
npm install

# Install Playwright browsers
npx playwright install chromium

# Run tests
npx playwright test

# Run tests with UI
npx playwright test --ui

# View test report
npx playwright show-report
```


The tests automatically start a development server using `trunk serve` before running.
