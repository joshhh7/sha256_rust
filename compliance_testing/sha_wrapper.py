"""Wrapper template for SHA implementations.

Full API documentation at:
https://quarkslab.github.io/crypto-condor/latest/wrapper-api/SHA.html

Usage:
    crypto-condor-cli test wrapper SHA sha_wrapper.py
"""
import subprocess

def CC_SHA_256_digest(data: bytes) -> bytes:
    """Wrapper function for a SHA-256 implementation.

    Args:
        data: The input data.

    Returns:
        The digest of the data.
    """

    data_hex = data.hex()
    process = subprocess.run(["./target/release/sha256"], input=data_hex,capture_output=True,check=True, text=True)
    # text=True pour que ça envoie/reçoit des str et pas des bytes à/de l'exécutable

    result_hex = process.stdout.strip()
    # .strip() enlève les espaces et sauts de ligne

    return bytes.fromhex(result_hex)