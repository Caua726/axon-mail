# Hyper-Detailed Documentation: `search_interface.rs`

This document provides a line-by-line analysis of the `frontend/src/components/search_interface.rs` file.

## Overview

This file defines the `SearchInterface` component, which is the main view on the home page. It provides a user-friendly interface for starting an email search, with both a simple search bar and an extensive advanced options modal.

## Code Breakdown

### `SearchInterfaceProps` Struct

```rust
#[derive(Properties, PartialEq)]
pub struct SearchInterfaceProps {
    pub on_start_search: Callback<()>,
}
```

- **`pub on_start_search: Callback<()>`**: This is the single property for the component. It's a callback that gets invoked when the user clicks the "Email Search" button, signaling the parent component to initiate the search and navigate to the results page.

### `SearchInterface` Component

This is a large component with a significant amount of state, managed by numerous `use_state` hooks.

#### State Management

The component uses `use_state` for all its dynamic data:

- `search_query`: Stores the text from the main search input.
- `show_advanced`: A boolean that controls the visibility of the advanced options modal.
- `search_category`: Stores the selected search type (e.g., "bulk_business").
- `ai_model`, `ollama_model`: Store the selected AI models.
- **Form-specific state**: A group of `use_state` hooks is dedicated to each form within the advanced modal (e.g., `business_category`, `company_name`, `website_domain`).
- `results_limit`, `search_mode`: Store the values for the general settings.

#### Callbacks

- **`toggle_advanced`**: A simple callback that inverts the boolean value of `show_advanced`.
- **`close_advanced`**: Sets `show_advanced` to `false`. This is used for the modal's close button and for the background overlay click.
- **`on_search_query_change`**: Updates the `search_query` state. It's a standard pattern for handling input changes in Yew.

#### Template (HTML)

The component's template is divided into two main parts: the main search interface and the advanced options modal.

##### Main Search Interface

- It's designed to be clean and focused, with the "axonemail" logo, a prominent search bar, and two main action buttons.
- The "Email Search" button's `onclick` handler clones the `on_start_search` prop and emits it, triggering the navigation in the parent component.
- The "Advanced Options" button's `onclick` handler is bound to the `toggle_advanced` callback.

##### Advanced Options Modal

- This section is rendered conditionally using `if *show_advanced { ... }`.
- It includes a semi-transparent background overlay that, when clicked, closes the modal.
- The modal itself is a large, scrollable container with a fixed position.
- **AI Model Selection**: A section with dropdowns to select the desired AI model. The second dropdown for the Ollama model is rendered conditionally based on the selection in the first.
- **Search Type Selection**: A set of radio buttons that allows the user to choose between different search categories. The selected category is stored in the `search_category` state.
- **Dynamic Options**: Based on the value of `search_category`, a different set of form fields is rendered. This is a key feature of the component's interactivity.
- **General Settings**: A final section with options that apply to all search types, such as the results limit and search mode.
- **Action Buttons**: "Cancel" and "Search" buttons at the bottom of the modal. Currently, both close the modal. In a real implementation, the "Search" button would gather all the state from the form fields and pass it to the `on_start_search` callback.