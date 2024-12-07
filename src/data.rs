use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "LG OLED 4K Smart TV".to_string(),
            price: 1299.99,
            description: "Experience stunning visuals with the LG OLED 4K Smart TV. Featuring self-lit pixels, Dolby Vision, and AI-powered picture and sound, it's the ultimate choice for your entertainment needs.".to_string(),
            image: "/lg-tv.png".to_string()
        },
        Product {
            id: 2,
            name: "Samsung Galaxy S23 Ultra".to_string(),
            price: 1199.99,
            description: "Discover the future of mobile innovation with the Samsung Galaxy S23 Ultra. Boasting a dynamic AMOLED display, cutting-edge camera technology, and exceptional performance, it's built for everything you love.".to_string(),
            image: "/samsung.png".to_string()
        },
        Product {
            id: 3,
            name: "Sony Alpha a7 III Mirrorless Camera".to_string(),
            price: 1999.99,
            description: "Capture life's moments in stunning detail with the Sony Alpha a7 III. This full-frame mirrorless camera offers exceptional image quality, fast autofocus, and 4K video recording.".to_string(),
            image: "/Camera.png".to_string()
        },
        Product {
            id: 4,
            name: "Origin Millennium Gaming PC".to_string(),
            price: 3499.99,
            description: "Unleash the ultimate gaming experience with the Origin Millennium Gaming PC. Equipped with top-tier hardware, customizable RGB lighting, and advanced cooling, it's built for gamers who demand the best.".to_string(),
            image: "/pc.png".to_string()
        },
        Product {
            id: 5,
            name: "HP Omen Gaming Laptop".to_string(),
            price: 1599.99,
            description: "Game on the go with the HP Omen Gaming Laptop. Featuring high-performance graphics, a fast refresh rate display, and powerful internals, it’s perfect for both casual and competitive gamers.".to_string(),
            image: "/laptop.png".to_string()
        },
        Product {
            id: 6,
            name: "Meta Quest 3 VR Headset".to_string(),
            price: 499.99,
            description: "Step into the future of virtual reality with the Meta Quest 3. This all-in-one VR headset offers immersive gaming, social interactions, and entertainment without the need for a PC.".to_string(),
            image: "/vr.png".to_string()
        },
        Product {
            id: 7,
            name: "Ninebot Max Electric Scooter".to_string(),
            price: 799.99,
            description: "Navigate your city with ease using the Ninebot Max Electric Scooter. With a long battery life, robust design, and powerful motor, it’s the perfect companion for urban commuting.".to_string(),
            image: "/scooter.png".to_string()
        },
        Product {
            id: 8,
            name: "Sony WH-1000XM5 Noise-Canceling Headphones".to_string(),
            price: 399.99,
            description: "Immerse yourself in music with the Sony WH-1000XM5 headphones. Featuring industry-leading noise cancellation, crystal-clear sound, and exceptional comfort, they’re perfect for any audiophile.".to_string(),
            image: "/sony.png".to_string()
        },
        Product {
            id: 9,
            name: "Sony PlayStation 5".to_string(),
            price: 499.99,
            description: "Dive into next-gen gaming with the Sony PlayStation 5. With lightning-fast load times, stunning graphics, and an extensive library of exclusive games, it’s the ultimate gaming console.".to_string(),
            image: "/play.png".to_string()
        },
        Product {
            id: 10,
            name: "Microsoft Xbox Series X".to_string(),
            price: 499.99,
            description: "Experience the power of next-gen gaming with the Xbox Series X. Featuring 4K gaming, lightning-fast load times, and access to a massive library of games, it's designed for gamers who want it all.".to_string(),
            image: "/xbox.png".to_string()
        }
        
    ]
}