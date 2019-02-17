void async function main() {
    const wasm = await import("./pkg");
    try {
        wasm.run_test()
        console.error("no longer broken")
    } catch (e) {
        console.info("generated code is broken");
    }
}()
