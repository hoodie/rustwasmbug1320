void async function main() {
    const wasm = await import("./pkg");
    try {
        wasm.run_test()
        console.error("no longer broken")
        document.write("no longer broken")
    } catch (e) {
        console.info("generated code is broken");
        document.write("generated code is broken");
    }
}()
