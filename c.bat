@set d=%cd%\
@set v=%d%vite-app\
@set v_src=%v%src\
@set y=%d%yew-app\
@set builddir=build\
@set outdir=%builddir%outdir\

@if "%1" == "" ( @start && @exit )
:: Vite-app dizinine gider.
@if "%1" == "v" ( @cd %v% )
:: Yew-app dizinine gider.
@if "%1" == "y" ( @cd %y% )
:: Vite-app dizininde ki kodları temizler.
@if "%1" == "vf" ( @deno fmt %v_src% && @up.bat )
:: Yew-app dizininde ki kodları temizler.
@if "%1" == "yf" ( @cd %y% && @cargo clippy --fix --allow-no-vcs && @up.bat )
:: Yew projesini Wasm-pack ile derler.
@if "%1" == "wpb" ( @wasm-pack build %y% --target web )
:: Vite projesini dev olarak derler.
@if "%1" == "vd" ( @cd %v% && @npm run dev && @up.bat )
:: Vite projesini build olarak derler.
@if "%1" == "vb" ( @cd %v% && @npm run build && @up.bat )
:: Çıktı klasörünü temizler.
@if "%1" == "dl" ( @if exist %builddir% ( @rmdir /s /q %builddir% ) )
:: Dev modunda projeyi çalıştırır.
@if "%1" == "dev" ( @dev.bat )
:: Build modunda projeyi çalıştırır.
@if "%1" == "build" ( @build.bat )

