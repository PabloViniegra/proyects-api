-- Migration: Seed test data
-- This migration populates the database with test data for development and testing purposes
-- To skip this migration in production, you can delete this file before deployment

-- Insert Technologies
INSERT INTO technologies (id, name, description, created_at) VALUES
  ('550e8400-e29b-41d4-a716-446655440001', 'Rust', 'Systems programming language focused on safety, speed, and concurrency', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440002', 'Python', 'High-level programming language known for simplicity and versatility', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440003', 'JavaScript', 'Programming language for web development and beyond', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440004', 'TypeScript', 'Typed superset of JavaScript for large-scale applications', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440005', 'Go', 'Statically typed, compiled language designed at Google', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440006', 'Axum', 'Ergonomic and modular web framework for Rust', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440007', 'SQLx', 'Async SQL toolkit for Rust with compile-time checked queries', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440008', 'React', 'JavaScript library for building user interfaces', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440009', 'Next.js', 'React framework for production-grade applications', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440010', 'PostgreSQL', 'Advanced open source relational database', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440011', 'SQLite', 'Lightweight, serverless SQL database engine', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440012', 'Docker', 'Platform for developing, shipping, and running applications in containers', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440013', 'Kubernetes', 'Container orchestration platform for automating deployment', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440014', 'Redis', 'In-memory data structure store used as database and cache', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440015', 'GraphQL', 'Query language for APIs and runtime for executing queries', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440016', 'Tokio', 'Asynchronous runtime for Rust', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440017', 'FastAPI', 'Modern, fast web framework for building APIs with Python', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440018', 'Django', 'High-level Python web framework', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440019', 'Node.js', 'JavaScript runtime built on Chrome V8 engine', datetime('now')),
  ('550e8400-e29b-41d4-a716-446655440020', 'Express', 'Minimal and flexible Node.js web application framework', datetime('now'));

-- Insert Users
INSERT INTO users (id, name, email, created_at) VALUES
  ('650e8400-e29b-41d4-a716-446655440001', 'Alice Johnson', 'alice.johnson@example.com', datetime('now', '-180 days')),
  ('650e8400-e29b-41d4-a716-446655440002', 'Bob Smith', 'bob.smith@example.com', datetime('now', '-150 days')),
  ('650e8400-e29b-41d4-a716-446655440003', 'Charlie Brown', 'charlie.brown@example.com', datetime('now', '-120 days')),
  ('650e8400-e29b-41d4-a716-446655440004', 'Diana Prince', 'diana.prince@example.com', datetime('now', '-90 days')),
  ('650e8400-e29b-41d4-a716-446655440005', 'Eve Martinez', 'eve.martinez@example.com', datetime('now', '-60 days')),
  ('650e8400-e29b-41d4-a716-446655440006', 'Frank Zhang', 'frank.zhang@example.com', datetime('now', '-45 days')),
  ('650e8400-e29b-41d4-a716-446655440007', 'Grace Lee', 'grace.lee@example.com', datetime('now', '-30 days')),
  ('650e8400-e29b-41d4-a716-446655440008', 'Henry Wilson', 'henry.wilson@example.com', datetime('now', '-15 days'));

-- Insert Projects
INSERT INTO projects (id, name, description, repository_url, language, rating, created_at, updated_at) VALUES
  (
    '750e8400-e29b-41d4-a716-446655440001',
    'Rust Web API Starter',
    'A production-ready starter template for building REST APIs with Rust, Axum, and SQLx. Includes authentication, database migrations, OpenAPI documentation, and comprehensive testing.',
    'https://github.com/example/rust-web-api-starter',
    'Rust',
    4.8,
    datetime('now', '-60 days'),
    datetime('now', '-5 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440002',
    'E-commerce Platform',
    'Full-stack e-commerce platform with React frontend and Python FastAPI backend. Features include product catalog, shopping cart, payment integration, and admin dashboard.',
    'https://github.com/example/ecommerce-platform',
    'Python',
    4.5,
    datetime('now', '-90 days'),
    datetime('now', '-10 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440003',
    'Task Management System',
    'Collaborative task management and project tracking system built with Next.js and PostgreSQL. Supports teams, workflows, and real-time updates.',
    'https://github.com/example/task-manager',
    'TypeScript',
    4.7,
    datetime('now', '-45 days'),
    datetime('now', '-2 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440004',
    'Microservices Template',
    'Production-ready microservices architecture template using Go, Docker, and Kubernetes. Includes service discovery, API gateway, and observability stack.',
    'https://github.com/example/microservices-template',
    'Go',
    4.9,
    datetime('now', '-120 days'),
    datetime('now', '-20 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440005',
    'Real-time Chat Application',
    'WebSocket-based real-time chat application with React frontend and Node.js backend. Features include private messages, group chats, and file sharing.',
    'https://github.com/example/realtime-chat',
    'JavaScript',
    4.3,
    datetime('now', '-75 days'),
    datetime('now', '-8 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440006',
    'Machine Learning Pipeline',
    'End-to-end ML pipeline for training, evaluating, and deploying models. Built with Python, includes data preprocessing, model versioning, and API serving.',
    'https://github.com/example/ml-pipeline',
    'Python',
    4.6,
    datetime('now', '-100 days'),
    datetime('now', '-15 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440007',
    'GraphQL API Server',
    'Flexible GraphQL API server with TypeScript, Apollo Server, and PostgreSQL. Includes subscriptions, dataloaders, and comprehensive schema design.',
    'https://github.com/example/graphql-server',
    'TypeScript',
    4.4,
    datetime('now', '-55 days'),
    datetime('now', '-7 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440008',
    'IoT Data Collector',
    'High-performance IoT data collection and processing system built with Rust. Handles millions of events per second with low latency.',
    'https://github.com/example/iot-collector',
    'Rust',
    4.9,
    datetime('now', '-80 days'),
    datetime('now', '-3 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440009',
    'Content Management System',
    'Headless CMS with Django backend and React admin interface. Features include custom content types, media management, and RESTful API.',
    'https://github.com/example/headless-cms',
    'Python',
    4.2,
    datetime('now', '-110 days'),
    datetime('now', '-12 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440010',
    'Mobile Backend Service',
    'Backend-as-a-Service for mobile apps with authentication, push notifications, and cloud storage. Built with Node.js and Redis.',
    'https://github.com/example/mobile-backend',
    'JavaScript',
    4.5,
    datetime('now', '-65 days'),
    datetime('now', '-6 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440011',
    'Analytics Dashboard',
    'Real-time analytics dashboard with data visualization and reporting. Uses Next.js, PostgreSQL, and time-series optimization.',
    'https://github.com/example/analytics-dashboard',
    'TypeScript',
    NULL,
    datetime('now', '-30 days'),
    datetime('now', '-1 days')
  ),
  (
    '750e8400-e29b-41d4-a716-446655440012',
    'API Gateway',
    'High-performance API gateway with rate limiting, authentication, and load balancing. Written in Rust for maximum throughput.',
    'https://github.com/example/api-gateway',
    'Rust',
    NULL,
    datetime('now', '-25 days'),
    datetime('now', '-4 days')
  );

-- Insert Project-Technology relationships
INSERT INTO project_technologies (project_id, technology_id, created_at) VALUES
  -- Rust Web API Starter (Rust, Axum, SQLx, SQLite, Tokio)
  ('750e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440001', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440006', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440007', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440011', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440016', datetime('now')),

  -- E-commerce Platform (Python, FastAPI, React, PostgreSQL, Redis, Docker)
  ('750e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440002', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440017', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440008', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440010', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440014', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440012', datetime('now')),

  -- Task Management System (TypeScript, Next.js, PostgreSQL, React)
  ('750e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440004', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440009', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440010', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440008', datetime('now')),

  -- Microservices Template (Go, Docker, Kubernetes)
  ('750e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440005', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440012', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440013', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440010', datetime('now')),

  -- Real-time Chat (JavaScript, Node.js, Express, React, Redis)
  ('750e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440003', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440019', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440020', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440008', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440014', datetime('now')),

  -- ML Pipeline (Python, Docker)
  ('750e8400-e29b-41d4-a716-446655440006', '550e8400-e29b-41d4-a716-446655440002', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440006', '550e8400-e29b-41d4-a716-446655440012', datetime('now')),

  -- GraphQL Server (TypeScript, GraphQL, PostgreSQL, Node.js)
  ('750e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440004', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440015', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440010', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440019', datetime('now')),

  -- IoT Data Collector (Rust, Tokio, Redis, Docker)
  ('750e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440001', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440016', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440014', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440012', datetime('now')),

  -- CMS (Python, Django, React, PostgreSQL)
  ('750e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440002', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440018', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440008', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440010', datetime('now')),

  -- Mobile Backend (JavaScript, Node.js, Redis)
  ('750e8400-e29b-41d4-a716-446655440010', '550e8400-e29b-41d4-a716-446655440003', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440010', '550e8400-e29b-41d4-a716-446655440019', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440010', '550e8400-e29b-41d4-a716-446655440014', datetime('now')),

  -- Analytics Dashboard (TypeScript, Next.js, PostgreSQL)
  ('750e8400-e29b-41d4-a716-446655440011', '550e8400-e29b-41d4-a716-446655440004', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440011', '550e8400-e29b-41d4-a716-446655440009', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440011', '550e8400-e29b-41d4-a716-446655440010', datetime('now')),

  -- API Gateway (Rust, Tokio, Redis)
  ('750e8400-e29b-41d4-a716-446655440012', '550e8400-e29b-41d4-a716-446655440001', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440012', '550e8400-e29b-41d4-a716-446655440016', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440012', '550e8400-e29b-41d4-a716-446655440014', datetime('now'));

-- Insert Project-User relationships with roles
INSERT INTO project_users (project_id, user_id, role, created_at) VALUES
  -- Rust Web API Starter: Alice (owner), Bob (contributor), Charlie (viewer)
  ('750e8400-e29b-41d4-a716-446655440001', '650e8400-e29b-41d4-a716-446655440001', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440001', '650e8400-e29b-41d4-a716-446655440002', 'contributor', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440001', '650e8400-e29b-41d4-a716-446655440003', 'viewer', datetime('now')),

  -- E-commerce Platform: Bob (owner), Diana (contributor), Eve (contributor)
  ('750e8400-e29b-41d4-a716-446655440002', '650e8400-e29b-41d4-a716-446655440002', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440002', '650e8400-e29b-41d4-a716-446655440004', 'contributor', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440002', '650e8400-e29b-41d4-a716-446655440005', 'contributor', datetime('now')),

  -- Task Management: Charlie (owner), Alice (contributor)
  ('750e8400-e29b-41d4-a716-446655440003', '650e8400-e29b-41d4-a716-446655440003', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440003', '650e8400-e29b-41d4-a716-446655440001', 'contributor', datetime('now')),

  -- Microservices Template: Diana (owner), Frank (contributor), Grace (contributor), Henry (viewer)
  ('750e8400-e29b-41d4-a716-446655440004', '650e8400-e29b-41d4-a716-446655440004', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440004', '650e8400-e29b-41d4-a716-446655440006', 'contributor', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440004', '650e8400-e29b-41d4-a716-446655440007', 'contributor', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440004', '650e8400-e29b-41d4-a716-446655440008', 'viewer', datetime('now')),

  -- Real-time Chat: Eve (owner), Bob (contributor)
  ('750e8400-e29b-41d4-a716-446655440005', '650e8400-e29b-41d4-a716-446655440005', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440005', '650e8400-e29b-41d4-a716-446655440002', 'contributor', datetime('now')),

  -- ML Pipeline: Frank (owner), Diana (contributor)
  ('750e8400-e29b-41d4-a716-446655440006', '650e8400-e29b-41d4-a716-446655440006', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440006', '650e8400-e29b-41d4-a716-446655440004', 'contributor', datetime('now')),

  -- GraphQL Server: Grace (owner), Charlie (contributor), Eve (viewer)
  ('750e8400-e29b-41d4-a716-446655440007', '650e8400-e29b-41d4-a716-446655440007', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440007', '650e8400-e29b-41d4-a716-446655440003', 'contributor', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440007', '650e8400-e29b-41d4-a716-446655440005', 'viewer', datetime('now')),

  -- IoT Collector: Henry (owner), Alice (contributor), Frank (contributor)
  ('750e8400-e29b-41d4-a716-446655440008', '650e8400-e29b-41d4-a716-446655440008', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440008', '650e8400-e29b-41d4-a716-446655440001', 'contributor', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440008', '650e8400-e29b-41d4-a716-446655440006', 'contributor', datetime('now')),

  -- CMS: Alice (owner), Grace (contributor)
  ('750e8400-e29b-41d4-a716-446655440009', '650e8400-e29b-41d4-a716-446655440001', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440009', '650e8400-e29b-41d4-a716-446655440007', 'contributor', datetime('now')),

  -- Mobile Backend: Bob (owner), Henry (contributor)
  ('750e8400-e29b-41d4-a716-446655440010', '650e8400-e29b-41d4-a716-446655440002', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440010', '650e8400-e29b-41d4-a716-446655440008', 'contributor', datetime('now')),

  -- Analytics Dashboard: Charlie (owner)
  ('750e8400-e29b-41d4-a716-446655440011', '650e8400-e29b-41d4-a716-446655440003', 'owner', datetime('now')),

  -- API Gateway: Diana (owner), Alice (contributor)
  ('750e8400-e29b-41d4-a716-446655440012', '650e8400-e29b-41d4-a716-446655440004', 'owner', datetime('now')),
  ('750e8400-e29b-41d4-a716-446655440012', '650e8400-e29b-41d4-a716-446655440001', 'contributor', datetime('now'));
