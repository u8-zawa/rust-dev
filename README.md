# rust-dev

DockerfileでRustの簡易開発環境を構築

## イメージのビルド

```bash
docker build -t rust-dev .
```

## アプリケーションの実行

```bash
docker run --rm rust-dev
```

※デフォルトはhello_world.rs

## 特定のアプリケーションを実行

```bash
docker build -t rust-dev --build-arg BIN_NAME=[ファイル名] .
docker run rust-dev
```
