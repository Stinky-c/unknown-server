---
applyTo: "unknown-web/**"
---

# Frontend Development Instructions

## Tech Stack Overview

The `unknown-web` frontend uses a modern static site approach with:

- **Parcel v2**: Build system with automatic dependency resolution
- **TailwindCSS v4**: Utility-first CSS framework
- **DaisyUI v5**: Component library built on Tailwind
- **HTMX v2**: Server-side rendering with minimal JavaScript
- **PostHTML**: Template processing with includes

## CSS Framework Usage

### TailwindCSS v4 Configuration

TailwindCSS is configured via:

- `src/css/app.css`: Main stylesheet with `@import "tailwindcss"`
- `tailwind.config.js`: Configuration file (currently minimal)
- Content scanning: `./src/*.{html,js}` for class detection

**Key TailwindCSS Patterns:**

- Use utility classes for layout: `flex`, `grid`, `container mx-auto`
- Responsive design: `sm:`, `md:`, `lg:`, `xl:` prefixes
- Spacing system: `p-4`, `m-2`, `gap-6`, etc.
- Colors: Use semantic color names that work with DaisyUI themes

### DaisyUI Component System

DaisyUI is loaded via `@plugin "daisyui"` in the CSS file and provides:

**Layout Components:**

- `navbar`: Navigation bars with `navbar-start`, `navbar-center`, `navbar-end`
- `hero`: Landing page hero sections with `hero-content`
- `footer`: Page footers with flexible layouts
- `container`: Responsive containers with `mx-auto px-6`

**UI Components:**

- `btn`: Buttons with variants (`btn-primary`, `btn-outline`, `btn-lg`)
- `card`: Content cards with `card-body`, `card-title`, `card-actions`
- `modal`: Dialog modals with `modal-backdrop`
- `form-control`: Form layouts with `label` and `input`
- `loading`: Loading spinners and skeletons

**GitHub Dimmed Dark Theme:**

- **Default Theme**: `data-theme="github-dark"` applied to `<html>` element
- **Color System**: Based on GitHub's dimmed dark palette for consistent visual experience
- **Base Canvas**: `bg-base-100` (#22272e), `bg-base-200` (#2d333b), `bg-base-300` (#373e47)
- **Text Colors**: `text-base-content` (#adbac7), `text-muted-foreground` (#768390)
- **Accent Colors**: Primary (#539bf5), Success (#57ab5a), Warning (#c69026), Danger (#e5534b)
- **Custom Variables**: Extended TailwindCSS with `--gh-*` color tokens for semantic theming

**Example DaisyUI Usage:**

```html
<div class="card bg-base-100 shadow-md">
  <div class="card-body">
    <h2 class="card-title">Card Title</h2>
    <p>Card content goes here</p>
    <div class="card-actions justify-end">
      <button class="btn btn-primary">Action</button>
    </div>
  </div>
</div>
```

## HTMX Integration

### Setup and Configuration

HTMX is imported in `src/js/app.js`:

```javascript
import "htmx.org";
```

The library is automatically available globally and requires no additional setup.

### HTMX Usage Patterns

**Basic Attributes:**

- `hx-get="/api/endpoint"`: GET requests
- `hx-post="/api/submit"`: POST requests with form data
- `hx-target="#result"`: Element to update with response
- `hx-swap="innerHTML"`: How to insert content (innerHTML, outerHTML, afterbegin, etc.)

**Form Handling:**

```html
<form hx-post="/api/users" hx-target="#user-list" hx-swap="beforeend">
  <input name="name" required />
  <button type="submit" class="btn btn-primary">Add User</button>
</form>
```

**Dynamic Content Loading:**

```html
<div hx-get="/api/stats" hx-trigger="load, every 30s" hx-target="this">
  Loading stats...
</div>
```

**Partial Updates:**

```html
<button
  hx-delete="/api/item/{id}"
  hx-target="closest .card"
  hx-swap="outerHTML"
  hx-confirm="Are you sure?"
  class="btn btn-error btn-sm"
>
  Delete
</button>
```

**Advanced Features:**

- `hx-trigger="click, keyup delay:500ms"`: Multiple triggers with delays
- `hx-headers='{"X-Custom": "value"}'`: Custom headers
- `hx-vals='{"extra": "data"}'`: Additional data to send
- `hx-indicator="#spinner"`: Show loading states

### HTMX Best Practices

1. **Server Responses**: Return HTML fragments that match your target elements
2. **Loading States**: Use `hx-indicator` with DaisyUI loading components
3. **Error Handling**: Use `hx-on::response-error` for error states
4. **Progressive Enhancement**: Forms should work without JavaScript
5. **SEO Considerations**: Important content should be server-rendered initially

## Component Architecture

### PostHTML Includes

Use `<include src="path/to/component.html">` for reusable components:

```html
<!-- In index.html -->
<include src="src/components/navbar.html"></include>

<!-- In src/components/navbar.html -->
<nav class="navbar bg-base-100">
  <div class="navbar-start">
    <a class="btn btn-ghost text-xl">Logo</a>
  </div>
</nav>
```

### Component Organization

- `src/components/`: Reusable HTML components
- `src/pages/`: Page-specific templates
- `src/css/`: Stylesheets and custom CSS
- `src/js/`: JavaScript modules and entry points

## Development Workflow

### Available Tasks (use `mise run <task>`)

- `unknown-web:dev`: Development server with hot reload
- `unknown-web:build`: Production build to `dist/`
- `unknown-web:clean`: Clean build artifacts

### Development Server

The Parcel dev server:

- Runs on `http://localhost:1234` by default
- Proxies `/api` requests to backend at `http://localhost:3000` (via `.proxyrc`)
- Provides hot reload for HTML, CSS, and JS changes
- Auto-installs npm dependencies on first import

### Build Process

Production builds:

1. Process PostHTML includes and templates
2. Bundle and minify CSS with TailwindCSS
3. Bundle JavaScript with dependencies
4. Output to `dist/` directory
5. Copy to `unknown-server/assets/` for embedding

## Styling Guidelines

### Responsive Design

```html
<!-- Mobile-first responsive layout -->
<div class="flex flex-col lg:flex-row gap-6">
  <div class="w-full lg:w-1/2">Content</div>
  <div class="w-full lg:w-1/2">Sidebar</div>
</div>
```

### Form Styling

```html
<div class="form-control w-full max-w-xs">
  <label class="label">
    <span class="label-text">Username</span>
  </label>
  <input type="text" class="input input-bordered" name="username" />
  <label class="label">
    <span class="label-text-alt text-error">Error message</span>
  </label>
</div>
```

### Interactive States

```html
<button
  class="btn btn-primary loading:btn-disabled"
  hx-post="/api/action"
  hx-indicator="this"
>
  <span class="loading loading-spinner loading-sm htmx-indicator"></span>
  Submit
</button>
```

## GitHub Dimmed Dark Theme System

### Theme Configuration

The project uses a custom GitHub-inspired dimmed dark theme configured through:

**TailwindCSS Extensions** (`tailwind.config.js`):

- Custom color palette matching GitHub's dimmed dark theme
- Extended color tokens: `gh-canvas-*`, `gh-fg-*`, `gh-accent-*`, `gh-success-*`, etc.
- Consistent typography and spacing scales

**DaisyUI Theme** (`src/css/app.css`):

- Custom `github-dark` theme with OKLCH color space values
- Semantic color mapping: primary, secondary, accent, neutral states
- Component-level theming for buttons, cards, forms, and alerts

### Color Palette

**Canvas Colors:**

```css
--gh-canvas-default: #22272e; /* Main background */
--gh-canvas-overlay: #2d333b; /* Cards, modals, overlays */
--gh-canvas-inset: #1c2128; /* Input backgrounds, code blocks */
--gh-canvas-subtle: #373e47; /* Subtle backgrounds, disabled states */
```

**Foreground Colors:**

```css
--gh-fg-default: #adbac7; /* Primary text */
--gh-fg-muted: #768390; /* Secondary text, placeholders */
--gh-fg-subtle: #545d68; /* Subtle text, hints */
--gh-fg-on-emphasis: #ffffff; /* Text on colored backgrounds */
```

**Semantic Colors:**

```css
--gh-accent-fg: #539bf5; /* Links, primary actions */
--gh-success-fg: #57ab5a; /* Success states */
--gh-attention-fg: #c69026; /* Warning states */
--gh-danger-fg: #e5534b; /* Error states, destructive actions */
```

**Border Colors:**

```css
--gh-border-default: #444c56; /* Standard borders */
--gh-border-muted: #373e47; /* Subtle borders */
--gh-border-subtle: #2d333b; /* Very subtle dividers */
```

### Theme Usage Patterns

**Apply Theme to Pages:**

```html
<html lang="en" data-theme="github-dark">
  <body class="bg-base-200 text-base-content">
    <!-- Content uses theme automatically -->
  </body>
</html>
```

**DaisyUI Components with Theme:**

```html
<!-- Buttons automatically use theme colors -->
<button class="btn btn-primary">Primary Action</button>
<button class="btn btn-secondary">Secondary Action</button>
<button class="btn btn-error">Destructive Action</button>

<!-- Cards with proper contrast -->
<div class="card bg-base-100 shadow-md">
  <div class="card-body">
    <h2 class="card-title">Themed Card</h2>
  </div>
</div>

<!-- Form inputs with theme styling -->
<input class="input input-bordered" placeholder="Themed input" />
```

**Custom Components with Theme Variables:**

```css
/* Use CSS custom properties for consistency */
.custom-component {
  background: var(--gh-canvas-overlay);
  border: 1px solid var(--gh-border-default);
  color: var(--gh-fg-default);
}

.custom-component:hover {
  border-color: var(--gh-accent-fg);
}
```

**TailwindCSS Utility Classes:**

```html
<!-- Extended GitHub color utilities -->
<div class="bg-gh-canvas-overlay border-gh-border-default">
  <span class="text-gh-fg-muted">Muted text</span>
  <a class="text-gh-accent-fg hover:text-gh-accent-emphasis">Link</a>
</div>
```

### Alert and Status Components

**Themed Alert Patterns:**

```html
<!-- Success alert -->
<div
  class="alert alert-success bg-gh-success-subtle border-gh-success-muted text-gh-success-fg"
>
  <span>Operation completed successfully</span>
</div>

<!-- Error alert -->
<div
  class="alert alert-error bg-gh-danger-subtle border-gh-danger-muted text-gh-danger-fg"
>
  <span>An error occurred</span>
</div>

<!-- Warning alert -->
<div
  class="alert alert-warning bg-gh-attention-subtle border-gh-attention-muted text-gh-attention-fg"
>
  <span>Please review this action</span>
</div>

<!-- Info alert -->
<div
  class="alert alert-info bg-gh-accent-subtle border-gh-accent-muted text-gh-accent-fg"
>
  <span>Additional information</span>
</div>
```

### Authentication Pages

Authentication pages (`login.html`, `signup.html`) are built entirely with TailwindCSS and DaisyUI:

**Page Structure:**

```html
<html lang="en" data-theme="github-dark">
  <body class="min-h-screen bg-base-100 flex items-center justify-center p-4">
    <main
      class="card bg-base-200 shadow-xl border border-base-300 w-full max-w-md"
    >
      <div class="card-body">
        <!-- Form content -->
      </div>
    </main>
  </body>
</html>
```

**Form Components:**

```html
<!-- Form control with proper spacing -->
<div class="form-control">
  <label class="label" for="username">
    <span class="label-text font-medium">Username</span>
  </label>
  <input
    id="username"
    name="username"
    type="text"
    class="input input-bordered w-full bg-base-100 focus:input-primary"
  />
</div>

<!-- Error states -->
<div id="form-errors" class="alert alert-error hidden">
  <span id="error-message"></span>
</div>

<!-- HTMX form with loading indicator -->
<form
  hx-post="/api/auth/login"
  hx-indicator="#auth-indicator"
  class="space-y-4"
>
  <div class="flex items-center gap-3">
    <div
      class="loading loading-spinner loading-sm text-primary htmx-indicator"
    ></div>
    <button type="submit" class="btn btn-primary">Sign in</button>
  </div>
</form>
```

**Key Features:**

- **No Custom CSS**: Uses only TailwindCSS and DaisyUI classes
- **GitHub Theme Integration**: Automatic theme color application via `data-theme="github-dark"`
- **HTMX Integration**: Built-in loading indicators and error handling
- **Form Validation**: Client-side validation with visual feedback using `.invalid` class
- **Responsive Design**: Mobile-first with proper touch targets
- **Accessibility**: Proper ARIA labels and semantic HTML structure

**Enhanced JavaScript Features:**

- Real-time password confirmation validation (signup page)
- Error message display and auto-hiding
- Focus management for invalid fields
- HTMX response handling with user feedback

### Theme Customization

**Adding New Theme Variants:**

1. Extend `tailwind.config.js` with new color tokens
2. Add new theme in `src/css/app.css` using DaisyUI format
3. Update component styles to use new semantic colors
4. Test contrast ratios and accessibility

**Component Theme Overrides:**

```css
/* Override specific component themes */
.custom-card {
  background: var(--gh-canvas-inset);
  border: 1px solid var(--gh-border-muted);
}

.custom-card .title {
  color: var(--gh-fg-default);
}

.custom-card .subtitle {
  color: var(--gh-fg-muted);
}
```

## Integration with Backend

### Asset Embedding

- Frontend builds are embedded into the Rust binary via `memory-serve`
- Static assets served from `/public/` route
- API endpoints available at `/api/` prefix

### Authentication Flow

- Use HTMX forms for login/signup
- Sessions handled server-side via Redis
- Include CSRF tokens in forms when required

### Error Handling

```html
<div id="error-container" class="alert alert-error hidden">
  <span id="error-message"></span>
</div>

<script>
  document.body.addEventListener("htmx:responseError", function (e) {
    const errorDiv = document.getElementById("error-container");
    const errorMsg = document.getElementById("error-message");
    errorMsg.textContent = "An error occurred";
    errorDiv.classList.remove("hidden");
  });
</script>
```
