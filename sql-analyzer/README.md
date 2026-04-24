# SQL Analyzer

A CLI tool that generates SQL queries from natural language using Claude AI.

## Features

- 🗣️ Natural language to SQL conversion
- 📊 Support for custom database schemas
- 💬 Interactive mode for multiple queries
- 🎯 Single query mode for scripting

## Prerequisites

- Rust (for building)
- Claude CLI tool installed and configured ([claude-ai/claude-cli](https://github.com/anthropics/anthropic-cli))

## Installation

```bash
cd sql-analyzer
cargo build --release
```

The binary will be available at `target/release/sql-analyzer`

## Usage

### Interactive Mode (Default)

```bash
cargo run
# or
cargo run -- interactive
```

In interactive mode:
1. First, provide your database schema (tables and columns)
2. Then ask questions in natural language
3. Get SQL queries as output

Example:
```
> Show me all sales per city
> Get the top 10 customers by total purchase amount
> List products that are out of stock
```

### Single Query Mode

```bash
cargo run -- query -q "Show me all sales per city" -s schema.txt
# or without schema file (will prompt for schema)
cargo run -- query -q "Show me all sales per city"
```

### Schema File Format

Create a text file describing your database schema:

```
Tables:
- customers (id, name, email, city, country, created_at)
- products (id, name, description, price, stock_quantity, category)
- orders (id, customer_id, order_date, total_amount, status)
- order_items (id, order_id, product_id, quantity, unit_price)
```

You can also use SQL DDL format:

```sql
CREATE TABLE customers (
    id INT PRIMARY KEY,
    name VARCHAR(100),
    email VARCHAR(100),
    city VARCHAR(50),
    country VARCHAR(50),
    created_at TIMESTAMP
);

CREATE TABLE products (
    id INT PRIMARY KEY,
    name VARCHAR(100),
    description TEXT,
    price DECIMAL(10,2),
    stock_quantity INT,
    category VARCHAR(50)
);
-- ... more tables
```

## Example Session

```
🔍 SQL Query Generator - Interactive Mode
===========================================

Please provide your database schema information.
You can either:
  1. Describe tables manually
  2. Paste a schema definition

Enter your schema (press Ctrl+D or type 'END' on a new line when done):

Tables:
- customers (id, name, email, city)
- orders (id, customer_id, total_amount, order_date)
END

✓ Schema loaded successfully!

You can now ask questions in natural language.

You: Show me all sales per city

Generating SQL...

SQL Query:
SELECT 
    c.city,
    SUM(o.total_amount) as total_sales,
    COUNT(o.id) as order_count
FROM customers c
JOIN orders o ON c.id = o.customer_id
GROUP BY c.city
ORDER BY total_sales DESC;

You: exit

👋 Goodbye!
```

## How It Works

1. Takes your database schema as input
2. Constructs a prompt combining the schema with your natural language question
3. Calls Claude CLI to generate the SQL query
4. Formats and displays the result

## Tips

- Be specific in your questions for better results
- Include important constraints (e.g., "in the last 30 days", "top 10", "where status is active")
- The more detailed your schema, the better the SQL generation
- Add foreign key relationships in your schema description for join queries

## License

MIT
