# Geographical Data Population Excel Template

This document describes the structure of the Excel file used to populate geographical data into the system. The Excel file should contain three sheets: `Countries`, `Subdivisions`, and `Localities`.

**Important:** You must generate a unique UUID for each `id` field. These UUIDs are used to establish relationships between the entities (e.g., linking a subdivision to a country).

---

### Sheet 1: `Countries`

This sheet contains the list of countries.

| Column Header | Type         | Required | Description                               |
|---------------|--------------|----------|-------------------------------------------|
| `id`          | UUID         | Yes      | A unique identifier for the country.      |
| `iso2`        | Text (2)     | Yes      | The 2-letter ISO 3166-1 alpha-2 code.     |
| `name_l1`     | Text (100)   | Yes      | The country name in the primary language. |
| `name_l2`     | Text (100)   | No       | The country name in the second language.  |
| `name_l3`     | Text (100)   | No       | The country name in the third language.   |

---

### Sheet 2: `Subdivisions`

This sheet contains the list of country subdivisions (e.g., states, provinces, regions).

| Column Header | Type         | Required | Description                                             |
|---------------|--------------|----------|---------------------------------------------------------|
| `id`          | UUID         | Yes      | A unique identifier for the subdivision.                |
| `country_id`  | UUID         | Yes      | The `id` of the parent country from the `Countries` sheet. |
| `code`        | Text (10)    | Yes      | A code for the subdivision. **Format:** `{country_iso2}_{subdivision_code}` (e.g., `CM_AD`). If a country has no subdivisions, use the country's `iso2` code (e.g., `CF`). |
| `name_l1`     | Text (100)   | Yes      | The subdivision name in the primary language.           |
| `name_l2`     | Text (100)   | No       | The subdivision name in the second language.            |
| `name_l3`     | Text (100)   | No       | The subdivision name in the third language.             |

---

### Sheet 3: `Localities`

This sheet contains the list of localities (e.g., cities, towns).

| Column Header | Type         | Required | Description                                                   |
|---------------|--------------|----------|---------------------------------------------------------------|
| `id`          | UUID         | Yes      | A unique identifier for the locality.                         |
| `country_subdivision_id` | UUID | Yes    | The `id` of the parent subdivision from the `Subdivisions` sheet. |
| `code`        | Text (50)    | Yes      | A code for the locality. **Format:** `{country_iso2}_{first_5_chars_of_name_l1}{2_digits}` (e.g., `CM_NGAOU01`). The digits are used for uniqueness within a subdivision. |
| `name_l1`     | Text (50)    | Yes      | The locality name in the primary language.                    |
| `name_l2`     | Text (50)    | No       | The locality name in the second language.                     |
| `name_l3`     | Text (50)    | No       | The locality name in the third language.                      |
