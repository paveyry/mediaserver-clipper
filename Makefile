all: frontend backend

frontend:
	cd ui && trunk build --release --dist ../backend/dist

backend:
	cd backend && cargo build --release

clean:
	rm -rf backend/dist
	cargo clean