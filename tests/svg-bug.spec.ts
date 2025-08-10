import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test('SVG feDropShadow should render with red glow', async ({ page }) => {
  // Navigate to the page
  await page.goto('/');
  
  // Wait for the SVG to be present
  await page.waitForSelector('svg');
  
  // Wait a bit for rendering to complete
  await page.waitForTimeout(1000);
  
  // Take a screenshot
  const screenshot = await page.screenshot();
  
  // Save the screenshot for inspection
  const screenshotPath = path.join(__dirname, 'svg-test-screenshot.png');
  fs.writeFileSync(screenshotPath, screenshot);
  console.log(`Screenshot saved to: ${screenshotPath}`);
  
  // Use Playwright's evaluate to check pixels directly in the browser
  const pixelAnalysis = await page.evaluate(() => {
    // Create a canvas to render and analyze the page
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d', { willReadFrequently: true });
    
    // Set canvas size to match viewport
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    
    // Draw the current page content to canvas using SVG serialization
    return new Promise<{ hasNonWhitePixels: boolean; hasRedPixels: boolean; samplePixels: string[] }>((resolve) => {
      const svg = document.querySelector('svg');
      if (!svg) {
        resolve({ hasNonWhitePixels: false, hasRedPixels: false, samplePixels: [] });
        return;
      }
      
      // Get SVG bounds
      const rect = svg.getBoundingClientRect();
      
      // Create an image from the SVG
      const svgData = new XMLSerializer().serializeToString(svg);
      const svgBlob = new Blob([svgData], { type: 'image/svg+xml;charset=utf-8' });
      const url = URL.createObjectURL(svgBlob);
      
      const img = new Image();
      img.onload = () => {
        // Draw the SVG to canvas
        ctx.fillStyle = 'white';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, rect.left, rect.top, rect.width, rect.height);
        
        // Get image data
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;
        
        let hasNonWhitePixels = false;
        let hasRedPixels = false;
        const samplePixels: string[] = [];
        
        // Sample pixels around where the rect should be (center area)
        const centerX = Math.floor(rect.left + rect.width / 2);
        const centerY = Math.floor(rect.top + rect.height / 2);
        
        // Check a grid of pixels around the center
        for (let dy = -60; dy <= 60; dy += 10) {
          for (let dx = -60; dx <= 60; dx += 10) {
            const x = centerX + dx;
            const y = centerY + dy;
            
            if (x >= 0 && x < canvas.width && y >= 0 && y < canvas.height) {
              const idx = (y * canvas.width + x) * 4;
              const r = data[idx];
              const g = data[idx + 1];
              const b = data[idx + 2];
              
              // Log some sample pixels for debugging
              if (samplePixels.length < 10) {
                samplePixels.push(`(${x},${y}): rgb(${r},${g},${b})`);
              }
              
              // Check if pixel is not white (with tolerance)
              if (r < 250 || g < 250 || b < 250) {
                hasNonWhitePixels = true;
                
                // Check if pixel has significant red component (from red drop shadow)
                if (r > g + 10 && r > b + 10 && r > 50) {
                  hasRedPixels = true;
                }
              }
            }
          }
        }
        
        URL.revokeObjectURL(url);
        resolve({ hasNonWhitePixels, hasRedPixels, samplePixels });
      };
      
      img.onerror = () => {
        URL.revokeObjectURL(url);
        resolve({ hasNonWhitePixels: false, hasRedPixels: false, samplePixels: ['SVG rendering failed'] });
      };
      
      img.src = url;
    });
  });
  
  console.log('Pixel analysis:', pixelAnalysis);
  
  // Currently, yew lowercases camelCased svg tags, in browsers including chrome at least, the svg fails to render and the page fully white.
  // when https://github.com/yewstack/yew/pull/3875 is merged, we should expect this to be true.
  expect(pixelAnalysis.hasRedPixels).toBe(false);
});
