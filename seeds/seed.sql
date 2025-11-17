-- Seed data for rust_diesel_api
-- Note: This seeds only items. Users require hashed passwords and should be created via API.

-- Optional: Uncomment to reset items before seeding
-- TRUNCATE TABLE items RESTART IDENTITY CASCADE;

INSERT INTO items (name, description, price, stock) VALUES
  ('Laptop', 'Gaming laptop', 1499.99, 10),
  ('Mouse', 'Wireless mouse', 25.99, 100),
  ('Keyboard', 'Mechanical keyboard', 79.90, 50),
  ('Monitor', '27-inch IPS monitor', 229.00, 30),
  ('USB-C Cable', '1m braided cable', 9.99, 200);
