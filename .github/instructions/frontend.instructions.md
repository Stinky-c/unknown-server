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

**Theme System:**

- Uses CSS custom properties for theming
- Base colors: `bg-base-100`, `bg-base-200`, `bg-base-300`
- Text colors: `text-base-content`, `text-muted-foreground`
- Semantic colors: `btn-primary`, `btn-secondary`, `btn-accent`

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
