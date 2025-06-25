
export interface GameExecutable {
  is_launcher: boolean;
  name: string;
  os: string;
  filename?: string;
  path?: string;
  segments?: Number;
  is_running?: boolean;
  is_installed?: boolean;
  /**
   * A virtual process identifier assigned to the game executable.
   */
  vpid?: string;
}
export interface Game {
    uid?: string;
    id: string;
    name: string;
    executables: GameExecutable[];
    aliases?: string[];
    themes?: string[];
    is_running?: boolean;
    is_installed?: boolean;
}

export interface GameActionsProvider {
  canPlayGame: (game: Game | null) => boolean;
  isGameInstalled: (game: Game | null) => boolean;
  isExecutableRunning: (executable: GameExecutable) => boolean;
  isGameExecutableInstalled: (executable: GameExecutable) => boolean;
}