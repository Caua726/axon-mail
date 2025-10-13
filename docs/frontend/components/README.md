# Components Module Documentation

This document provides a general overview of the UI components used in the Axon-Mail frontend application.

## Overview

The `components` module is the heart of the application's user interface. It contains a collection of reusable Yew components that are composed to create the different pages and views of the application. Each component is designed to be self-contained, with its own logic, state, and presentation.

## Component List

Below is a summary of the main components in this module:

### `Sidebar`

- **Purpose**: A collapsible navigation panel that provides access to the main sections of the application.
- **Features**: Includes a header, navigation links, and a footer with user plan information.
- **Detailed Docs**: [`sidebar.md`](./sidebar.md)

### `Header`

- **Purpose**: The top navigation bar of the application.
- **Features**: Contains the application title, a button to toggle the sidebar, and other action buttons.
- **Detailed Docs**: [`header.md`](./header.md)

### `SearchInterface`

- **Purpose**: The main interface for starting a new email search.
- **Features**: A simple search bar for quick searches and a comprehensive "Advanced Options" modal for more detailed queries.
- **Detailed Docs**: [`search_interface.md`](./search_interface.md)

### `EmailSearch`

- **Purpose**: Displays the real-time results and progress of an active email search.
- **Features**: A dashboard with search metrics, a filterable results table, and a simulation of the search process.
- **Detailed Docs**: [`email_search.md`](./email_search.md)

## Composition

These components are composed in `src/main.rs` to create the two main pages of the application: the `HomePage` (which includes the `Header` and `SearchInterface`) and the `BuscandoPage` (which includes the `Header` and `EmailSearch` component). The `Sidebar` is included in both pages to provide consistent navigation.