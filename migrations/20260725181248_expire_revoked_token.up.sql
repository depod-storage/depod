alter table revoked_tokens add column expires_at timestamptz not null;
