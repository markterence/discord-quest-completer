import { Component, createApp } from "vue";
import { Router } from "vue-router";

export interface DQCOptions {
    router: Router,
    modules?: string[]
}

export default function DQC(app: Component, options: DQCOptions) {
    const vueApp = createApp(app);

    vueApp.use(options.router);

    if (options?.modules) {
        const moduleGlob = import.meta.glob<{ install: any }>('./modules/*.ts', { eager: true });
        options.modules.forEach(moduleName => {
            const modulePath = `./modules/${moduleName}.ts`;
            const module = moduleGlob[modulePath];
            if (module && module.install) {
                module.install({ 
                    app: vueApp,
                });
            } else {
                console.error(`Module ${moduleName} not found or does not have an install method.`);
            }
        });
    }

    vueApp.mount("#app");

    return vueApp;
}