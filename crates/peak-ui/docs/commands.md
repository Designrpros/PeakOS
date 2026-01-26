# Neural Control API Commands 🧠🎮

This document lists all available commands for the PeakUI Neural Control API. Commands are sent by writing a JSON object to `.peak/command.json`.

## Navigation Commands

### SetTab
Navigates to a specific page in the application.

**Format:**
```json
{
  "SetTab": "PageVariant"
}
```

**Common Page Variants:**
- `Introduction`
- `VStack`
- `HStack`
- `ZStack`
- `Button`
- `ScrollView`
- `PeakDB`
- `Appearance`

## Component Lab Commands

### UpdateButtonVariant
Changes the visual variant of the button in the lab.

**Format:**
```json
{
  "UpdateButtonVariant": "VariantName"
}
```
**Variants:** `Solid`, `Soft`, `Outline`, `Ghost`

### UpdateButtonIntent
Changes the semantic intent (status) of the button in the lab.

**Format:**
```json
{
  "UpdateButtonIntent": "IntentName"
}
```
**Intents:** `Primary`, `Secondary`, `Success`, `Warning`, `Danger`, `Info`, `Neutral`

## Usage Example
To navigate to the button lab and set it to an outline danger status:
```bash
echo '{"SetTab": "Button"}' > .peak/command.json
sleep 0.5
echo '{"UpdateButtonVariant": "Outline"}' > .peak/command.json
sleep 0.5
echo '{"UpdateButtonIntent": "Danger"}' > .peak/command.json
```

## Usage Example
To navigate to the VStack documentation:
```bash
echo '{"SetTab": "VStack"}' > .peak/command.json
```

---
*Note: This API is currently enabled only in the Desktop build.*
 Wildebeest
