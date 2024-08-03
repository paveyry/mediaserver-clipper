all: frontend backend

frontend:
	cd ui && trunk build --release --dist ../backend/dist && cd ..

backend:
	cd backend && cargo build --release && cd ..

clean:
	rm -rf backend/dist
	cargo clean