CREATE TABLE IF NOT EXISTS fcm_messages ();

ALTER TABLE fcm_messages
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN title VARCHAR NOT NULL,
  ADD COLUMN body VARCHAR NOT NULL,
  ADD COLUMN created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP;

SELECT diesel_manage_updated_at('fcm_messages');
INSERT INTO fcm_messages (title, body) VALUES
  ('Bienvenido', 'Body bienvenida a la app.'),
  ('Nuevo contenido', 'Notificación nuevo contenido.'),
  ('Recuerda', 'Completa el contenido para continuar.')
  ;
