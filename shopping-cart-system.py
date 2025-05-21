import psycopg2

# Connect to your PostgreSQL DB
conn = psycopg2.connect(
    dbname="postgres",
    user="postgres",
    password="12345678",
    host="localhost",
    port="5432"
)
cursor = conn.cursor()

# Cart structure: { product_id: quantity }
cart = {}

# Fetch product catalog from database
def get_products():
    cursor.execute("SELECT id, name, price FROM shopping_catalog;")
    rows = cursor.fetchall()
    products = [{"id": row[0], "name": row[1], "price": row[2]} for row in rows]
    return products

# Display product catalog
def show_products():
    products = get_products()
    print("\n=== Product Catalog ===")
    print(f"{'ID':<5} {'Name':<15} {'Price (₹)':>10}")
    print("-" * 32)
    for item in products:
        print(f"{item['id']:<5} {item['name']:<15} {item['price']:>10}")
    print("-" * 32)

# Display cart contents
def view_cart():
    if not cart:
        print("\nYour cart is empty.")
        return

    products = get_products()
    print("\n=== Your Cart ===")
    print(f"{'Name':<15} {'Qty':<5} {'Price':<8} {'Subtotal':<10}")
    print("-" * 42)
    total = 0
    for product_id, quantity in cart.items():
        product = next((p for p in products if p['id'] == product_id), None)
        if product:
            subtotal = product['price'] * quantity
            total += subtotal
            print(f"{product['name']:<15} {quantity:<5} {product['price']:<8} {subtotal:<10}")
    print("-" * 42)
    print(f"{'Total':>35}: ₹{total}")
    print("-" * 42)

# Add item to cart
def add_to_cart():
    products = get_products()
    show_products()
    try:
        product_id = int(input("Enter product ID to add: "))
        if not any(p['id'] == product_id for p in products):
            print("Invalid product ID.")
            return
        quantity = int(input("Enter quantity: "))
        if quantity <= 0:
            print("Quantity must be positive.")
            return
        if product_id in cart:
            cart[product_id] += quantity
        else:
            cart[product_id] = quantity
        product = next(p for p in products if p['id'] == product_id)
        print(f"Added {quantity} x {product['name']} to cart.")
    except ValueError:
        print("Please enter valid numbers.")

# Remove item from cart
def remove_from_cart():
    if not cart:
        print("\nYour cart is empty.")
        return
    view_cart()
    try:
        product_id = int(input("Enter product ID to remove: "))
        if product_id in cart:
            del cart[product_id]
            print("Item removed from cart.")
        else:
            print("Item not found in cart.")
    except ValueError:
        print("Invalid input.")

# Checkout
def checkout():
    if not cart:
        print("\nYour cart is empty.")
        return
    view_cart()
    confirm = input("Proceed to checkout? (yes/no): ").strip().lower()
    if confirm == "yes":
        print("Thank you for your purchase!")
        cart.clear()
    else:
        print("Checkout cancelled.")

# Main loop
def main():
    while True:
        print("\n===== Shopping Cart Menu =====")
        print("1. Show Product Catalog")
        print("2. Add Item to Cart")
        print("3. View Cart")
        print("4. Remove Item from Cart")
        print("5. Checkout")
        print("6. Exit")
        choice = input("Choose an option (1-6): ").strip()

        if choice == "1":
            show_products()
        elif choice == "2":
            add_to_cart()
        elif choice == "3":
            view_cart()
        elif choice == "4":
            remove_from_cart()
        elif choice == "5":
            checkout()
        elif choice == "6":
            print("Goodbye!")
            break
        else:
            print("Invalid option. Try again.")

# Run the app
main()

# Close the DB connection on exit
cursor.close()
conn.close()
