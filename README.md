# NVIDIA Made Notebook

A minimal Jupyter workspace for testing NVIDIA-hosted language models through the NVIDIA API Catalog's OpenAI-compatible endpoint.

## Quick start

1. Create and activate a Python virtual environment.
2. Install dependencies:

   ```bash
   pip install -r requirements.txt
   ```

3. Copy `.env.example` to `.env` and add your NVIDIA API key and model identifier.
4. Start Jupyter:

   ```bash
   jupyter notebook
   ```

5. Open `notebooks/01_nvidia_api_smoke_test.ipynb` and run the cells from top to bottom.

## Configuration

- `NVIDIA_API_KEY`: API key from NVIDIA API Catalog.
- `NVIDIA_MODEL`: exact model identifier shown by NVIDIA for the model you want to test.
- `NVIDIA_BASE_URL`: defaults to `https://integrate.api.nvidia.com/v1`.

Keep secrets in `.env`. Do not commit real API keys.

## Notes

This is a starter workspace, not a model-training pipeline. Model availability, context limits, and supported request parameters depend on the selected NVIDIA endpoint.
