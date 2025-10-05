use crate::opts::text::TextSignFormat;
use crate::{TextSubCommand, get_reader};
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::fs;
use std::io::Read;
use tracing::log;

pub fn process_text(sub_command: &TextSubCommand) -> anyhow::Result<()> {
    match sub_command {
        TextSubCommand::Sign(opts) => {
            process_text_sign(&opts.input, &opts.key, opts.format)?;
        }
        TextSubCommand::Verify(opts) => {
            log::info!("{:?}", opts);
        }
    }
    Ok(())
}

fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)
        }
        TextSignFormat::Ed25519 => {
            todo!()
        }
    }?;

    let signed = BASE64_URL_SAFE_NO_PAD.encode(&signed);

    println!("{}", signed);

    Ok(())
}

fn process_verify(input: &str, data: &str) -> anyhow::Result<()> {
    todo!()
}

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool>;
}

trait KeyLoader {
    fn load(key: &str) -> anyhow::Result<Self>
    where
        Self: Sized;
}

struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3 { key };
        Ok(signer)
    }
}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let binding = blake3::hash(&buf);
        let hash = binding.as_bytes();
        Ok(hash == sig)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key[..32])
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key[..32])
    }
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = SigningKey::try_from(key)?;
        Ok(Self { key })
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}
impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: &str) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Ed25519Verifier { key }
    }

    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = VerifyingKey::try_from(key)?;
        Ok(Self { key })
    }
}
