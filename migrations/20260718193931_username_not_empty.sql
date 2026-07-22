alter table users add constraint users_username_not_empty check (length(trim(username)) > 0);
