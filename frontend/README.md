# German Learn Frontend

This SvelteKit application bootstraps the frontend for the `german_learn` project.
It is seeded with the UI prototypes that live under the `Pages/` directory, so each
screen can be previewed directly in the browser.

## Getting started

```bash
cd frontend
npm install
npm run dev
```

The dev server runs on <http://localhost:5173> by default.

## Available routes

The landing page (`/`) renders the primary dashboard experience. Additional prototype
screens can be accessed with paths such as:

- `/learning`
- `/learning-alt`
- `/learning-modules`
- `/course-summary`
- `/course-summary-alt`
- `/questions`
- `/questions-popup`
- `/quiz-feedback`
- `/phrase-practice`
- `/profile`
- `/vocabulary-list`
- `/home-alt`

These routes map 1:1 with the exported code inside `Pages/`. They are static mocks today,
ready to be wired up to real data as the backend evolves.

## Building for production

```bash
npm run build
npm run preview
```

The build output can be deployed behind any SvelteKit-compatible adapter. Switch adapters
as needed in `svelte.config.js`.
