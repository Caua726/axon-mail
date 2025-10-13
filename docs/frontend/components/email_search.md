# Hyper-Detailed Documentation: `email_search.rs`

This document provides a line-by-line analysis of the `frontend/src/components/email_search.rs` file.

## Overview

This file defines the `EmailSearch` component, which is responsible for displaying the real-time progress and results of an email search. The component is highly dynamic and uses a combination of state management and timers to simulate a live search process.

## Code Breakdown

### Structs and Enums

- **`EmailSearchProps`**: Defines the properties for the component, including the `search_id` and an `on_back` callback to return to the home page.
- **`SearchStatus`**: An enum that represents the different states of the search process (`Initializing`, `Searching`, `Completed`). This is used to display different UI elements, such as the status badge.
- **`EmailResult`**: A struct that holds the data for a single email result.
- **`SearchMetrics`**: A struct that aggregates all the real-time statistics of the search, such as the number of emails found, domains searched, and progress percentage.

### `EmailSearch` Component

This is a complex component with several key parts:

#### State Management

- **`status`**: Stores the current `SearchStatus` of the search.
- **`metrics`**: Holds the `SearchMetrics` struct, which is updated throughout the search simulation.
- **`results`**: A vector of `EmailResult` structs, representing all the results found so far.
- **`filtered_results`**: A separate vector that holds the results after applying the user's filters.
- **`iteration`**: A counter for the simulation timer.
- **`search_filter`**, **`status_filter`**: Store the current values of the text and status filters.

#### Search Simulation (`use_effect_with`)

- The core of the component's dynamic behavior is a `use_effect_with` hook that runs once when the component mounts.
- **`gloo_timers::callback::Timeout`**: This is used to create a timer that fires every second.
- **`run_simulation` function**: This is a recursive function that simulates one step of the search process. In each step, it:
  - Updates the `iteration` count.
  - Changes the `status` to `Searching`.
  - Calculates the search progress.
  - Selects a new `current_domain` from a predefined list.
  - Adds a new batch of `EmailResult`s from the `sample_data` to the `results` state.
  - Updates the `metrics` state with the new data.
  - If the simulation is not complete, it schedules the next call to itself.
- **`handle.forget()`**: This is a crucial line that prevents the `Timeout` from being dropped, which would cancel the timer.

#### Filtering Logic (`use_effect_with`)

- A second `use_effect_with` hook is used to handle the filtering of the results.
- It depends on `results`, `search_filter`, and `status_filter`. Whenever any of these change, the effect re-runs.
- It starts with a clone of the full `results` vector and then applies the filters:
  - **Text filter**: It retains only the results where the email, name, company, or title contains the search string.
  - **Status filter**: It retains results based on the `verified` status.
- The final filtered list is stored in the `filtered_results` state.

#### Template (HTML)

- **Header**: A compact header with a "Back" button, the search title and ID, and real-time metrics like the status badge and elapsed time. It also includes an integrated progress bar.
- **Metrics Cards**: A row of cards that provide a high-level overview of the key search metrics.
- **Results Table**:
  - **Toolbar**: Contains the results count, a search input, a status filter dropdown, and a CSV download button.
  - **Empty State**: If there are no results, it displays a message prompting the user to wait.
  - **Table**: If there are results, it renders a table with a header and a body.
  - **Rendering Logic**: It checks if any filters are active. If not, it renders the full `results` vector. Otherwise, it renders the `filtered_results` vector. This is an efficient way to handle the display without modifying the original results list.
  - **Row Styling**: Each row in the table is styled based on the data, for example, using different colors for the confidence score and different icons for the verification status.