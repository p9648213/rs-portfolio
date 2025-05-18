-- SCHEMA

CREATE SCHEMA IF NOT EXISTS realestate

-- ENUM

CREATE TYPE realestate."Highlight" AS ENUM ('HighSpeedInternetAccess', 'WasherDryer', 'AirConditioning', 'Heating', 'SmokeFree', 'CableReady', 'SatelliteTV', 'DoubleVanities', 'TubShower', 'Intercom', 'SprinklerSystem', 'RecentlyRenovated', 'CloseToTransit', 'GreatView', 'QuietNeighborhood');

CREATE TYPE realestate."Amenity" AS ENUM ('WasherDryer', 'AirConditioning', 'Dishwasher', 'HighSpeedInternet', 'HardwoodFloors', 'WalkInClosets', 'Microwave', 'Refrigerator', 'Pool', 'Gym', 'Parking', 'PetsAllowed', 'WiFi');

CREATE TYPE realestate."PropertyType" AS ENUM ('Rooms', 'Tinyhouse', 'Apartment', 'Villa', 'Townhouse', 'Cottage');

CREATE TYPE realestate."ApplicationStatus" AS ENUM ('Pending', 'Denied', 'Approved');

CREATE TYPE realestate."PaymentStatus" AS ENUM ('Pending', 'Paid', 'PartiallyPaid', 'Overdue');

-- TABLE

CREATE TABLE realestate.property (
    id SERIAL NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    price_per_month DOUBLE PRECISION NOT NULL,
    security_deposit DOUBLE PRECISION NOT NULL,
    application_fee DOUBLE PRECISION NOT NULL,
    photo_urls TEXT[],
    amenities realestate."Amenity"[],
    highlights realestate."Highlight"[],
    is_pets_allowed BOOLEAN NOT NULL DEFAULT false,
    is_parking_included BOOLEAN NOT NULL DEFAULT false,
    beds INTEGER NOT NULL,
    baths DOUBLE PRECISION NOT NULL,
    square_feet INTEGER NOT NULL,
    property_type realestate."PropertyType" NOT NULL,
    posted_date TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    average_rating DOUBLE PRECISION DEFAULT 0,
    number_of_reviews INTEGER DEFAULT 0,
    location_id INTEGER NOT NULL,
    manager_id VARCHAR(255) NOT NULL,

    CONSTRAINT "property_pkey" PRIMARY KEY ("id")
);

CREATE TABLE realestate.location (
    id SERIAL NOT NULL,
    address TEXT NOT NULL,
    city VARCHAR(255) NOT NULL,
    state VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL,
    postal_code VARCHAR(255) NOT NULL,
    coordinates geography(Point, 4326) NOT NULL,

    CONSTRAINT "location_pkey" PRIMARY KEY ("id")
);

CREATE TABLE realestate.application (
    id SERIAL NOT NULL,
    application_date TIMESTAMP(3) NOT NULL,
    status realestate."ApplicationStatus" NOT NULL,
    property_id INTEGER NOT NULL,
    tenant_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone_number VARCHAR(255) NOT NULL,
    message TEXT,
    lease_id INTEGER,

    CONSTRAINT "application_pkey" PRIMARY KEY ("id")
);

CREATE TABLE realestate.lease (
    id SERIAL NOT NULL,
    start_date TIMESTAMP(3) NOT NULL,
    end_date TIMESTAMP(3) NOT NULL,
    rent DOUBLE PRECISION NOT NULL,
    deposit DOUBLE PRECISION NOT NULL,
    property_id INTEGER NOT NULL,
    tenant_id VARCHAR(255) NOT NULL,

    CONSTRAINT "lease_pkey" PRIMARY KEY ("id")
);

CREATE TABLE realestate.payment (
    id SERIAL NOT NULL,
    amount_due DOUBLE PRECISION NOT NULL,
    amount_paid DOUBLE PRECISION NOT NULL,
    due_date TIMESTAMP(3) NOT NULL,
    payment_date TIMESTAMP(3) NOT NULL,
    payment_status realestate."PaymentStatus" NOT NULL,
    lease_id INTEGER NOT NULL,

    CONSTRAINT "payment_pkey" PRIMARY KEY ("id")
);

CREATE TABLE realestate.tenant_favorites (
    property_id INTEGER NOT NULL,
    user_id VARCHAR(255) NOT NULL,

    CONSTRAINT "tenant_favorites_AB_pkey" PRIMARY KEY ("property_id", "user_id")
);

CREATE TABLE realestate.tenant_properties (
    property_id INTEGER NOT NULL,
    user_id VARCHAR(255) NOT NULL,

    CONSTRAINT "tenant_properties_AB_pkey" PRIMARY KEY ("property_id","user_id")
);

-- AddForeignKey

ALTER TABLE realestate.property ADD CONSTRAINT "property_location_fkey" FOREIGN KEY ("location_id") REFERENCES realestate.location("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE realestate.property ADD CONSTRAINT "property_user_fkey" FOREIGN KEY ("manager_id") REFERENCES users("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE realestate.application ADD CONSTRAINT "application_property_fkey" FOREIGN KEY ("property_id") REFERENCES realestate.property("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE realestate.application ADD CONSTRAINT "application_user_fkey" FOREIGN KEY ("tenant_id") REFERENCES users("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE realestate.application ADD CONSTRAINT "application_lease_fkey" FOREIGN KEY ("lease_id") REFERENCES realestate.lease("id") ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE realestate.lease ADD CONSTRAINT "lease_property_fkey" FOREIGN KEY ("property_id") REFERENCES realestate.property("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE realestate.lease ADD CONSTRAINT "lease_tenant_fkey" FOREIGN KEY ("tenant_id") REFERENCES users("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE realestate.payment ADD CONSTRAINT "payment_lease_fkey" FOREIGN KEY ("lease_id") REFERENCES realestate.lease("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE realestate.tenant_favorites ADD CONSTRAINT "tenant_favorites_property_fkey" FOREIGN KEY ("property_id") REFERENCES realestate.property("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE realestate.tenant_favorites ADD CONSTRAINT "tenant_favorites_user_fkey" FOREIGN KEY ("user_id") REFERENCES users("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE realestate.tenant_properties ADD CONSTRAINT "tenant_properties_property_fkey" FOREIGN KEY ("property_id") REFERENCES realestate.property("id") ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE realestate.tenant_properties ADD CONSTRAINT "tenant_properties_user_fkey" FOREIGN KEY ("user_id") REFERENCES users("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- SAMPLE DATA
INSERT INTO realestate.location (id, address, city, state, country, postal_code, coordinates)
VALUES
  (1, '123 Colorado Blvd', 'Pasadena', 'CA', 'United States', '91105', ST_GeogFromText('SRID=4326;POINT(-118.144516 34.147785)')),
  (2, '456 Ocean Ave', 'Santa Monica', 'CA', 'United States', '90401', ST_GeogFromText('SRID=4326;POINT(-118.496513 34.013654)')),
  (3, '789 Hollywood Way', 'Burbank', 'CA', 'United States', '91505', ST_GeogFromText('SRID=4326;POINT(-118.328661 34.180839)')),
  (4, '101 Pine Ave', 'Long Beach', 'CA', 'United States', '90802', ST_GeogFromText('SRID=4326;POINT(-118.192604 33.766720)')),
  (5, '555 Manhattan Ave', 'New York', 'NY', 'United States', '10001', ST_GeogFromText('SRID=4326;POINT(-74.005941 40.712784)')),
  (6, '888 Malibu Road', 'Malibu', 'CA', 'United States', '90265', ST_GeogFromText('SRID=4326;POINT(-118.774518 34.025922)')),
  (7, '777 Brand Blvd', 'Glendale', 'CA', 'United States', '91203', ST_GeogFromText('SRID=4326;POINT(-118.254708 34.142508)')),
  (8, '555 Torrance Blvd', 'Torrance', 'CA', 'United States', '90503', ST_GeogFromText('SRID=4326;POINT(-118.352575 33.835849)')),
  (9, '1234 Ocean Dr', 'Miami', 'FL', 'United States', '33139', ST_GeogFromText('SRID=4326;POINT(-80.130045 25.782551)')),
  (10, '789 Rodeo Dr', 'Beverly Hills', 'CA', 'United States', '90210', ST_GeogFromText('SRID=4326;POINT(-118.400356 34.073620)'));

INSERT INTO realestate.property (
  id, 
  name, 
  description, 
  price_per_month, 
  security_deposit, 
  application_fee, 
  photo_urls, 
  amenities, 
  highlights, 
  is_pets_allowed, 
  is_parking_included, 
  beds, 
  baths, 
  square_feet, 
  property_type, 
  posted_date, 
  average_rating, 
  number_of_reviews, 
  location_id, 
  manager_id
) VALUES
(
    1,
    'Sunny Downtown Apartment',
    'A beautiful apartment in the heart of downtown with plenty of natural light.',
    1500,
    1500,
    50,
    ARRAY['https://example.com/apartment1_1.jpg', 'https://example.com/apartment1_2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'WasherDryer'::realestate."Amenity", 'Parking'::realestate."Amenity"],
    ARRAY['HighSpeedInternetAccess'::realestate."Highlight", 'CloseToTransit'::realestate."Highlight"],
    true,
    false,
    2,
    1,
    800,
    'Apartment'::realestate."PropertyType",
    '2023-05-15T00:00:00Z',
    4.5,
    10,
    1,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    2,
    'Cozy Beach House',
    'A charming beach house with stunning ocean views.',
    2000,
    2000,
    75,
    ARRAY['https://example.com/beachhouse1.jpg', 'https://example.com/beachhouse2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'WasherDryer'::realestate."Amenity", 'Parking'::realestate."Amenity", 'Pool'::realestate."Amenity"],
    ARRAY['GreatView'::realestate."Highlight", 'CloseToTransit'::realestate."Highlight"],
    true,
    true,
    3,
    2,
    1200,
    'Villa'::realestate."PropertyType",
    '2023-06-01T00:00:00Z',
    4.8,
    5,
    2,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    3,
    'Modern City Loft',
    'Sleek and stylish loft in the heart of the city.',
    2200,
    2200,
    60,
    ARRAY['https://example.com/cityloft1.jpg', 'https://example.com/cityloft2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'WasherDryer'::realestate."Amenity", 'Gym'::realestate."Amenity"],
    ARRAY['HighSpeedInternetAccess'::realestate."Highlight", 'CloseToTransit'::realestate."Highlight"],
    true,
    false,
    1,
    1,
    900,
    'Apartment'::realestate."PropertyType",
    '2023-07-01T00:00:00Z',
    4.7,
    8,
    3,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    4,
    'Spacious Family Home',
    'Large family home with a beautiful backyard and modern amenities.',
    2500,
    2500,
    80,
    ARRAY['https://example.com/familyhome1.jpg', 'https://example.com/familyhome2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'WasherDryer'::realestate."Amenity", 'Parking'::realestate."Amenity", 'Dishwasher'::realestate."Amenity"],
    ARRAY['QuietNeighborhood'::realestate."Highlight"],
    true,
    false,
    4,
    3,
    2000,
    'Villa'::realestate."PropertyType",
    '2023-06-15T00:00:00Z',
    4.9,
    12,
    4,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    5,
    'Luxury Penthouse',
    'Stunning penthouse with panoramic city views and high-end finishes.',
    5000,
    5000,
    100,
    ARRAY['https://example.com/penthouse1.jpg', 'https://example.com/penthouse2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'WasherDryer'::realestate."Amenity", 'Gym'::realestate."Amenity", 'Pool'::realestate."Amenity"],
    ARRAY['GreatView'::realestate."Highlight"],
    true,
    false,
    3,
    3,
    2500,
    'Apartment'::realestate."PropertyType",
    '2023-07-01T00:00:00Z',
    5,
    15,
    5,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    6,
    'Cozy Studio Apartment',
    'Efficient studio apartment perfect for students or young professionals.',
    1200,
    1200,
    40,
    ARRAY['https://example.com/studio1.jpg', 'https://example.com/studio2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'HighSpeedInternet'::realestate."Amenity"],
    ARRAY['CloseToTransit'::realestate."Highlight"],
    true,
    false,
    0,
    1,
    400,
    'Apartment'::realestate."PropertyType",
    '2023-08-01T00:00:00Z',
    4.2,
    6,
    6,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    7,
    'Historic Brownstone',
    'Charming brownstone with original features and modern updates.',
    3000,
    3000,
    70,
    ARRAY['https://example.com/brownstone1.jpg', 'https://example.com/brownstone2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'WasherDryer'::realestate."Amenity", 'HighSpeedInternet'::realestate."Amenity"],
    ARRAY['RecentlyRenovated'::realestate."Highlight"],
    true,
    false,
    3,
    2,
    1800,
    'Townhouse'::realestate."PropertyType",
    '2023-09-01T00:00:00Z',
    4.6,
    9,
    7,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    8,
    'Urban Micro-Apartment',
    'Compact and efficient living space in the heart of the city.',
    1000,
    1000,
    30,
    ARRAY['https://example.com/micro1.jpg', 'https://example.com/micro2.jpg'],
    ARRAY['AirConditioning'::realestate."Amenity", 'HighSpeedInternet'::realestate."Amenity"],
    ARRAY['CloseToTransit'::realestate."Highlight"],
    true,
    false,
    0,
    1,
    300,
    'Apartment'::realestate."PropertyType",
    '2023-08-01T00:00:00Z',
    4.3,
    7,
    8,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    9,
    'Mountain View Cabin',
    'Rustic cabin with breathtaking mountain views and modern amenities.',
    1800,
    1800,
    60,
    ARRAY['https://example.com/cabin1.jpg', 'https://example.com/cabin2.jpg'],
    ARRAY['WasherDryer'::realestate."Amenity", 'AirConditioning'::realestate."Amenity"],
    ARRAY['GreatView'::realestate."Highlight", 'QuietNeighborhood'::realestate."Highlight"],
    true,
    false,
    2,
    1,
    1000,
    'Cottage'::realestate."PropertyType",
    '2023-08-15T00:00:00Z',
    4.9,
    11,
    9,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  ),
(
    10,
    'Eco-Friendly Tiny House',
    'Sustainable living in a compact, well-designed tiny house.',
    900,
    900,
    35,
    ARRAY['https://example.com/tinyhouse1.jpg', 'https://example.com/tinyhouse2.jpg'],
    ARRAY['WasherDryer'::realestate."Amenity", 'AirConditioning'::realestate."Amenity", 'HighSpeedInternet'::realestate."Amenity"],
    ARRAY['SmokeFree'::realestate."Highlight"],
    true,
    false,
    1,
    1,
    250,
    'Tinyhouse'::realestate."PropertyType",
    '2023-08-10T00:00:00Z',
    4.7,
    8,
    10,
    '7ff4109f-de50-490f-989e-db3518368c3e'
  );

INSERT INTO realestate.lease (id, start_date, end_date, rent, deposit, property_id, tenant_id)
VALUES
    (1, '2023-07-01 00:00:00', '2024-06-30 00:00:00', 1500.0, 1500.0, 1, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (2, '2023-08-01 00:00:00', '2024-07-31 00:00:00', 1800.0, 1800.0, 2, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (3, '2023-09-01 00:00:00', '2024-08-31 00:00:00', 2200.0, 2200.0, 3, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (4, '2023-07-15 00:00:00', '2024-07-14 00:00:00', 1700.0, 1700.0, 4, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (5, '2023-08-15 00:00:00', '2024-08-14 00:00:00', 2000.0, 2000.0, 5, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (6, '2023-09-15 00:00:00', '2024-09-14 00:00:00', 2400.0, 2400.0, 6, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (7, '2023-10-01 00:00:00', '2024-09-30 00:00:00', 2200.0, 2200.0, 3, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (8, '2023-08-01 00:00:00', '2024-07-31 00:00:00', 5000.0, 5000.0, 5, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (9, '2023-11-01 00:00:00', '2024-10-31 00:00:00', 3000.0, 3000.0, 7, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (10, '2023-08-15 00:00:00', '2024-02-14 00:00:00', 2000.0, 2000.0, 2, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (11, '2023-09-01 00:00:00', '2024-08-31 00:00:00', 1000.0, 1000.0, 8, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (12, '2023-10-01 00:00:00', '2023-12-31 00:00:00', 1800.0, 1800.0, 9, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (13, '2023-09-15 00:00:00', '2024-03-14 00:00:00', 900.0, 900.0, 10, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (14, '2023-09-01 00:00:00', '2024-08-31 00:00:00', 1500.0, 1500.0, 1, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5'),
    (15, '2023-10-01 00:00:00', '2024-09-30 00:00:00', 2500.0, 2500.0, 4, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5');  

INSERT INTO realestate.application (
  id, 
  application_date, 
  status, 
  property_id, 
  tenant_id, 
  name, 
  email, 
  phone_number, 
  message, 
  lease_id
) VALUES 
  (1, '2023-05-20T00:00:00Z', 'Approved', 1, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Alice Brown', 'alice.brown@example.com', '+1 (555) 111-2222', 'I am interested in this property.', 1),
  (2, '2023-05-25T00:00:00Z', 'Pending', 2, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Bob Green', 'bob.green@example.com', '+1 (555) 333-4444', 'Looking forward to viewing this apartment.', NULL),
  (3, '2023-06-01T00:00:00Z', 'Denied', 3, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Carol White', 'carol.white@example.com', '+1 (555) 555-6666', 'Excited about the possibility of renting this house.', NULL),
  (4, '2023-06-10T00:00:00Z', 'Approved', 4, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'David Lee', 'david.lee@example.com', '+1 (555) 777-8888', 'This property looks perfect for my needs.', 4),
  (5, '2023-06-15T00:00:00Z', 'Pending', 5, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Emma Taylor', 'emma.taylor@example.com', '+1 (555) 999-0000', 'I''m very interested in this apartment.', NULL),
  (6, '2023-06-20T00:00:00Z', 'Approved', 6, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Frank Wilson', 'frank.wilson@example.com', '+1 (555) 222-3333', 'This property seems to fit all my requirements.', 6),
  (7, '2023-06-25T00:00:00Z', 'Pending', 3, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Grace Miller', 'grace.miller@example.com', '+1 (555) 444-5555', 'I''m interested in this loft and would like to schedule a viewing.', NULL),
  (8, '2023-07-01T00:00:00Z', 'Approved', 5, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Henry Wilson', 'henry.wilson@example.com', '+1 (555) 444-5555', 'I''m very interested in this luxury penthouse.', 8),
  (9, '2023-07-05T00:00:00Z', 'Pending', 7, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Isabella Garcia', 'isabella.garcia@example.com', '+1 (555) 666-7777', 'The historic brownstone looks perfect for my family.', NULL),
  (10, '2023-07-10T00:00:00Z', 'Denied', 2, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Jack Thompson', 'jack.thompson@example.com', '+1 (555) 888-9999', 'I''d love to rent this beach house for the summer.', NULL),
  (11, '2023-07-15T00:00:00Z', 'Pending', 8, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Karen Martinez', 'karen.martinez@example.com', '+1 (555) 123-4567', 'I''m interested in this micro-apartment for its central location.', NULL),
  (12, '2023-07-20T00:00:00Z', 'Approved', 9, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Liam Johnson', 'liam.johnson@example.com', '+1 (555) 987-6543', 'The mountain view cabin looks perfect for a weekend getaway.', 12),
  (13, '2023-07-25T00:00:00Z', 'Pending', 10, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Mia Rodriguez', 'mia.rodriguez@example.com', '+1 (555) 246-8135', 'I''m curious about the eco-friendly features of this tiny house.', NULL),
  (14, '2023-07-30T00:00:00Z', 'Approved', 1, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Noah Kim', 'noah.kim@example.com', '+1 (555) 369-2580', 'The downtown apartment seems ideal for my work location.', 14),
  (15, '2023-08-05T00:00:00Z', 'Pending', 4, 'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', 'Olivia Chen', 'olivia.chen@example.com', '+1 (555) 159-7531', 'I''m looking for a spacious family home like this one.', NULL);

INSERT INTO realestate.payment (amount_due, amount_paid, due_date, payment_date, payment_status, lease_id)
VALUES
    (1500.0, 1500.0, '2023-07-01T00:00:00Z', '2023-06-28T00:00:00Z', 'Paid', 1),
    (1800.0, 0.0, '2023-08-01T00:00:00Z', '2023-07-28T00:00:00Z', 'Pending', 2),
    (2200.0, 2200.0, '2023-09-01T00:00:00Z', '2023-08-30T00:00:00Z', 'Paid', 3),
    (1700.0, 1700.0, '2023-07-15T00:00:00Z', '2023-07-14T00:00:00Z', 'Paid', 4),
    (2000.0, 0.0, '2023-08-15T00:00:00Z', '2023-08-14T00:00:00Z', 'Pending', 5),
    (2400.0, 2400.0, '2023-09-15T00:00:00Z', '2023-09-14T00:00:00Z', 'Paid', 6),
    (2200.0, 1100.0, '2023-10-01T00:00:00Z', '2023-09-25T00:00:00Z', 'PartiallyPaid', 7),
    (5000.0, 5000.0, '2023-08-01T00:00:00Z', '2023-07-30T00:00:00Z', 'Paid', 8),
    (3000.0, 1500.0, '2023-11-01T00:00:00Z', '2023-10-25T00:00:00Z', 'PartiallyPaid', 9),
    (2000.0, 2000.0, '2023-08-15T00:00:00Z', '2023-08-10T00:00:00Z', 'Paid', 10),
    (1000.0, 0.0, '2023-09-01T00:00:00Z', '2023-08-30T00:00:00Z', 'Pending', 11),
    (1800.0, 1800.0, '2023-10-01T00:00:00Z', '2023-09-28T00:00:00Z', 'Paid', 12),
    (900.0, 450.0, '2023-09-15T00:00:00Z', '2023-09-10T00:00:00Z', 'PartiallyPaid', 13),
    (1500.0, 1500.0, '2023-09-01T00:00:00Z', '2023-08-30T00:00:00Z', 'Paid', 14),
    (2500.0, 0.0, '2023-10-01T00:00:00Z', '2023-09-28T00:00:00Z', 'Paid', 15);  