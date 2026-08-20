import { DialogContext } from "@/components/Dialog.vue";
import { InjectionKey } from "vue";

export const GameActionsKey = Symbol() as InjectionKey<string>;
export const DialogInjectionKey = Symbol() as InjectionKey<DialogContext> 